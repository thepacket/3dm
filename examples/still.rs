//! Headless still renderer.
//!
//! Renders one frame of the default scene straight to a PNG with no window and
//! no egui, which makes it the quickest way to check the shader in CI or over
//! SSH — and it is the seed of the app's eventual high-resolution export.
//!
//!     cargo run --example still -- [out.png] [width] [height] [stack]
//!
//! `stack` selects one of the demo formula stacks below, which is the quickest
//! way to eyeball a hybrid without driving the UI.

use eframe::wgpu;
use three_dm::formulas::{Builtin, DeMode, FormulaKind, FormulaSlot, FormulaStack};
use three_dm::params::DebugView;
use three_dm::{
    Camera, SceneParams,
    render::{FractalPipeline, Uniforms},
};

/// `copy_texture_to_buffer` requires each row to start on a 256-byte boundary.
const COPY_ALIGN: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "still.png".to_owned());
    let width: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(960);
    let height: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(600);
    let stack = args.next().unwrap_or_else(|| "mandelbulb".to_owned());

    pollster::block_on(render(&path, width, height, &stack));
    println!("wrote {path} ({width}x{height}, stack: {stack})");
}

/// Demo stacks, chosen to exercise each distinct code path: a single
/// escape-time formula, a single linear one, and genuine hybrids.
fn preset(name: &str) -> FormulaStack {
    let mut stack = FormulaStack {
        slots: vec![],
        de_mode_override: None,
    };
    match name {
        "mandelbox" => stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox)),
        "sierpinski" => stack.slots.push(FormulaSlot::builtin(Builtin::Sierpinski)),
        "juliabulb" => stack.slots.push(FormulaSlot::builtin(Builtin::Juliabulb)),
        "mandelbulb" => stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbulb)),
        // A bulb for the first few iterations, then a box for the rest: the
        // disjoint ranges are what fuse two shapes instead of averaging them.
        "hybrid" => {
            let mut bulb = FormulaSlot::builtin(Builtin::Mandelbulb);
            bulb.end_iter = 3;
            let mut box_ = FormulaSlot::builtin(Builtin::Mandelbox);
            box_.start_iter = 3;
            stack.slots.push(bulb);
            stack.slots.push(box_);
        }
        // Rotating between folds is the classic way to break a Mandelbox's
        // axis alignment.
        "rotbox" => {
            stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox));
            stack.slots.push(FormulaSlot::builtin(Builtin::Rotate));
        }
        // MB3D's hybrid model, which a contiguous range cannot express: both
        // formulas cover the whole loop and take alternate iterations.
        "alternate" => {
            let mut bulb = FormulaSlot::builtin(Builtin::Mandelbulb);
            bulb.period = 2;
            bulb.phase = 0;
            let mut box_ = FormulaSlot::builtin(Builtin::Mandelbox);
            box_.period = 2;
            box_.phase = 1;
            stack.slots.push(bulb);
            stack.slots.push(box_);
        }
        // The same pair, cross-faded across the loop instead of alternating:
        // the bulb hands over to the box rather than interleaving with it.
        "crossfade" => {
            let mut bulb = FormulaSlot::builtin(Builtin::Mandelbulb);
            bulb.weight_start = 1.0;
            bulb.weight_end = 0.0;
            let mut box_ = FormulaSlot::builtin(Builtin::Mandelbox);
            box_.weight_start = 0.0;
            box_.weight_end = 1.0;
            stack.slots.push(bulb);
            stack.slots.push(box_);
        }
        // A rotation confined to one axis, which is the third thing a range
        // cannot say.
        "axisbox" => {
            stack.slots.push(FormulaSlot::builtin(Builtin::Mandelbox));
            let mut rot = FormulaSlot::builtin(Builtin::Rotate);
            rot.axes = [true, false, false, true];
            stack.slots.push(rot);
        }
        // Anything else names a transpiled Mandelbulber formula, which is the
        // whole point of this example now: rendering one is the only way to
        // tell a correct translation from a merely compiling one.
        other => match FormulaKind::find(other) {
            Some(kind) => stack.slots.push(FormulaSlot::new(kind)),
            None => panic!("unknown formula {other:?} — no builtin and no transpiled match"),
        },
    }
    stack
}

