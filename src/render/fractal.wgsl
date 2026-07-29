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
    // x = AO strength, y = spare, z = specular, w = glow
    shade: vec4<f32>,
    // rgb = fill-light tint, w = ambient
    light: vec4<f32>,
    // rgb = palette base, w = palette phase
    palette_base: vec4<f32>,
    // rgb = palette amplitude, w = palette frequency
    palette_amp: vec4<f32>,
    // rgb = background, w = fog
    bg: vec4<f32>,
    // x = brightness, y = contrast, z = gamma, w = saturation
    picture: vec4<f32>,
    // x = perspective type, y = 1 when tone mapping is skipped,
    // z = chromatic aberration, w = occlusion samples
    view: vec4<f32>,
    // rgb = reflection tint, w = how much of the view reflects
    reflection: vec4<f32>,
    // rgb = occlusion tint
    ao_tint: vec4<f32>,
    // x = focus distance, y = aperture, z = max blur, w = blur opacity
    dof: vec4<f32>,
    // rgb = fog colour, w = visibility distance
    fog: vec4<f32>,
    // rgb = near glow colour, w = glow intensity
    glow_a: vec4<f32>,
    // rgb = far glow colour, w = iteration-fog strength
    glow_b: vec4<f32>,
    // rgb = iteration fog near colour, w = low trim
    iter_fog_a: vec4<f32>,
    // rgb = iteration fog far colour, w = high trim
    iter_fog_b: vec4<f32>,
    // rgb = flat surface colour, w = shading strength
    surface: vec4<f32>,
    // rgb = specular colour, w = brightness
    specular: vec4<f32>,
    // rgb = emitted colour, w = luminosity
    emission: vec4<f32>,
    // x = palette offset, y = colour speed, z = specular width,
    // w = 1 when the gradient is in use
    material: vec4<f32>,
    // The surface gradient, baked to a table so the shader does one read
    // rather than walking a list of stops per pixel.
    gradient: array<vec4<f32>, 64>,
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
    // x = how many entries of `lights` are live. Disabled lights are dropped
    // before upload, so there is nothing to branch around here.
    lights_meta: vec4<f32>,
    // Five vec4s per light, flat rather than an array of structs so the
    // element stride is obvious on both sides. See `light_*` accessors below
    // and `pack_light` in `render/mod.rs`, which must agree exactly.
    lights: array<vec4<f32>, 40>,
    // rgb = background colour at the horizon, w = background brightness
    bg2: vec4<f32>,
    // rgb = background colour below the horizon, w = background gamma
    bg3: vec4<f32>,
    // x = 1 with a three-colour gradient, y = 1 when the image is in use,
    // z = map type, w = horizontal scale
    bg_flags: vec4<f32>,
    // x = vertical scale, yz = texture offset
    bg_tex: vec4<f32>,
    // Rows of the rotation applied to a ray before it samples the image.
    bg_rot: array<vec4<f32>, 3>,
    // Post effects, which happen in the blit and are never read here. Declared
    // anyway: this block is one flat run of bytes, so a field the CPU writes
    // and the shader omits shifts every offset after it.
    post_fx: vec4<f32>,
};

const LIGHT_VEC4S: i32 = 5;
// These must match the discriminants of `params::LightKind`, which is what
// `pack_light` writes into the `w` of the position slot.
const LIGHT_DIRECTIONAL: f32 = 0.0;
const LIGHT_CONE: f32 = 2.0;

// Deliberately not called `u`: Mandelbulber's formulas declare short lowercase
// locals, and one called `u` shadowed this binding so that every `u.pool` read
// in that formula became a field access on an f32.
@group(0) @binding(0) var<uniform> dm3_u: Uniforms;
// The background panorama. One white pixel when none is loaded, in which case
// `bg_flags.y` is zero and nothing ever samples it.
@group(0) @binding(1) var dm3_bg_tex: texture_2d<f32>;
@group(0) @binding(2) var dm3_bg_samp: sampler;

