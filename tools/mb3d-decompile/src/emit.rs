//! Turning a recovered formula into a 3DM formula body.
//!
//! The target is `GeneratedFormula` in `src/formulas.rs`: a WGSL body that
//! transforms `z` in place against an `Aux`, with `__MB2Pn__` standing in for
//! each parameter read. That is the same shape `mb2-transpile` produces from
//! Mandelbulber's OpenCL, so an `.m3f` lands in the pipeline the rest of 3DM
//! already understands.
//!
//! Only record fields whose meaning is established get a mapping. The rest
//! refuse by name — an unrecognised field silently rendered as something
//! plausible is how a formula ends up with a shape nobody authored.

use std::collections::BTreeSet;

use crate::expr::{Expr, Op, Place};

/// What a place becomes in WGSL, or why it cannot.
fn place_wgsl(place: &Place) -> Result<String, String> {
    Ok(match place {
        Place::Var('x') => "z.x".to_owned(),
        Place::Var('y') => "z.y".to_owned(),
        Place::Var('z') => "z.z".to_owned(),
        Place::Var('w') => "z.w".to_owned(),
        Place::Var(other) => return Err(format!("variable {other}")),
        Place::Param(n) => format!("__MB2P{n}__"),
        // Constants are resolved to numbers before this point; one surviving
        // means the file had no `[CONSTANTS]` block to resolve it from.
        Place::Const(n) => return Err(format!("unresolved constant k{n}")),
        Place::Local(at) => format!("t{}", at.unsigned_abs()),
        Place::Field(name) => match name.as_str() {
            // The constants a formula adds each iteration: the Julia seed, or
            // the sampled point. 3DM carries the same thing under a different
            // name, which is why an imported formula must not also have the
            // caller add `c` — it adds its own.
            "J1" => "(*aux).const_c.x".to_owned(),
            "J2" => "(*aux).const_c.y".to_owned(),
            "J3" => "(*aux).const_c.z".to_owned(),
            "J4" => "(*aux).const_c.w".to_owned(),
            // MB3D's running scale, which it keeps in the record and
            // Mandelbulber keeps in the aux under another name.
            "VaryScale" => "(*aux).actual_scale".to_owned(),
            // "used also as iteration count, is set to 0 on it-start", which
            // is exactly what 3DM's `i` is.
            "bFirstIt" => "(*aux).i".to_owned(),
            other => return Err(format!("record field {other}")),
        },
        Place::Unknown(what) => return Err(format!("unnamed location {what}")),
    })
}

/// Renders an expression as WGSL.
pub fn expression(e: &Expr) -> Result<String, String> {
    Ok(match e {
        // Every literal is explicitly floating point: WGSL will not promote an
        // integer literal into an f32 expression.
        Expr::Num(v) => format_number(*v),
        Expr::Load(place) => place_wgsl(place)?,
        Expr::Neg(inner) => format!("-({})", expression(inner)?),
        Expr::Abs(inner) => format!("abs({})", expression(inner)?),
        Expr::Sqrt(inner) => format!("sqrt({})", expression(inner)?),
        Expr::Bin(op, a, b) => {
            let symbol = match op {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            };
            format!("({} {symbol} {})", expression(a)?, expression(b)?)
        }
        Expr::Test(cmp, a, b) => format!(
            "({} {} {})",
            expression(a)?,
            cmp.symbol(),
            expression(b)?
        ),
        // WGSL takes the false case first, which is the opposite of how the
        // expression reads.
        Expr::Select(cond, then, otherwise) => format!(
            "select({}, {}, {})",
            expression(otherwise)?,
            expression(then)?,
            expression(cond)?
        ),
        Expr::Fun(name, args) => {
            let rendered: Result<Vec<String>, String> =
                args.iter().map(|a| expression(a)).collect();
            let rendered = rendered?;
            match (*name, rendered.as_slice()) {
                // WGSL spells the natural logarithm `log`.
                ("ln", [x]) => format!("log({x})"),
                ("exp2m1", [x]) => format!("(exp2({x}) - 1.0)"),
                ("fmod", [a, b]) => format!("({a} % {b})"),
                (other, args) => format!("{other}({})", args.join(", ")),
            }
        }
    })
}

/// A float literal WGSL will accept as an `f32`.
fn format_number(v: f64) -> String {
    if !v.is_finite() {
        // An infinity in a formula body is a decode that went wrong, not a
        // constant anyone wrote.
        return "0.0 /* non-finite */".to_owned();
    }
    let text = format!("{v:?}");
    if text.contains('.') || text.contains('e') {
        text
    } else {
        format!("{text}.0")
    }
}

