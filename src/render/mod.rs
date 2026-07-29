//! The wgpu side of 3DM.
//!
//! The fractal is drawn as an egui paint callback: egui owns the render pass and
//! the surface, and we inject one fullscreen triangle into it. That keeps the UI
//! and the fractal in a single pass with no intermediate textures.
//!
//! Because the distance estimator is generated from the formula stack, there is
//! one pipeline per stack *structure*, cached by signature. Parameter edits
//! reuse a pipeline; adding or reordering formulas compiles a new one.

pub mod codegen;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

// Use eframe's re-export so we are guaranteed to be on the same wgpu version
// egui-wgpu was built against.
use eframe::egui_wgpu::{self, CallbackResources, CallbackTrait, ScreenDescriptor};
use eframe::wgpu;

use crate::camera::Camera;
use crate::formulas::{MAX_SLOTS, POOL_FLOATS_PER_SLOT, POOL_VEC4S_PER_SLOT};
use crate::params::{GRADIENT_LUT, Light, LightsParams, MAX_LIGHTS, SceneParams};

/// `vec4`s of parameter pool in the uniform block. Must match the array
/// length declared in `fractal.wgsl`.
pub const POOL_VEC4S: usize = MAX_SLOTS * POOL_VEC4S_PER_SLOT;

/// A single float per probe reading. `r32float` is renderable without any
/// optional feature, which matters because this has to work on the web too.
const PROBE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;

/// Two pixels: the cursor ray's landing point, and the clear space ahead.
const PROBE_WIDTH: u32 = 2;

/// Copies the offscreen fractal into egui's target.
///
/// The fractal is not drawn straight into the window. At this display's full
/// screen size that is 21.5 megapixels of raymarching, which costs about half a
/// second on empty space and one and a half once the fractal fills the frame —
/// so the window grows and the app stops being interactive. It is rendered
/// smaller instead and stretched, at a scale the app picks from how long the
/// last frames took.
const BLIT_WGSL: &str = r#"
@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
// xy = the fraction of the texture actually drawn this frame. The texture is
// allocated once at the window's size and a smaller frame occupies its
// top-left corner, so nothing is reallocated when the scale changes.
// xy = the fraction of the texture drawn, zw = half a texel of inset.
@group(0) @binding(2) var<uniform> region: vec4<f32>;
struct Post {
    // x = chromatic aberration, y = max blur as a fraction of height,
    // z = blur opacity. The per-pixel blur amount rides in the source alpha.
    // w = the aberration's smear along the radius, in pixels.
    a: vec4<f32>,
    // x = HDR blur radius (Mandelbulber's units), y = its intensity,
    // z = 1 to reverse the aberration's colour order.
    b: vec4<f32>,
};
@group(0) @binding(3) var<uniform> post: Post;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_blit(@builtin(vertex_index) vi: u32) -> VsOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let c = corners[vi];
    var out: VsOut;
    out.pos = vec4<f32>(c, 0.0, 1.0);
    // Flip y: clip space is y-up, texture space is y-down.
    out.uv = vec2<f32>(c.x, -c.y) * 0.5 + 0.5;
    return out;
}

// One texel of the offscreen texture, in uv. `region.zw` is half of it.
fn texel_size() -> vec2<f32> {
    return 2.0 * region.zw;
}

// The drawn frame's size in pixels, which the two filters below state their
// radii in. Derived rather than passed: `region` already carries both the
// fraction drawn and the texel size, and their ratio is exactly this.
fn drawn_pixels() -> vec2<f32> {
    return region.xy / max(texel_size(), vec2<f32>(1e-9));
}

// Bright areas bleeding into their surroundings.
//
// Mandelbulber weights every pixel within the radius by
// `1 / (r^2 / (0.2 * radius) + intensity)` and renormalises — so the centre
// tap is worth `1/intensity` and a low intensity leaves the pixel almost
// untouched. That weighting is reproduced exactly; what is not is the tap
// count. Mandelbulber walks the whole square, which at its default radius is
// some five hundred texture reads per pixel — fine for a still that renders
// for a minute, not for something redrawn while the camera moves. A Fibonacci
// spiral samples the same disc with a fixed budget, and because the weight
// falls off as 1/r^2 the taps that are skipped are the ones contributing
// least.
fn hdr_blur(uv: vec2<f32>, limit: vec2<f32>, color: vec3<f32>) -> vec3<f32> {
    let pixels = drawn_pixels();
    let radius = post.b.x * (pixels.x + pixels.y) * 0.001;
    if (radius < 0.5) {
        return color;
    }
    let intensity = max(post.b.y, 1e-4);
    let texel = texel_size();

    // The centre tap, at r = 0.
    var weight = 1.0 / intensity;
    var sum = color * weight;

    let taps = 48;
    for (var i = 0; i < taps; i = i + 1) {
        // Golden-angle spiral: even coverage of the disc with no ring
        // structure to alias against the fractal's own detail.
        let a = f32(i) * 2.39996;
        let r = radius * sqrt((f32(i) + 0.5) / f32(taps));
        let w = 1.0 / (r * r / (0.2 * radius) + intensity);
        let tap = min(
            max(uv + vec2<f32>(cos(a), sin(a)) * r * texel, vec2<f32>(0.0)),
            limit);
        sum = sum + textureSample(src, samp, tap).rgb * w;
        weight = weight + w;
    }
    return sum / weight;
}

