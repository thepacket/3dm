//! Measures how legible the thumbnail atlas actually is.
//!
//!     cargo run --release --example atlas_stats -- [atlas.png]
//!
//! "Too dark" is a judgement, but it rests on numbers, and those are what this
//! prints: how bright the lit part of a tile is, and how much of the tile is lit
//! at all. Comparing two atlases is the point — a change to the thumbnail render
//! should move the median of the lit pixels, not merely the overall mean, which
//! a change in coverage alone would also move.

/// Perceptual luma, on the stored sRGB values rather than linear light: the
/// question here is what the eye is given, not what the renderer computed.
fn luma(p: &[u8]) -> f32 {
    0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32
}

fn percentile(sorted: &[f32], q: f32) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    let i = ((sorted.len() - 1) as f32 * q).round() as usize;
    sorted[i]
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/formula_thumbs.png".to_owned());

    let img = image::open(&path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .to_rgba8();
    let (w, h) = img.dimensions();

    if std::env::args().any(|a| a == "--per-tile") {
        per_tile(&img);
        return;
    }

    let mut all = Vec::with_capacity((w * h) as usize);
    // The atlas is drawn over a transparent clear, so alpha separates the shape
    // from the empty ground around it. Averaging the ground in would report a
    // dark atlas no matter how bright the fractals are.
    let mut lit = Vec::new();
    let mut histogram = [0u64; 16];

    for p in img.pixels() {
        let l = luma(&p.0);
        all.push(l);
        histogram[((l / 256.0) * 16.0) as usize % 16] += 1;
        if p.0[3] > 8 {
            lit.push(l);
        }
    }

    all.sort_by(f32::total_cmp);
    lit.sort_by(f32::total_cmp);

    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len().max(1) as f32;
    let coverage = lit.len() as f32 / all.len() as f32;

    println!("{path}: {w}x{h}, {} pixels", all.len());
    println!("  lit coverage      {:.1}%", coverage * 100.0);
    println!(
        "  luma over all     mean {:.1}  median {:.1}  p90 {:.1}",
        mean(&all),
        percentile(&all, 0.5),
        percentile(&all, 0.9)
    );
    println!(
        "  luma where lit    mean {:.1}  median {:.1}  p10 {:.1}  p90 {:.1}",
        mean(&lit),
        percentile(&lit, 0.5),
        percentile(&lit, 0.1),
        percentile(&lit, 0.9)
    );
    // The complaint was "too dark", so the tail that matters is the bottom one:
    // pixels a viewer cannot separate from the panel behind them.
    let murky = lit.iter().filter(|&&l| l < 32.0).count();
    println!(
        "  lit but under 32  {:.1}% of lit pixels",
        murky as f32 / lit.len().max(1) as f32 * 100.0
    );

    println!("  histogram (16 buckets of 16 luma, all pixels):");
    for (i, n) in histogram.iter().enumerate() {
        let share = *n as f64 / all.len() as f64;
        println!(
            "    {:>3}..{:<3} {:>6.2}% {}",
            i * 16,
            i * 16 + 15,
            share * 100.0,
            "#".repeat((share * 200.0).round() as usize)
        );
    }
}

/// Classifies every tile, which is what answers "why is this picker entry
/// blank?" — and the two blank cases have different causes.
///
/// A tile the generator never wrote is still fully transparent, because the
/// atlas starts zeroed and `thumbs` skips a formula whose shader will not
/// build. A tile it *did* write but which holds nothing but its own background
/// is a formula that compiled and then drew nothing at any framing it tried.
/// The first is a build failure, the second a rendering one, and conflating
/// them sends you looking in the wrong place.
fn per_tile(img: &image::RgbaImage) {
    use three_dm::formulas::generated::GENERATED;

    const TILE: u32 = 64;
    let cols = three_dm::formulas::atlas_cols(GENERATED.len()) as u32;

    let mut unwritten = Vec::new();
    let mut featureless = Vec::new();
    let mut ok = 0usize;

    for (index, f) in GENERATED.iter().enumerate() {
        let (tx, ty) = ((index as u32 % cols) * TILE, (index as u32 / cols) * TILE);

        let mut opaque = 0usize;
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for y in 0..TILE {
            for x in 0..TILE {
                let p = img.get_pixel(tx + x, ty + y).0;
                if p[3] > 8 {
                    opaque += 1;
                }
                let l = luma(&p);
                min = min.min(l);
                max = max.max(l);
            }
        }

        // Nothing opaque at all: the tile was never drawn into.
        if opaque == 0 {
            unwritten.push(f.name);
        // Drawn, but every pixel is the same shade — background only.
        } else if max - min < 4.0 {
            featureless.push(f.name);
        } else {
            ok += 1;
        }
    }

    let split = |names: &[&str]| {
        let m3d = names.iter().filter(|n| n.starts_with("M3D ")).count();
        (names.len() - m3d, m3d)
    };
    let (u_mb2, u_m3d) = split(&unwritten);
    let (f_mb2, f_m3d) = split(&featureless);

    println!("{} tiles: {ok} with a picture", GENERATED.len());
    println!(
        "  never drawn (shader would not build): {} ({u_mb2} MB2, {u_m3d} M3D)",
        unwritten.len()
    );
    println!(
        "  drawn but empty (background only):    {} ({f_mb2} MB2, {f_m3d} M3D)",
        featureless.len()
    );

    for (title, names) in [
        ("never drawn", &unwritten),
        ("drawn but empty", &featureless),
    ] {
        if names.is_empty() {
            continue;
        }
        println!("\n{title}:");
        for n in names.iter() {
            println!("    {n}");
        }
    }
}
