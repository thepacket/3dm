//! Turns a [`FormulaStack`] into WGSL.
//!
//! The alternative — one shader containing every formula behind a `switch` —
//! would branch inside the innermost loop of the raymarcher, which runs on the
//! order of iterations × steps × DE-taps times per pixel. Generating
//! straight-line code per stack configuration and caching the pipeline keeps
//! that loop branch-free; the cost is a shader rebuild when the stack's
//! *structure* changes, which is rare compared to dragging a slider.

use crate::formulas::{DeMode, FormulaStack, VEC4S_PER_SLOT};

/// Where the generated distance estimator is spliced into the static shader.
const MARKER: &str = "//__GENERATED_DE__";

const TEMPLATE: &str = include_str!("fractal.wgsl");

/// Builds the complete shader source for `stack`.
pub fn generate(stack: &FormulaStack) -> String {
    let mut functions = String::new();
    let mut dispatch = String::new();

    for (index, slot) in stack.active() {
        let base = index * VEC4S_PER_SLOT;

        // Params 0-3 sit in the slot's first vec4, 4-5 in the second; the
        // second vec4's zw carry the iteration range.
        let body = slot
            .kind
            .wgsl_body()
            .replace("P0", &format!("u.slots[{base}].x"))
            .replace("P1", &format!("u.slots[{base}].y"))
            .replace("P2", &format!("u.slots[{base}].z"))
            .replace("P3", &format!("u.slots[{base}].w"))
            .replace("P4", &format!("u.slots[{}].x", base + 1))
            .replace("P5", &format!("u.slots[{}].y", base + 1));

        functions.push_str(&format!(
            "// slot {index}: {}\n\
             fn slot_{index}(z: ptr<function, vec3<f32>>, dr: ptr<function, f32>, c: vec3<f32>) {{{body}}}\n\n",
            slot.kind.name(),
        ));

        // The iteration range is a uniform read, so dragging it does not
        // invalidate this pipeline.
        dispatch.push_str(&format!(
            "        if (fi >= u.slots[{hi}].z && fi < u.slots[{hi}].w) {{ slot_{index}(&z, &dr, pos); }}\n",
            hi = base + 1,
        ));
    }

    // An empty stack still has to compile and produce something finite.
    if dispatch.is_empty() {
        dispatch.push_str("        // empty stack — nothing to iterate\n");
    }

    let distance = match stack.de_mode() {
        DeMode::Log => "0.5 * log(safe_r) * safe_r / max(dr, 1e-9)",
        DeMode::Linear => "safe_r / max(dr, 1e-9)",
    };

    let de = format!(
        r#"{functions}fn fractal_de(pos: vec3<f32>) -> Field {{
    let max_iter = i32(u.fractal.y);
    let bailout = u.fractal.z;

    var z = pos;
    var dr = 1.0;
    var r = length(pos);
    var trap = 1e20;
    var i = 0;

    loop {{
        if (i >= max_iter) {{ break; }}
        r = length(z);
        if (r > bailout) {{ break; }}
        trap = min(trap, r);

        let fi = f32(i);
{dispatch}
        i = i + 1;
    }}

    var out: Field;
    let safe_r = max(r, 1e-9);
    out.dist = {distance};
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
    TEMPLATE.replace(MARKER, &de)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas::{FormulaKind, FormulaSlot};

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
        stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbox));
        let src = generate(&stack);
        // Slot 1 must read its own vec4 pair, not slot 0's.
        assert!(src.contains("u.slots[2].x"));
        assert!(!src.contains("P0"));
    }

    #[test]
    fn disabled_slots_are_not_emitted() {
        let mut stack = FormulaStack::default();
        stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbox));
        stack.slots[1].enabled = false;
        assert!(!generate(&stack).contains("fn slot_1("));
    }

    #[test]
    fn a_linear_formula_switches_the_estimator() {
        let mut stack = FormulaStack::default();
        assert!(generate(&stack).contains("log(safe_r)"));

        stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbox));
        let src = generate(&stack);
        assert!(!src.contains("log(safe_r)"));
        assert!(src.contains("safe_r / max(dr"));
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

        stack.slots.push(FormulaSlot::new(FormulaKind::Sierpinski));
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
