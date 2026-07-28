//! Recovers each formula's renderer metadata from its C++ definition.
//!
//! A `.cl` body says how to transform `z`; it does not say how to turn the
//! result into a distance, whether the caller must add the sampled point back
//! each iteration, or how far the orbit should be allowed to run. All three
//! live in the formula's `formula/definition/fractal_*.cpp` constructor, and
//! all three are needed to render — guessing any of them produces a picture
//! that is wrong in a way that looks plausible.

use std::collections::HashMap;
use std::path::Path;

/// Which closed form turns the running `(r, DE)` into a distance.
///
/// These mirror Mandelbulber's `DEAnalyticFunction`, which is what its engine
/// actually switches on — not the `DEFunctionType` sitting beside it in the
/// same constructor. The two disagree for a large part of the corpus, and
/// following the wrong one silently estimates 93 formulas with a closed form
/// they were never meant to use.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeFunction {
    /// `0.5·r·log(r)/DE`, and zero inside the unit sphere.
    Logarithmic,
    /// `r/DE`.
    Linear,
    /// `(r − 2)/DE` — the offset is what makes an IFS's estimate hold up.
    Ifs,
    /// The formula wrote its own distance into `aux.dist`.
    Custom,
    /// The formula contributes no estimator of its own. For a transform that
    /// is correct — the base formula in the stack decides.
    None,
    /// No analytic derivative at all: the distance has to be measured by
    /// iterating neighbouring points. Logarithmic closed form.
    Delta,
    /// Delta DE with the linear closed form.
    DeltaLinear,
    /// `max(rxy − auxDE, |rxy·z.z|/r) / DE`, where `rxy` is the radius in xy.
    PseudoKleinian,
    /// Needs parameters 3DM does not carry, such as the Jos-Kleinian tweaks.
    Unsupported,
}

impl DeFunction {
    /// The Rust variant name to emit.
    pub fn variant(self) -> &'static str {
        match self {
            Self::Logarithmic => "Logarithmic",
            Self::Linear => "Linear",
            Self::Ifs => "Ifs",
            Self::Custom => "Custom",
            Self::None => "None",
            Self::Delta => "Delta",
            Self::DeltaLinear => "DeltaLinear",
            Self::PseudoKleinian => "PseudoKleinian",
            Self::Unsupported => "Unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub de_function: DeFunction,
    /// Whether the iteration loop adds the sampled point back after this
    /// formula runs. Mandelbulber does this outside the formula body, so a
    /// transpiled escape-time formula is missing its `+ c` without it.
    pub add_c: bool,
    pub bailout: f32,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            de_function: DeFunction::Unsupported,
            add_c: false,
            bailout: 10.0,
        }
    }
}

/// Metadata for every formula, keyed by Mandelbulber's `internalName` — which
/// is also the stem of the `.cl` file, so it joins the two directories.
#[derive(Default)]
pub struct Metas(HashMap<String, Meta>);

impl Metas {
    pub fn parse(definition_dir: &Path) -> Self {
        let mut map = HashMap::new();

        let Ok(entries) = std::fs::read_dir(definition_dir) else {
            return Self(map);
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "cpp") {
                continue;
            }
            let Ok(src) = std::fs::read_to_string(&path) else {
                continue;
            };
            // Only the constructor's assignments are of interest, and the file
            // starts with a licence banner and ends with the formula body.
            if let Some((name, meta)) = parse_one(&src) {
                map.insert(name, meta);
            }
        }

        Self(map)
    }

    pub fn get(&self, internal_name: &str) -> Meta {
        self.0.get(internal_name).copied().unwrap_or_default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

fn parse_one(src: &str) -> Option<(String, Meta)> {
    // Mandelbulber is clang-formatted to a narrow column and wraps long
    // statements across lines, so anything parsed line-by-line silently drops
    // data. Split into statements first.
    let statements: Vec<String> = src
        .split(';')
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();

    let value_of = |key: &str| -> Option<String> {
        statements.iter().find_map(|s| {
            // Guard against the commented-out assignment in one of the files.
            let s = s.trim();
            if s.starts_with("//") || s.starts_with('*') {
                return None;
            }
            let (lhs, rhs) = s.rsplit_once('=')?;
            (lhs.trim().ends_with(key)).then(|| rhs.trim().to_owned())
        })
    };

    let name = value_of("internalName")?.trim_matches('"').to_owned();

    let meta = Meta {
        // Note these are *not* spelled like the `DEFunctionType` constants
        // sitting two lines above them in the same constructor, which carry
        // long-standing typos (`cutomDEFunction`, `peudoKleinianDEFunction`).
        // Matching those spellings here silently classifies every custom-DE
        // formula as unsupported.
        de_function: match value_of("DEAnalyticFunction").as_deref() {
            Some("analyticFunctionLogarithmic") => DeFunction::Logarithmic,
            Some("analyticFunctionLinear") => DeFunction::Linear,
            Some("analyticFunctionIFS") => DeFunction::Ifs,
            Some("analyticFunctionCustomDE") => DeFunction::Custom,
            Some("analyticFunctionPseudoKleinian") => DeFunction::PseudoKleinian,
            Some("analyticFunctionNone") => DeFunction::None,
            _ => DeFunction::Unsupported,
        },
        // `cpixelAlreadyHa`(ndled) means the body adds it itself.
        add_c: value_of("cpixelAddition").as_deref() == Some("cpixelEnabledByDefault"),
        bailout: value_of("defaultBailout")
            .and_then(|v| v.parse().ok())
            .unwrap_or(10.0),
    };

    // `DEType` overrules the analytic function: a delta-DE formula has no
    // analytic derivative to divide by at all, so its distance has to be
    // measured by iterating neighbouring points. Which closed form that
    // measurement feeds is the one place `DEFunctionType` is still the right
    // field to read.
    let meta = if value_of("DEType").as_deref() == Some("analyticDEType") {
        meta
    } else {
        Meta {
            de_function: match value_of("DEFunctionType").as_deref() {
                Some("linearDEFunction") => DeFunction::DeltaLinear,
                _ => DeFunction::Delta,
            },
            ..meta
        }
    };

    Some((name, meta))
}