// A lens disperses more the further from its axis, so the channels are spread
// radially from the centre rather than by a constant offset.
//
// Mandelbulber picks each channel from a different point along the radius and
// smears it over a blur radius; both are reproduced here by walking the radial
// line rather than its whole neighbourhood, which is where the cost was.
fn aberration(uv: vec2<f32>, limit: vec2<f32>, color: vec3<f32>) -> vec3<f32> {
    let centre = region.xy * 0.5;
    let from_centre = uv - centre;
    let pixels = drawn_pixels();
    let texel = texel_size();

    // Zero at the axis and growing outward, which is what makes the corners
    // fringe while the middle of the frame stays clean.
    let radial = from_centre / max(length(from_centre), 1e-6);
    let spread = post.a.x * length(from_centre / region.xy) * pixels.x * 0.007;
    let smear = max(post.a.w, 0.0) * 0.5;
    let reverse = select(1.0, -1.0, post.b.z > 0.5);

    var sum = vec3<f32>(0.0);
    var weight = vec3<f32>(0.0);
    let taps = 15;
    for (var i = 0; i < taps; i = i + 1) {
        // -1..1 along the radius, so the centre tap lands on the pixel itself.
        let t = (f32(i) / f32(taps - 1)) * 2.0 - 1.0;
        let along = t * (spread + smear);
        let tap = min(
            max(uv + radial * along * texel, vec2<f32>(0.0)),
            limit);
        let s = textureSample(src, samp, tap).rgb;

        // Red is picked from further out along the radius than blue — or the
        // other way round with the order reversed. Each channel's weight is a
        // triangle centred on its own offset.
        let width = max(spread * 0.5, 0.75);
        let w = vec3<f32>(
            max(1.0 - abs(along - spread * 0.5 * reverse) / width, 0.0),
            max(1.0 - abs(along) / width, 0.0),
            max(1.0 - abs(along + spread * 0.5 * reverse) / width, 0.0));
        sum = sum + s * w;
        weight = weight + w;
    }
    // A weight can fall to zero where the spread is smaller than one tap's
    // step; the untouched pixel is the right answer there.
    return select(color, sum / max(weight, vec3<f32>(1e-6)), weight > vec3<f32>(1e-6));
}

@fragment
fn fs_blit(in: VsOut) -> @location(0) vec4<f32> {
    // Clamped just inside the drawn region: without it the sampler's filter
    // reaches past the edge into whatever the rest of the texture holds.
    let limit = region.xy - region.zw;
    let uv = min(in.uv * region.xy, limit);

    var color = textureSample(src, samp, uv);

    // Depth of field. The fractal pass wrote each pixel's circle of confusion
    // into alpha, so the blur radius is already known here and costs one disc
    // of taps rather than a second march.
    if (post.a.z > 0.0001 && color.a > 0.001) {
        let radius = color.a * post.a.y;
        var sum = color.rgb;
        var weight = 1.0;
        // A twelve-tap spiral: enough to read as defocus rather than as a
        // pattern, cheap enough to run every frame.
        for (var i = 0; i < 12; i = i + 1) {
            let a = f32(i) * 2.39996;
            let r = radius * sqrt(f32(i + 1) / 12.0);
            let tap = min(max(uv + vec2<f32>(cos(a), sin(a)) * r, vec2<f32>(0.0)), limit);
            let s = textureSample(src, samp, tap);
            // Weighted by the tap's own blur, so a sharp foreground does not
            // smear itself across a background that is meant to be soft.
            let w = max(s.a, 0.05);
            sum += s.rgb * w;
            weight += w;
        }
        color = vec4<f32>(mix(color.rgb, sum / weight, post.a.z), color.a);
    }

    // Post effects, in Mandelbulber's own order: the bleed first, then the
    // lens. Swapping them would fringe the bloom rather than the picture.
    var rgb = color.rgb;
    if (post.b.x > 0.0001) {
        rgb = hdr_blur(uv, limit, rgb);
    }
    if (post.a.x > 0.0001) {
        rgb = aberration(uv, limit, rgb);
    }
    return vec4<f32>(rgb, 1.0);
}
"#;

/// GPU-side scene description. Laid out as `vec4`s so that WGSL's `uniform`
/// address space alignment rules are satisfied without any padding fields.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    cam_pos: [f32; 4],
    cam_right: [f32; 4],
    cam_up: [f32; 4],
    cam_fwd: [f32; 4],
    screen: [f32; 4],
    fractal: [f32; 4],
    march: [f32; 4],
    /// x = AO strength, y = spare, z = specular, w = glow.
    shade: [f32; 4],
    /// rgb = fill-light tint, w = ambient.
    light: [f32; 4],
    palette_base: [f32; 4],
    palette_amp: [f32; 4],
    bg: [f32; 4],
    /// x = brightness, y = contrast, z = gamma, w = saturation.
    picture: [f32; 4],
    /// x = perspective type, y = 1 when tone mapping is skipped,
    /// z = chromatic aberration, w = occlusion samples.
    view: [f32; 4],
    /// rgb = reflection tint, w = how much of the view reflects.
    reflection: [f32; 4],
    /// rgb = occlusion tint.
    ao_tint: [f32; 4],
    /// x = focus distance, y = aperture, z = max blur, w = blur opacity.
    dof: [f32; 4],
    /// rgb = fog colour, w = visibility distance.
    fog: [f32; 4],
    /// rgb = near glow colour, w = glow intensity.
    glow_a: [f32; 4],
    /// rgb = far glow colour, w = iteration-fog strength.
    glow_b: [f32; 4],
    /// rgb = iteration fog near colour, w = low trim.
    iter_fog_a: [f32; 4],
    /// rgb = iteration fog far colour, w = high trim.
    iter_fog_b: [f32; 4],
    // Field order from here down must match `fractal.wgsl` exactly. It is a
    // flat block of bytes with no names attached, so inserting a field on one
    // side and not the other silently shifts every offset past it — which
    // renders as a plausible-looking picture of the wrong thing, not an error.
    /// rgb = surface colour when the gradient is off, w = shading strength.
    surface: [f32; 4],
    /// rgb = specular colour, w = its brightness.
    specular: [f32; 4],
    /// rgb = emitted colour, w = how much of it.
    emission: [f32; 4],
    /// x = palette offset, y = colour speed, z = specular width,
    /// w = 1 when the gradient is in use.
    material: [f32; 4],
    /// The surface gradient, baked flat.
    gradient: [[f32; 4]; GRADIENT_LUT],
    /// Two per slot: how it is gated, then how it is blended. See
    /// `dm3_slot_applies` and friends in `fractal.wgsl`.
    ranges: [[f32; 4]; MAX_SLOTS * 2],
    pool: [[f32; 4]; POOL_VEC4S],
    /// x = how many of `lights` are live. Disabled lights are dropped on the
    /// way here rather than branched around per pixel.
    lights_meta: [f32; 4],
    /// [`LIGHT_VEC4S`] consecutive vec4s per light — see [`pack_light`].
    lights: [[f32; 4]; MAX_LIGHTS * LIGHT_VEC4S],
    /// rgb = background colour at the horizon, w = background brightness.
    bg2: [f32; 4],
    /// rgb = background colour below the horizon, w = background gamma.
    bg3: [f32; 4],
    /// x = 1 with a three-colour gradient, y = 1 when the image is in use,
    /// z = map type, w = horizontal scale.
    bg_flags: [f32; 4],
    /// x = vertical scale, yz = texture offset.
    bg_tex: [f32; 4],
    /// Rows of the rotation applied to a ray before it samples the image.
    bg_rot: [[f32; 4]; 3],
    /// x = HDR blur radius, y = its intensity, z = the aberration's smear,
    /// w = 1 to reverse the aberration's colour order.
    ///
    /// The fractal pass never reads this — the post effects happen in the
    /// blit. It rides here because this is the block every renderer already
    /// builds, so `still` and `stress` get them without a second channel.
    post_fx: [f32; 4],
    /// x = absolute minimum step, y = absolute maximum step, z = minimum step
    /// as a multiple of the hit threshold, w = maximum. Zero on a maximum
    /// means no limit; zero on a minimum means no floor.
    step_limits: [f32; 4],
    /// x = constant detail size (0 = track the pixel), y = detail ceiling,
    /// z = detail floor, w = the delta estimator's sampling offset.
    detail: [f32; 4],
    /// x = 1 to treat a non-escaping point as solid, y = normal smoothness,
    /// z = nearest distance a hit counts at, w = 1 when the box clips.
    engine: [f32; 4],
    /// The clipping box, when `engine.w` is set.
    limit_min: [f32; 4],
    limit_max: [f32; 4],
}

