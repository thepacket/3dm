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
