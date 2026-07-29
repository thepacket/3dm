//! Renders every transpiled formula and reports which ones produce a picture.
//!
//! "Compiles" and "renders" are different claims, and the gap between them is
//! where the remaining risk in the formula stack lives. This builds a real wgpu
//! pipeline per formula, draws a small frame, and classifies the result — so a
//! formula that compiles into an empty screen, or into a solid block, is
//! counted as the failure it is.
//!
//!     cargo run --release --example mb2_sweep [-- <out-dir>] [--hybrid]
//!
//! With an output directory it also writes a PNG per formula that rendered,
//! which is the only way to judge whether the shape is actually *right*.
//!
//! `--hybrid` answers a question the solo sweep cannot. Much of Mandelbulber's
//! corpus is transforms — folds, rotations, inversions — that draw nothing on
//! their own and are meant to be composed with a base formula. Rendering one
//! alone proves only that it is a transform. So this mode stacks each formula
//! on top of a known-good Mandelbulb and asks whether the picture changed: a
//! transform that leaves the bulb untouched is broken, one that reshapes it is
//! working, and neither conclusion needs anyone to look at a contact sheet.

use eframe::wgpu;
use three_dm::formulas::{
    Builtin, DeMode, DeriveOp, FormulaKind, FormulaSlot, FormulaStack, ParamKind,
    generated::GENERATED,
};
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

/// What a formula did when stacked on top of the base fractal.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Hybrid {
    ShaderError,
    /// The picture is indistinguishable from the base alone, so whatever the
    /// formula does, it does not reach the geometry.
    Inert,
    /// The base fractal disappeared entirely.
    Erased,
    /// The frame filled — usually the estimate collapsing to zero.
    Flooded,
    /// The shape changed. The formula is doing something.
    Reshaped,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let hybrid = args.iter().any(|a| a == "--hybrid");
    // Rotation angles default to zero in Mandelbulber too, so a rotation
    // transform is *correctly* inert out of the box. Dialling one in is the
    // only way to show that the matrix derivation actually works.
    let rotate = args.iter().any(|a| a == "--rotate");
    // Two escape-time formulas both applied on every iteration make the orbit
    // diverge at once, which is a property of that combination and not a fault
    // in either. A real hybrid interleaves them over different parts of the
    // loop, the way MB3D always did.
    let stagger = args.iter().any(|a| a == "--stagger");
    // `--de log` overrides the estimator every formula declares. The decompiled
    // MB3D formulas do not record which one they want, so sweeping the corpus
    // under each and comparing the classifications is the evidence.
    let de = match args.iter().find_map(|a| a.strip_prefix("--de=")) {
        Some("log") => Some(DeMode::Log),
        Some("linear") => Some(DeMode::Linear),
        Some(other) => panic!("unknown --de={other}, want log or linear"),
        None => None,
    };
    let out_dir = args.into_iter().find(|a| !a.starts_with("--"));
    pollster::block_on(run(out_dir.as_deref(), hybrid, rotate, stagger, de));
}

/// The scene for one sweep entry.
///
/// In hybrid mode the base fractal's framing and iteration budget are pinned so
/// that only the extra slot varies — letting `retune_for_stack` run again would
/// move the camera and change the picture without any geometry moving.
///
/// The estimator is deliberately *not* pinned. An earlier version forced it to
/// logarithmic, reasoning that a formula which merely flips the stack's
/// estimator would otherwise look like it had reshaped the fractal. That
/// reasoning is wrong for the custom-DE formulas, where writing `aux.dist` *is*
/// the whole mechanism: pinning the estimator threw their output away and
/// reported the entire `TransfDIFS*` family as doing nothing. Instead each
/// hybrid is compared against the base rendered under that same estimator, so
/// the comparison stays honest without discarding what the formula does.
fn scene(
    index: Option<usize>,
    hybrid: bool,
    force: Option<DeMode>,
    rotate: bool,
    stagger: bool,
) -> SceneParams {
    let base = if hybrid {
        vec![FormulaSlot::builtin(Builtin::Mandelbulb)]
    } else {
        vec![]
    };

    let mut params = SceneParams {
        stack: FormulaStack {
            slots: base,
            de_mode_override: None,
        },
        ..Default::default()
    };

    if hybrid {
        params.retune_for_stack();
        // The base is truncated for the reference too, or the comparison is
        // between a three-iteration bulb and a full one and every result
        // "changed" for the wrong reason.
        if stagger {
            params.stack.slots[0].end_iter = 3;
        }
        if let Some(i) = index {
            let mut slot = FormulaSlot::new(FormulaKind::Generated(i));
            if rotate {
                set_rotations(&mut slot, 45.0);
            }
            if stagger {
                slot.start_iter = 3;
            }
            params.stack.slots.push(slot);
        }
        params.stack.de_mode_override = force;
    } else {
        params
            .stack
            .slots
            .push(FormulaSlot::new(FormulaKind::Generated(index.unwrap())));
        params.retune_for_stack();
        // Solo sweeps honour a forced estimator too. Which estimator a
        // decompiled MB3D formula should use is not recorded in its `.m3f`, so
        // sweeping the corpus twice and comparing is how that gets settled.
        params.stack.de_mode_override = force;
    }

    // A sweep is about coverage, not beauty; a small budget keeps 361
    // pipelines to a sane runtime.
    params.fractal.iterations = 10;
    params.march.max_steps = 96;
    params
}

