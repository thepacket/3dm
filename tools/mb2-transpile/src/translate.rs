//! OpenCL C → WGSL rewriting for a single formula.
//!
//! This is a source-to-source rewriter over a very regular corpus, not a C
//! compiler. The guiding rule is that it must never emit WGSL it is unsure
//! about: anything containing a construct we do not handle is *rejected* with a
//! reason, so the coverage report reflects reality instead of producing shaders
//! that fail to compile later.

use crate::types::{ParamType, TypeMap};

/// A parameter this formula reads, in first-use order.
#[derive(Clone, Debug)]
pub struct Param {
    /// Path as written in the source, e.g. `transformCommon.scale`.
    pub path: String,
    pub ty: ParamType,
    /// Offset in f32 units within the formula's parameter block.
    pub offset: usize,
}

pub struct Formula {
    /// e.g. `TransfRotation`, derived from `TransfRotationIteration`.
    pub name: String,
    pub source_file: String,
    pub wgsl_body: String,
    pub params: Vec<Param>,
    /// Total f32 slots the parameter block needs.
    pub param_floats: usize,
}

pub enum Rejected {
    /// The construct is listed so the report says what to implement next.
    Unsupported(String),
    Malformed(String),
}

/// Constructs that would silently mistranslate. Each is a genuine WGSL gap
/// rather than laziness: WGSL has no pointers into vector components, no
/// `while` with the same semantics we rely on here, and no C-style casts.
const UNSUPPORTED: &[(&str, &str)] = &[
    ("REAL *", "pointer aliasing into vector components"),
    ("__constant REAL *", "pointer into constant memory"),
    ("while (", "while loop"),
    ("goto ", "goto"),
    ("aux->pseudoKleinianDE", "pseudo-Kleinian DE state"),
];

pub fn translate(
    source_file: &str,
    source: &str,
    types: &TypeMap,
) -> Result<Formula, Rejected> {
    let Some((name, body)) = extract_function(source) else {
        return Err(Rejected::Malformed("no `…Iteration` function found".into()));
    };

    for (needle, why) in UNSUPPORTED {
        if body.contains(needle) {
            return Err(Rejected::Unsupported((*why).to_owned()));
        }
    }

    let body = strip_comments(&body);
    let mut params: Vec<Param> = Vec::new();
    let mut cursor = 0usize;

    let body = rewrite_params(&body, types, &mut params, &mut cursor)?;
    let body = rewrite_tokens(&body);
    let body = rewrite_vector_literals(&body);
    let body = rewrite_casts(&body);
    let body = rewrite_bool_context(&body, &params);
    let body = rewrite_declarations(&body);
    let body = brace_control_flow(&body);
    let body = rewrite_switch(&body);

    let body = rewrite_ternaries(&body);
    if body.contains('?') {
        // Anything left is nested or spans a construct the scanner declines to
        // guess at; better to reject than to emit subtly wrong arithmetic.
        return Err(Rejected::Unsupported("ternary conditional".into()));
    }

    Ok(Formula {
        name: name.trim_end_matches("Iteration").to_owned(),
        source_file: source_file.to_owned(),
        wgsl_body: body,
        params,
        param_floats: cursor,
    })
}

/// Pulls the `…Iteration` function name and its brace-balanced body.
fn extract_function(source: &str) -> Option<(String, String)> {
    let start = source.find("REAL4 ")?;
    let rest = &source[start..];
    let paren = rest.find('(')?;
    let name = rest[6..paren].trim().to_owned();
    if !name.ends_with("Iteration") {
        return None;
    }

    let open = rest.find('{')?;
    let mut depth = 0usize;
    for (i, c) in rest[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((name, rest[open + 1..open + i].to_owned()));
                }
            }
            _ => {}
        }
    }
    None
}

fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let bytes: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '/' && i + 1 < bytes.len() && bytes[i + 1] == '/' {
            while i < bytes.len() && bytes[i] != '\n' {
                i += 1;
            }
        } else if bytes[i] == '/' && i + 1 < bytes.len() && bytes[i + 1] == '*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == '*' && bytes[i + 1] == '/') {
                i += 1;
            }
            i += 2;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

