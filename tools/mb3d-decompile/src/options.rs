//! `[OPTIONS]` declarations, and the parameter slots they occupy.
//!
//! This is what turns a recovered `p3` into "Fold", and so what lets a
//! community `.m3p` — which carries values in slot order — be read against a
//! formula at all.
//!
//! MB3D's parser is `CustomFormulas.pas`. It matches the first word of each
//! line against a table of datatype keywords:
//!
//! ```text
//! .DOUBLE.SINGLE.INTEGER.DOUBLEANGLE.SINGLEANGLE.3DOUBLEANGLES.3SINGLEANGLES
//! .BOXSCALE.FOLDING.DSQUARE.NOVARIABLE.FOLDING16.6SINGLEANGLES.DRECIPRO
//! .2DOUBLES..DSQRRECI..2SINGLES..4SINGLES..3SCALESANGLES.SCALESROT.2INTEGER.
//! .SRECI2....DRECI2.
//! ```
//!
//! and some of them stand for more than one value: the parser expands the
//! three-angle types into X, Y and Z entries and the six-angle type into six.
//! That is why a formula's slot count is not its declaration count, and why
//! numbering them by position silently misaligns every parameter after the
//! first compound one.

/// A declared parameter and how many `PVar` slots it takes.
pub struct Declared {
    pub name: String,
    pub keyword: String,
    pub slots: usize,
    /// The value the author shipped. A formula given zeros instead renders
    /// nothing at all, so this is not decoration.
    pub default: f64,
}

/// Slots each datatype keyword occupies.
///
/// The multi-slot cases are the ones that matter. `3DOUBLEANGLES` and its
/// relatives are expanded by MB3D's parser into one entry per axis;
/// `BOXSCALE` is two, which is measurable: `ABoxMod1` declares seven
/// parameters, six of them plain `Double`, and its compiled code reads eight
/// slots.
fn slots_for(keyword: &str) -> Option<usize> {
    Some(match keyword {
        "DOUBLE" | "SINGLE" | "INTEGER" | "DOUBLEANGLE" | "SINGLEANGLE" | "FOLDING"
        | "DSQUARE" | "DRECIPRO" | "DSQRRECI" | "NOVARIABLE" => 1,
        // `DRECI2`/`SRECI2` are not in the keyword list quoted above — they
        // postdate it. Two slots is not a guess from the name: at two the
        // corpus check agrees on 254 formulas, at one it agrees on 247, and
        // the seven that move are exactly the ones declaring them.
        "BOXSCALE" | "2DOUBLES" | "2SINGLES" | "DRECI2" | "SRECI2" | "2INTEGER" => 2,
        // An angle is not stored as an angle: MB3D precomputes its sine and
        // cosine, so three angles occupy six slots. Measured rather than
        // assumed — at three the corpus check agrees on 258 formulas with 30
        // reading past their declarations, at six on 284 with 4, and the
        // formulas that move are exactly the ones declaring these.
        "3SINGLEANGLES" | "3SCALESANGLES" => 6,
        "6SINGLEANGLES" => 12,
        // No formula in the corpus declares this one, so there is nothing to
        // measure against and the literal reading is what it keeps. If one
        // ever appears, `--params` will say so rather than quietly misplacing
        // every parameter after it.
        "3DOUBLEANGLES" => 3,
        "4SINGLES" | "FOLDING16" => 4,
        // Deliberately not guessed. An unknown keyword shifts every slot after
        // it, so reporting it is worth far more than assuming one.
        _ => return None,
    })
}

/// Engine settings rather than user parameters: these occupy no slot.
const ENGINE_KEYS: &[&str] = &[
    "VERSION",
    "DESCALE",
    "ADESCALE",
    "DEOPTION",
    "RSTOP",
    "SIPOWER",
    "SIPOW",
    "ITERATIONS",
    "MAXIT",
];

/// Reads `[OPTIONS]` into the parameters it declares, in slot order.
///
/// Returns `Err` with the offending keyword if one is not in the table, since
/// a guess there would misalign everything after it.
pub fn declared(options: &[(String, String)]) -> Result<Vec<Declared>, String> {
    let mut out = Vec::new();
    for (key, value) in options {
        let mut words = key.split_whitespace();
        let Some(first) = words.next() else { continue };
        let keyword = first.to_ascii_uppercase();
        if ENGINE_KEYS.contains(&keyword.as_str()) {
            continue;
        }
        let name: String = words.collect::<Vec<_>>().join(" ");
        if name.is_empty() {
            // A single word that is not an engine setting is a setting this
            // table has not met, not a nameless parameter.
            continue;
        }
        let slots = slots_for(&keyword).ok_or_else(|| keyword.clone())?;
        out.push(Declared {
            name,
            keyword,
            slots,
            default: value.trim().parse().unwrap_or(0.0),
        });
    }
    Ok(out)
}

/// The name and default for each slot, expanding compound declarations.
///
/// A compound declaration ships one value for all its parts, which is what
/// MB3D's own parser does when it fans a three-angle entry out into three.
pub fn slots(declared: &[Declared]) -> Vec<(String, f64)> {
    slot_names(declared)
        .into_iter()
        .zip(
            declared
                .iter()
                .flat_map(|d| std::iter::repeat_n(d.default, d.slots)),
        )
        .collect()
}

/// The name for each slot, expanding compound declarations.
pub fn slot_names(declared: &[Declared]) -> Vec<String> {
    let mut names = Vec::new();
    for d in declared {
        if d.slots == 1 {
            names.push(d.name.clone());
            continue;
        }
        // Angle sets are named for the plane each rotates in, not for an
        // axis. MB3D's own suffix list is `sra` in `CustomFormulas.pas`; the
        // three-part types take its first three.
        const PLANES: [&str; 6] = ["YZ", "XZ", "XY", "XW", "YW", "ZW"];
        for i in 0..d.slots {
            let suffix = if d.keyword.contains("ANGLE") {
                PLANES[i.min(5)].to_owned()
            } else {
                format!("{}", i + 1)
            };
            names.push(format!("{} {suffix}", d.name));
        }
    }
    names
}
