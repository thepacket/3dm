//! Compiles each translated formula with naga.
//!
//! "The rewriter did not bail" and "this is valid WGSL" are very different
//! claims, and only the second one is worth reporting. Every formula is wrapped
//! in a minimal but realistic shader and run through the same front end wgpu
//! uses, so the coverage number means the shader would actually build.

use crate::translate::Formula;
use crate::types::ParamType;

/// Declarations the generated bodies are written against.
const PRELUDE: &str = r#"
struct Aux {
    de: f32,
    color: f32,
    r: f32,
    i: i32,
    actual_scale: f32,
    actual_scale_a: f32,
    c: vec4<f32>,
    const_c: vec4<f32>,
    old_z: vec4<f32>,
};

struct Params {
    v: array<vec4<f32>, 256>,
};

@group(0) @binding(0) var<uniform> params: Params;

fn mb2_param(i: i32) -> f32 {
    return params.v[i / 4][i % 4];
}

fn mb2_param4(i: i32) -> vec4<f32> {
    return params.v[i / 4];
}

fn mb2_param3(i: i32) -> vec3<f32> {
    return params.v[i / 4].xyz;
}

fn mb2_mat(i: i32) -> mat3x3<f32> {
    return mat3x3<f32>(
        params.v[i / 4].xyz,
        params.v[i / 4 + 1].xyz,
        params.v[i / 4 + 2].xyz);
}

fn mb2_divide(a: f32, b: f32) -> f32 { return a / b; }
fn mb2_recip(a: f32) -> f32 { return 1.0 / a; }
fn mb2_mat_mul(m: mat3x3<f32>, v: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(m * v.xyz, v.w);
}
"#;

/// Builds a compilable shader for one formula by substituting its `Pn`
/// placeholders with typed uniform reads.
pub fn shader_for(f: &Formula) -> String {
    let mut body = f.wgsl_body.clone();

    // Longest offsets first, so `P1` does not clobber the start of `P12`.
    let mut params = f.params.clone();
    params.sort_by_key(|p| std::cmp::Reverse(p.offset));

    for p in &params {
        let read = match p.ty {
            ParamType::Float => format!("mb2_param({})", p.offset),
            ParamType::Int => format!("i32(mb2_param({}))", p.offset),
            ParamType::Bool => format!("(mb2_param({}) > 0.5)", p.offset),
            ParamType::Float3 => format!("mb2_param3({})", p.offset),
            ParamType::Float4 => format!("mb2_param4({})", p.offset),
            ParamType::Matrix33 => format!("mb2_mat({})", p.offset),
        };
        body = body.replace(&format!("P{}", p.offset), &read);
    }

    format!(
        "{PRELUDE}\nfn formula(z_in: vec4<f32>, aux: ptr<function, Aux>) -> vec4<f32> {{\n    var z = z_in;\n{body}\n}}\n"
    )
}

/// `Ok(())` when naga accepts the shader, otherwise the first error.
pub fn check(f: &Formula) -> Result<(), String> {
    let src = shader_for(f);
    naga::front::wgsl::parse_str(&src)
        .map(|_| ())
        .map_err(|e| first_line(&e.emit_to_string(&src)))
}

fn first_line(s: &str) -> String {
    s.lines()
        .find(|l| l.contains("error"))
        .unwrap_or_else(|| s.lines().next().unwrap_or(""))
        .trim()
        .to_owned()
}
