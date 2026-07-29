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
/// so with the absolute values substituted in. It declares exactly two
/// parameters — `Scale` then `Fix` — and reads them from `PVar-16` and
/// `PVar-24`, which is MB3D's documented slot rule and makes them `p0` and
/// `p1`. `J1` is the Julia constant the ABI names.
#[test]
fn kalisets1_matches_its_description() {
    let lines = decompile("Kalisets1");
    let expected = [
        "x = abs(x) * p0 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p1) + J1",
        "y = abs(y) * p0 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p1) + J2",
        "z = abs(z) * p0 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p1) + J3",
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
/// with `k0` = sqrt(2/3), `k1` = sqrt(1/3) and `k2` = sqrt(1/2) read from the
/// constant pool, which counts up from `PVar + 0`. The compiler reordered the assignments and factored the two
/// `sqrt(1/2)` products, both of which preserve the value.
#[test]
fn benesipine1_reproduces_the_benesi_fold() {
    let lines = decompile("BenesiPine1");
    let fold = &lines[..8];
    assert_eq!(
        fold,
        [
            "t8 = (x * k0 + -(z * k1)) * k2",
            "y = abs(t8 + y * k2)",
            "x = abs(t8 + -(y * k2))",
            "z = abs(x * k1 + z * k0)",
            "t8 = (x + y) * k2",
            "z = -(t8 * k1) + z * k0",
            "x = t8 * k0 + z * k1",
            "y = (-x + y) * k2",
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

/// The slot rule from MB3D's `FormulaCompiler.pas`. Getting this wrong maps a
/// community `.m3p`'s values onto the wrong parameters — which produces a
/// picture, just not the one whoever shared it made.
#[test]
fn parameter_slots_follow_mb3ds_own_rule() {
    use mb3d_decompile::abi::parameter_slot;
    assert_eq!(parameter_slot(-16), Some(0), "first parameter sits at PVar-16");
    assert_eq!(parameter_slot(-24), Some(1));
    assert_eq!(parameter_slot(-72), Some(7));
    // At and above PVar is the constant pool, not parameters.
    assert_eq!(parameter_slot(0), None);
    assert_eq!(parameter_slot(-8), None);
    // Half a Double is not a slot.
    assert_eq!(parameter_slot(-20), None);
}

/// `[OPTIONS]` declarations do not map one-to-one onto parameter slots: some
/// datatypes stand for several values. Numbering by declaration order instead
/// misaligns every parameter after the first compound one.
///
/// `ABoxMod1` is the measurable case. It declares seven parameters, six of
/// them plain `Double` and one `.Boxscale`, and its compiled code reads eight
/// slots — so `Boxscale` is two, and `Fold` is `p3` rather than `p2`. That
/// agrees with the arithmetic recovered from the code, where the value at
/// `PVar-40` is the one the description calls `Fold`.
#[test]
fn boxscale_occupies_two_slots() {
    use mb3d_decompile::{extract, options};
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/ABoxMod1.m3f");
    let formula = extract::parse(&path).expect("fixture should parse");
    let declared = options::declared(&formula.options).expect("known datatypes");
    let names = options::slot_names(&declared);

    assert_eq!(names.len(), 8, "seven declarations, eight slots");
    assert_eq!(names[0], "Scale");
    assert_eq!(names[1], "Min R 1");
    assert_eq!(names[2], "Min R 2");
    assert_eq!(names[3], "Fold");
    assert_eq!(names[4], "Scale vary");
    assert_eq!(names[5], "FoldXMod");
}
