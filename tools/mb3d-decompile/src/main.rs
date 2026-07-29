//! Recovers readable formulas from Mandelbulb 3D's compiled `.m3f` blobs.
//!
//! MB3D ships 454 of its 460 formulas as hex-encoded 32-bit x86 inside a
//! `[CODE]` block. The files are ASCII, which is misleading — the payload is
//! machine code, and the Pascal it was compiled from is not in the repository.
//!
//! Recovering it is tractable because of what the code *is*: small, mostly
//! straight-line x87 arithmetic against a fixed calling convention. x87 is a
//! stack machine, so an expression tree falls out of simulating the stack —
//! there is no register allocation to undo first.
//!
//!     cargo run -p mb3d-decompile -- <M3Formulas dir> [--asm NAME]

use std::collections::BTreeMap;
use std::path::Path;

mod abi;
mod extract;

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().unwrap_or_else(|| {
        eprintln!("usage: mb3d-decompile <M3Formulas dir> [--asm NAME]");
        std::process::exit(2);
    });
    let mut only: Option<String> = None;
    let mut dump_asm = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--asm" => {
                dump_asm = true;
                only = args.next();
            }
            other => only = Some(other.to_owned()),
        }
    }

    let formulas = extract::load_dir(Path::new(&dir));
    if formulas.is_empty() {
        eprintln!("no .m3f files with a [CODE] block under {dir}");
        std::process::exit(1);
    }

    if dump_asm {
        let Some(name) = only.as_ref() else {
            eprintln!("--asm needs a formula name");
            std::process::exit(2);
        };
        let Some(f) = formulas.iter().find(|f| f.name.eq_ignore_ascii_case(name)) else {
            eprintln!("no formula called {name}");
            std::process::exit(1);
        };
        println!("; {} — {} bytes", f.name, f.code.len());
        if !f.options.is_empty() {
            println!("; options, in declaration order:");
            for (key, value) in &f.options {
                println!(";   {key} = {value}");
            }
        }
        // The rebase happens a few instructions in, and until it does the
        // record pointer means something different. Annotating as though it
        // had already happened mislabels the `PVar` load itself — which is the
        // single most important instruction in the function.
        let mut rebased = false;
        for (offset, text) in disassemble(&f.code) {
            match annotate(&text, rebased) {
                Some(note) => println!("{offset:04x}  {text:<28} ; {note}"),
                None => println!("{offset:04x}  {text}"),
            }
            if text.replace(' ', "").eq_ignore_ascii_case("addedi,80h") {
                rebased = true;
            }
        }
        if !f.description.trim().is_empty() {
            println!("\n; ---- description as shipped ----");
            for line in f.description.lines() {
                println!("; {line}");
            }
        }
        return;
    }

    survey(&formulas);
}

/// Decodes one blob to `(offset, text)` pairs.
fn disassemble(code: &[u8]) -> Vec<(u64, String)> {
    use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};

    let mut decoder = Decoder::with_ip(32, code, 0, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::new();
    let mut out = Vec::new();
    let mut instruction = Instruction::default();
    let mut text = String::new();
    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        text.clear();
        formatter.format(&instruction, &mut text);
        out.push((instruction.ip(), text.clone()));
    }
    out
}

/// Puts a name to a memory operand, where the ABI says what lives there.
///
/// Textual rather than structural on purpose: this is a reading aid for
/// working out the remaining semantics by eye, and it should be obvious that
/// it is a comment and not a claim the decoder is making.
fn annotate(text: &str, rebased: bool) -> Option<String> {
    // `esi` is `PVar` and `edi` the record rebased by 0x80 — the two idioms
    // every compiled formula opens with.
    let (base, rest) = if let Some(rest) = text.split_once("[esi").map(|(_, r)| r) {
        ("param", rest)
    } else if let Some(rest) = text.split_once("[edi").map(|(_, r)| r) {
        ("record", rest)
    } else {
        return match text {
            t if t.contains("[ebx]") || t.contains("[eax]") => Some("x".to_owned()),
            t if t.contains("[edx]") => Some("y".to_owned()),
            t if t.contains("[ecx]") => Some("z".to_owned()),
            _ => None,
        };
    };

    let displacement = rest.split(']').next()?;
    let value = parse_displacement(displacement)?;
    Some(match base {
        "param" => abi::parameter(value),
        _ if rebased => abi::field(value + abi::RECORD_REBASE)?,
        _ => abi::field(value)?,
    })
}

/// Reads `+30h`, `-10h` or an empty string as a signed byte count.
fn parse_displacement(text: &str) -> Option<i64> {
    let text = text.trim();
    if text.is_empty() {
        return Some(0);
    }
    let (sign, digits) = match text.as_bytes()[0] {
        b'+' => (1, &text[1..]),
        b'-' => (-1, &text[1..]),
        _ => return None,
    };
    let digits = digits.trim_end_matches(['h', 'H']);
    i64::from_str_radix(digits, 16).ok().map(|v| sign * v)
}

/// What the corpus is actually made of.
///
/// Written before any decompiler, deliberately: a survey says which
/// instructions must be handled and which can be left to fail loudly. Guessing
/// the shape from one example and meeting the rest at scale is how a
/// translator ends up producing plausible nonsense — the one failure mode
/// there is no way to notice across 454 formulas.
fn survey(formulas: &[extract::Formula]) {
    use iced_x86::{Code, Decoder, DecoderOptions, Instruction, Mnemonic};

    let mut mnemonics: BTreeMap<String, usize> = BTreeMap::new();
    let mut invalid = 0usize;
    let mut total_instructions = 0usize;
    let mut with_branches = 0usize;
    let mut with_calls = 0usize;
    let mut sizes = Vec::new();

    for f in formulas {
        let mut decoder = Decoder::with_ip(32, &f.code, 0, DecoderOptions::NONE);
        let mut instruction = Instruction::default();
        let mut branches = false;
        let mut calls = false;
        let mut count = 0usize;
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            count += 1;
            if instruction.code() == Code::INVALID {
                invalid += 1;
                continue;
            }
            *mnemonics
                .entry(format!("{:?}", instruction.mnemonic()).to_lowercase())
                .or_default() += 1;
            if instruction.is_jcc_short_or_near() || instruction.is_jmp_short_or_near() {
                branches = true;
            }
            if instruction.mnemonic() == Mnemonic::Call {
                calls = true;
            }
        }
        total_instructions += count;
        sizes.push(f.code.len());
        if branches {
            with_branches += 1;
        }
        if calls {
            with_calls += 1;
        }
    }

    sizes.sort_unstable();
    println!("formulas with code:   {}", formulas.len());
    println!("instructions decoded: {total_instructions}");
    println!("undecodable:          {invalid}");
    println!(
        "blob bytes:           min {} median {} max {}",
        sizes.first().unwrap_or(&0),
        sizes[sizes.len() / 2],
        sizes.last().unwrap_or(&0)
    );
    println!("with any branch:      {with_branches}");
    println!("with a call:          {with_calls}");
    println!();

    // The long tail is what decides whether this is days or months of work.
    let mut by_count: Vec<_> = mnemonics.into_iter().collect();
    by_count.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("{} distinct mnemonics:", by_count.len());
    for (name, count) in &by_count {
        println!("  {count:8}  {name}");
    }
}
