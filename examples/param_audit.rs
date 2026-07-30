//! Are the transpiled formulas complete, parameter-wise?
//!
//!     cargo run --release --example param_audit [--m3d]
//!
//! "It compiles and renders" says nothing about whether every knob the original
//! formula had is present, defaulted, and reachable from the UI. Three distinct
//! ways that can be false, and this separates them:
//!
//!   * a parameter with **no recovered default** — the transpiler found the
//!     declaration but not the value, so it starts at zero, and zero is very
//!     often "this fold never fires";
//!   * a parameter that exists but is **not exposed as a control**, so it is
//!     stuck at whatever it defaulted to;
//!   * a parameter **partially exposed** — `controls()` caps at four rows, so a
//!     nine- or twelve-float matrix would show only its first row.
//!
//! Counts only, plus the offending names. By default it reports the
//! Mandelbulber corpus alone, which is what 3DM is being finished on.

use three_dm::formulas::{FormulaKind, ParamKind, generated::GENERATED, mb2};

fn main() {
    let include_m3d = std::env::args().any(|a| a == "--m3d");

    let mut formulas = 0usize;
    let mut params = 0usize;
    let mut no_default = Vec::new();
    let mut all_zero_default = Vec::new();
    let mut hidden = Vec::new();
    let mut partially_exposed = Vec::new();
    let mut short_default = Vec::new();
    let mut by_kind = std::collections::BTreeMap::<String, usize>::new();

    for (i, f) in GENERATED.iter().enumerate() {
        let is_m3d = f.name.starts_with("M3D ");
        if is_m3d && !include_m3d {
            continue;
        }
        formulas += 1;

        let layout = mb2::layout(f);
        // Derived parameters are recomputed on every upload from their sources,
        // so being absent from the control list is correct for them, not a gap.
        let derived: Vec<usize> = f.derivations.iter().map(|d| d.target).collect();
        let controls = FormulaKind::Generated(i).controls();

        for p in &layout.placements {
            params += 1;
            *by_kind.entry(format!("{:?}", p.param.kind)).or_default() += 1;

            let is_derived = derived.contains(&p.param.offset);
            let floats = p.param.kind.floats();

            if p.param.default.is_empty() {
                if !is_derived {
                    no_default.push(format!("{} :: {}", f.name, p.param.path));
                }
            } else if p.param.default.iter().all(|v| *v == 0.0) && !is_derived {
                all_zero_default.push(format!("{} :: {}", f.name, p.param.path));
            }

            // A vector kind whose default carries fewer components than it has
            // floats: the rest silently stay zero.
            if !is_derived
                && !p.param.default.is_empty()
                && matches!(p.param.kind, ParamKind::Float3 | ParamKind::Float4)
                && p.param.default.len() < 3
            {
                short_default.push(format!(
                    "{} :: {} ({} of {} components)",
                    f.name,
                    p.param.path,
                    p.param.default.len(),
                    floats
                ));
            }

            if is_derived {
                continue;
            }

            let exposed = controls.iter().filter(|c| c.path == p.param.path).count();
            if exposed == 0 {
                hidden.push(format!(
                    "{} :: {} ({:?})",
                    f.name, p.param.path, p.param.kind
                ));
            } else if exposed < floats.min(4) {
                partially_exposed.push(format!(
                    "{} :: {} ({exposed} of {floats} floats)",
                    f.name, p.param.path
                ));
            } else if floats > 4 {
                partially_exposed.push(format!(
                    "{} :: {} ({exposed} controls for a {floats}-float parameter)",
                    f.name, p.param.path
                ));
            }
        }
    }

    println!(
        "audited {formulas} formulas ({}), {params} parameters",
        if include_m3d {
            "whole corpus"
        } else {
            "Mandelbulber only"
        }
    );
    println!("  by kind: {by_kind:?}");

    let report = |title: &str, items: &[String]| {
        println!("\n{title}: {}", items.len());
        for item in items.iter().take(25) {
            println!("    {item}");
        }
        if items.len() > 25 {
            println!("    ... and {} more", items.len() - 25);
        }
    };

    report("no recovered default (starts at zero)", &no_default);
    report("default is all zeros", &all_zero_default);
    report("vector default shorter than its kind", &short_default);
    report("exists but not editable", &hidden);
    report("only partially editable", &partially_exposed);
}
