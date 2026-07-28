// 3DM — distance-estimated Mandelbulb, sphere-traced in a single fullscreen pass.
//
// The whole image is produced by one fragment shader: for every pixel we build a
// camera ray, march it against the fractal's distance estimator, then shade the
// hit with an orbit-trap palette, a soft shadow and ambient occlusion.

struct Uniforms {
    // xyz = eye position, w = tan(fov_y / 2)
    cam_pos: vec4<f32>,
    // xyz = camera basis vectors (w unused)
    cam_right: vec4<f32>,
    cam_up: vec4<f32>,
    cam_fwd: vec4<f32>,
    // x = viewport width px, y = viewport height px, z = time s,
    // w = 1.0 when we must encode sRGB ourselves (non-sRGB target format)
    screen: vec4<f32>,
    // x = power, y = iterations, z = bailout, w = DE scale
    fractal: vec4<f32>,
    // x = max steps, y = max distance, z = epsilon scale, w = bounding radius
    march: vec4<f32>,
    // x = AO strength, y = shadow softness, z = specular, w = glow
    shade: vec4<f32>,
    // xyz = light direction, w = ambient
    light: vec4<f32>,
    // rgb = palette base, w = palette phase
    palette_base: vec4<f32>,
    // rgb = palette amplitude, w = palette frequency
    palette_amp: vec4<f32>,
    // rgb = background, w = fog
    bg: vec4<f32>,
    // One vec4 per formula slot: x = start iteration, y = end iteration.
    // Kept in a uniform rather than in generated code so that dragging a
    // formula's iteration range does not rebuild the shader.
    ranges: array<vec4<f32>, 6>,
    // Every formula's parameters, one fixed-size block per slot. A transpiled
    // Mandelbulber formula can carry over a hundred floats — rotation matrices,
    // 4D offsets — which is why these live in a pool instead of in `ranges`.
    // Generated code indexes this with constants; the stride must match
    // `formulas::POOL_VEC4S_PER_SLOT`.
    pool: array<vec4<f32>, 240>,
};

// Deliberately not called `u`: Mandelbulber's formulas declare short lowercase
// locals, and one called `u` shadowed this binding so that every `u.pool` read
// in that formula became a field access on an f32.
@group(0) @binding(0) var<uniform> dm3_u: Uniforms;

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    // Normalised device coords of this pixel, -1..1, y up.
    @location(0) ndc: vec2<f32>,
};

// One oversized triangle covering the viewport — no vertex buffer needed.
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let c = corners[vi];
    var out: VsOut;
    // WebGPU clip space is y-up and the rasteriser handles the flip to the
    // top-left framebuffer origin, so `ndc` can be used directly as y-up.
    out.clip_pos = vec4<f32>(c, 0.0, 1.0);
    out.ndc = c;
    return out;
}

// ---------------------------------------------------------------------------
// Distance estimator
// ---------------------------------------------------------------------------

struct Field {
    // Signed distance estimate to the surface (positive outside).
    dist: f32,
    // Closest approach of the orbit to the origin — drives colour.
    trap: f32,
    // Fraction of the iteration budget used before escaping, 0..1.
    escape: f32,
};

// The distance estimator is generated from the formula stack — see
// `render/codegen.rs`. It defines one `slot_N` function per active formula and
// a `fractal_de` that chains them inside the escape-time loop.
//__GENERATED_DE__

fn map(p: vec3<f32>) -> f32 {
    return fractal_de(p).dist * dm3_u.fractal.w;
}

// Tetrahedral gradient — four taps instead of six.
fn surface_normal(p: vec3<f32>, h: f32) -> vec3<f32> {
    let k = vec2<f32>(1.0, -1.0);
    return normalize(
        k.xyy * map(p + k.xyy * h) +
        k.yyx * map(p + k.yyx * h) +
        k.yxy * map(p + k.yxy * h) +
        k.xxx * map(p + k.xxx * h)
    );
}

// ---------------------------------------------------------------------------
// Lighting helpers
// ---------------------------------------------------------------------------

// Scene scale relative to the Mandelbulb this shader was originally tuned
// against. Every absolute distance below is expressed in these units, so a
// Mandelbox six times the size is lit the same way rather than reading as one
// solid shadow. At bounds = 1.5 this is exactly 1.0, so the bulb is unchanged.
fn scene_scale() -> f32 {
    return max(dm3_u.march.w, 0.1) / 1.5;
}

// Marches toward the light; the closest miss along the way widens the penumbra.
//
// `eps` is the same hit threshold the primary ray used. Anchoring the start
// offset and the minimum step to it is what lets the ray escape the surface it
// started on: with a fixed minimum step, a large fractal's DE-limited steps are
// so small that the ray never gets clear within the loop budget and every lit
// point reports itself as shadowed.
// How close this formula's distance estimate is to a true distance, sampled at
// the shading point. A logarithmic Mandelbulb estimate returns nearly the real
// distance (~1.0); folding formulas like the Mandelbox are valid lower bounds
// but can underestimate by two orders of magnitude.
//
// Everything in the shadow march is a comparison against a distance, so without
// this correction those formulas report the surface as occluding itself and the
// whole model goes black — which is exactly what happened before it existed.
fn de_tightness(p: vec3<f32>, n: vec3<f32>, probe: f32) -> f32 {
    return clamp(map(p + n * probe) / max(probe, 1e-9), 0.01, 1.0);
}