const BG_EQUIRECTANGULAR: f32 = 0.0;
const BG_DOUBLE_HEMISPHERE: f32 = 1.0;
const BG_FLAT: f32 = 2.0;

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
//
// `penetrate` fades an occluder by how far along the ray it was met, so a
// caster near the light blocks less than one hugging the surface. Mandelbulber
// calls this penetrating light and applies exactly this ratio.
fn soft_shadow(
    ro: vec3<f32>,
    rd: vec3<f32>,
    eps: f32,
    maxt: f32,
    k: f32,
    tightness: f32,
    penetrate: f32,
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
        // How much of this occluder counts. Unity unless the light penetrates,
        // in which case it decays to nothing at the light itself.
        let weight = select(1.0, clamp((maxt - t) / maxt, 0.0, 1.0), penetrate > 0.5);
        if (h < thresh) { return 1.0 - weight; }
        res = min(res, 1.0 - (1.0 - clamp(k_eff * h / t, 0.0, 1.0)) * weight);
        // Crawl while hugging the surface, but stride once the field opens up.
        t = t + clamp(h, eps * 2.0, maxt * 0.1);
        if (t > maxt) { break; }
    }
    return clamp(res, 0.0, 1.0);
}

// One light, resolved at one shading point.
struct LightSample {
    // Unit vector from the surface toward the light.
    dir: vec3<f32>,
    // Colour times intensity times distance and cone falloff — everything
    // about the light that does not depend on the surface.
    radiance: vec3<f32>,
    // How far a shadow ray may usefully travel: to the light, or to the edge
    // of the scene for a light that has no position.
    max_dist: f32,
    // Penumbra constant. Large is a hard edge.
    k: f32,
    cast_shadows: f32,
    penetrating: f32,
};

// The penumbra a size-1 light produces at one scene radius, which is the
// constant the shader was tuned with when there was a single hard-coded light.
// Every other size and distance is stated relative to it, so the whole family
// of lights stays consistent as the scene is scaled.
const REFERENCE_SOFT_RANGE: f32 = 0.0625;

fn light_sample(index: i32, p: vec3<f32>) -> LightSample {
    let base = index * LIGHT_VEC4S;
    let color = dm3_u.lights[base];
    let pos_kind = dm3_u.lights[base + 1];
    let dir_decay = dm3_u.lights[base + 2];
    let shape = dm3_u.lights[base + 3];
    let flags = dm3_u.lights[base + 4];

    var out: LightSample;
    out.cast_shadows = flags.x;
    out.penetrating = flags.y;

    let axis = normalize(dir_decay.xyz);

    if (pos_kind.w < LIGHT_DIRECTIONAL + 0.5) {
        // Directional: parallel rays, no falloff, and a penumbra fixed by the
        // stated cone angle rather than by how far away anything is.
        out.dir = axis;
        out.radiance = color.rgb * color.w;
        out.max_dist = 4.0 * scene_scale();
        out.k = flags.z;
        return out;
    }

    let to_light = pos_kind.xyz - p;
    let dist = length(to_light);
    out.dir = to_light / max(dist, 1e-9);
    out.max_dist = dist;

    // Distances are measured in scene radii, so a light one radius away is at
    // full strength whatever the fractal's size. Mandelbulber instead carries
    // a fixed 100/6 factor tuned to its own default scale.
    let units = max(dist / scene_scale(), 1e-4);
    var falloff = 1.0 / pow(units, dir_decay.w);

    if (pos_kind.w > LIGHT_CONE - 0.5) {
        // Inside the inner cone it is full strength, outside the outer one it
        // is dark, and the soft angle is the band between them.
        let axiality = dot(-out.dir, axis);
        falloff = falloff * smoothstep(shape.w, shape.z, axiality);
    }

    out.radiance = color.rgb * color.w * falloff;
    // A bigger or nearer lamp casts a softer shadow, which is the one part of
    // this that cannot be precomputed: it depends on the shading point.
    out.k = units / max(shape.x * REFERENCE_SOFT_RANGE, 1e-6);
    return out;
}

