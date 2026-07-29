//! The emitted formula bodies must be WGSL 3DM can actually compile.
//!
//! Checked against naga, pinned to the version wgpu bundles — a check against
//! a different one is not a check, which the main crate learned when two
//! formulas passed naga 30 and failed on the GPU.

use std::path::Path;

use mb3d_decompile::{emit, exec, extract, options};

/// Emits one fixture's body with its parameters substituted for their
/// defaults, wrapped in the minimum that makes it a shader.
fn shader_for(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(format!("{name}.m3f"));
    let formula = extract::parse(&path).expect("fixture should parse");
    let result = exec::run_with_constants(&formula.code, &formula.constants);
    assert!(result.bailed.is_none(), "{name}: {:?}", result.bailed);

    let stores = exec::final_stores(&result.stores);
    let mut body = emit::body(&stores).expect("should emit");

    // `__MB2Pn__` is filled in by 3DM's own codegen from the parameter pool;
    // here the declared defaults stand in so the result is self-contained.
    let declared = options::declared(&formula.options).expect("known datatypes");
    for (i, (_, default)) in options::slots(&declared).iter().enumerate().rev() {
        body = body.replace(&format!("__MB2P{i}__"), &format!("{default:?}"));
    }

    format!(
        "struct Aux {{ de: f32, i: f32, actual_scale: f32, const_c: vec4<f32> }};\n\
         fn slot(z_in: vec4<f32>, aux: ptr<function, Aux>) -> vec4<f32> {{\n\
         \x20   var z = z_in;\n{body}    return z;\n}}\n"
    )
}

fn assert_valid(source: &str) {
    let module = match naga::front::wgsl::parse_str(source) {
        Ok(module) => module,
        Err(e) => panic!("does not parse: {}\n{source}", e.emit_to_string(source)),
    };
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap_or_else(|e| panic!("does not validate: {e:?}\n{source}"));
}

#[test]
fn kalisets1_emits_valid_wgsl() {
    let shader = shader_for("Kalisets1");
    // Its own maths, in 3DM's terms: the fold, the inverse-square scale, and
    // the constant the formula adds itself rather than leaving to the caller.
    assert!(shader.contains("(*aux).const_c.x"));
    assert!(shader.contains("abs(z.x)"));
    assert_valid(&shader);
}

/// The harder one: eight assignments and a reused temporary, which has to come
/// out as a declared `var` rather than a name from nowhere.
#[test]
fn benesipine1_emits_valid_wgsl_with_its_temporary() {
    let shader = shader_for("BenesiPine1");
    assert!(
        shader.contains("var t8: f32 ="),
        "the temporary should be declared before use"
    );
    assert_eq!(
        shader.matches("var t8: f32 =").count(),
        1,
        "and declared only once, though it is assigned twice"
    );
    assert_valid(&shader);
}

/// A branching formula: its conditionals have to become `select`, with the
/// arms the right way round.
#[test]
fn aboxmod1_emits_valid_wgsl_with_selects() {
    let shader = shader_for("ABoxMod1");
    assert!(shader.contains("select("), "conditionals become select");
    assert_valid(&shader);
}
