//! Checks the decompiler against formulas that published their own maths.
//!
//! This is the only test that can catch the failure that matters. A decode
//! which is subtly wrong produces arithmetic that looks exactly as plausible
//! as a correct one — there is no crash, no warning, and no way to tell by
//! reading it. Around 94 of MB3D's 460 formulas state their mathematics in the
//! prose after `[END]`, and those are the ground truth.
//!
//! Two are vendored here rather than pointing at a checkout, so the test runs
//! anywhere. Both are LGPL-2.1 from `github.com/thargor6/mb3d`.

use std::path::Path;

fn decompile(name: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(format!("{name}.m3f"));
    mb3d_decompile::decompile_file(&path).expect("should decompile without bailing")
}

/// Its description says:
///
/// ```text
/// x = abs(x); y = abs(y); z = abs(z)
/// m = Scale/(x*x + y*y + z*z + Fix)
/// x = x*m + Cx
/// ```
///
/// so with the absolute values substituted in, and `p1` = Scale, `p2` = Fix,
/// `J1` = Cx — the Julia constant the ABI calls `J1`.
#[test]
fn kalisets1_matches_its_description() {
    let lines = decompile("Kalisets1");
    let expected = [
        "x = abs(x) * p1 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p2) + J1",
        "y = abs(y) * p1 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p2) + J2",
        "z = abs(z) * p1 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p2) + J3",
    ];
    assert_eq!(lines, expected);
}

/// The harder case: eight steps with a reused temporary, which is what proves
/// the spill slots are being tracked rather than the expression being inlined
/// into something unreadable. Its description gives "Benesi fold 1" as
///
/// ```text
/// tx=(x*sqrt(2/3)-z*sqrt(1/3))*sqrt(1/2);
/// z=abs(x*sqrt(1/3) + z*sqrt(2/3));
/// x=abs(tx-y*sqrt(1/2));
/// y=abs(tx+y*sqrt(1/2));
/// tx=x*sqrt(1/2)+y*sqrt(1/2);
/// y=-x*sqrt(1/2)+y*sqrt(1/2);
/// x=tx*sqrt(2/3)+z*sqrt(1/3);
/// z=-tx*sqrt(1/3)+z*sqrt(2/3);
/// ```
///
/// with `k1` = sqrt(2/3), `k2` = sqrt(1/3) and `k3` = sqrt(1/2) from the
/// constant pool. The compiler reordered the assignments and factored the two
/// `sqrt(1/2)` products, both of which preserve the value.
#[test]
fn benesipine1_reproduces_the_benesi_fold() {
    let lines = decompile("BenesiPine1");
    let fold = &lines[..8];
    assert_eq!(
        fold,
        [
            "t8 = (x * k1 + -(z * k2)) * k3",
            "y = abs(t8 + y * k3)",
            "x = abs(t8 + -(y * k3))",
            "z = abs(x * k2 + z * k1)",
            "t8 = (x + y) * k3",
            "z = -(t8 * k2) + z * k1",
            "x = t8 * k1 + z * k2",
            "y = (-x + y) * k3",
        ]
    );
}

/// The temporary must survive as a name. Inlining it is arithmetically
/// identical and useless: this formula's expression grows past ten thousand
/// characters when every reuse is substituted.
#[test]
fn temporaries_are_kept_rather_than_inlined() {
    let lines = decompile("BenesiPine1");
    assert!(lines.iter().any(|l| l.starts_with("t8 = ")));
    assert!(
        lines.iter().all(|l| l.len() < 200),
        "an inlined expression got through: {:?}",
        lines.iter().max_by_key(|l| l.len())
    );
}
