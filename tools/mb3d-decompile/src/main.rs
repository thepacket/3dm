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

use mb3d_decompile::{abi, audit, emit, exec, expr, extract, options};

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().unwrap_or_else(|| {
        eprintln!("usage: mb3d-decompile <M3Formulas dir> [--asm NAME]");
        std::process::exit(2);
    });
    let mut only: Option<String> = None;
    let mut dump_asm = false;
    let mut decompile = false;
    let mut params_report = false;
    let mut audit_report = false;
    let mut wgsl = false;
    let mut generate: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--asm" => {
                dump_asm = true;
                only = args.next();
            }
            "--decompile" => {
                decompile = true;
                only = args.next();
            }
            "--params" => params_report = true,
            "--audit" => audit_report = true,
            "--wgsl" => {
                wgsl = true;
                only = args.next();
            }
            "--generate" => generate = args.next(),
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

    if let Some(out) = generate {
        generate_file(&formulas, std::path::Path::new(&out));
        return;
    }

    if wgsl {
        match only {
            Some(name) => match formulas.iter().find(|f| f.name.eq_ignore_ascii_case(&name)) {
                Some(f) => print_wgsl(f),
                None => {
                    eprintln!("no formula called {name}");
                    std::process::exit(1);
                }
            },
            None => wgsl_all(&formulas),
        }
        return;
    }

    if audit_report {
        run_audit(&formulas);
        return;
    }

    if params_report {
        params(&formulas);
        return;
    }

    if decompile {
        match only {
            Some(name) => {
                let Some(f) = formulas.iter().find(|f| f.name.eq_ignore_ascii_case(&name)) else {
                    eprintln!("no formula called {name}");
                    std::process::exit(1);
                };
                print_decompiled(f);
            }
            None => decompile_all(&formulas),
        }
        return;
    }

    survey(&formulas);
}

fn print_decompiled(f: &extract::Formula) {
    println!("{} — {} bytes", f.name, f.code.len());
    let result = exec::run_with_constants(&f.code, &f.constants);
    for (place, value) in exec::final_stores(&result.stores) {
        println!("  {place} = {}", expr::render(&value));
    }
    if let Some(reason) = &result.bailed {
        println!("  ... stopped: {reason}");
    }
    if !f.description.trim().is_empty() {
        println!("\n  ---- as shipped ----");
        for line in f.description.lines().filter(|l| !l.trim().is_empty()) {
            println!("  {line}");
        }
    }
}