// The lights themselves, seen head-on against the background.
//
// Without this a positional light is invisible until something reflects it,
// which makes placing one guesswork. Visibility defaults to zero, so a scene
// that never asks for it renders exactly as it did before.
fn visible_lights(ro: vec3<f32>, rd: vec3<f32>) -> vec3<f32> {
    var sum = vec3<f32>(0.0);
    let count = i32(dm3_u.lights_meta.x);
    for (var i = 0; i < count; i = i + 1) {
        let base = i * LIGHT_VEC4S;
        let color = dm3_u.lights[base];
        let pos_kind = dm3_u.lights[base + 1];
        let shape = dm3_u.lights[base + 3];
        let visibility = dm3_u.lights[base + 4].w;
        if (visibility <= 0.0) { continue; }

        // Where on the sky the light sits: opposite its travel direction for a
        // directional light, and at its actual position for the rest.
        var toward: vec3<f32>;
        if (pos_kind.w < LIGHT_DIRECTIONAL + 0.5) {
            toward = normalize(dm3_u.lights[base + 2].xyz);
        } else {
            let d = pos_kind.xyz - ro;
            if (dot(d, rd) <= 0.0) { continue; }
            toward = normalize(d);
        }

        // Angular distance from the ray, scaled so `size` reads as a width in
        // degrees, then rolled off by the contour sharpness.
        let offset = (1.0 - dot(rd, toward)) * 360.0 / max(shape.x, 1e-4);
        let disc = 1.0 / (1.0 + pow(max(offset, 0.0), 6.0 * shape.y));
        sum = sum + color.rgb * (disc * visibility * color.w);
    }
    return sum;
}

