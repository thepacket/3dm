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

/// Where the first user parameter sits, relative to `PVar`.
///
/// From MB3D's own `FormulaCompiler.pas`, which emits every declared value's
/// accessor: constants count up from `PVar + 0`, parameters count *down* from
/// `PVar - 16`, and each step is the size of that value's datatype.
///
/// ```pascal
/// COffset := 0;
/// VOffset := 16;
/// ...
/// Name := PDouble(Integer(PIteration3D^.PVar) - VOffset)^;
/// Inc(VOffset, JITValueDatatypeSize(Pair.Datatype));
/// ```
///
/// This is the rule that lets a recovered formula's `p0`, `p1` be matched back
/// to names in `[OPTIONS]` — and so lets a community `.m3p` parameter file be
/// read against it, which is the whole point of recovering them.
pub const FIRST_PARAM_OFFSET: i64 = 16;

/// Slot size for a `Double`, which is what nearly every parameter is.
const DOUBLE: i64 = 8;

/// Names a user parameter at `offset` from `PVar`.
pub fn parameter(offset: i64) -> String {
    match parameter_slot(offset) {
        Some(n) => format!("p{n}"),
        None if offset >= 0 => format!("k{}", offset / DOUBLE),
        None => format!("pvar{offset}"),
    }
}

/// The parameter slot a `PVar`-relative offset names, if it is one.
pub fn parameter_slot(offset: i64) -> Option<usize> {
    if offset > -FIRST_PARAM_OFFSET {
        return None;
    }
    let from_first = -offset - FIRST_PARAM_OFFSET;
    // A misaligned read is a formula treating a Double as two halves, which
    // is not a parameter slot and should not be given a slot's name.
    (from_first % DOUBLE == 0).then_some((from_first / DOUBLE) as usize)
}

/// The same, as a structured place for the symbolic executor.
pub fn parameter_place(offset: i64) -> crate::expr::Place {
    use crate::expr::Place;
    match parameter_slot(offset) {
        Some(n) => Place::Param(n),
        None if offset >= 0 => Place::Const((offset / DOUBLE) as usize),
        None => Place::Unknown(format!("pvar{offset}")),
    }
}
