//! Reports how far apart two renders are.
//!
//!     cargo run --release --example diffpng -- before.png after.png
//!
//! Regression-checking a shader change with `cmp` answers the wrong question:
//! adding a field to a struct changes the compiled code, which moves the last
//! bits of a float, which moves a silhouette pixel by one march step. The bytes
//! differ and nothing is wrong. This says *how much* differs, which is the
//! difference between "35 edge pixels" and "the fractal changed".

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(before), Some(after)) = (args.next(), args.next()) else {
        eprintln!("usage: diffpng <before.png> <after.png>");
        std::process::exit(2);
    };

    let a = image::open(&before)
        .expect("cannot read the first image")
        .to_rgba8();
    let b = image::open(&after)
        .expect("cannot read the second image")
        .to_rgba8();
    assert_eq!(a.dimensions(), b.dimensions(), "images differ in size");

    let total = (a.width() * a.height()) as f32;
    let mut differing = 0usize;
    let mut worst = 0i32;

    for (p, q) in a.pixels().zip(b.pixels()) {
        // Alpha is ignored: these renders are always opaque.
        let delta = (0..3)
            .map(|i| (p.0[i] as i32 - q.0[i] as i32).abs())
            .max()
            .unwrap_or(0);
        if delta > 0 {
            differing += 1;
        }
        worst = worst.max(delta);
    }

    println!(
        "{differing} of {total:.0} pixels differ ({:.3}%), worst channel delta {worst}/255",
        differing as f32 / total * 100.0
    );
}
