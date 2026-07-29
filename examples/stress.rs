//! Repeats the app's per-frame work, headless, to find what accumulates.
//!
//!     cargo run --release --example stress -- [frames]
//!
//! Navigating for a while degrades 3DM into a near-freeze, and doing it faster
//! gets there sooner — which is what a per-operation leak looks like. A window
//! cannot be driven from a test, but everything the window does each frame can
//! be: move the camera, step the distance probe, render offscreen, blit,
//! submit. If something accumulates per frame, it accumulates here too.

use eframe::wgpu;
use three_dm::{
    Camera, SceneParams,
    render::{FractalPipeline, Uniforms},
};

/// Overridden from the command line, because the load has to resemble the one
/// that misbehaves: a full-screen Retina window is twenty-one megapixels, and a
/// comfortable 1280x800 exercises none of the same limits.
static mut SIZE: [u32; 2] = [1280, 800];
fn size() -> [u32; 2] {
    unsafe { SIZE }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let frames: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(3000);
    if let (Some(w), Some(h)) = (
        args.next().and_then(|s| s.parse().ok()),
        args.next().and_then(|s| s.parse().ok()),
    ) {
        unsafe { SIZE = [w, h] };
    }
    pollster::block_on(run(frames));
}

/// Resident set size in megabytes, via `ps` — good enough to see a trend.
fn rss_mb() -> f32 {
    let out = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok();
    out.and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|kb| kb / 1024.0)
        .unwrap_or(0.0)
}

async fn run(frames: u32) {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no suitable GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("3dm.stress.device"),
            ..Default::default()
        })
        .await
        .expect("could not create device");

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut pipeline = FractalPipeline::new(&device, format);

    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3dm.stress.target"),
        size: wgpu::Extent3d {
            width: size()[0],
            height: size()[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());

    let mut params = SceneParams::default();
    params.retune_for_stack();
    let (key, source) = params.shader();
    pipeline.ensure(&device, key, &source);

    let mut camera = Camera::default();
    camera.frame(params.fractal.bounding_radius);

    let start = std::time::Instant::now();
    let baseline = rss_mb();
    println!("frames  elapsed   ms/frame   RSS MB   delta");

    let mut last = std::time::Instant::now();
    for i in 0..frames {
        // Zoom in and out and orbit, the operations that provoke it.
        let phase = i as f32 * 0.05;
        camera.orbit(0.004, 0.0);
        camera.zoom(if phase.sin() > 0.0 { 1.01 } else { 1.0 / 1.01 });

        // Scale wanders the way the adaptive path made it, so any per-size
        // reallocation shows up.
        // DM3_SCALE=1 pins it to full size, for a load that resembles the app
        // at "always full" rather than an average of reduced frames.
        let scale = match std::env::var("DM3_SCALE").ok().and_then(|v| v.parse::<f32>().ok()) {
            Some(fixed) => fixed,
            None => 0.5 + 0.5 * (phase * 0.3).sin().abs(),
        };
        let render_size = [
            ((size()[0] as f32 * scale) as u32).max(16),
            ((size()[1] as f32 * scale) as u32).max(16),
        ];

        let uniforms = Uniforms::build(
            &camera,
            &params,
            [render_size[0] as f32, render_size[1] as f32],
            0.0,
            !format.is_srgb(),
            [0.0, 0.0],
        );
        pipeline.upload(&queue, &uniforms);

        let mut encoder = device.create_command_encoder(&Default::default());
        pipeline.step_probe(&mut encoder, key, true);
        pipeline.render_offscreen(&device, &queue, &mut encoder, key, size(), render_size);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3dm.stress.blit"),
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
        }
        queue.submit([encoder.finish()]);
        // Wait for the frame to finish, as presenting to a surface does. Without
        // this the loop submits far faster than the GPU drains and the backlog
        // grows without bound — which looks exactly like a memory leak, at a
        // steady megabyte a frame, and is nothing of the sort.
        device.poll(wgpu::PollType::wait_indefinitely()).ok();

        if (i + 1) % 25 == 0 {
            let now = std::time::Instant::now();
            let ms = now.duration_since(last).as_secs_f32() * 1000.0 / 25.0;
            last = now;
            let rss = rss_mb();
            println!(
                "{:>6}  {:>6.1}s  {:>8.2}   {:>6.1}  {:+.1}",
                i + 1,
                start.elapsed().as_secs_f32(),
                ms,
                rss,
                rss - baseline
            );
        }
    }
}
