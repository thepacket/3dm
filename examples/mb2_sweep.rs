//! Renders every transpiled formula and reports which ones produce a picture.
//!
//! "Compiles" and "renders" are different claims, and the gap between them is
//! where the remaining risk in the formula stack lives. This builds a real wgpu
//! pipeline per formula, draws a small frame, and classifies the result — so a
//! formula that compiles into an empty screen, or into a solid block, is
//! counted as the failure it is.
//!
//!     cargo run --release --example mb2_sweep [-- <out-dir>]
//!
//! With an output directory it also writes a PNG per formula that rendered,
//! which is the only way to judge whether the shape is actually *right*.

use eframe::wgpu;
use three_dm::formulas::{FormulaKind, FormulaSlot, FormulaStack, generated::GENERATED};
use three_dm::{
    Camera, SceneParams,
    render::{FractalPipeline, Uniforms},
};

const W: u32 = 192;
const H: u32 = 144;
const COPY_ALIGN: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Verdict {
    /// The pipeline failed to build.
    ShaderError,
    /// Nothing was drawn — the ray never hit anything.
    Empty,
    /// The fractal filled essentially the whole frame, which in practice means
    /// the camera is inside a solid or the estimate collapsed to zero.
    Solid,
    /// Geometry, but smooth — no fractal detail at any scale. Usually a bare
    /// sphere. Many Mandelbulber formulas are transforms meant to be stacked
    /// with others and genuinely do look like this alone, so this is not
    /// automatically a bug; it does mean the tile proves nothing.
    Featureless,
    Rendered,
}

fn main() {
    let out_dir = std::env::args().nth(1);
    pollster::block_on(run(out_dir.as_deref()));
}

async fn run(out_dir: Option<&str>) {
    if let Some(d) = out_dir {
        std::fs::create_dir_all(d).expect("cannot create output directory");
    }

    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no suitable GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("3dm.sweep.device"),
            ..Default::default()
        })
        .await
        .expect("could not create device");

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3dm.sweep.target"),
        size: wgpu::Extent3d {
            width: W,
            height: H,
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

    let padded = (W * 4).div_ceil(COPY_ALIGN) * COPY_ALIGN;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("3dm.sweep.readback"),
        size: (padded * H) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut pipeline = FractalPipeline::new(&device, format);
    let mut tally = [0usize; 5];
    let mut failures = Vec::new();
    // Kept for the contact sheet. 300 tiles at this size is a few tens of MB,
    // which is cheaper than decoding the PNGs back off disk.
    let mut sheet: Vec<(&str, Vec<u8>)> = Vec::new();

    for (index, f) in GENERATED.iter().enumerate() {
        let mut params = SceneParams {
            stack: FormulaStack {
                slots: vec![FormulaSlot::new(FormulaKind::Generated(index))],
                de_mode_override: None,
            },
            ..Default::default()
        };
        params.retune_for_stack();
        // A sweep is about coverage, not beauty; a small budget keeps 361
        // pipelines to a sane runtime.
        params.fractal.iterations = 10;
        params.march.max_steps = 96;

        let mut camera = Camera::default();
        camera.frame(params.fractal.bounding_radius);
        let (key, source) = params.shader();

        // A formula that fails to build must not take the sweep down with it,
        // so pipeline creation runs inside an error scope rather than under the
        // panicking uncaptured-error handler.
        let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        pipeline.ensure(&device, key, &source);
        let built = scope.pop().await.is_none();

        let verdict = if !built {
            Verdict::ShaderError
        } else {
            pipeline.upload(
                &queue,
                &Uniforms::build(&camera, &params, [W as f32, H as f32], 0.0, !format.is_srgb()),
            );
            let pixels = draw(&device, &queue, &view, &target, &readback, &mut pipeline, key).await;
            let verdict = classify(&pixels, out_dir, f.name);
            if verdict == Verdict::Rendered {
                sheet.push((f.name, pixels));
            }
            verdict
        };

        tally[verdict as usize] += 1;
        if verdict != Verdict::Rendered {
            failures.push((f.name, verdict));
        }
    }

    println!("\nswept {} transpiled formulas at {W}x{H}", GENERATED.len());
    println!("  fractal detail  {}", tally[Verdict::Rendered as usize]);
    println!("  smooth blob     {}", tally[Verdict::Featureless as usize]);
    println!("  empty frame     {}", tally[Verdict::Empty as usize]);
    println!("  solid frame     {}", tally[Verdict::Solid as usize]);
    println!("  shader error    {}", tally[Verdict::ShaderError as usize]);

    if !failures.is_empty() {
        println!("\nnot rendering:");
        for (name, why) in &failures {
            println!("  {name:34} {why:?}");
        }
    }

    if let Some(dir) = out_dir {
        contact_sheet(&sheet, dir);
    }
}

