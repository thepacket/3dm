// Declarations the transpiled Mandelbulber2 formula bodies are written
// against. Shared verbatim by the real shader (`render::codegen`) and by the
// transpiler's compile check, so a formula that validates in one validates in
// the other.
//
// Ported from Mandelbulber2 (<https://github.com/buddhi1980/mandelbulber2>,
// © Mandelbulber Team, GPL-3.0).

// Mandelbulber threads a mutable bag of per-iteration state through every
// formula. Most fields are used by a handful of formulas each, but they all
// have to exist for any body to compile.
struct Aux {
    de: f32,
    color: f32,
    r: f32,
    // The iteration index, deliberately f32 rather than i32. Mandelbulber
    // compares it against integer parameters and also multiplies it into float
    // expressions; C++ converts implicitly and WGSL does not, so keeping the
    // whole formula ABI in floats is what lets the translated bodies typecheck.
    i: f32,
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

const M_PI_F: f32 = 3.14159265358979323846;
const M_PI_2_F: f32 = 1.57079632679489661923;
const M_PI_4_F: f32 = 0.78539816339744830962;
// From Mandelbulber's defines_cl.h, verbatim.
const M_PI_8_F: f32 = 0.3926990816987;
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

// Mandelbulber's points are 4D even in 3D fractals; a 3x3 rotation leaves w
// alone.
fn mb2_mat_mul(m: mat3x3<f32>, v: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(m * v.xyz, v.w);
}

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
