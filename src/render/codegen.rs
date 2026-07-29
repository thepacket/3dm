//! Turns a [`FormulaStack`] into WGSL.
//!
//! The alternative — one shader containing every formula behind a `switch` —
//! would branch inside the innermost loop of the raymarcher, which runs on the
//! order of iterations × steps × DE-taps times per pixel. Generating
//! straight-line code per stack configuration and caching the pipeline keeps
//! that loop branch-free; the cost is a shader rebuild when the stack's
//! *structure* changes, which is rare compared to dragging a slider.

use crate::formulas::generated::{ENUMERATORS, GENERATED};
use crate::formulas::{DeMode, FormulaKind, FormulaStack, POOL_FLOATS_PER_SLOT, mb2};

/// Where the generated distance estimator is spliced into the static shader.
const MARKER: &str = "//__GENERATED_DE__";

const TEMPLATE: &str = include_str!("fractal.wgsl");

/// The pool expression a formula's parameter reads are written against.
const POOL: &str = "dm3_u.pool";

/// This slot's WGSL body, with parameter placeholders resolved to pool reads.
///
/// The two formula sources use different placeholder spellings — `P0`..`P5` for
/// the hand-written ones, `__MB2Pn__` for the transpiled ones — but they are
/// resolved against the same pool and produce the same ABI, so the iteration
/// loop does not care which it got.
fn body(kind: FormulaKind, base: usize) -> String {
    match kind {
        FormulaKind::Builtin(b) => {
            let read = |n: usize| {
                let at = base + n;
                format!("{POOL}[{}].{}", at / 4, ["x", "y", "z", "w"][at % 4])
            };
            // Descending, so `P1` cannot clobber the start of a hypothetical
            // `P1x`, and because `P0`..`P5` are not delimited the way the
            // transpiled placeholders are.
            (0..crate::formulas::MAX_PARAMS)
                .rev()
                .fold(b.wgsl_body().to_owned(), |s, n| {
                    s.replace(&format!("P{n}"), &read(n))
                })
        }
        FormulaKind::Generated(i) => mb2::substitute(&GENERATED[i], POOL, base),
    }
}