/// How many vec4s one light occupies. Must match `Light` in `fractal.wgsl`.
pub(crate) const LIGHT_VEC4S: usize = 5;

/// Flattens one light into the layout the shader reads.
///
/// `relative_to_camera` is resolved here rather than in the shader: the camera
/// basis is already to hand, and a light that rides with the camera is still
/// just a light at a world position once the frame is fixed.
fn pack_light(light: &Light, common: &LightsParams, camera: &Camera) -> [[f32; 4]; LIGHT_VEC4S] {
    let (right, up, fwd) = camera.basis();
    let eye = camera.position();
    let to_world = |v: [f32; 3], is_point: bool| -> [f32; 3] {
        if !light.relative_to_camera {
            return v;
        }
        let base = if is_point { eye } else { [0.0; 3] };
        [
            base[0] + right[0] * v[0] + up[0] * v[1] + fwd[0] * v[2],
            base[1] + right[1] * v[0] + up[1] * v[1] + fwd[1] * v[2],
            base[2] + right[2] * v[0] + up[2] * v[1] + fwd[2] * v[2],
        ]
    };

    let position = to_world(light.position, true);
    let direction = to_world(light.direction, false);
    // Half-angles: the panel states the full opening, and the soft angle is
    // the width of the fade outside it rather than a second absolute angle.
    let inner = (light.cone_angle * 0.5).to_radians();
    let outer = ((light.cone_angle + light.cone_soft_angle * 2.0) * 0.5).to_radians();

    [
        [
            light.color[0],
            light.color[1],
            light.color[2],
            light.intensity * common.all_intensity,
        ],
        [position[0], position[1], position[2], light.kind as u32 as f32],
        [
            direction[0],
            direction[1],
            direction[2],
            light.decay.exponent(),
        ],
        [
            (light.size * common.all_size).max(1e-4),
            light.contour_sharpness.max(1e-3),
            inner.cos(),
            outer.cos().min(inner.cos() - 1e-4),
        ],
        [
            if light.cast_shadows { 1.0 } else { 0.0 },
            if light.penetrating { 1.0 } else { 0.0 },
            light.shadow_k(),
            light.visibility,
        ],
    ]
}

impl Default for Uniforms {
    fn default() -> Self {
        bytemuck::Zeroable::zeroed()
    }
}

