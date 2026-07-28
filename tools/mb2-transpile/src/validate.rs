//! Compiles each translated formula with naga.
//!
//! "The rewriter did not bail" and "this is valid WGSL" are very different
//! claims, and only the second one is worth reporting. Every formula is wrapped
//! in a minimal but realistic shader and run through the same front end wgpu
//! uses, so the coverage number means the shader would actually build.

use crate::translate::Formula;
use crate::types::ParamType;

use std::sync::OnceLock;

static ENUMS: OnceLock<String> = OnceLock::new();

/// Installs the enumerator constants harvested from Mandelbulber's header.
pub fn set_enumerators(defs: &[(String, i64)]) {
    let mut s = String::new();
    for (name, value) in defs {
        s.push_str(&format!("const {name}: i32 = {value};\n"));
    }
    ENUMS.set(s).ok();
}

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
    de0: f32,
    dist: f32,
    pos_neg: f32,
    temp1000: f32,
    color_hybrid: f32,
    r_dz: f32,
    old_r: f32,
    last_z: vec4<f32>,
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

const M_PI_F: f32 = 3.14159265358979323846;
const M_PI_2_F: f32 = 1.57079632679489661923;
const M_PI_4_F: f32 = 0.78539816339744830962;
const M_PI_180_F: f32 = 0.01745329251994329577;
const M_PI_360_F: f32 = 0.00872664625997164788;
const M_1_PI_F: f32 = 0.31830988618379067154;
const M_E_F: f32 = 2.71828182845904523536;
const SQRT_3_F: f32 = 1.73205080756887729353;
const SQRT_2_F: f32 = 1.41421356237309504880;
// Ported verbatim from Mandelbulber2's OpenCL headers so the numbers match.
const SQRT_1_3_F: f32 = 0.577350269189625;
const SQRT_1_2_F: f32 = 0.707106781186547;
const SQRT_2_3_F: f32 = 0.816496580927726;
const SQRT_3_2_F: f32 = 1.224744871391589;
const SQRT_3_4_F: f32 = 0.866025403784438;
const FRAC_1_3_F: f32 = 0.333333333333333;
const M_PI_2x_F: f32 = 6.2831853071795;
const M_PI_2x_INV_F: f32 = 0.1591549430919;

fn mb2_divide(a: f32, b: f32) -> f32 { return a / b; }
fn mb2_recip(a: f32) -> f32 { return 1.0 / a; }
fn mb2_mat_mul(m: mat3x3<f32>, v: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(m * v.xyz, v.w);
}

// Mandelbulber helpers, ported from its OpenCL sources.
fn fmod(a: f32, b: f32) -> f32 { return a % b; }

fn SmoothConditionAGreaterB(a: f32, b: f32, sharpness: f32) -> f32 {
    return 1.0 / (1.0 + exp(sharpness * (b - a)));
}

fn SmoothConditionALessB(a: f32, b: f32, sharpness: f32) -> f32 {
    return 1.0 / (1.0 + exp(sharpness * (a - b)));
}

fn RotateAroundVectorByAngle4(origin4d: vec4<f32>, axis: vec3<f32>, angle: f32) -> vec4<f32> {
    let origin = origin4d.xyz;
    var v = origin * cos(angle);
    v += cross(axis, origin) * sin(angle);
    v += axis * dot(axis, origin) * (1.0 - cos(angle));
    return vec4<f32>(v, origin4d.w);
}

fn wrap(x_in: vec3<f32>, a: vec3<f32>, s: vec3<f32>) -> vec3<f32> {
    let x = x_in - s;
    return x - a * floor(x / a) + s;
}
"#;

/// Builds a compilable shader for one formula by substituting its `Pn`
/// placeholders with typed uniform reads.
///
/// These rules must match `three_dm::formulas::mb2::substitute`, because the
/// only thing this check is worth is predicting whether *that* compiles. They
/// did not match for a long time: this side cast integer parameters to `i32`
/// while the renderer left them as `f32`, so five formulas passed here and
/// failed there. Any rule added on one side belongs on the other.
pub fn shader_for(f: &Formula) -> String {
    let mut body = boolify_select_conditions(&f.wgsl_body);

    // Longest offsets first, so `P1` does not clobber the start of `P12`.
    let mut params = f.params.clone();
    params.sort_by_key(|p| std::cmp::Reverse(p.offset));

    for p in &params {
        // Everything lives in an f32 pool, integer parameters included.
        let read = match p.ty {
            ParamType::Float | ParamType::Int => format!("mb2_param({})", p.offset),
            ParamType::Bool => format!("(mb2_param({}) > 0.5)", p.offset),
            ParamType::Float3 => format!("mb2_param3({})", p.offset),
            ParamType::Float4 => format!("mb2_param4({})", p.offset),
            ParamType::Matrix33 => format!("mb2_mat({})", p.offset),
        };
        let placeholder = format!("__MB2P{}__", p.offset);
        let int = format!("i32({read})");

        // The two places WGSL insists on a genuine integer.
        body = body.replace(
            &format!("switch {placeholder}"),
            &format!("switch {int}"),
        );
        for op in ["==", "!=", "<", ">"] {
            body = body.replace(
                &format!("{placeholder} {op} multi_"),
                &format!("{int} {op} multi_"),
            );
        }

        body = body.replace(&placeholder, &read);
    }

    let enums = ENUMS.get().map(String::as_str).unwrap_or("");
    format!(
        "{PRELUDE}\n{enums}\nfn formula(z_in: vec4<f32>, aux: ptr<function, Aux>) -> vec4<f32> {{\n    var z = z_in;\n{body}\n}}\n"
    )
}

/// The full naga diagnostic, with source context — for `--dump`.
pub fn full_error(f: &Formula) -> Option<String> {
    let src = shader_for(f);
    naga::front::wgsl::parse_str(&src)
        .err()
        .map(|e| e.emit_to_string(&src))
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

/// A bare flag parameter used as a `select` condition needs to become a
/// comparison. Mirrors `three_dm::formulas::mb2`; see `shader_for`.
fn boolify_select_conditions(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;

    while let Some(at) = rest.find("select(") {
        let open = at + "select(".len() - 1;
        let Some(close) = matching_paren(rest, open) else {
            break;
        };
        out.push_str(&rest[..close]);

        let args = &rest[open + 1..close];
        let last = args
            .rfind(',')
            .map(|i| args[i + 1..].trim())
            .unwrap_or_default();
        if is_placeholder(last) {
            out.push_str(" != 0.0");
        }
        out.push(')');
        rest = &rest[close + 1..];
    }

    out.push_str(rest);
    out
}

fn matching_paren(s: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices().skip(open) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn is_placeholder(s: &str) -> bool {
    s.starts_with("__MB2P")
        && s.ends_with("__")
        && s.len() > 8
        && s[6..s.len() - 2].chars().all(|c| c.is_ascii_digit())
}