/// Builds the complete shader source for `stack`.
pub fn generate(stack: &FormulaStack) -> String {
    let mut functions = String::new();
    let mut dispatch = String::new();

    for (index, slot) in stack.active() {
        functions.push_str(&format!(
            "// slot {index}: {name}\n\
             fn slot_{index}(z_in: vec4<f32>, aux: ptr<function, Aux>) -> vec4<f32> {{\n\
             \x20   var z = z_in;\n{body}}}\n\n",
            name = slot.kind.name(),
            body = body(slot.kind, index * POOL_FLOATS_PER_SLOT),
        ));

        // Mandelbulber's formulas do the `z -> z^n` half of an escape-time
        // iteration and leave the `+ c` to their caller, which is this loop.
        // Whether a given formula wants it is recorded in its C++ definition
        // and recovered by the transpiler; without it an escape-time formula
        // never sees the sampled point and renders as a plain sphere.
        let add_c = if slot.kind.adds_c() {
            " z = z + aux.const_c;"
        } else {
            ""
        };

        // The iteration range is a uniform read, so dragging it does not
        // invalidate this pipeline.
        dispatch.push_str(&format!(
            "        if (aux.i >= dm3_u.ranges[{index}].x && aux.i < dm3_u.ranges[{index}].y) \
             {{ z = slot_{index}(z, &aux);{add_c} }}\n"
        ));
    }

    // Mandelbulber seeds `actualScale` from the scene's Mandelbox scale before
    // the first iteration — not from 1.0. Its default is 2.0, and a formula
    // carrying its own `mandelbox.scale` supplies the value directly. 57
    // formulas read this field, and seeding it wrong changes their shape from
    // the first fold onwards rather than failing outright.
    let actual_scale = stack
        .active()
        .next()
        .and_then(|(index, slot)| {
            let FormulaKind::Generated(g) = slot.kind else {
                return None;
            };
            let f = &GENERATED[g];
            let p = mb2::layout(f)
                .placements
                .into_iter()
                .find(|p| p.param.path == "mandelbox.scale")?;
            let at = index * POOL_FLOATS_PER_SLOT + p.index;
            Some(format!("{POOL}[{}].{}", at / 4, ["x", "y", "z", "w"][at % 4]))
        })
        .unwrap_or_else(|| "2.0".to_owned());

    // An empty stack still has to compile and produce something finite.
    if dispatch.is_empty() {
        dispatch.push_str("        // empty stack — nothing to iterate\n");
    }

    // Mandelbulber treats a stack of formulas differently from a single one,
    // and the difference is the sign of the derivative. A fold can legitimately
    // drive `de` negative; alone that means the point is inside and the
    // distance is zero, but in a stack the magnitude is still the right scale
    // to march by, so the hybrid branch takes the absolute value.
    let hybrid = stack.active().count() > 1;
    let mode = stack.de_mode();
    let divisor = if hybrid {
        "max(abs(it.de), 1e-9)"
    } else {
        "max(it.de, 1e-9)"
    };

    // Ported from Mandelbulber's `compute_fractal.cl`, including the parts that
    // are easy to leave out and wrong to: the logarithmic form is zero inside
    // the unit sphere rather than negative, the IFS form carries a −2 offset,
    // and a formula that computed its own distance is simply believed.
    let distance = match mode {
        DeMode::Log => "select(0.0, 0.5 * safe_r * log(safe_r) / de, safe_r > 1.0)",
        DeMode::Linear => "safe_r / de",
        DeMode::Ifs => "(safe_r - 2.0) / de",
        DeMode::Custom => "it.dist",
        // `rxy` is the radius in the xy plane; the formula folds a reference
        // radius into `pseudo_kleinian_de` as it iterates.
        DeMode::PseudoKleinian => {
            "max(length(it.z.xy) - it.pseudo_kleinian_de, \
             abs(length(it.z.xy) * it.z.z) / safe_r) / de"
        }
        // Handled by its own body below.
        DeMode::Delta | DeMode::DeltaLinear => "0.0",
    };

    // A single formula with a non-positive derivative is inside the surface,
    // which Mandelbulber reports as a distance of zero rather than as a miss.
    let guard = if hybrid {
        format!("max({distance}, 0.0)")
    } else {
        format!("select(0.0, max({distance}, 0.0), it.de > 0.0)")
    };

    // Some formulas maintain no derivative at all, so there is nothing to
    // divide by and the gradient has to be measured instead: iterate the point
    // and six neighbours, and see how much faster the neighbours escape. The
    // neighbours run for exactly the iteration count the centre used and
    // without their own bailout — Mandelbulber's `deltaDEMaxN` — or they would
    // be comparing orbits of different lengths.
    let body = if mode.is_delta() {
        let closed_form = if mode == DeMode::DeltaLinear {
            "0.5 * r / d"
        } else {
            "select(0.0, 0.5 * r * log(r) / d, r > 1.0)"
        };
        format!(
            r#"    let centre = dm3_iterate(pos, max_iter, true);
    let r = length(centre.z);
    let n = max(centre.n - 1, 1);

    // Large enough to survive f32 cancellation, small enough to be local.
    let delta = max(length(pos) * 1e-6, dm3_u.march.w * 1e-4);
    let dx0 = length(dm3_iterate(pos + vec3<f32>(delta, 0.0, 0.0), n, false).z);
    let dx1 = length(dm3_iterate(pos - vec3<f32>(delta, 0.0, 0.0), n, false).z);
    let dy0 = length(dm3_iterate(pos + vec3<f32>(0.0, delta, 0.0), n, false).z);
    let dy1 = length(dm3_iterate(pos - vec3<f32>(0.0, delta, 0.0), n, false).z);
    let dz0 = length(dm3_iterate(pos + vec3<f32>(0.0, 0.0, delta), n, false).z);
    let dz1 = length(dm3_iterate(pos - vec3<f32>(0.0, 0.0, delta), n, false).z);

    // The nearer of each opposed pair: the one that did not step across a
    // discontinuity in the orbit.
    let dr = vec3<f32>(
        min(abs(dx0 - r), abs(dx1 - r)),
        min(abs(dy0 - r), abs(dy1 - r)),
        min(abs(dz0 - r), abs(dz1 - r))) / delta;
    let d = length(dr);

    var out: Field;
    out.dist = select(max({closed_form}, 0.0), 0.0, d == 0.0);
    out.trap = centre.trap;
    out.escape = f32(centre.n) / max(f32(max_iter), 1.0);
    return out;"#
        )
    } else {
        format!(
            r#"    let it = dm3_iterate(pos, max_iter, true);

    var out: Field;
    let safe_r = max(length(it.z), 1e-9);
    let de = {divisor};
    // Mandelbulber clamps the finished estimate at zero. Without it the
    // logarithmic form reports negative distances inside the unit sphere and
    // the marcher walks backwards through the surface.
    out.dist = {guard};
    out.trap = it.trap;
    out.escape = f32(it.n) / max(f32(max_iter), 1.0);
    return out;"#
        )
    };

    let de = format!(
        r#"{ENUMERATORS}
{functions}// One run of the escape-time loop. Split out of `fractal_de` because delta DE
// needs seven of them per distance: the point and six neighbours.
struct Iter {{
    z: vec4<f32>,
    de: f32,
    dist: f32,
    trap: f32,
    n: i32,
    pseudo_kleinian_de: f32,
}};

fn dm3_iterate(pos: vec3<f32>, max_n: i32, bail: bool) -> Iter {{
    let bailout = dm3_u.fractal.z;

    var z = vec4<f32>(pos, 0.0);
    var r = length(pos);
    var trap = 1e20;
    var i = 0;

    // Mandelbulber threads this through every formula. Most fields exist only
    // so that the transpiled bodies compile, but `de` is the running derivative
    // the distance estimate divides by, and `const_c` is the sampled point that
    // escape-time formulas add back each iteration.
    var aux: Aux;
    aux.c = vec4<f32>(pos, 0.0);
    aux.const_c = vec4<f32>(pos, 0.0);
    // Seeded to the starting point, then left alone: Mandelbulber's loop never
    // touches it and the formulas that care manage it themselves.
    aux.old_z = z;
    aux.pos_neg = 1.0;
    aux.r = r;
    aux.de = 1.0;
    aux.de0 = 0.0;
    // Formulas accumulate their own estimate with `min`, so this has to start
    // high. Starting it at zero makes every custom-DE formula report a
    // distance of zero everywhere.
    aux.dist = 1000.0;
    aux.actual_scale = {actual_scale};
    aux.actual_scale_a = 0.0;
    aux.color = 1.0;
    aux.color_hybrid = 0.0;
    aux.temp1000 = 1000.0;
    aux.i = 0.0;
    aux.last_z = z;
    aux.r_dz = 1.0;
    aux.old_r = 0.0;
    aux.pseudo_kleinian_de = 1.0;

    loop {{
        if (i >= max_n) {{ break; }}
        r = length(z);
        // A neighbour run is capped by iteration count alone, so that it
        // measures the same orbit length as the centre.
        if (bail && r > bailout) {{ break; }}
        trap = min(trap, r);

        aux.old_r = aux.r;
        aux.r = r;
        aux.i = f32(i);
{dispatch}
        i = i + 1;
    }}

    var out: Iter;
    out.z = z;
    out.de = aux.de;
    out.dist = aux.dist;
    out.trap = trap;
    out.n = i;
    out.pseudo_kleinian_de = aux.pseudo_kleinian_de;
    return out;
}}

fn fractal_de(pos: vec3<f32>) -> Field {{
    let max_iter = i32(dm3_u.fractal.y);
{body}
}}
"#
    );

    debug_assert!(
        TEMPLATE.contains(MARKER),
        "fractal.wgsl lost its {MARKER} marker"
    );
    // The prelude declares `Aux` and Mandelbulber's helpers, which every
    // formula body is written against.
    TEMPLATE.replace(MARKER, &format!("{}\n{de}", mb2::PRELUDE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas::{Builtin, FormulaSlot};

    #[test]
    fn default_stack_generates_a_de_and_consumes_the_marker() {
        let src = generate(&FormulaStack::default());
        assert!(!src.contains(MARKER), "marker should be replaced");
        assert!(src.contains("fn fractal_de("));
        assert!(src.contains("fn slot_0("));
    }

    /// The lights array is one flat block of bytes with no names attached, so
    /// a mismatched length here shifts nothing — it silently truncates the
    /// last lights, or reads past what Rust wrote. Cheaper to assert than to
    /// debug from a picture.
    #[test]
    fn the_shader_declares_exactly_as_many_light_vec4s_as_rust_uploads() {
        let slots = crate::params::MAX_LIGHTS * crate::render::LIGHT_VEC4S;
        assert!(
            TEMPLATE.contains(&format!("lights: array<vec4<f32>, {slots}>")),
            "fractal.wgsl must declare {slots} light vec4s"
        );
        assert!(TEMPLATE.contains(&format!(
            "const LIGHT_VEC4S: i32 = {};",
            crate::render::LIGHT_VEC4S
        )));
    }

    #[test]
    fn params_are_rewritten_to_this_slots_uniforms() {
        let mut stack = FormulaStack::default();
        stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox));
        let src = generate(&stack);
        // Slot 1 must read its own block, not slot 0's: it starts at
        // `POOL_FLOATS_PER_SLOT`, i.e. vec4 `POOL_VEC4S_PER_SLOT`.
        assert!(src.contains(&format!(
            "dm3_u.pool[{}].x",
            crate::formulas::POOL_VEC4S_PER_SLOT
        )));
        assert!(!src.contains("P0"));
    }

    #[test]
    fn disabled_slots_are_not_emitted() {
        let mut stack = FormulaStack::default();
        stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox));
        stack.slots[1].enabled = false;
        assert!(!generate(&stack).contains("fn slot_1("));
    }

    #[test]
    fn a_linear_formula_switches_the_estimator() {
        let mut stack = FormulaStack::default();
        assert!(generate(&stack).contains("log(safe_r)"));

        stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox));
        let src = generate(&stack);
        assert!(!src.contains("log(safe_r)"));
        assert!(src.contains("safe_r / de"));
    }

    #[test]
    fn the_most_specific_estimator_in_the_stack_wins() {
        use crate::formulas::{DeFunction, DeMode, FormulaKind, generated::GENERATED};

        // A formula that computed its own distance knows more than any closed
        // form we would pick for it, so it must win over a logarithmic
        // neighbour rather than being averaged away.
        let custom = GENERATED
            .iter()
            .position(|f| f.de_function == DeFunction::Custom)
            .map(FormulaKind::Generated)
            .expect("the corpus has custom-DE formulas");

        let mut stack = FormulaStack::default();
        assert_eq!(stack.de_mode(), DeMode::Log);

        stack.slots.push(FormulaSlot::new(custom));
        assert_eq!(stack.de_mode(), DeMode::Custom);
        assert!(generate(&stack).contains("aux.dist"));

        // Disabling it hands the choice back to the bulb.
        stack.slots[1].enabled = false;
        assert_eq!(stack.de_mode(), DeMode::Log);
    }

    #[test]
    fn the_estimate_is_never_negative() {
        // The logarithmic form goes negative inside the unit sphere, which
        // walks the marcher backwards through the surface. Mandelbulber clamps
        // it and so must we.
        let src = generate(&FormulaStack::default());
        assert!(src.contains("max("), "the estimate must be clamped at zero");
        assert!(src.contains("safe_r > 1.0"), "log DE is zero inside r = 1");
    }

    #[test]
    fn signature_tracks_structure_but_not_parameter_values() {
        let mut stack = FormulaStack::default();
        let before = stack.signature();

        stack.slots[0].params[0] = 5.0;
        stack.slots[0].start_iter = 3;
        assert_eq!(
            stack.signature(),
            before,
            "params and iteration ranges are uniforms, not generated code"
        );

        stack.slots.push(FormulaSlot::builtin(Builtin::Sierpinski));
        assert_ne!(stack.signature(), before, "adding a formula changes the shader");
    }

    #[test]
    fn an_empty_stack_still_compiles_to_valid_structure() {
        let stack = FormulaStack {
            slots: vec![],
            de_mode_override: None,
        };
        let src = generate(&stack);
        assert!(src.contains("fn fractal_de("));
        assert!(!src.contains("slot_0(&z"));
    }
}