impl Uniforms {
    pub fn build(
        camera: &Camera,
        params: &SceneParams,
        viewport_px: [f32; 2],
        time: f32,
        encode_srgb: bool,
        // Cursor position in normalised device coords, y up.
        cursor_ndc: [f32; 2],
    ) -> Self {
        let eye = camera.position();
        let (right, up, fwd) = camera.basis();
        let f = &params.fractal;
        let m = &params.march;
        let s = &params.shading;
        let mat = &params.material;
        let pic = &params.picture;
        let bg = &params.background;
        let px = &params.post;
        let en = &params.engine;

        // Each slot's parameters go to the same fixed address the generated
        // shader reads them from; anything the formula does not use stays zero.
        let mut ranges = [[0.0f32; 4]; MAX_SLOTS * 2];
        let mut pool = [[0.0f32; 4]; POOL_VEC4S];
        for (index, slot) in params.stack.active() {
            ranges[index * 2] = [
                slot.start_iter as f32,
                slot.end_iter as f32,
                slot.period.max(1) as f32,
                slot.phase.max(0) as f32,
            ];
            ranges[index * 2 + 1] = [
                slot.weight_start,
                slot.weight_end,
                slot.axis_bits(),
                0.0,
            ];
            // Mandelbulber recomputes its derived parameters after every edit;
            // 3DM does it here, on the way to the GPU, so the editable values
            // and the ones the shader reads can share one block.
            let mut block: Vec<f32> = slot.params.clone();
            if let crate::formulas::FormulaKind::Generated(g) = slot.kind {
                crate::formulas::mb2::recompute(
                    &crate::formulas::generated::GENERATED[g],
                    &mut block,
                );
            }

            let base = index * POOL_FLOATS_PER_SLOT;
            for (i, v) in block.iter().take(POOL_FLOATS_PER_SLOT).enumerate() {
                pool[(base + i) / 4][(base + i) % 4] = *v;
            }
        }

        let mut lights = [[0.0f32; 4]; MAX_LIGHTS * LIGHT_VEC4S];
        let mut live = 0usize;
        for light in params.lights.lights.iter().filter(|l| l.enabled) {
            if live >= MAX_LIGHTS {
                break;
            }
            let packed = pack_light(light, &params.lights, camera);
            lights[live * LIGHT_VEC4S..(live + 1) * LIGHT_VEC4S].copy_from_slice(&packed);
            live += 1;
        }

        Self {
            cam_pos: [eye[0], eye[1], eye[2], camera.tan_half_fov()],
            // The basis `w` slots were padding; the cursor's normalised
            // position rides in them so the probe can build its ray.
            cam_right: [right[0], right[1], right[2], cursor_ndc[0]],
            cam_up: [up[0], up[1], up[2], cursor_ndc[1]],
            cam_fwd: [fwd[0], fwd[1], fwd[2], 0.0],
            screen: [
                viewport_px[0],
                viewport_px[1],
                time,
                if encode_srgb { 1.0 } else { 0.0 },
            ],
            // x carries the debug view; `power` now belongs to the formulas.
            fractal: [
                params.debug_view as u32 as f32,
                f.iterations as f32,
                f.bailout,
                f.de_scale,
            ],
            march: [
                m.max_steps as f32,
                m.max_dist,
                m.epsilon_scale,
                f.bounding_radius,
            ],
            shade: [s.ao_strength, 0.0, s.specular, s.glow],
            // Shadow softness moved onto the individual lights, so `light`
            // carries the fill tint the ambient term is coloured with and `y`
            // above is spare.
            light: [
                params.lights.fill_color[0],
                params.lights.fill_color[1],
                params.lights.fill_color[2],
                s.ambient,
            ],
            palette_base: [
                s.palette_base[0],
                s.palette_base[1],
                s.palette_base[2],
                s.palette_phase,
            ],
            palette_amp: [
                s.palette_amp[0],
                s.palette_amp[1],
                s.palette_amp[2],
                s.palette_freq,
            ],
            // Colour #1 alone, not the gradient: this doubles as the fog
            // target and the ambient sky tint, and both want one colour rather
            // than whatever happens to be overhead.
            bg: [bg.color1[0], bg.color1[1], bg.color1[2], s.fog],
            picture: [pic.brightness, pic.contrast, pic.gamma.max(1e-3), pic.saturation],
            view: [
                pic.perspective as u32 as f32,
                if pic.hdr { 1.0 } else { 0.0 },
                if px.aberration { px.aberration_intensity } else { 0.0 },
                s.ao_quality.clamp(1, 16) as f32,
            ],
            reflection: [
                mat.reflection_color[0],
                mat.reflection_color[1],
                mat.reflection_color[2],
                mat.reflectance,
            ],
            ao_tint: [s.ao_color[0], s.ao_color[1], s.ao_color[2], 0.0],
            dof: [
                pic.focus_distance,
                pic.aperture,
                pic.max_blur,
                pic.blur_opacity,
            ],
            fog: [s.fog_color[0], s.fog_color[1], s.fog_color[2], s.fog_visibility],
            glow_a: [s.glow_near[0], s.glow_near[1], s.glow_near[2], s.glow],
            glow_b: [s.glow_far[0], s.glow_far[1], s.glow_far[2], s.iter_fog],
            iter_fog_a: [
                s.iter_fog_near[0],
                s.iter_fog_near[1],
                s.iter_fog_near[2],
                s.iter_fog_low,
            ],
            iter_fog_b: [
                s.iter_fog_far[0],
                s.iter_fog_far[1],
                s.iter_fog_far[2],
                s.iter_fog_high.max(s.iter_fog_low + 1e-3),
            ],
            surface: [
                mat.single_color[0],
                mat.single_color[1],
                mat.single_color[2],
                mat.shading,
            ],
            specular: [
                mat.specular_color[0],
                mat.specular_color[1],
                mat.specular_color[2],
                mat.specular_brightness,
            ],
            emission: [
                mat.luminosity_color[0],
                mat.luminosity_color[1],
                mat.luminosity_color[2],
                mat.luminosity,
            ],
            material: [
                mat.palette_offset,
                mat.color_speed,
                mat.specular_width.max(1e-3),
                if mat.use_gradient { 1.0 } else { 0.0 },
            ],
            gradient: mat.lut(),
            ranges,
            pool,
            lights_meta: [live as f32, 0.0, 0.0, 0.0],
            lights,
            bg2: [bg.color2[0], bg.color2[1], bg.color2[2], bg.brightness],
            bg3: [bg.color3[0], bg.color3[1], bg.color3[2], bg.gamma.max(1e-3)],
            bg_flags: [
                if bg.three_colors { 1.0 } else { 0.0 },
                // Provisional. Whether an image is actually loaded is the
                // pipeline's to know, so `upload` clears this again if there
                // is nothing to sample — falling back to the colours beats
                // drawing a frame of flat white.
                if bg.textured { 1.0 } else { 0.0 },
                bg.map as u32 as f32,
                // A zero scale divides the texture coordinate by nothing.
                if bg.h_scale.abs() < 1e-4 { 1e-4 } else { bg.h_scale },

            ],
            bg_tex: [
                if bg.v_scale.abs() < 1e-4 { 1e-4 } else { bg.v_scale },
                bg.offset[0],
                bg.offset[1],
                0.0,
            ],
            bg_rot: bg.rotation_rows(),
            post_fx: [
                if px.hdr_blur { px.hdr_blur_radius.max(0.0) } else { 0.0 },
                px.hdr_blur_intensity.max(1e-4),
                px.aberration_blur.max(0.0),
                if px.aberration_reverse { 1.0 } else { 0.0 },
            ],
            step_limits: [
                en.abs_min_step.max(0.0),
                en.abs_max_step.max(0.0),
                en.rel_min_step.max(0.0),
                en.rel_max_step.max(0.0),
            ],
            detail: [
                en.constant_detail.max(0.0),
                en.max_detail.max(0.0),
                en.min_detail.max(0.0),
                en.delta_de_delta.max(1e-12),
            ],
            engine: [
                if en.stop_at_max_iter { 1.0 } else { 0.0 },
                en.smoothness.max(1e-3),
                en.min_view_distance.max(0.0),
                if en.limits { 1.0 } else { 0.0 },
            ],
            limit_min: [en.limits_min[0], en.limits_min[1], en.limits_min[2], 0.0],
            limit_max: [en.limits_max[0], en.limits_max[1], en.limits_max[2], 0.0],
        }
    }