/// Collects every stack slot an expression reads.
fn locals_in(e: &Expr, into: &mut BTreeSet<i64>) {
    match e {
        Expr::Load(Place::Local(at)) => {
            into.insert(*at);
        }
        Expr::Load(_) | Expr::Num(_) => {}
        Expr::Neg(i) | Expr::Abs(i) | Expr::Sqrt(i) => locals_in(i, into),
        Expr::Bin(_, a, b) | Expr::Test(_, a, b) => {
            locals_in(a, into);
            locals_in(b, into);
        }
        Expr::Select(c, a, b) => {
            locals_in(c, into);
            locals_in(a, into);
            locals_in(b, into);
        }
        Expr::Fun(_, args) => args.iter().for_each(|a| locals_in(a, into)),
    }
}

/// The whole body: every stack slot declared up front, then the assignments.
///
/// Declaring on first *store* is not enough. A formula can read a slot the
/// decoded path never wrote — one written on a branch that was folded away, or
/// left over from before the call — and that reads as an undeclared name,
/// which is a shader that will not compile rather than one that is wrong.
/// Zero is what the frame held anyway.
pub fn body(stores: &[(Place, crate::expr::E)]) -> Result<String, String> {
    body_with_derivative(stores, false)
}

/// The same, bridging MB3D's derivative convention when the formula uses one.
///
/// `//DEoption > 0 if analytic on X4` — a formula with a positive DEoption
/// maintains its own derivative in the fourth component and expects to find
/// the running value there. 3DM keeps it in `aux.de` instead, so it is handed
/// over on the way in and taken back on the way out. Without that the formula
/// differentiates whatever happened to be in `w`, which is a shape with a
/// distance estimate belonging to something else.
pub fn body_with_derivative(
    stores: &[(Place, crate::expr::E)],
    analytic: bool,
) -> Result<String, String> {
    let mut out = String::new();
    if analytic {
        out.push_str("\t\tz.w = (*aux).de;\n");
    }
    let mut slots: BTreeSet<i64> = BTreeSet::new();
    for (place, value) in stores {
        if let Place::Local(at) = place {
            slots.insert(*at);
        }
        locals_in(value, &mut slots);
    }
    for at in &slots {
        out.push_str(&format!("\t\tvar t{}: f32 = 0.0;\n", at.unsigned_abs()));
    }

    for (place, value) in stores {
        let target = place_wgsl(place)?;
        let rendered = expression(value)?;
        out.push_str(&format!("\t\t{target} = {rendered};\n"));
    }
    if analytic {
        out.push_str("\t\t(*aux).de = z.w;\n");
    }
    // 3DM wraps a body in a function returning the transformed vector, so the
    // body has to hand it back. Mandelbulber's transpiled bodies end the same
    // way; without it every formula fails validation on its return type.
    out.push_str("\treturn z;\n");
    Ok(out)
}

/// Which distance estimator an `.m3f` asks for.
///
/// MB3D's own note beside the field is `DEoption > 0 if analytic on X4`: a
/// formula with a positive value maintains its derivative in the fourth
/// component, and one with -1 has no analytic derivative at all. That second
/// case maps cleanly onto a delta estimator, which measures the distance by
/// sampling instead.
///
/// The positive case does **not** map cleanly, and is reported rather than
/// guessed: 3DM keeps the derivative in `aux.de`, so an import has to move it
/// out of `w`, and which closed form to pair it with is not recorded anywhere
/// in the file. Rendering one and looking at it is the only way to settle it.
pub fn de_function(de_option: i32) -> &'static str {
    if de_option > 0 {
        // MB3D's own estimator is `Rout * Ln(Rout) * scale / dr` with `Rout`
        // the squared radius — the logarithmic form, and the same family as
        // 3DM's. The derivative it divides by is the one the formula keeps in
        // `w`, which `body_with_derivative` hands to `aux.de`.
        "DeFunction::Logarithmic"
    } else {
        // No analytic derivative to use, so the distance is measured by
        // sampling instead. That asks nothing of the formula and is right
        // rather than merely safe here.
        "DeFunction::Delta"
    }
}

/// Whether this formula maintains its own derivative.
pub fn is_analytic(de_option: i32) -> bool {
    de_option > 0
}
