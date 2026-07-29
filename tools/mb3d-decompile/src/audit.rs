//! Checking recovered formulas against what their authors said they do.
//!
//! Three bugs in this decompiler so far produced arithmetic that was wrong and
//! looked entirely reasonable: a conditional with its arms swapped, a compare
//! against itself, and an `fxch` that read as a no-op and turned every power
//! in the corpus into `log2(ln 2)`. None crashed. None looked odd in
//! isolation. Each was found only by putting one formula's output next to the
//! maths its author wrote down.
//!
//! Doing that by hand does not scale to 460 formulas, and the ones checked by
//! hand are exactly the ones already known to work. This does it for the whole
//! corpus.
//!
//! It compares *vocabulary*, not values. Proving two expressions equal would
//! need a symbolic algebra system and agreement about names the two sides do
//! not share — the description calls it `Fold`, the decode calls it `p3`. But
//! a description that says "sqrt" and a decode with no square root disagree
//! about something real, and so does a decode carrying `log2` when its author
//! never mentioned a logarithm. That second case is the fingerprint of
//! open-coding left un-collapsed, which is precisely what the `fxch` bug left
//! behind everywhere.

use std::collections::BTreeSet;

/// What a piece of text or an expression appears to use.
#[derive(Default, PartialEq, Eq)]
pub struct Vocabulary {
    pub functions: BTreeSet<&'static str>,
}

/// Operations worth comparing, and the words an author might use for each.
///
/// Deliberately generous on the prose side: a description saying "power",
/// "pow" or "^" all mean the same thing, and being strict there would report
/// disagreements that are only about English.
const SYNONYMS: &[(&str, &[&str])] = &[
    ("abs", &["abs(", "abs ", "|x|", "absolute"]),
    ("sqrt", &["sqrt", "square root", "√"]),
    ("pow", &["pow", "power", "^", "**"]),
    ("sin", &["sin(", "sin ", "sine"]),
    ("cos", &["cos(", "cos ", "cosine"]),
    ("atan2", &["atan", "arctan", "atan2"]),
    ("ln", &["ln(", "log(", "logarithm"]),
    ("exp", &["exp(", "exponent"]),
    ("min", &["min(", "minimum"]),
    ("max", &["max(", "maximum"]),
];

/// Reads the vocabulary an author's prose implies.
///
/// Commented-out lines are skipped, and they matter: `ABoxMod1` carries
/// `// rr = pow(...) <- removed to speedup`, which says the formula does
/// *not* take a power. Counting it reports a disagreement where author and
/// decode in fact agree.
pub fn from_description(text: &str) -> Vocabulary {
    let lower: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    let mut functions = BTreeSet::new();
    for (name, words) in SYNONYMS {
        if words.iter().any(|w| lower.contains(w)) {
            functions.insert(*name);
        }
    }
    Vocabulary { functions }
}

/// Reads the vocabulary a recovered formula actually uses.
pub fn from_expressions(rendered: &[String]) -> Vocabulary {
    let joined = rendered.join(" ");
    let mut functions = BTreeSet::new();
    for name in [
        "abs", "sqrt", "pow", "sin", "cos", "tan", "atan2", "ln", "exp", "min", "max", "log2",
        "exp2", "exp2m1", "round", "fmod",
    ] {
        if joined.contains(&format!("{name}(")) {
            functions.insert(name);
        }
    }
    Vocabulary { functions }
}

/// Residue from open-coding that a finished decode should not contain.
///
/// The FPU builds `exp`, `ln` and `pow` out of base-2 primitives. Seeing those
/// primitives in the output means the chain that reassembles them did not
/// fire — which is either an idiom this tool does not know, or a decode that
/// went wrong on the way.
pub const SCAFFOLDING: &[&str] = &["log2", "exp2", "exp2m1"];

impl Vocabulary {
    /// Operations the decode uses that the description never mentions.
    pub fn unexplained(&self, description: &Vocabulary) -> Vec<&'static str> {
        self.functions
            .iter()
            .filter(|f| !description.functions.contains(*f))
            // `abs` appears in almost every formula and almost no description
            // bothers to say so; reporting it would bury everything else.
            .filter(|f| **f != "abs")
            .copied()
            .collect()
    }

    /// Operations the description mentions that the decode does not use.
    pub fn missing(&self, description: &Vocabulary) -> Vec<&'static str> {
        description
            .functions
            .iter()
            .filter(|f| !self.functions.contains(*f))
            .copied()
            .collect()
    }

    pub fn scaffolding(&self) -> Vec<&'static str> {
        self.functions
            .iter()
            .filter(|f| SCAFFOLDING.contains(f))
            .copied()
            .collect()
    }
}

/// Whether a description says enough about its mathematics to check against.
///
/// Most do not: they credit an author, link a forum thread, and describe what
/// the shape looks like. Only the ones that actually write the formula down
/// are evidence.
pub fn states_its_maths(text: &str) -> bool {
    text.lines().any(|line| {
        let line = line.trim();
        // An assignment to one of the iterated variables.
        line.len() > 4
            && line
                .split(['=', ':'])
                .next()
                .is_some_and(|left| matches!(left.trim(), "x" | "y" | "z" | "w" | "r" | "rr"))
    })
}