    /// The eight floats the blit's post-effect uniform wants.
    ///
    /// Assembled here rather than at each call site so that `still`, `stress`
    /// and the app cannot disagree about the order.
    pub fn post(&self) -> [f32; 8] {
        [
            self.view[2],
            self.dof[2],
            // Depth of field is off entirely unless a focus distance was set.
            if self.dof[0] > 0.0 { self.dof[3] } else { 0.0 },
            self.post_fx[2],
            self.post_fx[0],
            self.post_fx[1],
            self.post_fx[3],
            0.0,
        ]
    }
}

/// Where the distance probe is in its copy/map cycle.
///
/// A buffer cannot be mapped while a copy into it is still in flight, so the
/// probe alternates: encode a copy one frame, ask for the mapping the next,
/// take the value when it lands. Three frames per reading at worst, which for
/// steering a camera is far more often than anyone can react.
mod probe_state {
    pub const IDLE: u8 = 0;
    pub const COPIED: u8 = 1;
    pub const MAPPING: u8 = 2;
}

/// Where the cursor is pointing, and how much room is ahead — read back from
/// the GPU because only the GPU can evaluate the estimator.
///
/// Two separate questions. The cursor's landing point is what lets the wheel
/// zoom somewhere the user chose rather than at the middle of the scene. The
/// clear space at the camera is what keeps forward flight from crossing a
/// surface, and it follows the view axis, which the cursor usually does not.
/// A finished export, waiting for the app to encode and save it.
///
/// The pixels arrive on a wgpu mapping callback, which is not the frame that
/// asked for them, so they land here and the app collects them next time it
/// looks. Same shape as [`CursorProbe`] and for the same reason.
/// An exported frame: width, height, and tightly packed RGBA.
pub type ExportImage = (u32, u32, Vec<u8>);

#[derive(Clone, Default)]
pub struct ExportSlot {
    inner: Arc<std::sync::Mutex<Option<ExportImage>>>,
    /// Set while a render is in flight, so a second click cannot start one on
    /// top of it and leave two mappings open.
    busy: Arc<AtomicU8>,
}

impl ExportSlot {
    /// Whether an export is still being rendered or read back.
    pub fn in_flight(&self) -> bool {
        self.busy.load(Ordering::Acquire) != 0
    }

    /// Takes the finished image, if one has landed: width, height, RGBA.
    pub fn take(&self) -> Option<ExportImage> {
        self.inner.lock().ok()?.take()
    }
}

#[derive(Clone)]
pub struct CursorProbe {
    /// World position under the cursor, then the ray distance to it. A
    /// negative distance means the cursor is not over the fractal.
    target: Arc<[AtomicU32; 4]>,
    clearance: Arc<AtomicU32>,
    state: Arc<AtomicU8>,
    /// Readbacks that have landed. If this stops climbing while the camera is
    /// moving, the probe has stalled mid-cycle and is holding a mapping open.
    completed: Arc<AtomicU32>,
}

impl CursorProbe {
    fn new() -> Self {
        Self {
            target: Arc::new([0, 0, 0, (-1.0f32).to_bits()].map(AtomicU32::new)),
            // Effectively unbounded until a reading lands, so nothing is
            // restricted by a measurement that does not exist yet.
            clearance: Arc::new(AtomicU32::new(f32::INFINITY.to_bits())),
            state: Arc::new(AtomicU8::new(probe_state::IDLE)),
            completed: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Readbacks completed, and whether a cycle is currently in flight.
    pub fn progress(&self) -> (u32, u8) {
        (
            self.completed.load(Ordering::Relaxed),
            self.state.load(Ordering::Relaxed),
        )
    }

    /// Clear space directly ahead of the camera, or infinity before the first
    /// reading arrives.
    pub fn clearance(&self) -> f32 {
        f32::from_bits(self.clearance.load(Ordering::Relaxed))
    }

    /// World point under the cursor, and how far along the ray it sits.
    /// `None` when the cursor is not over the fractal.
    pub fn target(&self) -> Option<([f32; 3], f32)> {
        let v: [f32; 4] =
            std::array::from_fn(|i| f32::from_bits(self.target[i].load(Ordering::Relaxed)));
        let distance: f32 = v[3];
        (distance > 0.0 && distance.is_finite()).then(|| ([v[0], v[1], v[2]], distance))
    }
}

/// A 1x1 stand-in so the background binding is never empty.
///
/// Its contents are never read: `upload` forces the textured-background flag
/// off whenever this is what is bound.
fn blank_texture(device: &wgpu::Device) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3dm.background.blank"),
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

/// The fractal pass's bind group. Rebuilt whenever a background image
/// replaces the one bound before it.
fn background_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniforms: &wgpu::Buffer,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("3dm.fractal.bg"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    })
}

