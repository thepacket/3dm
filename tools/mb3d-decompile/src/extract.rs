//! Pulling the machine code out of an `.m3f` file.
//!
//! The container is an INI-ish text format: an `[OPTIONS]` block of parameter
//! defaults and distance-estimator settings, a `[CODE]` block of hex, then
//! `[END]` and free prose. Some files spell the headings in mixed case, and a
//! few carry `[SOURCE]` — readable Pascal — instead of `[CODE]`; those are the
//! JIT formulas and need no decompiling at all.

use std::path::Path;

pub struct Formula {
    /// File stem, which is the name MB3D shows.
    pub name: String,
    /// The `[CODE]` block, decoded from hex.
    pub code: Vec<u8>,
    /// `[OPTIONS]` as given: parameter names in declaration order, with their
    /// defaults. The order matters — it is the order the compiled code reads
    /// them from the parameter block.
    pub options: Vec<(String, String)>,
    /// The `[CONSTANTS]` block, in declaration order — which is the order the
    /// compiled code reads them, counting up from `PVar + 0`. Recovering these
    /// is what turns a `k0` in an expression into the number it stands for.
    pub constants: Vec<f64>,
    /// Everything after `[END]`. Around a fifth of the corpus states its
    /// mathematics here in prose, which is the only ground truth there is.
    pub description: String,
}

/// Reads every `.m3f` in `dir` that carries compiled code.
pub fn load_dir(dir: &Path) -> Vec<Formula> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<Formula> = entries
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|x| x.eq_ignore_ascii_case("m3f"))
        })
        .filter_map(|e| parse(&e.path()))
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Parses one file, or `None` if it has no compiled code to work on.
pub fn parse(path: &Path) -> Option<Formula> {
    // Not UTF-8: a handful carry stray high bytes in their prose, and losing a
    // whole formula over one mangled character in a comment would be silly.
    let raw = std::fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&raw);

    let mut section = Section::None;
    let mut hex = String::new();
    let mut options = Vec::new();
    let mut constants = Vec::new();
    let mut description = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = match trimmed[1..trimmed.len() - 1].to_ascii_uppercase().as_str() {
                "OPTIONS" => Section::Options,
                "CODE" => Section::Code,
                "CONSTANTS" => Section::Constants,
                // A JIT formula: it has real Pascal and is not our problem.
                "SOURCE" => return None,
                "END" => Section::Description,
                _ => Section::None,
            };
            continue;
        }
        match section {
            Section::Options => {
                // `.Double Scale = 2` — the leading dot marks a declaration,
                // and the word after it is a type for some and part of the
                // name for others, so the whole thing is kept verbatim.
                if let Some((key, value)) = trimmed.split_once('=') {
                    let key = key.trim().trim_start_matches('.').trim();
                    if !key.is_empty() {
                        options.push((key.to_owned(), value.trim().to_owned()));
                    }
                }
            }
            Section::Code => hex.extend(trimmed.chars().filter(|c| c.is_ascii_hexdigit())),
            Section::Constants => {
                // `Double = 0.816496580927726`. The datatype word is present
                // but every one in the corpus is a Double, and a wrong guess
                // here would shift every later constant.
                if let Some((_, value)) = trimmed.split_once('=')
                    && let Ok(number) = value.trim().parse::<f64>()
                {
                    constants.push(number);
                }
            }
            Section::Description => {
                description.push_str(line);
                description.push('\n');
            }
            Section::None => {}
        }
    }

    if hex.is_empty() {
        return None;
    }
    Some(Formula {
        name: path.file_stem()?.to_string_lossy().into_owned(),
        code: from_hex(&hex),
        options,
        constants,
        description,
    })
}

enum Section {
    None,
    Options,
    Code,
    Constants,
    Description,
}

fn from_hex(hex: &str) -> Vec<u8> {
    let digits: Vec<u8> = hex.bytes().map(|b| hex_value(b).unwrap_or(0)).collect();
    digits.chunks_exact(2).map(|p| (p[0] << 4) | p[1]).collect()
}

fn hex_value(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
