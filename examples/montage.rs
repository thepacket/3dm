//! Packs a directory of same-sized PNGs into one contact sheet.
//!
//!     cargo run --release --example montage -- <dir> <out.png> [name-filter] [scale]
//!
//! `mb2_sweep -- <dir>` writes one PNG per formula, which is the only way to
//! judge whether a shape is *right* rather than merely present — but nobody
//! opens six hundred files. This lays them out in a grid, optionally filtered by
//! a substring of the filename, so a whole family can be looked at at once.
//!
//! Sorted by name so two runs of the same filter line up tile for tile, which is
//! what makes a before/after comparison possible.

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args
        .next()
        .expect("usage: montage <dir> <out.png> [filter] [scale]");
    let out = args.next().unwrap_or_else(|| "montage.png".to_owned());
    let filter = args.next().unwrap_or_default();
    let scale: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(2);

    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{dir}: {e}"))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "png"))
        .filter(|p| {
            filter.is_empty()
                || p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.contains(&filter))
        })
        .collect();
    paths.sort();

    if paths.is_empty() {
        eprintln!("no PNGs in {dir} matching {filter:?}");
        std::process::exit(1);
    }

    let first = image::open(&paths[0])
        .expect("cannot read the first tile")
        .to_rgba8();
    let (tw, th) = (first.width() / scale, first.height() / scale);
    // Square-ish grid: as many columns as rows, so the sheet stays viewable
    // rather than becoming a strip.
    let cols = (paths.len() as f64).sqrt().ceil() as u32;
    let rows = (paths.len() as u32).div_ceil(cols);

    let mut sheet = image::RgbaImage::new(cols * tw, rows * th);
    for (i, path) in paths.iter().enumerate() {
        let Ok(img) = image::open(path) else { continue };
        let img = image::imageops::resize(
            &img.to_rgba8(),
            tw,
            th,
            image::imageops::FilterType::Triangle,
        );
        let (x, y) = ((i as u32 % cols) * tw, (i as u32 / cols) * th);
        image::imageops::overlay(&mut sheet, &img, x as i64, y as i64);
    }

    sheet.save(&out).unwrap_or_else(|e| panic!("{out}: {e}"));
    println!("{out}: {} tiles, {cols}x{rows} of {tw}x{th}", paths.len());
    // The order is the only way to map a tile back to a formula.
    for (i, p) in paths.iter().enumerate() {
        if let Some(n) = p.file_stem().and_then(|n| n.to_str()) {
            println!("  {:3} {n}", i);
        }
    }
}
