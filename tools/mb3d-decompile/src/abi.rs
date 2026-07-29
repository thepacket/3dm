//! MB3D's formula calling convention, taken from its own source.
//!
//! `TIteration3Dext` in `TypeDefinitions.pas` carries a byte offset in a
//! comment against every field, which is the whole ABI written down. Nothing
//! here is inferred from the machine code — the point of naming the fields
//! from the declaration is that a decode can then be *checked* against
//! something, rather than being a plausible story about some offsets.
//!
//! A formula is entered as Delphi `register` convention:
//!
//! ```pascal
//! procedure Formula(var x, y, z: Double; PIteration3D: TPIteration3D);
//! ```
//!
//! so `eax`, `edx` and `ecx` hold pointers to x, y and z, and `[ebp+8]` holds
//! the record. Compiled formulas then almost always do two things immediately:
//! load `PVar` from `+48` into `esi`, and add `0x80` to the record pointer so
//! that the far end of the record reaches within a one-byte displacement.

/// Offsets are relative to the record's own zero, which sits at `C1` — not at
/// the allocation's start. Negative offsets are the fields declared above it.
const FIELDS: &[(i64, &str)] = &[
    (-56, "J4"),
    (-48, "Rold"),
    (-40, "RStopD"),
    (-32, "x"),
    (-24, "y"),
    (-16, "z"),
    (-8, "w"),
    (0, "C1"),
    (8, "C2"),
    (16, "C3"),
    (24, "J1"),
    (32, "J2"),
    (40, "J3"),
    (48, "PVar"),
    (52, "SmoothItD"),
    (56, "Rout"),
    (64, "ItResultI"),
    (68, "maxIt"),
    (72, "RStop"),
    (76, "nHybrid"),
    (100, "fHPVar"),
    (124, "fHybrid"),
    (148, "CalcSIT"),
    (152, "DoJulia"),
    (156, "LNRStop"),
    (160, "DEoption"),
    (164, "fHln"),
    (188, "iRepeatFrom"),
    (190, "iStartFrom"),
    (192, "OTrap"),
    (200, "VaryScale"),
    (208, "bFirstIt"),
    (212, "bTmp"),
    (216, "Dfree1"),
    (224, "Dfree2"),
    (232, "Deriv1"),
    (240, "Deriv2"),
    (248, "Deriv3"),
    (256, "SMatrix4"),
    (320, "Ju1"),
    (328, "Ju2"),
    (336, "Ju3"),
    (344, "Ju4"),
    (352, "PMapFunc"),
    (356, "PMapFunc2"),
    (360, "pInitialization"),
    (384, "bIsInsideRender"),
];

/// The rebase every compiled formula applies, so the record's tail is
/// reachable with a signed byte displacement.
pub const RECORD_REBASE: i64 = 0x80;

/// Where `PVar` sits in the record. Loading it is how a formula reaches its
/// own parameters, so recognising this one load is what makes every parameter
/// reference nameable.
pub const PVAR_OFFSET: i64 = 48;

/// Names the record field at `offset` from the record's zero, if one starts
/// exactly there. An offset landing inside an array or matrix reports the
/// field and the distance into it.
pub fn field(offset: i64) -> Option<String> {
    if let Some((_, name)) = FIELDS.iter().find(|(at, _)| *at == offset) {
        return Some((*name).to_owned());
    }
    // Inside a field rather than at its start: an array element, or the high
    // half of a double being read on its own.
    let (at, name) = FIELDS.iter().rev().find(|(at, _)| *at < offset)?;
    Some(format!("{name}+{}", offset - at))
}

/// Names a user parameter at `offset` from `PVar`.
///
/// Only the anchor is documented — MB3D's own comment says the user values run
/// at *decreasing* offsets and that `PVar-8` is always the constant 0.5. The
/// mapping from a given offset back to a name in `[OPTIONS]` is not a fixed
/// stride, so this reports the slot and leaves the naming to whatever can
/// actually establish it.
pub fn parameter(offset: i64) -> String {
    match offset {
        -8 => "const_0.5".to_owned(),
        o if o < 0 => format!("param[{}]", (-o - 8) / 8),
        0 => "const[0]".to_owned(),
        o => format!("const[{}]", o / 8),
    }
}

/// The same, as a structured place for the symbolic executor.
pub fn parameter_place(offset: i64) -> crate::expr::Place {
    use crate::expr::Place;
    match offset {
        -8 => Place::Const(0),
        o if o < 0 => Place::Param(((-o - 8) / 8) as usize),
        o => Place::Const((o / 8) as usize + 1),
    }
}