/// Composites every rendered formula into one grid, plus a text index naming
/// the tiles in reading order.
///
/// Three hundred separate files is not something anyone will page through, and
/// judging whether a translation is *correct* rather than merely non-empty
/// means looking at the shapes side by side.
fn contact_sheet(tiles: &[(&str, Vec<u8>)], dir: &str) {
    if tiles.is_empty() {
        return;
    }
    let cols = (tiles.len() as f32).sqrt().ceil() as u32;
    let rows = (tiles.len() as u32).div_ceil(cols);
    let (sw, sh) = (cols * W, rows * H);

    let mut sheet = vec![0u8; (sw * sh * 4) as usize];
    let mut index = String::new();
    for (n, (name, pixels)) in tiles.iter().enumerate() {
        let (cx, cy) = (n as u32 % cols, n as u32 / cols);
        for row in 0..H {
            let src = (row * W * 4) as usize;
            let dst = (((cy * H + row) * sw + cx * W) * 4) as usize;
            sheet[dst..dst + (W * 4) as usize]
                .copy_from_slice(&pixels[src..src + (W * 4) as usize]);
        }
        index.push_str(&format!("{:>4}  r{cy:02}c{cx:02}  {name}\n", n + 1));
    }

    let path = format!("{dir}/contact-sheet.png");
    image::save_buffer(&path, &sheet, sw, sh, image::ExtendedColorType::Rgba8)
        .expect("could not write contact sheet");
    std::fs::write(format!("{dir}/contact-sheet.txt"), index).ok();
    println!("\nwrote {path} ({cols}x{rows} tiles)");
}

/// Fraction of the frame that is not background, and whether it looks like a
/// picture rather than a blank or a fill.
fn classify(pixels: &[u8], out_dir: Option<&str>, name: &str) -> Verdict {
    let total = (W * H) as usize;
    // The top-left pixel is background by construction — the fractal is framed
    // centred and never reaches the corner. Measuring against it rather than
    // against a fixed threshold is what makes this independent of the
    // background colour and of the sRGB encoding, which is subtle enough to
    // have made an earlier version of this call every correct render "solid".
    let bg = &pixels[..3];
    let lit = pixels
        .chunks_exact(4)
        .filter(|p| {
            (0..3).any(|i| (p[i] as i32 - bg[i] as i32).abs() > 6)
        })
        .count();
    let coverage = lit as f32 / total as f32;

    // Fractal detail shows up as high-frequency variation across the surface.
    // A sphere shaded by a smooth lighting model has almost none, so comparing
    // each lit pixel with its right-hand neighbour separates "there is a shape
    // here" from "there is a shape here with structure in it".
    let rough = pixels
        .chunks_exact(4)
        .zip(pixels.chunks_exact(4).skip(1))
        .filter(|(a, b)| {
            let lit = (0..3).any(|i| (a[i] as i32 - bg[i] as i32).abs() > 6);
            lit && (0..3).any(|i| (a[i] as i32 - b[i] as i32).abs() > 10)
        })
        .count();
    let detail = rough as f32 / lit.max(1) as f32;

    let verdict = if coverage < 0.002 {
        Verdict::Empty
    } else if coverage > 0.98 {
        Verdict::Solid
    } else if detail < 0.06 {
        Verdict::Featureless
    } else {
        Verdict::Rendered
    };

    if let Some(dir) = out_dir {
        image::save_buffer(
            format!("{dir}/{verdict:?}-{name}.png"),
            pixels,
            W,
            H,
            image::ExtendedColorType::Rgba8,
        )
        .ok();
    }
    verdict
}

async fn draw(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    view: &wgpu::TextureView,
    target: &wgpu::Texture,
    readback: &wgpu::Buffer,
    pipeline: &mut FractalPipeline,
    key: u64,
) -> Vec<u8> {
    let padded = (W * 4).div_ceil(COPY_ALIGN) * COPY_ALIGN;

    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3dm.sweep.pass"),
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
        pipeline.draw(&mut pass, key);
    }
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(H),
            },
        },
        wgpu::Extent3d {
            width: W,
            height: H,
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
    let mut pixels = Vec::with_capacity((W * 4 * H) as usize);
    for row in 0..H {
        let start = (row * padded) as usize;
        pixels.extend_from_slice(&mapped[start..start + (W * 4) as usize]);
    }
    drop(mapped);
    readback.unmap();
    pixels
}
