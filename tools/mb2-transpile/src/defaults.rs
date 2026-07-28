//! Recovers each parameter's default value from Mandelbulber2's C++ sources.
//!
//! Without this every transpiled formula would run with zeroed parameters, and
//! a Mandelbox with `scale = 0` renders nothing at all — so the defaults are
//! not cosmetic, they are what makes a formula produce a shape.
//!
//! The value has to be joined across two files:
//!
//!   `src/fractal.cpp`        `mandelbox.foldingLimit = …Get<double>("mandelbox_folding_limit")`
//!   `src/initparameters.cpp` `par->addParam("mandelbox_folding_limit", 1.0, …)`
//!
//! giving struct path → settings name → default.

use std::collections::HashMap;

/// A parameter default, flattened to the f32 slots the GPU sees.
#[derive(Clone, Debug)]
pub struct Default {
    pub values: Vec<f32>,
}

pub struct Defaults {
    by_path: HashMap<String, Default>,
    /// Settings names that had no `addParam`, kept for reporting.
    pub unmatched: usize,
}

impl Defaults {
    pub fn parse(fractal_cpp: &str, initparameters_cpp: &str) -> Self {
        let names = parse_path_to_name(fractal_cpp);
        let values = parse_name_to_value(initparameters_cpp);

        let mut by_path = HashMap::new();
        let mut unmatched = 0;
        for (path, name) in names {
            match values.get(&name) {
                Some(v) => {
                    by_path.insert(path, v.clone());
                }
                None => unmatched += 1,
            }
        }

        Self {
            by_path,
            unmatched,
        }
    }

    pub fn get(&self, path: &str) -> Option<&Default> {
        self.by_path.get(path)
    }

    pub fn len(&self) -> usize {
        self.by_path.len()
    }
}

/// `foo.bar = container->Get<T>("settings_name");`
///
/// Mandelbulber wraps long assignments across lines, so this joins statements
/// on `;` rather than reading line by line — the same mistake, made once in the
/// declaration rewriter, silently loses a third of the parameters here.
fn parse_path_to_name(src: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in join_statements(src) {
        let line = line.trim();
        let Some((lhs, rhs)) = line.split_once('=') else {
            continue;
        };
        if !rhs.contains("container->Get<") {
            continue;
        }
        let path = lhs.trim();
        // Only plain `struct.field` assignments; anything computed is skipped.
        if path.is_empty() || !path.contains('.') || path.contains(' ') {
            continue;
        }
        let Some(open) = rhs.find('"') else { continue };
        let Some(close) = rhs[open + 1..].find('"') else {
            continue;
        };
        let name = &rhs[open + 1..open + 1 + close];
        out.push((path.to_owned(), name.to_owned()));
    }
    out
}

/// Splits a C++ source into statements, collapsing wrapped lines.
fn join_statements(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("//") {
            continue;
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(t);
        if t.ends_with(';') {
            out.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// `par->addParam("name", <default>, …);`
fn parse_name_to_value(src: &str) -> HashMap<String, Default> {
    let mut out = HashMap::new();
    for line in join_statements(src) {
        let line = line.trim();
        // The call itself may have wrapped, leaving `addParam( "name"` once
        // statements are joined, so the quote is found rather than assumed
        // adjacent.
        let Some(pos) = line.find("addParam(") else {
            continue;
        };
        let after_call = &line[pos + "addParam(".len()..];
        let Some(quote) = after_call.find('"') else {
            continue;
        };
        // Only whitespace may separate the call from its first argument.
        if !after_call[..quote].trim().is_empty() {
            continue;
        }
        let rest = &after_call[quote + 1..];
        let Some(close) = rest.find('"') else { continue };
        let name = rest[..close].to_owned();

        let after = rest[close + 1..].trim_start().trim_start_matches(',').trim();
        if let Some(v) = parse_value(after) {
            out.insert(name, Default { values: v });
        }
    }
    out
}

/// Reads the default out of the argument list: a scalar, a bool, or a vector
/// constructor.
fn parse_value(s: &str) -> Option<Vec<f32>> {
    let s = s.trim();

    for ctor in ["CVector4(", "CVector3(", "CVector2("] {
        if let Some(rest) = s.strip_prefix(ctor) {
            let end = rest.find(')')?;
            let mut v: Vec<f32> = rest[..end]
                .split(',')
                .filter_map(|p| p.trim().parse::<f32>().ok())
                .collect();
            // Pad to a full vec4 so the slot layout is uniform.
            while v.len() < 4 {
                v.push(0.0);
            }
            return Some(v);
        }
    }

    let token: String = s
        .chars()
        .take_while(|c| !matches!(c, ',' | ')'))
        .collect::<String>()
        .trim()
        .to_owned();

    if token == "true" {
        return Some(vec![1.0]);
    }
    if token == "false" {
        return Some(vec![0.0]);
    }
    token.parse::<f32>().ok().map(|v| vec![v])
}