/// How much of the corpus the executor can actually account for.
///
/// Reported as three separate numbers because they mean different things: a
/// formula that bails names an instruction still to implement, while one that
/// finishes but assigns nothing has been misunderstood in a way no error will
/// announce. The second kind is the dangerous one.
fn decompile_all(formulas: &[extract::Formula]) {
    let mut straight = 0usize;
    let mut recovered = 0usize;
    let mut empty = 0usize;
    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();

    for f in formulas {
        straight += 1;
        let result = exec::run_with_constants(&f.code, &f.constants);
        if let Some(reason) = &result.bailed {
            // Keep the mnemonic, drop the address, so the tally counts causes.
            let cause = reason.split(" at ").next().unwrap_or(reason).to_owned();
            *reasons.entry(cause).or_default() += 1;
            continue;
        }
        if exec::final_stores(&result.stores).is_empty() {
            empty += 1;
            continue;
        }
        recovered += 1;
    }

    println!("formulas attempted:      {straight}");
    println!("fully recovered:         {recovered}");
    println!("ran but assigned nothing:{empty}");
    println!("bailed:                  {}", straight - recovered - empty);
    if !reasons.is_empty() {
        println!("\nwhy they bailed:");
        let mut by_count: Vec<_> = reasons.into_iter().collect();
        by_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (cause, count) in by_count.iter().take(20) {
            println!("  {count:5}  {cause}");
        }
    }
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


/// Checks the `[OPTIONS]` slot table against what the code actually reads.
///
/// The two are derived independently — one from the declarations, one from the
/// compiled formula's memory operands — so agreement is real evidence and a
/// mismatch names a datatype whose slot count is wrong. Getting this wrong
/// does not fail loudly: it maps a shared `.m3p`'s values onto the wrong
/// parameters and renders a picture nobody made.
fn params(formulas: &[extract::Formula]) {
    let mut agreed = 0usize;
    let mut short = 0usize;
    let mut unknown: BTreeMap<String, usize> = BTreeMap::new();
    let mut mismatches: Vec<String> = Vec::new();

    for f in formulas {
        let declared = match options::declared(&f.options) {
            Ok(d) => d,
            Err(keyword) => {
                *unknown.entry(keyword).or_default() += 1;
                continue;
            }
        };
        let slots: usize = declared.iter().map(|d| d.slots).sum();
        let Some(highest) = highest_param_slot(&f.code) else {
            continue;
        };
        // The code may leave trailing parameters unread, but it must never
        // read past the last one declared.
        if highest < slots {
            agreed += 1;
            if highest + 1 < slots {
                short += 1;
            }
        } else {
            mismatches.push(format!(
                "{}: declares {slots} slots, reads up to p{highest}",
                f.name
            ));
        }
    }

    println!("slot table agrees:       {agreed}");
    println!("  (of which unused tail: {short})");
    println!("reads past declarations: {}", mismatches.len());
    println!("unknown datatypes:       {}", unknown.values().sum::<usize>());
    for (keyword, count) in &unknown {
        println!("  {count:4}  .{keyword}");
    }
    for line in mismatches.iter().take(15) {
        println!("  {line}");
    }
}

/// The highest parameter slot a blob reads, by scanning its memory operands.
fn highest_param_slot(code: &[u8]) -> Option<usize> {
    use iced_x86::{Decoder, DecoderOptions, Instruction, OpKind, Register};

    // Which register holds PVar is not fixed — some formulas keep it in esi,
    // others in edi — so both candidates are followed and the load that
    // reaches record offset 48 decides.
    let mut decoder = Decoder::with_ip(32, code, 0, DecoderOptions::NONE);
    let mut ins = Instruction::default();
    let mut pvar: Option<Register> = None;
    let mut record: Option<Register> = None;
    let mut highest: Option<usize> = None;

    while decoder.can_decode() {
        decoder.decode_out(&mut ins);
        if ins.mnemonic() == iced_x86::Mnemonic::Mov && ins.op0_kind() == OpKind::Register {
            if ins.op1_kind() == OpKind::Memory {
                let base = ins.memory_base();
                let displacement = ins.memory_displacement64() as i32 as i64;
                if base == Register::EBP && displacement > 0 {
                    record = Some(ins.op0_register());
                } else if Some(base) == record && displacement == abi::PVAR_OFFSET {
                    pvar = Some(ins.op0_register());
                }
            }
            continue;
        }
        if Some(ins.memory_base()) == pvar && ins.memory_base() != Register::None {
            let displacement = ins.memory_displacement64() as i32 as i64;
            if let Some(slot) = abi::parameter_slot(displacement) {
                highest = Some(highest.map_or(slot, |h: usize| h.max(slot)));
            }
        }
    }
    highest
}


/// Compares every recovered formula against what its author wrote down.
///
/// Reported in three columns because they mean different things. Scaffolding
/// is a decode this tool did not finish reassembling — `log2` and its
/// relatives are how the FPU builds `pow`, not something a fractal formula
/// asks for. An unexplained operation is one the decode invented. A missing
/// one is an operation the author described and the decode does not perform.
/// The first is a known gap, the other two are how a wrong decode announces
/// itself.
fn run_audit(formulas: &[extract::Formula]) {
    let mut checkable = 0usize;
    let mut clean = 0usize;
    let mut with_scaffolding = Vec::new();
    let mut with_unexplained = Vec::new();
    let mut with_missing = Vec::new();

    for f in formulas {
        if !audit::states_its_maths(&f.description) {
            continue;
        }
        let result = exec::run_with_constants(&f.code, &f.constants);
        if result.bailed.is_some() {
            continue;
        }
        let rendered: Vec<String> = exec::final_stores(&result.stores)
            .into_iter()
            .map(|(place, value)| format!("{place} = {}", expr::render(&value)))
            .collect();
        if rendered.is_empty() {
            continue;
        }
        checkable += 1;

        let said = audit::from_description(&f.description);
        let did = audit::from_expressions(&rendered);

        let scaffolding = did.scaffolding();
        let unexplained: Vec<_> = did
            .unexplained(&said)
            .into_iter()
            .filter(|op| !audit::SCAFFOLDING.contains(op))
            .collect();
        let missing = did.missing(&said);

        if !scaffolding.is_empty() {
            with_scaffolding.push(format!("{}: {}", f.name, scaffolding.join(", ")));
        }
        if !unexplained.is_empty() {
            with_unexplained.push(format!("{}: {}", f.name, unexplained.join(", ")));
        }
        if !missing.is_empty() {
            with_missing.push(format!("{}: {}", f.name, missing.join(", ")));
        }
        if scaffolding.is_empty() && unexplained.is_empty() && missing.is_empty() {
            clean += 1;
        }
    }

    // The most useful line in the report. A formula whose author wrote "^" or
    // "power" and whose decode carries `log2`/`exp2` instead is not two
    // findings: it is one, and it is the open-coding chain failing to
    // reassemble. Correlating them separates that known bug from anything new.
    let scaffolded: std::collections::BTreeSet<&str> = with_scaffolding
        .iter()
        .filter_map(|line| line.split(':').next())
        .collect();
    let correlated = with_missing
        .iter()
        .filter(|line| line.contains("pow"))
        .filter(|line| line.split(':').next().is_some_and(|n| scaffolded.contains(n)))
        .count();

    println!("formulas stating their maths and decoding: {checkable}");
    println!("  agreeing on every operation:             {clean}");
    println!("  carrying un-collapsed scaffolding:       {}", with_scaffolding.len());
    println!("  using something undescribed:             {}", with_unexplained.len());
    println!("  missing something described:             {}", with_missing.len());
    println!();
    println!("of those, explained by the open-coding bug:  {correlated}");
    println!("  (author wrote a power; decode has log2/exp2 instead)");
    println!("known false positives in `missing`: an operation applied to a");
    println!("constant folds away, so a described sqrt(2) leaves no sqrt behind.");

    for (title, list) in [
        ("scaffolding", &with_scaffolding),
        ("undescribed", &with_unexplained),
        ("missing", &with_missing),
    ] {
        if list.is_empty() {
            continue;
        }
        println!("\n{title}:");
        for line in list.iter().take(12) {
            println!("  {line}");
        }
        if list.len() > 12 {
            println!("  ... and {} more", list.len() - 12);
        }
    }
}


/// Emits one formula as a `GeneratedFormula` entry.
fn print_wgsl(f: &extract::Formula) {
    let result = exec::run_with_constants(&f.code, &f.constants);
    if let Some(reason) = result.bailed {
        eprintln!("{}: did not decompile — {reason}", f.name);
        std::process::exit(1);
    }
    let stores = exec::final_stores(&result.stores);
    let body = match emit::body(&stores) {
        Ok(body) => body,
        Err(reason) => {
            eprintln!("{}: cannot be emitted — {reason}", f.name);
            std::process::exit(1);
        }
    };

    let declared = options::declared(&f.options).unwrap_or_default();
    let slots = options::slots(&declared);
    let de_option: i32 = f
        .options
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("DEoption"))
        .and_then(|(_, v)| v.trim().parse().ok())
        .unwrap_or(0);
    let bailout: f32 = f
        .options
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("RStop"))
        .and_then(|(_, v)| v.trim().parse().ok())
        .unwrap_or(1024.0);

    println!("    GeneratedFormula {{");
    println!("        name: {:?},", f.name);
    println!("        source: {:?},", format!("{}.m3f", f.name));
    println!("        param_floats: {},", slots.len());
    match emit::de_function(de_option) {
        Ok(de) => println!("        de_function: {de},"),
        Err(reason) => println!("        // UNRESOLVED de_function: {reason}"),
    }
    // MB3D formulas add their own J1..J3, where Mandelbulber leaves `+ c` to
    // the caller. Setting this true would add it twice.
    println!("        add_c: false,");
    println!("        bailout: {bailout:?},");
    println!("        params: &[");
    for (i, (name, default)) in slots.iter().enumerate() {
        println!(
            "            GeneratedParam {{ path: {name:?}, kind: ParamKind::Float, offset: {i}, default: &[{default:?}] }},"
        );
    }
    println!("        ],");
    println!("        derivations: &[],");
    println!("        wgsl: r####\"");
    print!("{body}");
    println!("\"####,");
    println!("    }},");
}

