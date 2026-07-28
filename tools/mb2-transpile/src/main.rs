//! Translates Mandelbulber2's OpenCL fractal formulas into WGSL fragments for
//! 3DM's formula stack.
//!
//!     cargo run -p mb2-transpile -- <mandelbulber2-checkout> [--report-only]
//!
//! Mandelbulber2 is GPL-3.0, which is why 3DM is too. See the project README.

mod defaults;
mod translate;
mod types;
mod validate;

use std::collections::BTreeMap;
use std::path::Path;

use translate::Rejected;
use types::TypeMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| {
        eprintln!("usage: mb2-transpile <mandelbulber2-checkout> [--report-only]");
        std::process::exit(2);
    });
    let rest: Vec<String> = args.collect();
    let report_only = rest.iter().any(|a| a == "--report-only");
    let show_errors = rest.iter().any(|a| a == "--errors");
    // `--dump <file.cl>` prints the generated WGSL, which is the only sane way
    // to debug a rewrite rule.
    let dump = rest
        .iter()
        .position(|a| a == "--dump")
        .and_then(|i| rest.get(i + 1))
        .cloned();

    let root = Path::new(&root);
    let header = root.join("mandelbulber2/opencl/fractal_cl.h");
    let formula_dir = root.join("mandelbulber2/formula/opencl");

    let header_src = std::fs::read_to_string(&header)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", header.display()));
    let type_map = TypeMap::parse(&header_src);

    // Defaults come from the C++ sources, joined across two files.
    let read = |rel: &str| std::fs::read_to_string(root.join(rel)).unwrap_or_default();
    let defaults = defaults::Defaults::parse(
        &read("mandelbulber2/src/fractal.cpp"),
        &read("mandelbulber2/src/initparameters.cpp"),
    );
    validate::set_enumerators(&types::parse_enumerators(&header_src));

    let mut files: Vec<_> = std::fs::read_dir(&formula_dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", formula_dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "cl"))
        .collect();
    files.sort();

    let mut ok = Vec::new();
    let mut rejected: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut samples: Vec<String> = Vec::new();
    let mut undefined: BTreeMap<String, usize> = BTreeMap::new();

    for path in &files {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                rejected
                    .entry(format!("unreadable: {e}"))
                    .or_default()
                    .push(name);
                continue;
            }
        };

        if dump.as_deref() == Some(name.as_str()) {
            match translate::translate(&name, &src, &type_map) {
                Ok(f) => match validate::full_error(&f) {
                    Some(e) => println!("{e}"),
                    None => println!("{}", validate::shader_for(&f)),
                },
                Err(Rejected::Unsupported(w)) | Err(Rejected::Malformed(w)) => {
                    println!("// rejected before validation: {w}")
                }
            }
            return;
        }

        match translate::translate(&name, &src, &type_map) {
            // Translating is not the same as compiling: only formulas naga
            // accepts are counted, so the report reflects shippable coverage.
            Ok(f) => match validate::check(&f) {
                Ok(()) => ok.push(f),
                Err(e) => {
                    if let Some(id) = undefined_ident(&e) {
                        *undefined.entry(id).or_default() += 1;
                    }
                    if show_errors && samples.len() < 12 {
                        samples.push(format!("{name}: {e}"));
                    }
                    rejected.entry(classify(&e)).or_default().push(name)
                }
            },
            Err(Rejected::Unsupported(why)) | Err(Rejected::Malformed(why)) => {
                rejected.entry(why).or_default().push(name);
            }
        }
    }

    let total = files.len();
    println!("Mandelbulber2 -> WGSL coverage");
    println!("  translated : {} / {total}", ok.len());
    println!(
        "  rejected   : {} / {total}",
        total.saturating_sub(ok.len())
    );
    println!("\nrejections by cause:");
    let mut causes: Vec<_> = rejected.iter().collect();
    causes.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));
    for (why, which) in causes {
        println!("  {:>4}  {why}", which.len());
    }

    if !ok.is_empty() {
        // A parameter with no default renders as zero, which for most formulas
        // means an empty image — so this coverage matters as much as the
        // formula count itself.
        let (mut have, mut missing) = (0usize, 0usize);
        let mut missing_names: BTreeMap<String, usize> = BTreeMap::new();
        for f in &ok {
            for p in &f.params {
                // Rotation matrices are computed from Euler angles rather than
                // stored, so identity is their correct neutral default.
                if defaults.get(&p.path).is_some()
                    || p.ty == types::ParamType::Matrix33
                {
                    have += 1;
                } else {
                    missing += 1;
                    *missing_names.entry(p.path.clone()).or_default() += 1;
                }
            }
        }
        println!(
            "\nparameter defaults: {have} resolved, {missing} missing ({} known paths)",
            defaults.len()
        );
        let mut v: Vec<_> = missing_names.iter().collect();
        v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        for (path, n) in v.iter().take(10) {
            println!("  {n:>4}  {path}");
        }

        let params: Vec<usize> = ok.iter().map(|f| f.param_floats).collect();
        let max = params.iter().copied().max().unwrap_or(0);
        let mean = params.iter().sum::<usize>() as f32 / params.len() as f32;
        println!("\nparameter block (f32 slots): mean {mean:.1}, max {max}");
    }

    if !undefined.is_empty() {
        println!("\nmissing helper functions / constants:");
        let mut v: Vec<_> = undefined.iter().collect();
        v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        for (id, n) in v.iter().take(20) {
            println!("  {n:>4}  {id}");
        }
    }

    if show_errors {
        println!("\nsample errors:");
        for s in &samples {
            println!("  {s}");
        }
    }

    if report_only {
        return;
    }

    let out = Path::new("src/formulas/generated.rs");
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    std::fs::write(out, emit(&ok, &defaults)).unwrap_or_else(|e| panic!("cannot write {}: {e}", out.display()));
    println!("\nwrote {} formulas to {}", ok.len(), out.display());
}