// Marches toward the light; the closest miss along the way widens the penumbra.
//
// `eps` is the primary ray's hit threshold and `tightness` is the measure
// above; both the occlusion threshold and the penumbra ratio are expressed in
// terms of them, so the same constants work for every formula.
fn soft_shadow(
    ro: vec3<f32>,
    rd: vec3<f32>,
    eps: f32,
    maxt: f32,
    k: f32,
    tightness: f32,
) -> f32 {
    // Only a hit well inside the surface counts as occlusion. Scaling the
    // threshold by the tightness stops a conservative estimate from reporting
    // the ray's own starting surface as a hit.
    let thresh = eps * 0.05 * tightness;
    // A loose estimate needs a correspondingly larger penumbra constant, since
    // `h` is systematically smaller than the distance it stands for.
    let k_eff = k / tightness;

    var res = 1.0;
    var t = eps * 4.0;
    for (var i = 0; i < 48; i = i + 1) {
        let h = map(ro + rd * t);
        if (h < thresh) { return 0.0; }
        res = min(res, k_eff * h / t);
        // Crawl while hugging the surface, but stride once the field opens up.
        t = t + clamp(h, eps * 2.0, maxt * 0.1);
        if (t > maxt) { break; }
    }
    return clamp(res, 0.0, 1.0);
}

// Compares the true distance field against the distance we walked along the
// normal; where the field lags behind, geometry is nearby and occluding.
fn ambient_occlusion(p: vec3<f32>, n: vec3<f32>) -> f32 {
    let s = scene_scale();
    var occ = 0.0;
    var weight = 1.0;
    for (var i = 0; i < 5; i = i + 1) {
        let h = (0.008 + 0.09 * f32(i)) * s;
        occ = occ + (h - map(p + n * h)) * weight / s;
        weight = weight * 0.82;
    }
    return clamp(1.0 - 2.6 * occ, 0.0, 1.0);
}

// Cosine palette (Iñigo Quílez's formulation) driven by the orbit trap.
// The small per-channel phase offsets are what turn a single scalar into a
// gradient that sweeps through hue rather than just through brightness;
// spacing them a third of a turn apart instead washes out to grey.
fn palette(t: f32) -> vec3<f32> {
    let phase = vec3<f32>(0.0, 0.10, 0.20) + dm3_u.palette_base.w;
    return dm3_u.palette_base.rgb
        + dm3_u.palette_amp.rgb * cos(6.28318530718 * (dm3_u.palette_amp.w * t + phase));
}

fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

// Applies the output transfer function once, so debug views and the shaded
// image land in the same colour space.
fn shade_out(c: vec3<f32>) -> vec4<f32> {
    if (dm3_u.screen.w > 0.5) {
        return vec4<f32>(linear_to_srgb(c), 1.0);
    }
    return vec4<f32>(c, 1.0);
}