/// How much of the corpus could be emitted, and what stops the rest.
fn wgsl_all(formulas: &[extract::Formula]) {
    let mut emitted = 0usize;
    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();

    for f in formulas {
        let result = exec::run_with_constants(&f.code, &f.constants);
        if result.bailed.is_some() {
            continue;
        }
        let stores = exec::final_stores(&result.stores);
        if stores.is_empty() {
            continue;
        }
        match emit::body(&stores) {
            Ok(_) => emitted += 1,
            Err(reason) => {
                let cause = reason
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" ");
                *reasons.entry(cause).or_default() += 1;
            }
        }
    }

    println!("decompiled and emittable as WGSL: {emitted}");
    if !reasons.is_empty() {
        println!("\nwhat stops the rest:");
        let mut by_count: Vec<_> = reasons.into_iter().collect();
        by_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (cause, count) in by_count.iter().take(12) {
            println!("  {count:5}  {cause}");
        }
    }
}


/// Writes every emittable formula as entries for `GENERATED`.
///
/// Entries only — no array wrapper — because this is `include!`d into the one
/// Mandelbulber's transpiler emits. The two corpora are additive: 3DM keeps
/// all of Mandelbulber's formulas and gains MB3D's beside them.
fn generate_file(formulas: &[extract::Formula], out: &std::path::Path) {
    let mut text = String::new();
    let mut written = 0usize;

    for f in formulas {
        let result = exec::run_with_constants(&f.code, &f.constants);
        if result.bailed.is_some() {
            continue;
        }
        let stores = exec::final_stores(&result.stores);
        if stores.is_empty() {
            continue;
        }
        let Ok(body) = emit::body(&stores) else { continue };
        let Ok(declared) = options::declared(&f.options) else {
            continue;
        };
        let slots = options::slots(&declared);
        // A formula whose parameters this tool cannot account for would be
        // handed someone else's values by an `.m3p`, so it is left out. That
        // covers both having no slot table at all and reading past the end of
        // one — the case `--params` reports as an overrun.
        let highest_used = body
            .match_indices("__MB2P")
            .filter_map(|(at, _)| body[at + 6..].split("__").next())
            .filter_map(|n| n.parse::<usize>().ok())
            .max();
        if let Some(highest) = highest_used
            && highest >= slots.len()
        {
            continue;
        }
        let de_option: i32 = f
            .options
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("DEoption"))
            .and_then(|(_, v)| v.trim().parse().ok())
            .unwrap_or(0);
        // Until the analytic case is settled by rendering, everything takes
        // the delta estimator: it measures the distance by sampling and needs
        // no derivative from the formula at all, so it is right for the -1
        // formulas and safe for the rest.
        let _ = de_option;

        text.push_str("    GeneratedFormula {\n");
        text.push_str(&format!("        name: {:?},\n", f.name));
        text.push_str(&format!("        source: {:?},\n", format!("{}.m3f", f.name)));
        text.push_str(&format!("        param_floats: {},\n", slots.len().max(1)));
        text.push_str("        de_function: DeFunction::Delta,\n");
        text.push_str("        add_c: false,\n");
        let bailout: f32 = f
            .options
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("RStop"))
            .and_then(|(_, v)| v.trim().parse().ok())
            .unwrap_or(1024.0);
        text.push_str(&format!("        bailout: {bailout:?},\n"));
        text.push_str("        params: &[\n");
        for (i, (name, default)) in slots.iter().enumerate() {
            text.push_str(&format!(
                "            GeneratedParam {{ path: {name:?}, kind: ParamKind::Float, offset: {i}, default: &[{default:?}] }},\n"
            ));
        }
        text.push_str("        ],\n        derivations: &[],\n");
        text.push_str("        wgsl: r####\"\n");
        text.push_str(&body);
        text.push_str("\"####,\n    },\n");
        written += 1;
    }

    if let Err(e) = std::fs::write(out, &text) {
        eprintln!("could not write {}: {e}", out.display());
        std::process::exit(1);
    }
    println!("wrote {written} formulas to {}", out.display());
}
