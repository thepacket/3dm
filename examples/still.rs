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
use three_dm::formulas::{FormulaKind, FormulaSlot, FormulaStack};
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
        "mandelbox" => stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbox)),
        "sierpinski" => stack.slots.push(FormulaSlot::new(FormulaKind::Sierpinski)),
        "juliabulb" => stack.slots.push(FormulaSlot::new(FormulaKind::Juliabulb)),
        // A bulb for the first few iterations, then a box for the rest: the
        // disjoint ranges are what fuse two shapes instead of averaging them.
        "hybrid" => {
            let mut bulb = FormulaSlot::new(FormulaKind::Mandelbulb);
            bulb.end_iter = 3;
            let mut box_ = FormulaSlot::new(FormulaKind::Mandelbox);
            box_.start_iter = 3;
            stack.slots.push(bulb);
            stack.slots.push(box_);
        }
        // Rotating between folds is the classic way to break a Mandelbox's
        // axis alignment.
        "rotbox" => {
            stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbox));
            stack.slots.push(FormulaSlot::new(FormulaKind::Rotate));
        }
        _ => stack.slots.push(FormulaSlot::new(FormulaKind::Mandelbulb)),
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
        // A very large softness makes k*h/t exceed 1 everywhere, i.e. no shadow.
        Ok("noshadow") => params.shading.shadow_softness = 1.0e6,
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
        // Compensate the penumbra ratio for an underestimating DE.
        Ok("softk") => params.shading.shadow_softness = 256.0,
        Ok("light2") => {
            params.shading.ambient = 0.0;
            params.shading.light_dir = [0.6, 0.7, 0.4];
        }
        _ => {}
    }
    let mut camera = Camera::default();
    camera.frame(params.fractal.bounding_radius);
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

    pipeline.upload(
        &queue,
        &Uniforms::build(
            &camera,
            &params,
            [width as f32, height as f32],
            0.0,
            // The target is sRGB, so the hardware does the encoding for us.
            !format.is_srgb(),
        ),
    );

    // Round each row up to the copy alignment; we trim the padding on readback.
    let unpadded_bytes_per_row = width * 4;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(COPY_ALIGN) * COPY_ALIGN;

    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("3dm.still.readback"),
        size: (padded_bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&Default::default());
    {
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
    device.poll(wgpu::PollType::wait_indefinitely()).expect("GPU poll failed");
    rx.recv().expect("map channel closed").expect("map failed");

    let mapped = readback.slice(..).get_mapped_range();
    let mut pixels = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
    for row in 0..height {
        let start = (row * padded_bytes_per_row) as usize;
        pixels.extend_from_slice(&mapped[start..start + unpadded_bytes_per_row as usize]);
    }
    drop(mapped);
    readback.unmap();

    image::save_buffer(
        path,
        &pixels,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
    .expect("could not write PNG");
}
