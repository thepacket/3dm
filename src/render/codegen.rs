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

    // An empty stack still has to compile and produce something finite.
    if dispatch.is_empty() {
        dispatch.push_str("        // empty stack — nothing to iterate\n");
    }

    // Mandelbulber treats a stack of formulas differently from a single one,
    // and the difference is the sign of the derivative. A fold can legitimately
    // drive `de` negative; alone that means the point is inside and the
    // distance is zero, but in a stack the magnitude is still the right scale
    // to march by, so the hybrid branch takes the absolute value. 3DM did
    // neither — it clamped `de` up to a tiny positive number, turning a
    // negative derivative into an enormous step that flew straight past the
    // surface.
    let hybrid = stack.active().count() > 1;
    let divisor = if hybrid {
        "max(abs(aux.de), 1e-9)"
    } else {
        "max(aux.de, 1e-9)"
    };

    // Ported from Mandelbulber's `compute_fractal.cl`, including the parts that
    // are easy to leave out and wrong to: the logarithmic form is zero inside
    // the unit sphere rather than negative, the IFS form carries a −2 offset,
    // and a formula that computed its own distance is simply believed.
    let distance = match stack.de_mode() {
        DeMode::Log => "select(0.0, 0.5 * safe_r * log(safe_r) / de, safe_r > 1.0)",
        DeMode::Linear => "safe_r / de",
        DeMode::Ifs => "(safe_r - 2.0) / de",
        DeMode::Custom => "aux.dist",
    };

    // A single formula with a non-positive derivative is inside the surface,
    // which Mandelbulber reports as a distance of zero rather than as a miss.
    let guard = if hybrid {
        format!("max({distance}, 0.0)")
    } else {
        format!("select(0.0, max({distance}, 0.0), aux.de > 0.0)")
    };

    let de = format!(
        r#"{ENUMERATORS}
{functions}fn fractal_de(pos: vec3<f32>) -> Field {{
    let max_iter = i32(dm3_u.fractal.y);
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
    // Mandelbulber seeds this from the Mandelbox scale, which 3DM has no global
    // equivalent of; formulas that use it overwrite it on their first
    // iteration, so a neutral 1.0 is the safe seed.
    aux.actual_scale = 1.0;
    aux.actual_scale_a = 0.0;
    aux.color = 1.0;
    aux.color_hybrid = 0.0;
    aux.temp1000 = 1000.0;
    aux.i = 0.0;
    aux.last_z = z;
    aux.r_dz = 1.0;
    aux.old_r = 0.0;

    loop {{
        if (i >= max_iter) {{ break; }}
        r = length(z);
        if (r > bailout) {{ break; }}
        trap = min(trap, r);

        aux.old_r = aux.r;
        aux.r = r;
        aux.i = f32(i);
{dispatch}
        i = i + 1;
    }}

    var out: Field;
    let safe_r = max(r, 1e-9);
    let de = {divisor};
    // Mandelbulber clamps the finished estimate at zero. Without it the
    // logarithmic form reports negative distances inside the unit sphere and
    // the marcher walks backwards through the surface.
    out.dist = {guard};
    out.trap = trap;
    out.escape = f32(i) / max(f32(max_iter), 1.0);
    return out;
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