// What lies behind the fractal along `rd`: a flat colour, a vertical gradient,
// or a panorama.
//
// Everywhere the old code read `bg.rgb` for what the eye sees now calls this.
// The two places that did *not* — the fog target and the ambient sky tint —
// still read `bg.rgb`, which is colour #1: both want one settled colour rather
// than whatever happens to be overhead at that pixel.
fn background_color(rd: vec3<f32>) -> vec3<f32> {
    var pixel: vec3<f32>;

    if (dm3_u.bg_flags.y > 0.5) {
        // Rotate the ray rather than the image, which is the same thing and
        // costs three dot products instead of a resample.
        let v = normalize(vec3<f32>(
            dot(dm3_u.bg_rot[0].xyz, rd),
            dot(dm3_u.bg_rot[1].xyz, rd),
            dot(dm3_u.bg_rot[2].xyz, rd)));
        var uv: vec2<f32>;

        if (dm3_u.bg_flags.z < BG_DOUBLE_HEMISPHERE - 0.5) {
            // Equirectangular: longitude across, latitude up.
            let lon = atan2(v.x, -v.z);
            let lat = asin(clamp(v.y, -1.0, 1.0));
            uv = vec2<f32>(
                (lon / (2.0 * 3.14159265) + 0.5) / dm3_u.bg_flags.w,
                (0.5 - lat / 3.14159265) / dm3_u.bg_tex.x);
        } else if (dm3_u.bg_flags.z < BG_FLAT - 0.5) {
            // Double hemisphere: two fisheye circles side by side, the front
            // half in the left one and the back half in the right.
            var lon = atan2(v.y, v.x);
            var lat = atan2(-v.z, length(v.xy));
            var half_offset = 0.0;
            if (lat < 0.0) {
                lat = -lat;
                lon = 3.14159265 - lon;
                half_offset = 0.5;
            }
            let r = 1.0 - lat / (0.5 * 3.14159265);
            uv = vec2<f32>(
                0.25 + cos(lon) * r * 0.25 + half_offset,
                0.5 + sin(lon) * r * 0.5);
        } else {
            // Flat: a backdrop hung square to the view, filling the frame at
            // scale 1. Worked out in the camera's own frame rather than the
            // world's — pinning it to a world plane instead puts a seam across
            // the picture wherever the ray turns away from that plane.
            let camera_space = vec3<f32>(
                dot(rd, dm3_u.cam_right.xyz),
                dot(rd, dm3_u.cam_up.xyz),
                dot(rd, dm3_u.cam_fwd.xyz));
            let r = normalize(vec3<f32>(
                dot(dm3_u.bg_rot[0].xyz, camera_space),
                dot(dm3_u.bg_rot[1].xyz, camera_space),
                dot(dm3_u.bg_rot[2].xyz, camera_space)));
            // Behind the camera there is no plane to hit; hold the edge.
            let forward = max(r.z, 1e-4);
            let tan_half = dm3_u.cam_pos.w;
            let aspect = dm3_u.screen.x / max(dm3_u.screen.y, 1.0);
            uv = vec2<f32>(
                (r.x / forward) / (2.0 * tan_half * aspect) / dm3_u.bg_flags.w + 0.5,
                (-r.y / forward) / (2.0 * tan_half) / dm3_u.bg_tex.x + 0.5);
        }

        uv = uv + vec2<f32>(dm3_u.bg_tex.y, dm3_u.bg_tex.z);
        pixel = textureSampleLevel(dm3_bg_tex, dm3_bg_samp, uv, 0.0).rgb;
    } else if (dm3_u.bg_flags.x > 0.5) {
        // Colour #1 overhead, #2 at the horizon, #3 underneath — the same two
        // linear runs Mandelbulber uses, about its own up axis.
        let grad = clamp(rd.y, -1.0, 1.0) + 1.0;
        if (grad < 1.0) {
            pixel = mix(dm3_u.bg3.rgb, dm3_u.bg2.rgb, grad);
        } else {
            pixel = mix(dm3_u.bg2.rgb, dm3_u.bg.rgb, grad - 1.0);
        }
    } else {
        pixel = dm3_u.bg.rgb;
    }

    // Brightness then gamma, exactly as Mandelbulber orders them. Skipped
    // rather than applied as identities: `pow(x, 1.0)` is not guaranteed to
    // return `x` bit-for-bit, and the default background must not move.
    pixel = pixel * dm3_u.bg2.w;
    if (abs(dm3_u.bg3.w - 1.0) > 1e-6) {
        pixel = pow(max(pixel, vec3<f32>(0.0)), vec3<f32>(1.0 / dm3_u.bg3.w));
    }
    return pixel;
}

// Compares the true distance field against the distance we walked along the
// normal; where the field lags behind, geometry is nearby and occluding.
fn ambient_occlusion(p: vec3<f32>, n: vec3<f32>) -> f32 {
    let s = scene_scale();
    let samples = i32(dm3_u.view.w);
    var occ = 0.0;
    var weight = 1.0;
    // The step spacing is normalised by the sample count, so raising the
    // quality refines the same span rather than reaching further out and
    // darkening everything.
    let span = 0.45 / f32(max(samples, 1));
    for (var i = 0; i < samples; i = i + 1) {
        let h = (0.008 + span * f32(i)) * s;
        occ = occ + (h - map(p + n * h)) * weight / s;
        weight = weight * 0.82;
    }
    return clamp(1.0 - 2.6 * occ * (5.0 / f32(max(samples, 1))), 0.0, 1.0);
}

// The surface colour for an orbit-trap value, read from the baked gradient.
//
// Wrapped rather than clamped, so raising the colour speed sweeps the gradient
// repeatedly across the shape instead of flattening everything past the end
// into the last stop.
fn palette(t: f32) -> vec3<f32> {
    if (dm3_u.material.w < 0.5) {
        return dm3_u.surface.rgb;
    }
    let at = fract(t * dm3_u.material.y + dm3_u.material.x);
    let x = at * 63.0;
    let i = i32(floor(x));
    // Interpolated between entries: 64 steps are visible as banding on a
    // smooth surface otherwise.
    return mix(
        dm3_u.gradient[clamp(i, 0, 63)].rgb,
        dm3_u.gradient[clamp(i + 1, 0, 63)].rgb,
        fract(x));
}