/// Replaces every `fractal->path` with a uniform read, allocating a slot in the
/// formula's parameter block the first time each path is seen.
fn rewrite_params(
    body: &str,
    types: &TypeMap,
    params: &mut Vec<Param>,
    cursor: &mut usize,
) -> Result<String, Rejected> {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;

    while let Some(pos) = rest.find("fractal->") {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + "fractal->".len()..];
        let end = after
            .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
            .unwrap_or(after.len());
        let path = &after[..end];

        // A trailing dot means we split mid-swizzle (e.g. `…scale.x`); keep the
        // component and resolve the parameter without it.
        let (path, suffix) = split_swizzle(path);

        let Some(ty) = types.resolve("sFractalCl", path) else {
            return Err(Rejected::Unsupported(format!("unresolved param: {path}")));
        };

        let existing = params.iter().find(|p| p.path == path);
        let offset = match existing {
            Some(p) => p.offset,
            None => {
                let offset = *cursor;
                *cursor += ty.floats();
                params.push(Param {
                    path: path.to_owned(),
                    ty,
                    offset,
                });
                offset
            }
        };

        out.push_str(&format!("__MB2P{offset}__{suffix}"));
        rest = &after[end..];
    }
    out.push_str(rest);
    Ok(out)
}

/// Splits `foo.bar.x` into (`foo.bar`, `.x`) when the final segment is a
/// swizzle rather than a struct field.
fn split_swizzle(path: &str) -> (&str, &str) {
    if let Some(dot) = path.rfind('.') {
        let last = &path[dot + 1..];
        let is_swizzle =
            !last.is_empty() && last.len() <= 4 && last.chars().all(|c| "xyzw".contains(c));
        if is_swizzle {
            return (&path[..dot], &path[dot..]);
        }
    }
    (path, "")
}

/// Straight token substitutions: types, math builtins, aux state, literals.
fn rewrite_tokens(body: &str) -> String {
    let mut s = body.to_owned();

    // OpenCL's `native_*` are precision-relaxed variants of the same functions.
    for (from, to) in [
        ("native_divide", "mb2_divide"),
        ("native_recip", "mb2_recip"),
        ("native_powr", "pow"),
        ("native_sqrt", "sqrt"),
        ("native_rsqrt", "inverseSqrt"),
        ("native_exp", "exp"),
        ("native_log", "log"),
        ("native_sin", "sin"),
        ("native_cos", "cos"),
        ("native_tan", "tan"),
        ("half_sqrt", "sqrt"),
    ] {
        s = s.replace(from, to);
    }

    // Aux state maps onto 3DM's running iteration state.
    for (from, to) in [
        ("aux->DE0", "(*aux).de0"),
        ("aux->DE", "(*aux).de"),
        ("aux->color", "(*aux).color"),
        ("aux->const_c", "(*aux).const_c"),
        ("aux->old_z", "(*aux).old_z"),
        ("aux->actualScaleA", "(*aux).actual_scale_a"),
        ("aux->actualScale", "(*aux).actual_scale"),
        ("aux->r", "(*aux).r"),
        ("aux->i", "(*aux).i"),
        ("aux->c", "(*aux).c"),
        ("aux->dist", "(*aux).dist"),
        ("aux->pos_neg", "(*aux).pos_neg"),
        ("aux->temp1000", "(*aux).temp1000"),
        ("aux->colorHybrid", "(*aux).color_hybrid"),
        ("aux->r_dz", "(*aux).r_dz"),
        ("aux->old_r", "(*aux).old_r"),
        ("aux->lastZ", "(*aux).last_z"),
    ] {
        s = s.replace(from, to);
    }

    // Types. Longest first so REAL4 is not eaten by REAL.
    for (from, to) in [
        ("REAL4", "vec4<f32>"),
        ("REAL3", "vec3<f32>"),
        ("REAL2", "vec2<f32>"),
        ("REAL", "f32"),
    ] {
        s = s.replace(from, to);
    }

    for (from, to) in [
        ("fabs(", "abs("),
        ("Matrix33MulFloat4(", "mb2_mat_mul("),
        ("Q_UNUSED(aux);", ""),
        ("Q_UNUSED(fractal);", ""),
        ("mad(", "fma("),
    ] {
        s = s.replace(from, to);
    }

    s = strip_float_suffixes(&s);
    s = rewrite_increments(&s);
    s
}