/// Long-lived GPU objects, stashed in egui's `callback_resources` so they are
/// created once rather than per frame.
pub struct FractalPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    target_format: wgpu::TextureFormat,
    /// One compiled pipeline per formula-stack signature.
    pipelines: HashMap<u64, wgpu::RenderPipeline>,
    /// The 1x1 probe pipeline for each of those, sharing the shader module.
    probes: HashMap<u64, wgpu::RenderPipeline>,
    probe_target: wgpu::Texture,
    probe_view: wgpu::TextureView,
    probe_readback: wgpu::Buffer,
    /// Where the fractal is actually drawn, at or below the window's size.
    offscreen: Option<(wgpu::Texture, wgpu::TextureView, wgpu::BindGroup)>,
    offscreen_size: [u32; 2],
    blit: wgpu::RenderPipeline,
    blit_layout: wgpu::BindGroupLayout,
    blit_uniform: wgpu::Buffer,
    post_uniform: wgpu::Buffer,
    sampler: wgpu::Sampler,
    /// Viewport size the last frame was drawn at. A probe cycle spans three
    /// frames, and a surface reconfigure in the middle of one leaves a copy or
    /// a mapping straddling it.
    last_viewport: [f32; 2],
    pub cursor: CursorProbe,
    /// True when the surface format is linear and we must apply the sRGB
    /// transfer function in the shader ourselves.
    pub encode_srgb: bool,
    background_sampler: wgpu::Sampler,
    /// False while the blank 1x1 stand-in is bound, which is what stops the
    /// shader being told to sample an image that is not there.
    has_background_texture: bool,
}