// How defocused a pixel at distance `t` is, 0..1.
//
// Distance from the focal plane, divided by it, so the falloff is proportional
// — an aperture that reads well on a fractal an arm's length away still reads
// well on one across the scene. Rays that hit nothing are treated as far away,
// which puts the background at maximum blur where that is what is wanted.
fn circle_of_confusion(t: f32, hit: bool) -> f32 {
    let focus = dm3_u.dof.x;
    if (focus <= 0.0) {
        return 1.0;
    }
    let depth = select(dm3_u.march.y, t, hit);
    return clamp(abs(depth - focus) / focus * dm3_u.dof.y, 0.0, 1.0);
}

// Brightness, contrast, saturation and gamma, in that order.
//
// Applied to the finished image and nowhere else — none of this touches the
// geometry or the lighting, so a scene can be re-graded without re-deciding
// how it was lit. Contrast pivots about mid-grey; saturation blends toward
// luminance using the usual Rec. 709 weights.
fn adjust(c: vec3<f32>) -> vec3<f32> {
    var v = c * dm3_u.picture.x;
    v = (v - 0.5) * dm3_u.picture.y + 0.5;
    let luma = dot(max(v, vec3<f32>(0.0)), vec3<f32>(0.2126, 0.7152, 0.0722));
    v = mix(vec3<f32>(luma), v, dm3_u.picture.w);
    v = max(v, vec3<f32>(0.0));
    return pow(v, vec3<f32>(1.0 / dm3_u.picture.z));
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

// Colour along a secondary ray: one bounce, shaded plainly.
//
// No shadow, occlusion or fog on the bounce. Those cost their own marches and
// a reflection is already a mix of what it hits — spending three more marches
// per pixel to refine something at twenty percent strength is not a trade
// worth making at interactive rates.
fn reflected_color(ro: vec3<f32>, rd: vec3<f32>, eps_scale: f32, pixel_angle: f32) -> vec3<f32> {
    let bounds = max(dm3_u.march.w, 0.1);
    let b = dot(rd, ro);
    let disc = b * b - (dot(ro, ro) - bounds * bounds);
    if (disc < 0.0) {
        return background_color(rd);
    }

    var t = max(-b - sqrt(disc), 0.0);
    let max_steps = i32(dm3_u.march.x);
    var steps = 0;
    loop {
        if (steps >= max_steps) { break; }
        let p = ro + rd * t;
        let field = fractal_de(p);
        let d = field.dist * dm3_u.fractal.w;
        let eps = max(pixel_angle * t * eps_scale, 1e-6);
        if (d < eps) {
            let n = surface_normal(p, max(pixel_angle * t, 1e-5));
            // Every light, but none of their shadows — the whole point of this
            // function is that a bounce is not worth a second march per light.
            var lit = vec3<f32>(0.0);
            let count = i32(dm3_u.lights_meta.x);
            for (var li = 0; li < count; li = li + 1) {
                let s = light_sample(li, p);
                lit = lit + s.radiance * max(dot(n, s.dir), 0.0);
            }
            let sky = 0.5 + 0.5 * n.y;
            return palette(field.trap) * (lit + dm3_u.light.w * sky * dm3_u.light.rgb);
        }
        t = t + d;
        if (t > dm3_u.march.y) { break; }
        steps = steps + 1;
    }
    return background_color(rd) + visible_lights(ro, rd);
}

// The ray through a pixel, under the chosen projection.
//
// Only the three-point case has a flat image plane; the others map the frame
// onto angles, which is what lets one render cover a whole sphere or a dome.
fn camera_ray(ndc: vec2<f32>, aspect: f32, tan_half: f32) -> vec3<f32> {
    let fwd = dm3_u.cam_fwd.xyz;
    let right = dm3_u.cam_right.xyz;
    let up = dm3_u.cam_up.xyz;
    let mode = i32(dm3_u.view.x);

    // Equirectangular: longitude across the frame, latitude up it.
    if (mode == 2) {
        let lon = ndc.x * M_PI_F;
        let lat = ndc.y * M_PI_F * 0.5;
        return normalize(
            fwd * (cos(lat) * cos(lon))
            + right * (cos(lat) * sin(lon))
            + up * sin(lat));
    }

    // Fish eye and fulldome both sweep an angle outwards from the centre; they
    // differ only in how far the edge of the frame reaches.
    if (mode == 1 || mode == 3) {
        let v = vec2<f32>(ndc.x * aspect, ndc.y);
        let r = length(v);
        let edge = select(atan(tan_half) * 2.0, M_PI_F * 0.5, mode == 3);
        let theta = min(r, 1.0) * edge;
        let phi = atan2(v.y, v.x);
        return normalize(
            fwd * cos(theta)
            + (right * cos(phi) + up * sin(phi)) * sin(theta));
    }

    return normalize(fwd + right * ndc.x * aspect * tan_half + up * ndc.y * tan_half);
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let aspect = dm3_u.screen.x / max(dm3_u.screen.y, 1.0);
    let tan_half = dm3_u.cam_pos.w;

    let ro = dm3_u.cam_pos.xyz;
    let rd = camera_ray(in.ndc, aspect, tan_half);

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
        // Ray misses the fractal's bounding sphere entirely. Nothing can be in
        // the way, so the lights are drawn unconditionally.
        var bg = background_color(rd) + visible_lights(ro, rd);
        // The same tone map and grading every other pixel gets. Skipping them
        // here used to be invisible against a near-black backdrop, but it
        // prints the bounding sphere onto any bright background as a disc of
        // slightly the wrong colour.
        if (dm3_u.view.y < 0.5) { bg = bg / (1.0 + bg * 0.35); }
        bg = adjust(bg);
        if (dm3_u.screen.w > 0.5) { bg = linear_to_srgb(bg); }
        // `hit` is false, so the depth this reads is the far plane either way.
        return vec4<f32>(bg, circle_of_confusion(0.0, false));
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

        let base = palette(field.trap);
        // Reuse the primary ray's hit threshold so the shadow ray's scale
        // tracks both the zoom level and the size of the fractal.
        let surface_eps = max(pixel_angle * t * eps_scale, 1e-6);
        // Lift the origin off the surface along the normal, or the ray's first
        // samples are inside the geometry it started on.
        let tightness = de_tightness(p, n, surface_eps * 32.0);
        let shadow_origin = p + n * surface_eps * 3.0;
        // Highlight width is given as a fraction rather than an exponent: a
        // small number is a tight glint, which is the way it reads in the UI
        // and the way Mandelbulber states it.
        let sharpness = 2.0 / max(dm3_u.material.z * dm3_u.material.z, 1e-5);

        // Every light is gathered here. The diffuse and shadow terms below are
        // sums, so the diagnostics report the first light alone — the whole
        // point of those views is to isolate one thing at a time.
        var direct = vec3<f32>(0.0);
        var spec_sum = vec3<f32>(0.0);
        var diffuse = 0.0;
        var shadow = 1.0;
        let light_count = i32(dm3_u.lights_meta.x);
        for (var li = 0; li < light_count; li = li + 1) {
            let s = light_sample(li, p);
            // Shading strength dials down the angle-of-incidence term toward
            // flat lighting, which is Mandelbulber's "Shading" control.
            let lambert = max(dot(n, s.dir), 0.0);
            let shade_i = mix(1.0, lambert, clamp(dm3_u.surface.w, 0.0, 1.0));

            var shadow_i = 1.0;
            // A light too dim to see is not worth a shadow march, and at eight
            // lights that test is most of what keeps the frame rate.
            let strength = max(s.radiance.r, max(s.radiance.g, s.radiance.b));
            if (s.cast_shadows > 0.5 && strength > 1e-4) {
                shadow_i = soft_shadow(
                    shadow_origin,
                    s.dir,
                    surface_eps,
                    s.max_dist,
                    s.k,
                    tightness,
                    s.penetrating,
                );
            }

            direct = direct + s.radiance * shade_i * shadow_i;
            let h = normalize(s.dir - rd);
            spec_sum = spec_sum + s.radiance
                * pow(max(dot(n, h), 0.0), sharpness)
                * dm3_u.specular.w * shade_i * shadow_i;

            if (li == 0) {
                diffuse = shade_i;
                shadow = shadow_i;
            }
        }

        let occ = mix(1.0, ambient_occlusion(p, n), dm3_u.shade.x);
        // A tint on the occluded light rather than on the surface: occlusion
        // is light that did not arrive, so colouring it colours the shadowed
        // side without touching what is lit.
        let occ_tint = mix(dm3_u.ao_tint.rgb, vec3<f32>(1.0), occ);

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
        //
        // The fill-light colour tints it on top of that, which is what
        // Mandelbulber's "Fill light color" does; white leaves it alone.
        let sky = 0.5 + 0.5 * n.y;
        let peak = max(dm3_u.bg.r, max(dm3_u.bg.g, dm3_u.bg.b));
        let bg_tint = select(vec3<f32>(1.0), dm3_u.bg.rgb / peak, peak > 1e-4);
        let ambient = dm3_u.light.w * sky * dm3_u.light.rgb
            * mix(vec3<f32>(1.0), bg_tint, 0.5);

        // Occlusion attenuates the sky term only. Direct light already has
        // its own shadow term, and multiplying both by `occ` double-darkens —
        // barely visible on a sparse Mandelbulb, but it crushes a dense
        // Mandelbox to near-black.
        color = base * (direct + ambient * occ * occ_tint)
            + dm3_u.specular.rgb * spec_sum
            // Emission owes nothing to the light or to what shadows it.
            + dm3_u.emission.rgb * dm3_u.emission.w;

        // One reflected bounce, off the surface along the normal so the ray
        // does not immediately hit what it started on.
        if (dm3_u.reflection.w > 0.001) {
            let surface_offset = max(pixel_angle * t * eps_scale, 1e-6) * 3.0;
            let bounce = reflect(rd, n);
            let reflected = reflected_color(
                p + n * surface_offset,
                bounce,
                eps_scale,
                pixel_angle);
            color = mix(color, reflected * dm3_u.reflection.rgb, dm3_u.reflection.w);
        }

        // Fog laid on by iteration count rather than by distance: a point that
        // survived more iterations is deeper inside the structure, which is
        // what gives a fractal its sense of interior depth. `escape` is that
        // count as a fraction of the budget, already computed by the marcher.
        if (dm3_u.glow_b.w > 0.001) {
            let low = dm3_u.iter_fog_a.w;
            let high = dm3_u.iter_fog_b.w;
            let f = clamp((field.escape - low) / max(high - low, 1e-4), 0.0, 1.0);
            let tint = mix(dm3_u.iter_fog_a.rgb, dm3_u.iter_fog_b.rgb, f);
            color = mix(color, tint, f * dm3_u.glow_b.w);
        }

        // Fog toward the background, by depth into the bounding volume.
        let depth = clamp((t - entry_t) / (2.0 * bounds), 0.0, 1.0);
        color = mix(color, dm3_u.bg.rgb, 1.0 - exp(-dm3_u.bg.w * depth * 3.0));
    } else {
        let view = i32(dm3_u.fractal.x);
        if (view == 5) { return shade_out(heat(struggle)); }
        if (view != 0) { return shade_out(vec3<f32>(0.0)); }
        // Any light with a visibility above zero is drawn where the ray misses
        // everything, so a lamp can be seen rather than only inferred.
        color = background_color(rd) + visible_lights(ro, rd);
    }

    // Basic fog: haze closing in over a visibility distance, with its own
    // colour so it can sit in front of a dark sky rather than dissolving into
    // it. Measured from where the ray entered the bounding volume, so it
    // describes depth into the scene rather than distance from the camera.
    if (dm3_u.fog.w > 0.0001) {
        let travelled = max(t - entry_t, 0.0);
        color = mix(color, dm3_u.fog.rgb, 1.0 - exp(-travelled / dm3_u.fog.w));
    }

    // Glow applies to hit and miss alike — it is what makes the silhouette
    // wisp. It runs between two colours as the ray grazes closer, which is
    // what turns a flat halo into something with a hot core.
    let glow_tint = mix(dm3_u.glow_b.rgb, dm3_u.glow_a.rgb, struggle);
    color = color + glow_tint * struggle * struggle * dm3_u.glow_a.w;

    // Reinhard-ish tonemap keeps the specular from clipping to flat white.
    // Skipped in HDR, where the point is to let highlights run past it.
    if (dm3_u.view.y < 0.5) {
        color = color / (1.0 + color * 0.35);
    }

    color = adjust(color);

    if (dm3_u.screen.w > 0.5) {
        color = linear_to_srgb(color);
    }
    // Alpha carries this pixel's circle of confusion, not opacity. The blit
    // reads it to blur by the right amount without marching anything again.
    // Zero focus distance disables the effect, and then alpha is a plain 1.
    return vec4<f32>(color, circle_of_confusion(t, hit));
}