/// `i++` / `i--` have no WGSL equivalent as expressions; as statements they
/// become explicit assignments.
fn rewrite_increments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && (chars[i] == '+' || chars[i] == '-') && chars[i + 1] == chars[i] {
            // Walk back over the identifier we just emitted.
            let end = out.len();
            let start = out
                .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
                .map_or(0, |p| p + 1);
            if start < end {
                let name = out[start..end].to_owned();
                let op = if chars[i] == '+' { '+' } else { '-' };
                out.push_str(&format!(" = {name} {op} 1"));
                i += 2;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// OpenCL writes vector literals as `(REAL4){a, b, c, d}`; WGSL uses a
/// constructor call. By this point the type has already become `vec4<f32>`.
fn rewrite_vector_literals(src: &str) -> String {
    let mut s = src.to_owned();
    for ty in ["vec4<f32>", "vec3<f32>", "vec2<f32>"] {
        let open = format!("({ty}){{");
        while let Some(pos) = s.find(&open) {
            let after = pos + open.len();
            let Some(close) = find_matching(&s, after - 1, '{', '}') else {
                break;
            };
            let inner = s[after..close].to_owned();
            s = format!("{}{ty}({inner}){}", &s[..pos], &s[close + 1..]);
        }
    }
    s
}

/// C casts `(REAL4)(a, b, c, d)` are constructor calls in WGSL.
fn rewrite_casts(src: &str) -> String {
    let mut s = src.to_owned();
    for ty in ["vec4<f32>", "vec3<f32>", "vec2<f32>", "f32", "i32"] {
        s = s.replace(&format!("({ty})("), &format!("{ty}("));
    }
    s
}

/// C treats any non-zero integer as true; WGSL has no such coercion, so an
/// integer parameter used as a flag has to be compared explicitly.
///
/// The operator can be separated from the parameter by arbitrary whitespace —
/// Mandelbulber wraps long conditions across lines — so this scans tokens
/// rather than matching adjacent text.
fn rewrite_bool_context(src: &str, params: &[Param]) -> String {
    let int_offsets: Vec<usize> = params
        .iter()
        .filter(|p| p.ty == ParamType::Int)
        .map(|p| p.offset)
        .collect();
    if int_offsets.is_empty() {
        return src.to_owned();
    }

    let mut out = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;

    while i < src.len() {
        if !src[i..].starts_with("__MB2P") {
            out.push(src[i..].chars().next().unwrap());
            i += src[i..].chars().next().unwrap().len_utf8();
            continue;
        }

        // Read the placeholder number, which runs to the closing delimiter.
        let num_start = i + "__MB2P".len();
        let mut j = num_start;
        while j < src.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        let Ok(offset) = src[num_start..j].parse::<usize>() else {
            out.push_str("__MB2P");
            i = num_start;
            continue;
        };
        j += "__".len();

        if !int_offsets.contains(&offset) {
            out.push_str(&src[i..j]);
            i = j;
            continue;
        }

        let after = src[j..].trim_start();
        let before = out.trim_end();

        // Being *next to* `&&` is not enough. In
        // `enabled && aux->i >= startIterations && ...` the `startIterations`
        // parameter is followed by `&&`, but it is the right operand of `>=`
        // and so a number, not a condition. Wrapping it produced `i32 >= bool`,
        // which parses but fails validation — which is how it survived until
        // the compile check grew a validator. An operator that consumes a value
        // on either side therefore vetoes the boolean reading.
        const VALUE_OPS: [&str; 11] = [
            "<=", ">=", "==", "!=", "<", ">", "+", "-", "*", "/", "%",
        ];
        let is_value = VALUE_OPS.iter().any(|op| before.ends_with(op))
            || VALUE_OPS.iter().any(|op| after.starts_with(op));

        let boolean_context = !is_value
            && (after.starts_with("&&")
                || after.starts_with("||")
                || before.ends_with("&&")
                || before.ends_with("||")
                || before.ends_with('!')
                || (before.ends_with("if (") && after.starts_with(')')));

        if boolean_context {
            out.push_str(&format!("(__MB2P{offset}__ != 0)"));
        } else {
            out.push_str(&src[i..j]);
        }
        i = j;
    }

    out
}

/// C `switch` becomes WGSL `switch`: the condition loses its parentheses, each
/// case body gains braces, and `break` disappears because WGSL cases do not
/// fall through.
fn rewrite_switch(src: &str) -> String {
    let mut s = src.to_owned();
    let mut from = 0usize;

    while let Some(rel) = s[from..].find("switch") {
        let kw = from + rel;
        if !word_boundary(&s, kw, 6) {
            from = kw + 6;
            continue;
        }
        let Some(open_paren) = s[kw..].find('(').map(|i| i + kw) else {
            break;
        };
        let Some(close_paren) = find_matching(&s, open_paren, '(', ')') else {
            break;
        };
        let Some(open_brace) = s[close_paren..].find('{').map(|i| i + close_paren) else {
            break;
        };
        let Some(close_brace) = find_matching(&s, open_brace, '{', '}') else {
            break;
        };

        let cond = s[open_paren + 1..close_paren].trim().to_owned();
        let inner = s[open_brace + 1..close_brace].to_owned();
        let rebuilt = format!("switch {cond} {{\n{}\n}}", brace_cases(&inner));

        s = format!("{}{rebuilt}{}", &s[..kw], &s[close_brace + 1..]);
        from = kw + rebuilt.len();
    }
    s
}

/// Splits a switch body at its `case`/`default` labels and braces each arm.
fn brace_cases(inner: &str) -> String {
    let mut arms: Vec<(String, String)> = Vec::new();
    let mut label = String::new();
    let mut body = String::new();

    for line in inner.lines() {
        let t = line.trim();
        let is_label = (t.starts_with("case ") || t.starts_with("default")) && t.contains(':');
        if is_label {
            if !label.is_empty() {
                arms.push((label.clone(), body.clone()));
            }
            let (head, rest) = t.split_once(':').unwrap();
            label = head.trim().to_owned();
            body = rest.trim().to_owned();
            body.push('\n');
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }
    if !label.is_empty() {
        arms.push((label, body));
    }

    arms.iter()
        .map(|(label, body)| {
            let cleaned: String = body
                .lines()
                .filter(|l| l.trim() != "break;")
                .collect::<Vec<_>>()
                .join("\n");
            format!("{label}: {{\n{cleaned}\n}}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `cond ? a : b` becomes `select(b, a, cond)`.
///
/// Innermost-first, so nested ternaries collapse from the inside out. The
/// boundaries of each of the three expressions are found by scanning outward to
/// the nearest delimiter at the same nesting depth; anything that does not
/// resolve cleanly is left in place and the formula is rejected rather than
/// mistranslated.
fn rewrite_ternaries(src: &str) -> String {
    let mut s = src.to_owned();
    for _ in 0..64 {
        let Some(q) = last_innermost_question(&s) else {
            break;
        };
        let Some(colon) = matching_colon(&s, q) else {
            break;
        };
        let Some(start) = expr_start(&s, q) else { break };
        let Some(end) = expr_end(&s, colon) else { break };

        let cond = s[start..q].trim().to_owned();
        let then = s[q + 1..colon].trim().to_owned();
        let els = s[colon + 1..end].trim().to_owned();
        if cond.is_empty() || then.is_empty() || els.is_empty() {
            break;
        }

        s = format!(
            "{}select({els}, {then}, {cond}){}",
            &s[..start],
            &s[end..]
        );
    }
    s
}

/// The last `?` in the string is necessarily innermost.
fn last_innermost_question(s: &str) -> Option<usize> {
    s.rfind('?')
}

/// The `:` matching this `?`, at the same parenthesis depth.
fn matching_colon(s: &str, q: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    for i in q + 1..s.len() {
        match bytes[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
            }
            b':' if depth == 0 => return Some(i),
            b';' => return None,
            _ => {}
        }
    }
    None
}

/// Start of the condition: scan left to a delimiter at depth zero.
fn expr_start(s: &str, q: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut i = q;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b')' | b']' => depth += 1,
            b'(' | b'[' => {
                if depth == 0 {
                    return Some(i + 1);
                }
                depth -= 1;
            }
            b';' | b'{' | b'}' | b',' if depth == 0 => return Some(i + 1),
            b'=' if depth == 0 && bytes.get(i + 1) != Some(&b'=') => return Some(i + 1),
            _ => {}
        }
    }
    Some(0)
}

/// End of the else-expression: scan right to a delimiter at depth zero.
fn expr_end(s: &str, colon: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    for i in colon + 1..s.len() {
        match bytes[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
            b';' | b',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// WGSL requires braces on `if` / `else` / `for` bodies; C does not.
fn brace_control_flow(src: &str) -> String {
    let mut s = src.to_owned();
    let mut guard = 0;
    loop {
        guard += 1;
        if guard > 4000 {
            break;
        }
        let Some(edit) = next_braceless(&s) else { break };
        let (insert_at, end) = edit;
        s = format!("{{ {} }}", &s[insert_at..end])
            .to_owned()
            .pipe(|body| format!("{}{}{}", &s[..insert_at], body, &s[end..]));
    }
    s
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

/// Finds the next `if (…)` / `for (…)` / `else` whose body lacks braces,
/// returning the byte range of that body.
fn next_braceless(s: &str) -> Option<(usize, usize)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < s.len() {
        let (kw_end, needs_paren) = if s[i..].starts_with("if") && word_boundary(s, i, 2) {
            (i + 2, true)
        } else if s[i..].starts_with("for") && word_boundary(s, i, 3) {
            (i + 3, true)
        } else if s[i..].starts_with("else") && word_boundary(s, i, 4) {
            (i + 4, false)
        } else {
            i += 1;
            continue;
        };

        let is_else = !needs_paren;
        let body_start = if needs_paren {
            let open = s[kw_end..].find('(')? + kw_end;
            find_matching(s, open, '(', ')')? + 1
        } else {
            kw_end
        };

        let mut j = body_start;
        while j < s.len() && (bytes[j] as char).is_whitespace() {
            j += 1;
        }
        if j >= s.len() {
            return None;
        }
        // Already braced, or an `else if` chain, which WGSL allows unbraced.
        // An `if`/`for` body that happens to start with `if` still needs
        // braces, so the exemption applies only to `else`.
        if bytes[j] == b'{' || (is_else && s[j..].starts_with("if")) {
            i = j;
            continue;
        }

        // Body is a single statement: extend to its terminating semicolon.
        let mut depth = 0i32;
        let mut k = j;
        while k < s.len() {
            match bytes[k] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b';' if depth <= 0 => {
                    return Some((j, k + 1));
                }
                _ => {}
            }
            k += 1;
        }
        return None;
    }
    None
}

fn word_boundary(s: &str, start: usize, len: usize) -> bool {
    let before_ok = start == 0
        || !s[..start]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_');
    let after_ok = s[start + len..]
        .chars()
        .next()
        .is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
    before_ok && after_ok
}

fn find_matching(s: &str, open_at: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s[open_at..].char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(open_at + i);
            }
        }
    }
    None
}

/// `1.0f` → `1.0`; WGSL has no `f` suffix.
fn strip_float_suffixes(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len());
    for (i, &c) in chars.iter().enumerate() {
        if c == 'f'
            && i > 0
            && (chars[i - 1].is_ascii_digit() || chars[i - 1] == '.')
            && chars
                .get(i + 1)
                .is_none_or(|n| !n.is_alphanumeric() && *n != '_')
        {
            continue;
        }
        out.push(c);
    }
    out
}

/// C declarations become WGSL `var` bindings.
fn rewrite_declarations(body: &str) -> String {
    let types = [
        "vec4<f32>",
        "vec3<f32>",
        "vec2<f32>",
        "f32",
        "int",
        "i32",
        "bool",
    ];
    let mut out = String::new();

    for raw in body.lines() {
        let line = raw.trim();
        let indent = &raw[..raw.len() - raw.trim_start().len()];

        let mut rewritten: Option<String> = None;
        for ty in types {
            let prefix = format!("{ty} ");
            if !line.starts_with(&prefix) {
                continue;
            }
            let decl = line[prefix.len()..].trim_end_matches(';').trim();
            let wgsl_ty = if ty == "int" { "i32" } else { ty };

            // Mandelbulber wraps long initialisers across lines. Appending a
            // `;` to the first line of such a declaration silently truncates
            // the expression, so only complete statements are terminated here.
            let complete = line.ends_with(';');

            rewritten = Some(if let Some((lhs, rhs)) = decl.split_once('=') {
                if complete {
                    format!("{indent}var {}: {wgsl_ty} = {};", lhs.trim(), rhs.trim())
                } else {
                    // Keep the continuation intact; its own line ends the
                    // statement.
                    let head = line[prefix.len()..].split_once('=').map(|(l, r)| {
                        format!("{indent}var {}: {wgsl_ty} ={}", l.trim(), r)
                    });
                    match head {
                        Some(h) => h,
                        None => raw.to_owned(),
                    }
                }
            } else if !complete {
                raw.to_owned()
            } else {
                decl.split(',')
                    .map(|n| format!("{indent}var {}: {wgsl_ty};", n.trim()))
                    .collect::<Vec<_>>()
                    .join("\n")
            });
            break;
        }

        out.push_str(&rewritten.unwrap_or_else(|| raw.to_owned()));
        out.push('\n');
    }
    out
}