/// Emits the generated formula table as Rust.
/// Groups naga errors so the report shows what to fix next rather than 300
/// individually-worded messages.
/// Pulls the identifier out of naga's "no definition in scope" diagnostic.
fn undefined_ident(err: &str) -> Option<String> {
    let key = "identifier: `";
    let start = err.find(key)? + key.len();
    let end = err[start..].find('`')? + start;
    Some(err[start..end].to_owned())
}

fn classify(err: &str) -> String {
    for (needle, label) in [
        ("no definition in scope", "wgsl: undefined name"),
        ("expected", "wgsl: parse error"),
        ("type", "wgsl: type mismatch"),
        ("must be", "wgsl: type mismatch"),
        ("cannot", "wgsl: invalid operation"),
    ] {
        if err.contains(needle) {
            return label.to_owned();
        }
    }
    "wgsl: other".to_owned()
}

fn emit(formulas: &[translate::Formula], defaults: &defaults::Defaults) -> String {
    let mut s = String::new();
    s.push_str(
        "//! GENERATED by `tools/mb2-transpile` — do not edit.\n\
         //!\n\
         //! Translated from Mandelbulber2's OpenCL formula sources\n\
         //! (<https://github.com/buddhi1980/mandelbulber2>, © Mandelbulber Team,\n\
         //! GPL-3.0). 3DM is GPL-3.0-or-later as a result.\n\n\
         use super::{GeneratedFormula, GeneratedParam, ParamKind};\n\n\
         pub static GENERATED: &[GeneratedFormula] = &[\n",
    );

    for f in formulas {
        s.push_str(&format!(
            "    GeneratedFormula {{\n        name: {:?},\n        source: {:?},\n        param_floats: {},\n        params: &[\n",
            f.name, f.source_file, f.param_floats
        ));
        for p in &f.params {
            // Identity for matrices, which Mandelbulber computes rather than
            // stores; zeros only where the value is genuinely unknown.
            let default: Vec<f32> = match defaults.get(&p.path) {
                Some(d) => d.values.clone(),
                None if p.ty == types::ParamType::Matrix33 => {
                    vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
                }
                None => vec![0.0; p.ty.floats()],
            };
            s.push_str(&format!(
                "            GeneratedParam {{ path: {:?}, kind: ParamKind::{:?}, offset: {}, default: &{:?} }},\n",
                p.path, p.ty, p.offset, default
            ));
        }
        s.push_str("        ],\n        wgsl: r####\"");
        s.push_str(&f.wgsl_body);
        s.push_str("\"####,\n    },\n");
    }

    s.push_str("];\n");
    s
}