// ---------------------------------------------------------------------------
// Cursor probe
// ---------------------------------------------------------------------------

// Two numbers the CPU cannot work out for itself, read back from a 2x1 target.
//
// Pixel 0 marches the ray under the mouse and reports where it lands, which is
// what turns "zoom" into "go there": the wheel closes on a point the user chose
// rather than on the middle of the scene. Pixel 1 reports the clear space at
// the camera, which is what keeps forward flight from crossing a surface — a
// different question, since flight follows the view axis and the cursor
// usually does not.
@fragment
fn fs_probe(in: VsOut) -> @location(0) vec4<f32> {
    let ro = dm3_u.cam_pos.xyz;

    // Pixel 1: clear space straight ahead.
    if (in.clip_pos.x >= 1.0) {
        return vec4<f32>(fractal_de(ro).dist, 0.0, 0.0, 0.0);
    }

    // Pixel 0: march the cursor ray. The cursor's normalised position rides in
    // the spare w slots of the camera basis, which were padding.
    let aspect = dm3_u.screen.x / max(dm3_u.screen.y, 1.0);
    let tan_half = dm3_u.cam_pos.w;
    // The same projection the image uses, or the cross would point somewhere
    // the picture does not.
    let rd = camera_ray(vec2<f32>(dm3_u.cam_right.w, dm3_u.cam_up.w), aspect, tan_half);

    let pixel_angle = 2.0 * tan_half / max(dm3_u.screen.y, 1.0);
    let bounds = max(dm3_u.march.w, 0.1);
    let b = dot(rd, ro);
    let disc = b * b - (dot(ro, ro) - bounds * bounds);
    if (disc < 0.0) {
        // The cursor is off the fractal entirely; w < 0 says "no target".
        return vec4<f32>(ro, -1.0);
    }

    var t = max(-b - sqrt(disc), 0.0);
    let max_steps = i32(dm3_u.march.x);
    var steps = 0;
    loop {
        if (steps >= max_steps) { break; }
        let d = fractal_de(ro + rd * t).dist * dm3_u.fractal.w;
        let eps = max(pixel_angle * t * dm3_u.march.z, 1e-6);
        if (d < eps) {
            return vec4<f32>(ro + rd * t, t);
        }
        t = t + d;
        if (t > dm3_u.march.y) { break; }
        steps = steps + 1;
    }
    return vec4<f32>(ro, -1.0);
}
