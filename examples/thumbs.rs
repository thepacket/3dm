//! Renders one thumbnail per formula and packs them into a single atlas.
//!
//!     cargo run --release --example thumbs
//!
//! Writes `assets/formula_thumbs.png`, which the app embeds. Choosing a formula
//! from a list of four hundred names is guesswork; choosing from pictures is
//! how Mandelbulber does it and how anyone actually works.
//!
//! One image rather than four hundred files, because the atlas is a single
//! decode at startup and a single texture upload, and its tile index is just
//! the formula's index in `GENERATED` — no manifest to keep in step.

use eframe::wgpu;
use three_dm::formulas::{
    Builtin, DeFunction, FormulaKind, FormulaSlot, FormulaStack, generated::GENERATED,
};
use three_dm::{
    Camera, SceneParams,
    render::{FractalPipeline, Uniforms},
};

/// Tile edge in pixels. Small enough that four hundred of them stay a
/// reasonable download, large enough to tell two Mandelboxes apart.
const TILE: u32 = 64;
const COPY_ALIGN: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no suitable GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("3dm.thumbs.device"),
            ..Default::default()
        })
        .await
        .expect("could not create device");

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    // Rendered larger and boxed down: a 64px raymarch is all aliasing, and the
    // shading detail is exactly what distinguishes one formula from another.
    let super_sample = 3;
    let render = TILE * super_sample;

    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3dm.thumbs.target"),
        size: wgpu::Extent3d {
            width: render,
            height: render,
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

    let padded = (render * 4).div_ceil(COPY_ALIGN) * COPY_ALIGN;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("3dm.thumbs.readback"),
        size: (padded * render) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut pipeline = FractalPipeline::new(&device, format);

    let count = GENERATED.len();
    // The same derivation the picker uses, from the same function — the two
    // must agree exactly and there is nothing to catch it if they do not.
    let cols = three_dm::formulas::atlas_cols(count) as u32;
    let rows = (count as u32).div_ceil(cols);
    let mut atlas = vec![0u8; (cols * TILE * rows * TILE * 4) as usize];
    let atlas_w = cols * TILE;
    // How many formulas ended up at each framing. Printed at the end because
    // the coverage test that picks a frame reads the rendered pixels, so
    // anything that changes how a tile *looks* can silently change how tightly
    // every shape is framed. A shift in this tally is the signal.
    let mut framings = std::collections::BTreeMap::<String, usize>::new();

    for (index, f) in GENERATED.iter().enumerate() {
        // A transform — a fold, a rotation, an offset — contributes no distance
        // estimate of its own and has no standalone shape. Rendering one alone,
        // which is what this did, produces an empty tile: correct, and useless
        // as a catalogue picture. A third of the Mandelbulber corpus is
        // transforms, so that was a third of the picker showing nothing.
        //
        // So a transform is drawn the way it is *used*: stacked on a Mandelbulb,
        // the same base `mb2_sweep --hybrid` tests them against. The tile then
        // shows what the fold does to a shape you already recognise, which is
        // the only thing a picture can usefully say about a fold.
        let is_transform = f.de_function == DeFunction::None;
        let slots = if is_transform {
            vec![
                FormulaSlot::builtin(Builtin::Mandelbulb),
                FormulaSlot::new(FormulaKind::Generated(index)),
            ]
        } else {
            vec![FormulaSlot::new(FormulaKind::Generated(index))]
        };

        let mut params = SceneParams {
            stack: FormulaStack {
                slots,
                de_mode_override: None,
            },
            ..Default::default()
        };
        params.retune_for_stack();
        params.fractal.iterations = 14;
        params.march.max_steps = 128;

        // A thumbnail is not a small render of the scene — it is a picture in a
        // catalogue, and it is read at 64 pixels against a dark panel. The
        // scene's own near-black background (luma ~18 once encoded) put the
        // whole atlas within a few levels of the UI behind it: dark shape on
        // dark ground, which is exactly the "too dark" complaint. So the tile
        // gets a lighter ground and a little more ambient light, purely for the
        // catalogue. None of this touches SceneParams::default(), so the scene
        // the app opens with is unchanged.
        params.background.color1 = [0.055, 0.065, 0.085];
        params.shading.ambient = 0.55;
        params.picture.brightness = 1.1;

        let (key, source) = params.shader();

        // A formula that will not build gets an empty tile rather than
        // stopping the run.
        let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        pipeline.ensure(&device, key, &source);
        if scope.pop().await.is_some() {
            eprintln!("{}: shader failed to build, leaving tile blank", f.name);
            continue;
        }

        // The bounding radius is deliberately generous — it must not clip a
        // fractal it knows nothing about — so framing at it leaves most shapes
        // a speck in the middle of the tile. Try progressively tighter frames
        // and keep the first that fills a reasonable part of the picture.
        let mut best: Option<Vec<u8>> = None;
        // Reported so a change to the tile's *look* can be checked against the
        // framing it produces. The two are coupled through the coverage test,
        // and a brighter tile that quietly reframes every shape smaller would
        // undo the point of the exercise.
        let mut chosen = f32::NAN;
        for scale in [1.0, 0.55, 0.3, 0.16] {
            let mut camera = Camera::default();
            camera.frame(params.fractal.bounding_radius * scale);

            pipeline.upload(
                &queue,
                &Uniforms::build(
                    &camera,
                    &params,
                    [render as f32, render as f32],
                    0.0,
                    !format.is_srgb(),
                    [0.0, 0.0],
                ),
            );

            let mut encoder = device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("3dm.thumbs.pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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
                        bytes_per_row: Some(padded),
                        rows_per_image: Some(render),
                    },
                },
                wgpu::Extent3d {
                    width: render,
                    height: render,
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

            let (tile, coverage) = {
                let raw = readback.slice(..).get_mapped_range();
                let mut tile = vec![0u8; (TILE * TILE * 4) as usize];
                for y in 0..TILE {
                    for x in 0..TILE {
                        let mut acc = [0u32; 4];
                        for sy in 0..super_sample {
                            for sx in 0..super_sample {
                                let px = x * super_sample + sx;
                                let py = y * super_sample + sy;
                                let o = (py * padded + px * 4) as usize;
                                for (c, a) in acc.iter_mut().enumerate() {
                                    *a += raw[o + c] as u32;
                                }
                            }
                        }
                        let n = super_sample * super_sample;
                        let dst = ((y * TILE + x) * 4) as usize;
                        for c in 0..4 {
                            tile[dst + c] = (acc[c] / n) as u8;
                        }
                    }
                }
                // Against the corner pixel, not against alpha: the shader
                // writes an opaque background, so every pixel has alpha and
                // measuring that reported every frame as completely full.
                let bg = [tile[0], tile[1], tile[2]];
                let lit = tile
                    .chunks_exact(4)
                    .filter(|p| (0..3).any(|i| (p[i] as i32 - bg[i] as i32).abs() > 8))
                    .count();
                let coverage = lit as f32 / (TILE * TILE) as f32;
                (tile, coverage)
            };
            readback.unmap();

            let good = (0.10..=0.85).contains(&coverage);
            if good || best.is_none() {
                best = Some(tile);
                chosen = scale;
            }
            if good {
                break;
            }
        }

        *framings.entry(format!("{chosen}")).or_insert(0usize) += 1;

        if let Some(tile) = best {
            let (tx0, ty0) = ((index as u32 % cols) * TILE, (index as u32 / cols) * TILE);
            for y in 0..TILE {
                let src = (y * TILE * 4) as usize;
                let dst = (((ty0 + y) * atlas_w + tx0) * 4) as usize;
                atlas[dst..dst + (TILE * 4) as usize]
                    .copy_from_slice(&tile[src..src + (TILE * 4) as usize]);
            }
        }
    }

    std::fs::create_dir_all("assets").expect("cannot create assets/");
    image::save_buffer(
        "assets/formula_thumbs.png",
        &atlas,
        atlas_w,
        rows * TILE,
        image::ExtendedColorType::Rgba8,
    )
    .expect("could not write the atlas");

    let bytes = std::fs::metadata("assets/formula_thumbs.png")
        .map(|m| m.len())
        .unwrap_or(0);
    println!(
        "assets/formula_thumbs.png: {count} tiles, {cols}x{rows} of {TILE}px, {:.1} KB",
        bytes as f32 / 1024.0
    );
    let tally = framings
        .iter()
        .map(|(scale, n)| format!("{scale}x{n}"))
        .collect::<Vec<_>>()
        .join("  ");
    println!("framings chosen: {tally}");
}