impl FractalPipeline {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.fractal.uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("3dm.fractal.bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // The background image. Always bound — a shader cannot have an
                // optional binding, so with nothing loaded this is one white
                // pixel and `bg_flags.y` is forced off.
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let background_view =
            blank_texture(device).create_view(&wgpu::TextureViewDescriptor::default());
        let background_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("3dm.background.sampler"),
            // Repeating horizontally is what lets the horizontal scale tile a
            // panorama; vertically it would wrap the sky onto the ground.
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = background_bind_group(
            device,
            &bind_group_layout,
            &uniform_buffer,
            &background_view,
            &background_sampler,
        );

        // One float, but a copy destination has to be 256-byte aligned.
        let probe_target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("3dm.probe.target"),
            size: wgpu::Extent3d {
                width: PROBE_WIDTH,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: PROBE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let probe_view = probe_target.create_view(&wgpu::TextureViewDescriptor::default());
        let probe_readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.probe.readback"),
            size: wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let blit_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("3dm.blit.bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let blit_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.blit.region"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let post_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.blit.post"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3dm.blit.shader"),
            source: wgpu::ShaderSource::Wgsl(BLIT_WGSL.into()),
        });
        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3dm.blit.layout"),
            bind_group_layouts: &[Some(&blit_layout)],
            immediate_size: 0,
        });
        let blit = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.blit.pipeline"),
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader,
                entry_point: Some("vs_blit"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: Some("fs_blit"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        // Linear, so a reduced-resolution frame reads as soft rather than
        // blocky while the camera is moving.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("3dm.blit.sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            bind_group_layout,
            bind_group,
            uniform_buffer,
            target_format,
            offscreen: None,
            offscreen_size: [0, 0],
            blit,
            blit_layout,
            blit_uniform,
            post_uniform,
            sampler,
            pipelines: HashMap::new(),
            probes: HashMap::new(),
            probe_target,
            probe_view,
            probe_readback,
            last_viewport: [0.0, 0.0],
            cursor: CursorProbe::new(),
            encode_srgb: !target_format.is_srgb(),
            background_sampler,
            has_background_texture: false,
        }
    }

    /// Replaces the background image with `rgba`, which must be tightly packed.
    ///
    /// Rebuilds the bind group rather than writing into the old texture: a
    /// panorama that replaces another is rarely the same size, and this runs
    /// once per image rather than once per frame.
    pub fn set_background_image(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) {
        let size = wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("3dm.background.image"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // sRGB: the file's bytes are display-encoded, and everything the
            // shader mixes them with is linear.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.bind_group = background_bind_group(
            device,
            &self.bind_group_layout,
            &self.uniform_buffer,
            &view,
            &self.background_sampler,
        );
        self.has_background_texture = true;
    }

    /// Drops the background image, so the colours are drawn again.
    pub fn clear_background_image(&mut self, device: &wgpu::Device) {
        let view = blank_texture(device).create_view(&wgpu::TextureViewDescriptor::default());
        self.bind_group = background_bind_group(
            device,
            &self.bind_group_layout,
            &self.uniform_buffer,
            &view,
            &self.background_sampler,
        );
        self.has_background_texture = false;
    }

    /// Compiles the pipeline for `key` if it is not cached yet.
    pub fn ensure(&mut self, device: &wgpu::Device, key: u64, source: &str) {
        if self.pipelines.contains_key(&key) {
            return;
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3dm.fractal.shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3dm.fractal.layout"),
            bind_group_layouts: &[Some(&self.bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.fractal.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.target_format,
                    // The fractal is the backdrop of its panel and always opaque.
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let probe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.probe.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_probe"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: PROBE_FORMAT,
                    blend: None,
                    // All four: the cursor pixel carries a position *and* a
                    // distance. Masked to RED, as it was when this target held
                    // one number, the position reads back as (x, 0, 0) and the
                    // distance as whatever the clear left behind.
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        self.pipelines.insert(key, pipeline);
        self.probes.insert(key, probe);
    }

    /// Draws the fractal into the offscreen target, creating it if the size
    /// changed. Returns false if there is nothing to draw yet.
    pub fn render_offscreen(
        &mut self,
        gpu: (&wgpu::Device, &wgpu::Queue),
        encoder: &mut wgpu::CommandEncoder,
        key: u64,
        // Texture size, then the part of it this frame draws into.
        size: ([u32; 2], [u32; 2]),
        post: [f32; 8],
    ) -> bool {
        let (device, queue) = gpu;
        let (full, size) = size;
        if !self.pipelines.contains_key(&key) {
            return false;
        }
        // Allocated at the window's size and only when *that* changes. Sizing
        // it to the reduced frame instead meant a new texture, view and bind
        // group on every frame the scale moved — which is every frame while the
        // camera is moving. Creating and destroying a multi-megapixel texture
        // that often costs far more than the pixels it saves, which is why
        // reducing the resolution could make the app slower rather than faster.
        if self.offscreen.is_none() || self.offscreen_size != full {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("3dm.offscreen"),
                size: wgpu::Extent3d {
                    width: full[0],
                    height: full[1],
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.target_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("3dm.blit.bg"),
                layout: &self.blit_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.blit_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: self.post_uniform.as_entire_binding(),
                    },
                ],
            });
            self.offscreen = Some((texture, view, bind_group));
            self.offscreen_size = full;
        }

        // Which corner of the texture this frame occupies, and half a texel of
        // inset for the sampler to stay inside it.
        let fraction = [
            size[0] as f32 / full[0] as f32,
            size[1] as f32 / full[1] as f32,
        ];
        queue.write_buffer(
            &self.post_uniform,
            0,
            bytemuck::cast_slice(&post),
        );
        queue.write_buffer(
            &self.blit_uniform,
            0,
            bytemuck::cast_slice(&[
                fraction[0],
                fraction[1],
                0.5 / full[0] as f32,
                0.5 / full[1] as f32,
            ]),
        );

        let Some((_, view, _)) = &self.offscreen else {
            return false;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3dm.offscreen.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        // Draw only into the corner the reduced frame occupies.
        pass.set_viewport(0.0, 0.0, size[0] as f32, size[1] as f32, 0.0, 1.0);
        self.draw(&mut pass, key);
        true
    }

    /// Stretches the offscreen frame across egui's target.
    pub fn blit(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        let Some((_, _, bind_group)) = &self.offscreen else {
            return;
        };
        render_pass.set_pipeline(&self.blit);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    /// Advances the probe one step through its copy/map cycle.
    ///
    /// Encoded into egui's own command encoder, so the copy is submitted with
    /// the frame; the mapping is requested a frame later, once that submission
    /// has certainly happened.
    pub fn step_probe(&self, encoder: &mut wgpu::CommandEncoder, key: u64, stable: bool) {
        let probe = &self.cursor;
        match probe.state.load(Ordering::Acquire) {
            probe_state::IDLE => {
                // Never begin a cycle on a frame where the viewport changed
                // size. The surface is reconfigured on those frames, and
                // reconfiguring waits for outstanding GPU work — a copy or a
                // mapping left in flight across that point is exactly the kind
                // of thing that waits forever. Skipping costs one stale
                // reading; the camera does not notice.
                if !stable {
                    return;
                }
                let Some(pipeline) = self.probes.get(&key) else {
                    return;
                };
                {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("3dm.probe.pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &self.probe_view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &self.bind_group, &[]);
                    pass.draw(0..3, 0..1);
                }
                encoder.copy_texture_to_buffer(
                    self.probe_target.as_image_copy(),
                    wgpu::TexelCopyBufferInfo {
                        buffer: &self.probe_readback,
                        layout: wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT),
                            rows_per_image: Some(1),
                        },
                    },
                    wgpu::Extent3d {
                        width: PROBE_WIDTH,
                        height: 1,
                        depth_or_array_layers: 1,
                    },
                );
                probe.state.store(probe_state::COPIED, Ordering::Release);
            }
            probe_state::COPIED => {
                probe.state.store(probe_state::MAPPING, Ordering::Release);
                let target = Arc::clone(&probe.target);
                let clearance = Arc::clone(&probe.clearance);
                let state = Arc::clone(&probe.state);
                let completed = Arc::clone(&probe.completed);
                let buffer = self.probe_readback.clone();
                self.probe_readback.slice(..).map_async(
                    wgpu::MapMode::Read,
                    move |result| {
                        if result.is_ok() {
                            let raw = buffer.slice(..).get_mapped_range();
                            let at = |i: usize| {
                                let b = i * 4;
                                f32::from_le_bytes([raw[b], raw[b + 1], raw[b + 2], raw[b + 3]])
                            };
                            // Pixel 0 is the cursor's landing point, pixel 1 the
                            // clear space at the camera.
                            for i in 0..4 {
                                target[i].store(at(i).to_bits(), Ordering::Relaxed);
                            }
                            let ahead = at(4);
                            drop(raw);
                            buffer.unmap();
                            if ahead.is_finite() && ahead >= 0.0 {
                                clearance.store(ahead.to_bits(), Ordering::Relaxed);
                            }
                        }
                        completed.fetch_add(1, Ordering::Relaxed);
                        state.store(probe_state::IDLE, Ordering::Release);
                    },
                );
                // Deliberately no `device.poll` here. This runs inside egui's
                // paint callback, with an encoder open, and driving device
                // maintenance re-entrantly from that point is asking for
                // trouble — the mapping is serviced by the normal per-frame
                // maintenance a moment later instead.
            }
            _ => {}
        }
    }

    /// Renders one frame at an arbitrary size and reads it back.
    ///
    /// Everything here is its own: its own textures, sized to the export
    /// rather than the window, and — crucially — **its own submission**. The
    /// uniform, region and post buffers are shared with the live frame, and a
    /// `write_buffer` is applied once per submit, so recording the export into
    /// egui's encoder would leave both passes reading whichever set of
    /// uniforms was written last. Submitting separately, before the caller
    /// re-uploads the live scene, keeps the two frames from overwriting each
    /// other.
    ///
    /// The readback is mapped without polling here, exactly as the cursor
    /// probe does: the app's per-frame device maintenance services it a moment
    /// later. Polling from inside a paint callback with an encoder open is
    /// what an earlier version of the probe did, and it stalled the app.
    pub fn export_frame(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        key: u64,
        size: [u32; 2],
        uniforms: &Uniforms,
        slot: &ExportSlot,
    ) -> bool {
        if !self.pipelines.contains_key(&key) || slot.in_flight() {
            return false;
        }
        let [width, height] = size;
        if width == 0 || height == 0 {
            return false;
        }
        slot.busy.store(1, Ordering::Release);

        let descriptor = |label, usage| wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.target_format,
            usage,
            view_formats: &[],
        };
        // The fractal is drawn here, then the blit reads it — which is what
        // puts depth of field, chromatic aberration and the HDR blur into an
        // exported image rather than only on screen.
        let scene = device.create_texture(&descriptor(
            "3dm.export.scene",
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        ));
        let out = device.create_texture(&descriptor(
            "3dm.export.out",
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        ));
        let scene_view = scene.create_view(&wgpu::TextureViewDescriptor::default());
        let out_view = out.create_view(&wgpu::TextureViewDescriptor::default());

        let blit_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("3dm.export.blit.bg"),
            layout: &self.blit_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&scene_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.blit_uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.post_uniform.as_entire_binding(),
                },
            ],
        });

        self.upload(queue, uniforms);
        // The whole texture is drawn, so the region is the whole texture.
        queue.write_buffer(
            &self.blit_uniform,
            0,
            bytemuck::cast_slice(&[1.0f32, 1.0, 0.5 / width as f32, 0.5 / height as f32]),
        );
        queue.write_buffer(&self.post_uniform, 0, bytemuck::cast_slice(&uniforms.post()));

        let padded = (width * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.export.readback"),
            size: (padded as u64) * (height as u64),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("3dm.export.encoder"),
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3dm.export.scene.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &scene_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.draw(&mut pass, key);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3dm.export.blit.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &out_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.blit);
            pass.set_bind_group(0, &blit_bind, &[]);
            pass.draw(0..3, 0..1);
        }
        encoder.copy_texture_to_buffer(
            out.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        queue.submit([encoder.finish()]);

        let result = Arc::clone(&slot.inner);
        let busy = Arc::clone(&slot.busy);
        let buffer = readback.clone();
        readback.slice(..).map_async(wgpu::MapMode::Read, move |r| {
            if r.is_ok() {
                let raw = buffer.slice(..).get_mapped_range();
                // Drop the row padding the copy alignment forced on us.
                let row = (width * 4) as usize;
                let mut rgba = Vec::with_capacity(row * height as usize);
                for y in 0..height as usize {
                    let start = y * padded as usize;
                    rgba.extend_from_slice(&raw[start..start + row]);
                }
                drop(raw);
                buffer.unmap();
                if let Ok(mut slot) = result.lock() {
                    *slot = Some((width, height, rgba));
                }
            }
            busy.store(0, Ordering::Release);
        });
        true
    }

    /// Push a new scene to the GPU. Cheap enough to do every frame.
    pub fn upload(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        let mut uniforms = *uniforms;
        // The scene can ask for a textured background before an image has been
        // dropped on the window. The bound texture is 1x1 white in that case,
        // which would wash the whole frame out.
        if !self.has_background_texture {
            uniforms.bg_flags[1] = 0.0;
        }
        let uniforms = &uniforms;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    /// Issue the fullscreen triangle into an already-configured pass. The
    /// caller owns the viewport, so this works both inside egui's pass and in
    /// a standalone offscreen pass.
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>, key: u64) {
        let Some(pipeline) = self.pipelines.get(&key) else {
            return;
        };
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    /// Bytes of GPU memory wgpu is currently holding, if the backend reports
    /// it. A number that only ever grows while navigating is a leak.
    pub fn allocated_bytes(&self, device: &wgpu::Device) -> Option<u64> {
        let report = device.generate_allocator_report()?;
        Some(report.total_allocated_bytes)
    }

    /// Number of distinct stack structures compiled so far — surfaced in the UI
    /// so the cost of shader rebuilds is visible rather than mysterious.
    pub fn compiled_count(&self) -> usize {
        self.pipelines.len()
    }
}