// Blue -> red ramp for scalar diagnostics.
fn heat(x: f32) -> vec3<f32> {
    let v = clamp(x, 0.0, 1.0);
    return vec3<f32>(v, 1.0 - abs(v - 0.5) * 2.0, 1.0 - v);
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let aspect = dm3_u.screen.x / max(dm3_u.screen.y, 1.0);
    let tan_half = dm3_u.cam_pos.w;

    let ro = dm3_u.cam_pos.xyz;
    let rd = normalize(
        dm3_u.cam_fwd.xyz
        + dm3_u.cam_right.xyz * in.ndc.x * aspect * tan_half
        + dm3_u.cam_up.xyz * in.ndc.y * tan_half
    );

    // Angular size of one pixel, used as the distance-dependent hit threshold
    // so that detail stays exactly at the resolution limit at any zoom.
    let pixel_angle = 2.0 * tan_half / max(dm3_u.screen.y, 1.0);
    let eps_scale = dm3_u.march.z;

    let max_steps = i32(dm3_u.march.x);
    let max_dist = dm3_u.march.y;

    // Skip the empty space outside the fractal's bounding sphere. The radius
    // depends on the formula stack — a Mandelbox reaches far further out than a
    // Mandelbulb — so it comes in as a uniform rather than being baked in.
    var t = 0.0;
    let b = dot(rd, ro);
    let bounds = max(dm3_u.march.w, 0.1);
    let c = dot(ro, ro) - bounds * bounds;
    let disc = b * b - c;
    if (disc < 0.0) {
        // Ray misses the fractal's bounding sphere entirely.
        var bg = dm3_u.bg.rgb;
        if (dm3_u.screen.w > 0.5) { bg = linear_to_srgb(bg); }
        return vec4<f32>(bg, 1.0);
    }
    t = max(-b - sqrt(disc), 0.0);
    // Distance at which the ray entered the bounding sphere. Fog is measured
    // from here rather than from the camera, so it describes depth *into the
    // fractal* and stays identical whether the scene is 1 unit or 20 across.
    let entry_t = t;

    var hit = false;
    var steps = 0;
    var field: Field;
    var p = ro;

    loop {
        if (steps >= max_steps) { break; }
        p = ro + rd * t;
        field = fractal_de(p);
        let d = field.dist * dm3_u.fractal.w;
        let eps = max(pixel_angle * t * eps_scale, 1e-6);
        if (d < eps) {
            hit = true;
            break;
        }
        t = t + d;
        if (t > max_dist) { break; }
        steps = steps + 1;
    }

    var color: vec3<f32>;

    // Rays that grind through many steps without hitting are grazing the
    // surface; that near-miss count is what produces the fractal's halo.
    let struggle = f32(steps) / max(f32(max_steps), 1.0);

    if (hit) {
        let n = surface_normal(p, max(pixel_angle * t, 1e-5));
        let l = normalize(dm3_u.light.xyz);

        let base = palette(field.trap);
        let diffuse = max(dot(n, l), 0.0);
        // Reuse the primary ray's hit threshold so the shadow ray's scale
        // tracks both the zoom level and the size of the fractal.
        let surface_eps = max(pixel_angle * t * eps_scale, 1e-6);
        // Lift the origin off the surface along the normal, or the ray's first
        // samples are inside the geometry it started on.
        let tightness = de_tightness(p, n, surface_eps * 32.0);
        let shadow = soft_shadow(
            p + n * surface_eps * 3.0,
            l,
            surface_eps,
            4.0 * scene_scale(),
            dm3_u.shade.y,
            tightness,
        );
        let occ = mix(1.0, ambient_occlusion(p, n), dm3_u.shade.x);

        // Diagnostics: show one term on its own. Each of these is a factor in
        // the final colour, so a dark image alone cannot tell you which one is
        // responsible — this can.
        let view = i32(dm3_u.fractal.x);
        if (view == 1) { return shade_out(n * 0.5 + 0.5); }
        if (view == 2) { return shade_out(vec3<f32>(shadow)); }
        if (view == 3) { return shade_out(vec3<f32>(occ)); }
        if (view == 4) { return shade_out(vec3<f32>(diffuse)); }
        if (view == 5) { return shade_out(heat(struggle)); }
        if (view == 6) { return shade_out(vec3<f32>(field.trap)); }

        // Sky term: brighter from above, tinted with the background's hue.
        //
        // The tint is normalised to unit peak first. Multiplying by the raw
        // background meant a near-black backdrop cancelled the ambient term
        // entirely — invisible on a Mandelbulb, which is lit mostly by the key
        // light, but it blacks out any fractal dense enough to shadow itself.
        let sky = 0.5 + 0.5 * n.y;
        let peak = max(dm3_u.bg.r, max(dm3_u.bg.g, dm3_u.bg.b));
        let bg_tint = select(vec3<f32>(1.0), dm3_u.bg.rgb / peak, peak > 1e-4);
        let ambient = dm3_u.light.w * sky * mix(vec3<f32>(1.0), bg_tint, 0.5);

        let h = normalize(l - rd);
        let spec = pow(max(dot(n, h), 0.0), 32.0) * dm3_u.shade.z * diffuse * shadow;

        // Occlusion attenuates the sky term only. Direct light already has
        // its own shadow term, and multiplying both by `occ` double-darkens —
        // barely visible on a sparse Mandelbulb, but it crushes a dense
        // Mandelbox to near-black.
        color = base * (diffuse * shadow + ambient * occ) + vec3<f32>(spec);

        // Fog toward the background, by depth into the bounding volume.
        let depth = clamp((t - entry_t) / (2.0 * bounds), 0.0, 1.0);
        color = mix(color, dm3_u.bg.rgb, 1.0 - exp(-dm3_u.bg.w * depth * 3.0));
    } else {
        let view = i32(dm3_u.fractal.x);
        if (view == 5) { return shade_out(heat(struggle)); }
        if (view != 0) { return shade_out(vec3<f32>(0.0)); }
        color = dm3_u.bg.rgb;
    }

    // Glow applies to hit and miss alike — it is what makes the silhouette wisp.
    color = color + palette(0.35) * struggle * struggle * dm3_u.shade.w;

    // Reinhard-ish tonemap keeps the specular from clipping to flat white.
    color = color / (1.0 + color * 0.35);

    if (dm3_u.screen.w > 0.5) {
        color = linear_to_srgb(color);
    }
    return vec4<f32>(color, 1.0);
}

// ---------------------------------------------------------------------------
// Distance probe
// ---------------------------------------------------------------------------

// Evaluates the estimator once, at the camera, into a 1x1 target the CPU reads
// back. That single number is how far the camera can move before it touches
// anything, which is what lets forward flight approach a surface asymptotically
// instead of sailing through it. Nothing else in the frame needs it, so it is
// its own tiny pass rather than a channel stolen from the main one.
@fragment
fn fs_probe(in: VsOut) -> @location(0) vec4<f32> {
    let field = fractal_de(dm3_u.cam_pos.xyz);
    return vec4<f32>(field.dist, 0.0, 0.0, 1.0);
}
