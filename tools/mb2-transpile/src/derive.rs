//! Parameters Mandelbulber computes rather than stores.
//!
//! `sFractal::RecalculateFractalParams` runs once after the user edits
//! anything, filling in a set of fields that no `.cl` body ever assigns and no
//! settings file ever contains: squared fold radii, reciprocals, sines and
//! cosines of angles, and rotation matrices built from Euler angles.
//!
//! The transpiler sees these as ordinary parameters, finds no default for them
//! in `initparameters.cpp` — because there isn't one — and emits zero. A zero
//! `mandelbox.mR2` means a sphere fold whose condition never fires; a zero
//! `transformCommon.inv0` means an inversion by zero. The formula compiles,
//! renders, and is quietly wrong.
//!
//! So each derived parameter is recorded here with the stored parameter it is
//! computed from. The transpiler emits the source alongside it (picking up its
//! real default for free) plus the recipe, and 3DM recomputes the derived value
//! whenever the source changes — which is also what finally makes the rotation
//! angles editable, since the matrix is derived and the angles are not.

/// How a derived parameter is computed from its source.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Square,
    Reciprocal,
    Sqrt,
    /// `a / b`, taking two sources.
    Ratio,
    /// `cos` / `sin` of a source given in degrees.
    Cos,
    Sin,
    /// A 3x3 rotation from Euler angles in degrees, `Rz·Ry·Rx`
    /// (Mandelbulber's `SetRotation2`).
    Rotation2,
    /// The same from `Rx·Ry·Rz` (`SetRotation4`).
    Rotation4,
}

impl Op {
    pub fn variant(self) -> &'static str {
        match self {
            Self::Square => "Square",
            Self::Reciprocal => "Reciprocal",
            Self::Sqrt => "Sqrt",
            Self::Ratio => "Ratio",
            Self::Cos => "Cos",
            Self::Sin => "Sin",
            Self::Rotation2 => "Rotation2",
            Self::Rotation4 => "Rotation4",
        }
    }
}

pub struct Rule {
    pub target: &'static str,
    pub op: Op,
    pub sources: &'static [&'static str],
}

const fn r(target: &'static str, op: Op, sources: &'static [&'static str]) -> Rule {
    Rule {
        target,
        op,
        sources,
    }
}

/// Ordered, because some derived values feed others: `mboxFactor1` is a ratio
/// of two squares that are themselves derived.
pub static RULES: &[Rule] = &[
    r("mandelbox.fR2", Op::Square, &["mandelbox.foldingSphericalFixed"]),
    r("mandelbox.mR2", Op::Square, &["mandelbox.foldingSphericalMin"]),
    r(
        "mandelbox.mboxFactor1",
        Op::Ratio,
        &["mandelbox.fR2", "mandelbox.mR2"],
    ),
    r("transformCommon.sqtR", Op::Sqrt, &["transformCommon.minR05"]),
    r(
        "transformCommon.mboxFactor1",
        Op::Reciprocal,
        &["transformCommon.sqtR"],
    ),
    r(
        "transformCommon.inv0",
        Op::Reciprocal,
        &["transformCommon.invert0"],
    ),
    r(
        "transformCommon.inv1",
        Op::Reciprocal,
        &["transformCommon.invert1"],
    ),
    r(
        "transformCommon.maxMinR2factor",
        Op::Ratio,
        &["transformCommon.maxR2d1", "transformCommon.minR2p25"],
    ),
    r(
        "transformCommon.maxMinR0factor",
        Op::Ratio,
        &["transformCommon.maxR2d1", "transformCommon.minR0"],
    ),
    r("transformCommon.cosA", Op::Cos, &["transformCommon.angleDegA"]),
    r("transformCommon.cosB", Op::Cos, &["transformCommon.angleDegB"]),
    r("transformCommon.cosC", Op::Cos, &["transformCommon.angleDegC"]),
    r("transformCommon.sinA", Op::Sin, &["transformCommon.angleDegA"]),
    r("transformCommon.sinB", Op::Sin, &["transformCommon.angleDegB"]),
    r("transformCommon.sinC", Op::Sin, &["transformCommon.angleDegC"]),
    r(
        "transformCommon.rotationMatrix",
        Op::Rotation2,
        &["transformCommon.rotation"],
    ),
    r(
        "transformCommon.rotationMatrix2",
        Op::Rotation2,
        &["transformCommon.rotation2"],
    ),
    r(
        "transformCommon.rotationMatrixXYZ",
        Op::Rotation4,
        &["transformCommon.rotationXYZ"],
    ),
    r(
        "transformCommon.rotationMatrix2XYZ",
        Op::Rotation4,
        &["transformCommon.rotation2XYZ"],
    ),
    r("mandelbox.mainRot", Op::Rotation2, &["mandelbox.rotationMain"]),
];

pub fn rule_for(path: &str) -> Option<&'static Rule> {
    RULES.iter().find(|r| r.target == path)
}

/// The settings-file type of a source parameter, for the ones the corpus never
/// mentions directly and whose type therefore cannot be inferred from a `.cl`
/// body. Rotations are 3-vectors of degrees; everything else is a scalar.
pub fn source_is_vector(path: &str) -> bool {
    path.ends_with("rotation")
        || path.ends_with("rotation2")
        || path.ends_with("rotationXYZ")
        || path.ends_with("rotation2XYZ")
        || path.ends_with("rotationMain")
}