async fn render(path: &str, width: u32, height: u32, stack_name: &str) {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no suitable GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("3dm.still.device"),
            ..Default::default()
        })
        .await
        .expect("could not create device");

    // Surface the real reason for any shader or validation failure rather than
    // letting wgpu log it and carry on with a broken pipeline.
    device.on_uncaptured_error(std::sync::Arc::new(|e| panic!("wgpu error: {e}")));

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut pipeline = FractalPipeline::new(&device, format);

    let mut params = SceneParams::default();
    params.stack = preset(stack_name);
    // The transpiler does not recover which estimator a Mandelbulber formula
    // wants, so trying both is part of looking at a newly wired-up formula.
    params.stack.de_mode_override = match std::env::var("DM3_DE").as_deref() {
        Ok("log") => Some(DeMode::Log),
        Ok("linear") => Some(DeMode::Linear),
        _ => None,
    };
    params.retune_for_stack();

    // Diagnostic: strip everything that could darken the image, leaving the
    // palette and a plain lambert term. If a fractal is still dark here, the
    // problem is the distance estimate, not the shading.
    // DM3_VIEW renders one shading term directly instead of the final image.
    params.debug_view = match std::env::var("DM3_VIEW").as_deref() {
        Ok("normal") => DebugView::Normal,
        Ok("shadow") => DebugView::Shadow,
        Ok("occlusion") => DebugView::Occlusion,
        Ok("diffuse") => DebugView::Diffuse,
        Ok("steps") => DebugView::Steps,
        Ok("trap") => DebugView::Trap,
        _ => DebugView::Off,
    };

    match std::env::var("DM3_ISOLATE").as_deref() {
        Ok("flat") => {
            params.shading.ambient = 1.0;
            params.shading.ao_strength = 0.0;
            params.shading.glow = 0.0;
            params.shading.fog = 0.0;
            params.shading.specular = 0.0;
        }
        // Each of these disables exactly one term, so whichever one brightens
        // the image is the term responsible.
        Ok("noao") => params.shading.ao_strength = 0.0,
        Ok("nofog") => params.shading.fog = 0.0,
        Ok("noglow") => params.shading.glow = 0.0,
        Ok("noshadow") => {
            for l in &mut params.lights.lights {
                l.cast_shadows = false;
            }
        }
        Ok("amb1") => params.shading.ambient = 1.0,
        // Kills the sky term entirely: whatever is left is direct light only.
        Ok("amb0") => params.shading.ambient = 0.0,
        // Same elevation, but the light now comes from the camera's side of
        // the scene instead of grazing across it.
        // March far more conservatively. If this fixes the shading, the hit
        // points were landing inside the surface.
        Ok("safede") => params.fractal.de_scale = 0.25,
        // Give the orbit room to actually escape, so the linear DE reports a
        // real distance instead of "almost touching" everywhere.
        Ok("bigbail") => params.fractal.bailout = 1000.0,
        // Flat mid-grey albedo. If the fractal lights up here, the darkness
        // was the palette landing on a near-black colour, not the lighting.
        Ok("grey") => params.shading.palette_amp = [0.0, 0.0, 0.0],
        // Compensate the penumbra ratio for an underestimating DE. A narrow
        // cone is a large k, which is a hard-edged shadow.
        Ok("softk") => {
            for l in &mut params.lights.lights {
                l.soft_shadow_cone = 0.224;
            }
        }
        Ok("light2") => {
            params.shading.ambient = 0.0;
            params.lights.lights[0].direction = [0.6, 0.7, 0.4];
        }
        // Two more lights, to exercise the loop and the falloff: a red lamp
        // out to one side and a blue cone aimed back at the origin. The lamp
        // is given a visibility so its own disc is drawn as well, which is the
        // only part of the lighting that shows up against the background.
        Ok("lights3") => {
            params.lights.lights.push(three_dm::params::Light {
                kind: three_dm::params::LightKind::Point,
                color: [1.0, 0.25, 0.15],
                intensity: 2.0,
                position: [3.0, 0.5, 0.0],
                ..Default::default()
            });
            // Off the default camera's left shoulder, which is one of the few
            // directions with open background behind it — a light's own disc
            // is only drawn where the ray hits nothing.
            params.lights.lights.push(three_dm::params::Light {
                kind: three_dm::params::LightKind::Point,
                color: [1.0, 0.9, 0.6],
                intensity: 0.3,
                position: [0.2, -0.6, -2.8],
                visibility: 1.0,
                size: 3.0,
                ..Default::default()
            });
            params.lights.lights.push(three_dm::params::Light {
                kind: three_dm::params::LightKind::Cone,
                color: [0.2, 0.4, 1.0],
                intensity: 3.0,
                position: [-2.5, 2.5, -1.0],
                direction: [2.5, -2.5, 1.0],
                ..Default::default()
            });
        }
        _ => {}
    }
    // DM3_ITER and DM3_ZOOM exist to answer "does zooming in reveal more
    // detail, and what stops it" without driving the UI. DM3_ZOOM multiplies
    // the framed distance, so 0.02 is a 50x close-up.
    if let Ok(n) = std::env::var("DM3_ITER").map(|v| v.parse::<i32>()) {
        params.fractal.iterations = n.expect("DM3_ITER must be a number");
    }
    // DM3_PERSPECTIVE and DM3_PICTURE exist so the projections and the grading
    // can be checked without driving the UI.
    params.picture.perspective = match std::env::var("DM3_PERSPECTIVE").as_deref() {
        Ok("fisheye") => three_dm::params::Perspective::FishEye,
        Ok("equirect") => three_dm::params::Perspective::Equirectangular,
        Ok("fulldome") => three_dm::params::Perspective::Fulldome,
        _ => three_dm::params::Perspective::ThreePoint,
    };
    if let Ok(v) = std::env::var("DM3_PICTURE") {
        let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        if n.len() == 4 {
            params.picture.brightness = n[0];
            params.picture.contrast = n[1];
            params.picture.gamma = n[2];
            params.picture.saturation = n[3];
        }
    }
    if let Ok(v) = std::env::var("DM3_REFLECT").map(|v| v.parse::<f32>()) {
        params.material.reflectance = v.expect("DM3_REFLECT must be a number");
    }
    // "focus,aperture,maxblur,opacity"
    if let Ok(v) = std::env::var("DM3_DOF") {
        let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        if n.len() == 4 {
            params.picture.focus_distance = n[0];
            params.picture.aperture = n[1];
            params.picture.max_blur = n[2];
            params.picture.blur_opacity = n[3];
        }
    }
    // "visibility,iterfog" — the two volumetric terms, so they can be checked
    // without a window.
    if let Ok(v) = std::env::var("DM3_VOLUME") {
        let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        if n.len() == 2 {
            params.shading.fog_visibility = n[0];
            params.shading.iter_fog = n[1];
        }
    }
    let mut camera = Camera::default();
    camera.frame(params.fractal.bounding_radius);
    // DM3_PAN="dx,dy" moves the orbit target across the view, the way dragging
    // with the right button does — put it on a surface feature and DM3_ZOOM
    // then closes in on that feature rather than on the origin.
    if let Ok(p) = std::env::var("DM3_PAN") {
        let mut it = p
            .split(',')
            .map(|v| v.trim().parse::<f32>().expect("DM3_PAN is dx,dy"));
        camera.pan(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
    }
    // DM3_FLY descends along the view direction, the way W does in the app.
    if let Ok(f) = std::env::var("DM3_FLY").map(|v| v.parse::<f32>()) {
        camera.advance(f.expect("DM3_FLY must be a number"));
    }
    if let Ok(z) = std::env::var("DM3_ZOOM").map(|v| v.parse::<f32>()) {
        camera.zoom(z.expect("DM3_ZOOM must be a number"));
    }
    // DM3_FOV narrows the field of view without moving the eye, which is how to
    // magnify a long way without the camera burying itself in the solid — the
    // failure that makes DM3_ZOOM useless past a few hundred times. Magnifying
    // this way isolates *precision* from everything else: the eye stays at a
    // fixed distance, so coordinate magnitudes do not change, and the only thing
    // shrinking is the gap between neighbouring pixel rays. When that gap falls
    // under what f32 can resolve at these magnitudes, adjacent rays round to the
    // same position and the picture quantises into flat blocks. That is the
    // precision wall, and this is how to find it rather than calculate it.
    if let Ok(f) = std::env::var("DM3_FOV").map(|v| v.parse::<f32>()) {
        camera.fov_y = f.expect("DM3_FOV must be a number (degrees)");
    }
    let (key, source) = params.shader();
    pipeline.ensure(&device, key, &source);

    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3dm.still.target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());

    // DM3_BG picks the background: `gradient` for the three-colour sky, or a
    // path to an image to wrap around the scene. DM3_BG_MAP chooses how that
    // image is projected — the one part of this the colours cannot show.
    match std::env::var("DM3_BG").as_deref() {
        Ok("gradient") => {
            params.background.three_colors = true;
            params.background.color1 = [0.05, 0.18, 0.55];
            params.background.color2 = [0.75, 0.75, 0.75];
            params.background.color3 = [0.0, 0.15, 0.02];
        }
        Ok(path) if !path.is_empty() => {
            let decoded = image::open(path)
                .unwrap_or_else(|e| panic!("could not read background {path}: {e}"))
                .to_rgba8();
            let (w, h) = (decoded.width(), decoded.height());
            pipeline.set_background_image(&device, &queue, w, h, &decoded.into_raw());
            params.background.textured = true;
            params.background.map = match std::env::var("DM3_BG_MAP").as_deref() {
                Ok("double") => three_dm::params::BackgroundMap::DoubleHemisphere,
                Ok("flat") => three_dm::params::BackgroundMap::Flat,
                _ => three_dm::params::BackgroundMap::Equirectangular,
            };
        }
        _ => {}
    }

    // DM3_POST renders the way the window does: to an offscreen texture and
    // then through the blit, which is the only place depth of field, chromatic
    // aberration and the HDR blur happen. Absent, the direct render every
    // other diagnostic here compares against is what you get — and this path
    // is the seed of a real image export.
    //
    //   1          the blit alone, which should be a lossless copy
    //   hdr        HDR blur, with tone mapping off so highlights can bleed
    //   aberr      chromatic aberration; `aberr-rev` reverses its colours
    //   both       the two together, in the order the blit applies them
    // DM3_ENGINE exercises the Rendering engine controls, each of which
    // defaults to a value that leaves the marcher exactly as it was.
    match std::env::var("DM3_ENGINE").as_deref() {
        // The shape becomes the set itself rather than the estimate's level
        // surface, which shows up as sharper edges and iteration banding.
        Ok("maxiter") => params.engine.stop_at_max_iter = true,
        // A box narrower than the fractal, so the clipping is unmissable.
        Ok("limits") => {
            params.engine.limits = true;
            params.engine.limits_min = [-1.2, 0.0, -1.2];
            params.engine.limits_max = [1.2, 1.2, 1.2];
        }
        Ok("smooth") => params.engine.smoothness = 6.0,
        Ok("sharp") => params.engine.smoothness = 0.2,
        Ok("constdetail") => params.engine.constant_detail = 0.02,
        // A ceiling low enough that the marcher creeps instead of striding.
        Ok("capstep") => params.engine.abs_max_step = 0.02,
        _ => {}
    }

    let post_mode = std::env::var("DM3_POST").unwrap_or_default();
    let through_blit = !post_mode.is_empty();
    if matches!(post_mode.as_str(), "hdr" | "both") {
        params.post.hdr_blur = true;
        // Mandelbulber blurs the image before tone mapping, where a highlight
        // is genuinely brighter than white. This is the setting that leaves
        // those values intact, and without it the effect is far weaker.
        params.picture.hdr = true;
    }
    if matches!(post_mode.as_str(), "aberr" | "aberr-rev" | "both") {
        params.post.aberration = true;
        params.post.aberration_reverse = post_mode == "aberr-rev";
    }

    let uniforms = Uniforms::build(
        &camera,
        &params,
        [width as f32, height as f32],
        0.0,
        // The target is sRGB, so the hardware does the encoding for us.
        !format.is_srgb(),
        [0.0, 0.0],
    );
    pipeline.upload(&queue, &uniforms);

    // Round each row up to the copy alignment; we trim the padding on readback.
    let unpadded_bytes_per_row = width * 4;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(COPY_ALIGN) * COPY_ALIGN;

    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("3dm.still.readback"),
        size: (padded_bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // DM3_EXPORT drives the app's real export path instead of this example's
    // own readback: same pipeline call the Export button makes, same slot, same
    // mapping. Rendered at the same size it should be pixel-identical to
    // DM3_POST=1, which is the check worth having — the export is only useful
    // if it produces the picture the window shows.
    // DM3_EXPORT=<n> supersamples by n, exactly as the panel does: the frame
    // is rendered n times oversized and averaged down. The width and height
    // asked for on the command line are the *finished* size.
    if let Ok(mode) = std::env::var("DM3_EXPORT") {
        let aa: u32 = mode.parse().unwrap_or(1).clamp(1, 8);
        let (big_w, big_h) = (width * aa, height * aa);
        // Rebuilt at the supersampled size, or the marcher would chase detail
        // for a frame a fraction of the one it is drawing.
        let uniforms = Uniforms::build(
            &camera,
            &params,
            [big_w as f32, big_h as f32],
            0.0,
            !format.is_srgb(),
            [0.0, 0.0],
        );
        let slot = three_dm::render::ExportSlot::default();
        assert!(
            pipeline.export_frame(&device, &queue, key, [big_w, big_h], &uniforms, &slot),
            "export_frame refused the request"
        );
        // The app services this from its per-frame maintenance; here there is
        // no frame loop, so drive it directly until the mapping lands.
        let mut pixels = None;
        for _ in 0..1000 {
            device
                .poll(wgpu::PollType::wait_indefinitely())
                .expect("GPU poll failed");
            if let Some(got) = slot.take() {
                pixels = Some(got);
                break;
            }
        }
        let (w, h, rgba) = pixels.expect("the export never landed");
        assert_eq!((w, h), (big_w, big_h), "export came back the wrong size");
        let (w, h, mut rgba) = three_dm::params::downsample_srgb(&rgba, w, h, aa);
        assert_eq!(
            (w, h),
            (width, height),
            "downsample landed on the wrong size"
        );
        for pixel in rgba.chunks_exact_mut(4) {
            pixel[3] = 255;
        }
        image::save_buffer(path, &rgba, w, h, image::ColorType::Rgba8)
            .expect("could not write PNG");
        return;
    }

    let mut encoder = device.create_command_encoder(&Default::default());
    if through_blit {
        pipeline.render_offscreen(
            (&device, &queue),
            &mut encoder,
            key,
            ([width, height], [width, height]),
            uniforms.post(),
        );
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3dm.still.blit"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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
        pipeline.blit(&mut pass);
    } else {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3dm.still.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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
        pipeline.draw(&mut pass, key);
    }
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
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

    let (tx, rx) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |r| tx.send(r).ok().unwrap_or(()));
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("GPU poll failed");
    rx.recv().expect("map channel closed").expect("map failed");

    let mapped = readback.slice(..).get_mapped_range();
    let mut pixels = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
    for row in 0..height {
        let start = (row * padded_bytes_per_row) as usize;
        pixels.extend_from_slice(&mapped[start..start + unpadded_bytes_per_row as usize]);
    }
    // The shader puts the depth-of-field blur amount in alpha, which the blit
    // consumes. Nothing consumes it here, and a PNG would read it as opacity.
    //
    // DM3_COC writes it into the colour instead: the blur itself happens in the
    // blit, which only the windowed app runs, so this is the only way to check
    // the amount is right without a window.
    let show_coc = std::env::var("DM3_COC").is_ok();
    for pixel in pixels.chunks_exact_mut(4) {
        if show_coc {
            pixel[0] = pixel[3];
            pixel[1] = pixel[3];
            pixel[2] = pixel[3];
        }
        pixel[3] = 255;
    }
    drop(mapped);
    readback.unmap();

    // DM3_PROBE checks the distance probe end to end: the number it reads back
    // is what stops forward flight crossing the surface, and a probe that
    // silently returns nothing would fail open — the camera would fly straight
    // through exactly as it did before.
    if std::env::var("DM3_PROBE").is_ok() {
        for _ in 0..8 {
            let mut encoder = device.create_command_encoder(&Default::default());
            pipeline.step_probe(&mut encoder, key, true);
            queue.submit([encoder.finish()]);
            device.poll(wgpu::PollType::wait_indefinitely()).ok();
        }
        let eye = camera.position();
        let radius = (eye[0] * eye[0] + eye[1] * eye[1] + eye[2] * eye[2]).sqrt();
        // The cursor sits at screen centre here, so its ray is the view axis
        // and the two readings can be sanity-checked against each other.
        let cursor = match pipeline.cursor.target() {
            Some((p, d)) => format!(
                "cursor hit ({:.3}, {:.3}, {:.3}) at {d:.3}",
                p[0], p[1], p[2]
            ),
            None => "cursor over empty space".to_owned(),
        };
        println!(
            "probe: camera {radius:.3} from origin, clear space ahead {:.3}, {cursor}",
            pipeline.cursor.clearance()
        );
    }

    image::save_buffer(
        path,
        &pixels,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
    .expect("could not write PNG");
}
