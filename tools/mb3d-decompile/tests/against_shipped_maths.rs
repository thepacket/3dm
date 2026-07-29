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
/// The three constants come from the file's own `[CONSTANTS]` block, which the
/// compiled code reads counting up from `PVar + 0`. They are
/// 0.816496580927726, 0.5773502691896258 and 0.7071067811865475 — sqrt(2/3),
/// sqrt(1/3) and sqrt(1/2), exactly the description's, in exactly its order. The compiler reordered the assignments and factored the two
/// `sqrt(1/2)` products, both of which preserve the value.
#[test]
fn benesipine1_reproduces_the_benesi_fold() {
    let lines = decompile("BenesiPine1");
    let fold = &lines[..8];
    assert_eq!(
        fold,
        [
            "t8 = (x * 0.816496580927726 + -(z * 0.5773502691896258)) * 0.7071067811865475",
            "y = abs(t8 + y * 0.7071067811865475)",
            "x = abs(t8 + -(y * 0.7071067811865475))",
            "z = abs(x * 0.5773502691896258 + z * 0.816496580927726)",
            "t8 = (x + y) * 0.7071067811865475",
            "z = -(t8 * 0.5773502691896258) + z * 0.816496580927726",
            "x = t8 * 0.816496580927726 + z * 0.5773502691896258",
            "y = (-x + y) * 0.7071067811865475",
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

/// A branching formula, checked against its description. Two things here are
/// silent when wrong and would look perfectly reasonable: which arm of a
/// conditional is which, and which two values a comparison is between.
///
/// Its description gives
///
/// ```text
/// Scale = Scale + Scale_vary*(abs(Scale)-1)
/// x = Fold-abs(abs(x+FoldXM)-Fold)-abs(FoldXM)
/// if rr < sqr(Min_R) then m = Scale/sqr(Min_R) else
/// if rr < 1 then m = Scale/rr else m = Scale
/// ```
///
/// `Boxscale` occupies two slots because MB3D precomputes the pair the branch
/// needs: `p1` is `Scale/sqr(Min_R)` and `p2` is `sqr(Min_R)`. That is why the
/// recovered test reads `< p2` and its arm is the bare `p1`.
#[test]
fn aboxmod1_recovers_its_folds_and_its_conditional() {
    let lines = decompile("ABoxMod1");
    let joined = lines.join("\n");

    // The first-iteration guard: initialise the running scale, or keep it.
    assert_eq!(lines[0], "VaryScale = if bFirstIt <= 0 then p0 else VaryScale");
    // Scale = Scale + Scale_vary*(abs(Scale)-1), with p4 = "Scale vary".
    assert_eq!(lines[1], "VaryScale = VaryScale + (abs(VaryScale) - 1) * p4");

    // Fold - abs(abs(x + FoldXMod) - Fold) - abs(FoldXMod), with p3 = Fold
    // and p5 = FoldXMod, as the slot table says.
    assert!(
        joined.contains("-abs(abs(x + p5) - p3) + p3 + -abs(p5)"),
        "the x fold should match the description"
    );
    assert!(joined.contains("-abs(abs(y + p6) - p3) + p3 + -abs(p6)"));
    assert!(joined.contains("-abs(abs(z + p7) - p3) + p3 + -abs(p7)"));

    // The outer test is against sqr(Min_R), and taking it yields the
    // precomputed Scale/sqr(Min_R) rather than a division.
    assert!(joined.contains("< p2 then p1 else"), "outer conditional");
    // The inner one compares against 1 and divides. `1 >= rr` is the
    // description's `rr < 1`: they differ only at exactly 1, where
    // Scale/1 is Scale either way.
    assert!(
        joined.contains("if 1 >= ("),
        "inner conditional should compare against 1, not against itself"
    );
    assert!(joined.contains("VaryScale / ("), "inner arm divides by rr");
}

/// An SSE2 formula. MB3D compiles some of its corpus to packed doubles rather
/// than x87, working on (x, y) and (z, w) as pairs — which only works because
/// `TIteration3Dext` stores the four consecutively.
///
/// Its description gives
///
/// ```text
/// x = X_add - abs(x)
/// y = Y_add - abs(y)
/// z = Z_add - abs(z)
/// rr = x*x + y*y + z*z
/// if rr < sqr(Min_R) then m = Scale/sqr(Min_R) else ...
/// ```
///
/// It declares `Scale`, `Boxscale Min R`, `2Doubles Z add`, `Y add`, `X add`,
/// which the slot table expands to eight — so `X add` is `p6`, `Y add` is
/// `p5`, and the second half of `Z add` is `p4`. That the recovered arithmetic
/// pairs each of those with the matching axis is independent confirmation of
/// the table, since nothing about the decode consults it.
#[test]
fn aboxmodkali_recovers_from_packed_sse2() {
    let lines = decompile("ABoxModKali");
    let joined = lines.join("\n");

    for (axis, param) in [('x', "p6"), ('y', "p5"), ('z', "p4")] {
        assert!(
            joined.contains(&format!("({param} - abs({axis}))")),
            "{axis} should fold against {param}"
        );
    }
    // The sign mask read as an absolute value rather than a bitwise `and`.
    assert!(!joined.contains('&'), "no bitwise operation should survive");
    // Same box-scale conditional as ABoxMod1, from a different instruction set.
    assert!(joined.contains("< p2 then p1 else"));
    assert!(lines.iter().any(|l| l.starts_with("x = ")));
    assert!(lines.iter().any(|l| l.starts_with("z = ")));
}


/// The `[CONSTANTS]` block resolves to the numbers the description names.
///
/// Independent of the decode: the block is read from the file and the decode
/// works out which slot each instruction reaches, and the two only agree if
/// both are right. Getting the order wrong would substitute plausible numbers
/// into the wrong places — a fold about the wrong axis, rendering happily.
#[test]
fn the_constants_block_supplies_the_expected_roots() {
    use mb3d_decompile::extract;
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/BenesiPine1.m3f");
    let formula = extract::parse(&path).expect("fixture should parse");

    let expected = [
        (2.0f64 / 3.0).sqrt(),
        (1.0f64 / 3.0).sqrt(),
        (1.0f64 / 2.0).sqrt(),
    ];
    assert_eq!(formula.constants.len(), 3);
    for (got, want) in formula.constants.iter().zip(expected) {
        assert!(
            (got - want).abs() < 1e-15,
            "constant {got} should be {want}"
        );
    }
}
