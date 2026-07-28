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
use three_dm::formulas::mb2;

/// Pool big enough for the largest block, in `vec4`s.
const POOL_VEC4S: usize = 64;

fn shader_for(index: usize) -> String {
    let f = &GENERATED[index];
    let body = mb2::substitute(f, "pool.v", 0);
    format!(
        "{prelude}\n\
         struct Pool {{ v: array<vec4<f32>, {POOL_VEC4S}> }};\n\
         @group(0) @binding(0) var<uniform> pool: Pool;\n\
         {stubs}\n\
         fn formula(z_in: vec4<f32>, aux: ptr<function, Aux>) -> vec4<f32> {{\n\
         \x20   var z = z_in;\n{body}\n}}\n",
        prelude = mb2::PRELUDE,
        stubs = three_dm::formulas::generated::ENUMERATORS,
    )
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
    let show_errors = std::env::args().any(|a| a == "--errors");

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