/// Per-frame payload handed to egui.
pub struct FractalCallback {
    pub uniforms: Uniforms,
    /// Viewport size in physical pixels, used to spot a resize.
    pub viewport: [f32; 2],
    /// Size the fractal is actually rendered at, at or below the viewport.
    pub render_size: [u32; 2],
    pub shader_key: u64,
    /// Shared with the app so an unchanged stack costs no allocation.
    pub shader_source: Arc<str>,
    /// A newly dropped background image, as tightly packed RGBA. Present on
    /// exactly the one frame that follows the drop.
    pub background: Option<(u32, u32, Vec<u8>)>,
    /// An export to render this frame, if the button was pressed.
    pub export: Option<ExportRequest>,
}

/// One frame to render at its own size and read back.
///
/// The uniforms are built separately from the live frame's because the hit
/// threshold comes from a pixel's angular size: reusing the window's would
/// give a 4K export the detail of a preview.
pub struct ExportRequest {
    pub size: [u32; 2],
    pub uniforms: Box<Uniforms>,
    pub slot: ExportSlot,
}

impl CallbackTrait for FractalCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(res) = resources.get_mut::<FractalPipeline>() {
            res.ensure(device, self.shader_key, &self.shader_source);
            // Before the upload: `upload` reads `has_background_texture` to
            // decide whether the shader may sample it.
            if let Some((w, h, rgba)) = &self.background {
                res.set_background_image(device, queue, *w, *h, rgba);
            }
            // Also before, and in its own submission: it writes the same
            // uniform buffers this frame is about to, so it has to be finished
            // with them before the live scene goes up.
            if let Some(export) = &self.export {
                res.export_frame(
                    device,
                    queue,
                    self.shader_key,
                    export.size,
                    &export.uniforms,
                    &export.slot,
                );
            }
            res.upload(queue, &self.uniforms);
            let stable = res.last_viewport == self.viewport;
            res.last_viewport = self.viewport;
            res.step_probe(encoder, self.shader_key, stable);
            res.render_offscreen(
                (device, queue),
                encoder,
                self.shader_key,
                (
                    [self.viewport[0] as u32, self.viewport[1] as u32],
                    self.render_size,
                ),
                self.uniforms.post(),
            );
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &CallbackResources,
    ) {
        let Some(res) = resources.get::<FractalPipeline>() else {
            return;
        };
        // egui has already set the viewport to our widget's rect, so the
        // fullscreen triangle lands exactly inside the panel.
        res.blit(render_pass);
    }
}

/// Convenience wrapper matching egui's paint-callback shape.
pub fn callback(rect: egui::Rect, frame: FractalCallback) -> egui::PaintCallback {
    egui_wgpu::Callback::new_paint_callback(rect, frame)
}
