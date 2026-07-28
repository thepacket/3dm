//! Audits the transpiled formulas against a *full* naga front end.
//!
//! `tools/mb2-transpile` counts a formula as covered when
//! `naga::front::wgsl::parse_str` accepts it. Parsing resolves far less than
//! wgpu does: it will happily accept a function with no `return`, and it does
//! not run the validator that rejects, say, comparing an `i32` to a `bool`. So
//! the shipped coverage number is an upper bound, and this reports the real one.
//!
//!     cargo run --example mb2_audit [-- --errors]

use three_dm::formulas::generated::GENERATED;
use three_dm::formulas::{FormulaKind, FormulaSlot, FormulaStack, mb2};

/// The exact shader the renderer would build for this formula.
///
/// Not an approximation of it. Earlier versions of this file assembled their
/// own harness — their own prelude, their own pool expression — and every
/// difference from the real thing was a formula that passed here and failed on
/// the GPU: first because integer parameters were cast differently, then
/// because a formula whose local was called `u` shadowed a uniform this harness
/// happened to name something else. Going through `codegen` makes those
/// divergences impossible rather than merely unlikely.
fn shader_for(index: usize) -> String {
    three_dm::render::codegen::generate(&FormulaStack {
        slots: vec![FormulaSlot::new(FormulaKind::Generated(index))],
        de_mode_override: None,
    })
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Outcome {
    Ok,
    ParseError,
    ValidationError,
}

fn check(index: usize) -> (Outcome, String) {
    let src = shader_for(index);
    let module = match naga::front::wgsl::parse_str(&src) {
        Ok(m) => m,
        Err(e) => return (Outcome::ParseError, first_error(&e.emit_to_string(&src))),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    match validator.validate(&module) {
        Ok(_) => (Outcome::Ok, String::new()),
        Err(e) => (
            Outcome::ValidationError,
            first_error(&e.emit_to_string(&src)),
        ),
    }
}

/// naga reports a validation failure as a chain: a generic "this function is
/// invalid" header, then the labels that say why. The last label is the actual
/// cause, so that is what gets reported.
fn first_error(s: &str) -> String {
    let cause = s
        .lines()
        .rev()
        .map(str::trim)
        .find(|l| l.starts_with("= ") || l.contains("^^"))
        .unwrap_or("");
    let head = s
        .lines()
        .find(|l| l.contains("error"))
        .unwrap_or("")
        .trim();
    if cause.is_empty() {
        head.to_owned()
    } else {
        cause.trim_start_matches("= ").to_owned()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let show_errors = args.iter().any(|a| a == "--errors");

    // `--dump <Name>` prints one formula's full diagnostic, or its shader when
    // it compiles. The summary line is the last label naga emits, which names
    // the symptom; fixing a rewrite rule needs the whole message.
    if let Some(name) = args.iter().position(|a| a == "--dump").and_then(|i| args.get(i + 1)) {
        let Some(index) = GENERATED.iter().position(|f| f.name.eq_ignore_ascii_case(name)) else {
            eprintln!("no formula called {name}");
            std::process::exit(2);
        };
        let src = shader_for(index);
        match naga::front::wgsl::parse_str(&src) {
            Err(e) => println!("{}", e.emit_to_string(&src)),
            Ok(module) => {
                let mut v = naga::valid::Validator::new(
                    naga::valid::ValidationFlags::all(),
                    naga::valid::Capabilities::all(),
                );
                match v.validate(&module) {
                    Err(e) => println!("{}", e.emit_to_string(&src)),
                    Ok(_) => println!("{src}"),
                }
            }
        }
        return;
    }

    let mut ok = 0usize;
    let mut parse = Vec::new();
    let mut invalid = Vec::new();
    let mut widest = (0usize, "");

    for (index, f) in GENERATED.iter().enumerate() {
        let floats = mb2::layout(f).floats;
        if floats > widest.0 {
            widest = (floats, f.name);
        }
        match check(index) {
            (Outcome::Ok, _) => ok += 1,
            (Outcome::ParseError, e) => parse.push((f.name, e)),
            (Outcome::ValidationError, e) => invalid.push((f.name, e)),
        }
    }

    let total = GENERATED.len();
    println!("emitted formulas:      {total}");
    println!("parse + validate ok:   {ok}");
    println!("fail to parse:         {}", parse.len());
    println!("fail validation:       {}", invalid.len());
    println!(
        "widest block:          {} floats aligned ({})",
        widest.0, widest.1
    );

    if show_errors {
        for (label, list) in [("parse", &parse), ("validation", &invalid)] {
            println!("\n--- {label} failures ---");
            for (name, e) in list.iter() {
                println!("{name:32} {e}");
            }
        }
    }
}