/// Dials every Euler rotation source in a slot to `degrees`.
///
/// The matrix itself is derived and not editable; its source angles are, and
/// they are exactly the controls that were unreachable before the derivation
/// mechanism existed.
fn set_rotations(slot: &mut FormulaSlot, degrees: f32) {
    let FormulaKind::Generated(i) = slot.kind else {
        return;
    };
    let f = &GENERATED[i];
    let layout = three_dm::formulas::mb2::layout(f);
    for d in f.derivations {
        if !matches!(d.op, DeriveOp::Rotation2 | DeriveOp::Rotation4) {
            continue;
        }
        for source in d.sources {
            if let Some(p) = layout.placements.iter().find(|p| p.param.offset == *source) {
                for c in 0..3 {
                    if let Some(v) = slot.params.get_mut(p.index + c) {
                        *v = degrees;
                    }
                }
            }
        }
    }
}

/// Fraction of pixels that differ appreciably between two frames.
fn difference(a: &[u8], b: &[u8]) -> f32 {
    let differing = a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(p, q)| (0..3).any(|i| (p[i] as i32 - q[i] as i32).abs() > 6))
        .count();
    differing as f32 / (W * H) as f32
}

async fn run(
    out_dir: Option<&str>,
    hybrid: bool,
    rotate: bool,
    stagger: bool,
    de: Option<DeMode>,
) {
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
    // Selecting a formula in the UI compiles a pipeline on the spot, so this is
    // the latency the user actually feels.
    let mut build_ms: Vec<(f32, &str)> = Vec::new();

    // The base fractal alone under each estimator, so a hybrid is always
    // compared against a base drawn the same way it was.
    let mut references: Vec<(DeMode, Vec<u8>)> = Vec::new();
    if hybrid {
        for mode in DeMode::ALL {
            let params = scene(None, true, Some(mode), false, stagger);
            let mut camera = Camera::default();
            camera.frame(params.fractal.bounding_radius);
            let (key, source) = params.shader();
            pipeline.ensure(&device, key, &source);
            pipeline.upload(
                &queue,
                &Uniforms::build(
                    &camera,
                    &params,
                    [W as f32, H as f32],
                    0.0,
                    !format.is_srgb(),
                    [0.0, 0.0],
                ),
            );
            let pixels =
                draw(&device, &queue, &view, &target, &readback, &mut pipeline, key).await;
            if let Some(dir) = out_dir {
                image::save_buffer(
                    format!("{dir}/_base-{}.png", mode.label().replace(' ', "-")),
                    &pixels,
                    W,
                    H,
                    image::ExtendedColorType::Rgba8,
                )
                .ok();
            }
            references.push((mode, pixels));
        }
    }
    let mut hybrid_tally = [0usize; 5];
    let mut inert = Vec::new();

    for (index, f) in GENERATED.iter().enumerate() {
        let params = scene(Some(index), hybrid, de, rotate, stagger);

        let mut camera = Camera::default();
        camera.frame(params.fractal.bounding_radius);
        let (key, source) = params.shader();

        // A formula that fails to build must not take the sweep down with it,
        // so pipeline creation runs inside an error scope rather than under the
        // panicking uncaptured-error handler.
        let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let started = std::time::Instant::now();
        pipeline.ensure(&device, key, &source);
        let built = match scope.pop().await {
            None => true,
            Some(e) => {
                // The audit validates the formula on its own; this is the only
                // place the whole shader is built, so its errors are the ones
                // that count.
                eprintln!("--- {} failed to build:\n{e}", f.name);
                false
            }
        };
        build_ms.push((started.elapsed().as_secs_f32() * 1000.0, f.name));

        let pixels = if built {
            pipeline.upload(
                &queue,
                &Uniforms::build(
                    &camera,
                    &params,
                    [W as f32, H as f32],
                    0.0,
                    !format.is_srgb(),
                    [0.0, 0.0],
                ),
            );
            Some(draw(&device, &queue, &view, &target, &readback, &mut pipeline, key).await)
        } else {
            None
        };

        if hybrid {
            let verdict = match &pixels {
                None => Hybrid::ShaderError,
                Some(p) => {
                    // Compare against the base drawn under this stack's own
                    // estimator, not a fixed one.
                    let mode = params.stack.de_mode();
                    let reference = references
                        .iter()
                        .find(|(m, _)| *m == mode)
                        .map(|(_, r)| r.as_slice())
                        .unwrap_or(&[]);
                    let changed = difference(p, reference);
                    let coverage = lit_fraction(p);
                    if coverage < 0.002 {
                        Hybrid::Erased
                    } else if coverage > 0.98 {
                        Hybrid::Flooded
                    } else if changed < 0.005 {
                        Hybrid::Inert
                    } else {
                        Hybrid::Reshaped
                    }
                }
            };
            hybrid_tally[verdict as usize] += 1;
            if verdict == Hybrid::Inert {
                inert.push(f.name);
            }
            if let (Some(p), Some(dir)) = (&pixels, out_dir) {
                image::save_buffer(
                    format!("{dir}/{verdict:?}-{}.png", f.name),
                    p,
                    W,
                    H,
                    image::ExtendedColorType::Rgba8,
                )
                .ok();
            }
            if verdict == Hybrid::Reshaped && let Some(p) = pixels {
                sheet.push((f.name, p));
            }
            continue;
        }

        let verdict = match pixels {
            None => Verdict::ShaderError,
            Some(p) => {
                let verdict = classify(&p, out_dir, f.name);
                if verdict == Verdict::Rendered {
                    sheet.push((f.name, p));
                }
                verdict
            }
        };

        tally[verdict as usize] += 1;
        if verdict != Verdict::Rendered {
            failures.push((f.name, verdict));
        }
    }

    if hybrid {
        println!(
            "\nstacked {} transpiled formulas onto a Mandelbulb at {W}x{H}",
            GENERATED.len()
        );
        println!("  reshaped it   {}", hybrid_tally[Hybrid::Reshaped as usize]);
        println!("  no effect     {}", hybrid_tally[Hybrid::Inert as usize]);
        println!("  erased it     {}", hybrid_tally[Hybrid::Erased as usize]);
        println!("  flooded frame {}", hybrid_tally[Hybrid::Flooded as usize]);
        println!(
            "  shader error  {}",
            hybrid_tally[Hybrid::ShaderError as usize]
        );
        if !inert.is_empty() {
            // A transform that does nothing is not automatically broken. Most
            // of these are offsets whose recovered default is zero, so adding
            // them is a no-op until the user moves a slider — that is correct
            // behaviour. The rest are the interesting ones.
            println!("\nno effect on the base fractal:");
            for name in &inert {
                let f = GENERATED.iter().find(|g| &g.name == name).unwrap();
                let neutral = FormulaKind::find(name)
                    .map(|k| k.defaults().iter().all(|v| *v == 0.0))
                    .unwrap_or(false);
                let why = if neutral {
                    "every default is zero, so it is a no-op until edited"
                } else if f.params.iter().any(|p| p.kind == ParamKind::Matrix33) {
                    "rotation matrix defaults to identity — see the Euler-angle gap"
                } else {
                    "no obvious reason — worth a look"
                };
                println!("  {name:32} {why}");
            }
        }
        if let Some(dir) = out_dir {
            contact_sheet(&sheet, dir);
        }
        return;
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

    build_ms.sort_by(|a, b| b.0.total_cmp(&a.0));
    let total: f32 = build_ms.iter().map(|(ms, _)| ms).sum();
    println!(
        "\npipeline build: {:.0} ms mean, slowest {:.0} ms ({})",
        total / build_ms.len() as f32,
        build_ms[0].0,
        build_ms[0].1,
    );

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

/// Fraction of the frame that is not background.
///
/// The top-left pixel is background by construction — the fractal is framed
/// centred and never reaches the corner. Measuring against it rather than
/// against a fixed threshold is what makes this independent of the background
/// colour and of the sRGB encoding, which is subtle enough to have made an
/// earlier version of this call every correct render "solid".
fn lit_fraction(pixels: &[u8]) -> f32 {
    let bg = &pixels[..3];
    let lit = pixels
        .chunks_exact(4)
        .filter(|p| (0..3).any(|i| (p[i] as i32 - bg[i] as i32).abs() > 6))
        .count();
    lit as f32 / (W * H) as f32
}

/// Whether a solo render looks like a picture rather than a blank or a fill.
fn classify(pixels: &[u8], out_dir: Option<&str>, name: &str) -> Verdict {
    let bg = &pixels[..3];
    let lit = (lit_fraction(pixels) * (W * H) as f32) as usize;
    let coverage = lit_fraction(pixels);

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
