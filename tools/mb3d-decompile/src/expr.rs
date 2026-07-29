//! Expression trees, and printing them back as arithmetic.

use std::fmt::Write;
use std::rc::Rc;

/// A named location the formula reads or writes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Place {
    /// The iterated vector, passed by reference in `eax`, `edx`, `ecx`.
    Var(char),
    /// A user parameter, `n` slots below `PVar`.
    Param(usize),
    /// The constant pool at and above `PVar`.
    Const(usize),
    /// A named field of `TIteration3Dext`.
    Field(String),
    /// A slot in the formula's own stack frame. Compilers spill intermediate
    /// values here, so these have to be tracked by address rather than by
    /// instruction, or a store and its matching load never meet and the
    /// expression comes out in disconnected fragments.
    Local(i64),
    /// Somewhere the register model could not name.
    Unknown(String),
}

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Place::Var(c) => write!(f, "{c}"),
            Place::Param(n) => write!(f, "p{n}"),
            Place::Const(n) => write!(f, "k{n}"),
            Place::Field(name) => write!(f, "{name}"),
            Place::Local(at) => write!(f, "t{}", at.unsigned_abs()),
            Place::Unknown(what) => write!(f, "?{what}"),
        }
    }
}

/// How two values were compared. x87 sets its flags so that the unsigned
/// integer conditions read as the float ones, which is why `jb` means "less
/// than" here and not "below".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cmp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl Cmp {
    pub fn symbol(self) -> &'static str {
        match self {
            Cmp::Lt => "<",
            Cmp::Le => "<=",
            Cmp::Gt => ">",
            Cmp::Ge => ">=",
            Cmp::Eq => "==",
            Cmp::Ne => "!=",
        }
    }

    /// The condition that holds when this one does not — needed because a
    /// conditional jump names the branch that is *taken*, while the code that
    /// follows it is the branch that is not.
    pub fn negate(self) -> Self {
        match self {
            Cmp::Lt => Cmp::Ge,
            Cmp::Le => Cmp::Gt,
            Cmp::Gt => Cmp::Le,
            Cmp::Ge => Cmp::Lt,
            Cmp::Eq => Cmp::Ne,
            Cmp::Ne => Cmp::Eq,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn symbol(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }

    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Num(f64),
    Load(Place),
    Neg(Rc<Expr>),
    Abs(Rc<Expr>),
    Sqrt(Rc<Expr>),
    Bin(Op, Rc<Expr>, Rc<Expr>),
    /// Anything the FPU does that is not arithmetic: `fsin`, `fpatan` and the
    /// rest. Kept as a named call rather than being given a variant each,
    /// because what matters downstream is only that the name survives.
    Fun(&'static str, Vec<Rc<Expr>>),
    /// A comparison, which is only ever the test of a `Select`.
    Test(Cmp, Rc<Expr>, Rc<Expr>),
    /// The value a place holds after two paths rejoin.
    Select(Rc<Expr>, Rc<Expr>, Rc<Expr>),
}

pub type E = Rc<Expr>;

pub fn num(v: f64) -> E {
    Rc::new(Expr::Num(v))
}

pub fn load(place: Place) -> E {
    Rc::new(Expr::Load(place))
}

/// Builds `a op b`, folding the cases that otherwise litter the output.
///
/// Compiler output is full of identities that were free in registers and are
/// noise on the page — multiplying by one to get a value onto the stack, or
/// subtracting a zero the optimiser left behind. Folding them is what makes
/// the difference between a decode a person can read and one they cannot.
pub fn bin(op: Op, a: E, b: E) -> E {
    if let (Expr::Num(x), Expr::Num(y)) = (&*a, &*b) {
        let folded = match op {
            Op::Add => x + y,
            Op::Sub => x - y,
            Op::Mul => x * y,
            Op::Div => {
                if *y == 0.0 {
                    return Rc::new(Expr::Bin(op, a.clone(), b.clone()));
                }
                x / y
            }
        };
        return num(folded);
    }
    // `2^x - 1` then `+ 1` is how x87 spells `2^x`: `f2xm1` only covers a
    // narrow range, so the compiler always follows it with `fld1; faddp`.
    // Undoing that pair is the difference between a readable exponent and a
    // page of scaffolding.
    if op == Op::Add {
        if let (Expr::Fun("exp2m1", args), Expr::Num(one)) = (&*a, &*b)
            && *one == 1.0
            && args.len() == 1
        {
            return call("exp2", vec![args[0].clone()]);
        }
        if let (Expr::Num(one), Expr::Fun("exp2m1", args)) = (&*a, &*b)
            && *one == 1.0
            && args.len() == 1
        {
            return call("exp2", vec![args[0].clone()]);
        }
    }
    // Delphi has no `pow`: it open-codes `x^n` as `exp(n * ln x)`, and the
    // FPU has neither `exp` nor `ln` either, so both are in turn built from
    // `fldln2`, `fldl2e` and a base-2 exponent. Undoing that chain is three
    // exact identities, and it is the difference between a formula that reads
    // as a power and one that reads as a page of logarithms.
    if op == Op::Mul {
        // ln 2 * log2(x) is ln x.
        for (constant, other) in [(&a, &b), (&b, &a)] {
            if let (Expr::Num(v), Expr::Fun("log2", args)) = (&**constant, &**other)
                && *v == std::f64::consts::LN_2
                && args.len() == 1
            {
                return call("ln", vec![args[0].clone()]);
            }
        }
    }
    // `2^a * 2^b` is `2^(a+b)`, which is what `fscale` reassembles after
    // `frndint` split the exponent into its whole and fractional parts.
    if op == Op::Mul
        && let (Expr::Fun("exp2", left), Expr::Fun("exp2", right)) = (&*a, &*b)
        && left.len() == 1
        && right.len() == 1
    {
        return call("exp2", vec![bin(Op::Add, left[0].clone(), right[0].clone())]);
    }

    match (op, &*a, &*b) {
        (Op::Add, Expr::Num(z), _) if *z == 0.0 => return b,
        (Op::Add | Op::Sub, _, Expr::Num(z)) if *z == 0.0 => return a,
        (Op::Mul, Expr::Num(o), _) if *o == 1.0 => return b,
        (Op::Mul, _, Expr::Num(o)) if *o == 1.0 => return a,
        (Op::Div, _, Expr::Num(o)) if *o == 1.0 => return a,
        (Op::Mul, Expr::Num(z), _) | (Op::Mul, _, Expr::Num(z)) if *z == 0.0 => return num(0.0),
        // `0 - x` reads as a negation, which is what it is.
        (Op::Sub, Expr::Num(z), _) if *z == 0.0 => return neg(b),
        _ => {}
    }
    Rc::new(Expr::Bin(op, a, b))
}

pub fn neg(e: E) -> E {
    match &*e {
        Expr::Num(v) => num(-v),
        Expr::Neg(inner) => inner.clone(),
        _ => Rc::new(Expr::Neg(e)),
    }
}

pub fn abs(e: E) -> E {
    match &*e {
        Expr::Num(v) => num(v.abs()),
        // `abs` of something already absolute, or of a negation, is the same
        // as `abs` of the value itself.
        Expr::Abs(_) => e.clone(),
        Expr::Neg(inner) => abs(inner.clone()),
        _ => Rc::new(Expr::Abs(e)),
    }
}

pub fn sqrt(e: E) -> E {
    Rc::new(Expr::Sqrt(e))
}

/// Builds `name(args)`, folding it away where the answer is already known.
///
/// Compiler output is full of these. Delphi's `Exp` and `Power` are open-coded
/// against `ln 2` and `log2 e`, so a decode that does not fold leaves
/// `log2(0.6931471805599453)` sitting in the middle of an expression — a
/// constant, spelled as a computation, obscuring the shape of everything
/// around it.
pub fn call(name: &'static str, args: Vec<E>) -> E {
    // 2^(x * log2 e) is e^x, and e^(n * ln x) is x^n. Applied in that order
    // they turn the whole open-coded sequence back into a single power.
    if name == "exp2" && args.len() == 1 && let Expr::Bin(Op::Mul, left, right) = &*args[0] {
        {
            for (constant, other) in [(left, right), (right, left)] {
                if let Expr::Num(v) = &**constant
                    && *v == std::f64::consts::LOG2_E
                {
                    return call("exp", vec![other.clone()]);
                }
            }
        }
    }
    if name == "exp" && args.len() == 1 && let Expr::Bin(Op::Mul, left, right) = &*args[0] {
        {
            for (exponent, base) in [(left, right), (right, left)] {
                if let Expr::Fun("ln", inner) = &**base
                    && inner.len() == 1
                {
                    return Rc::new(Expr::Fun("pow", vec![inner[0].clone(), exponent.clone()]));
                }
            }
        }
    }
    if let [Expr::Num(v)] = args.iter().map(|a| (**a).clone()).collect::<Vec<_>>()[..] {
        let folded = match name {
            "sqrt" if v >= 0.0 => Some(v.sqrt()),
            "log2" if v > 0.0 => Some(v.log2()),
            "exp2" => Some(v.exp2()),
            "exp2m1" => Some(v.exp2() - 1.0),
            "ln" if v > 0.0 => Some(v.ln()),
            "exp" => Some(v.exp()),
            "sin" => Some(v.sin()),
            "cos" => Some(v.cos()),
            "tan" => Some(v.tan()),
            "round" => Some(v.round()),
            _ => None,
        };
        if let Some(value) = folded {
            return num(value);
        }
    }
    Rc::new(Expr::Fun(name, args))
}

pub fn test(cmp: Cmp, a: E, b: E) -> E {
    Rc::new(Expr::Test(cmp, a, b))
}

/// `if cond then a else b`, collapsing the case where the two agree.
///
/// They agree more often than one might expect: a branch that only assigns to
/// some places leaves the rest identical on both paths, and emitting a select
/// over two copies of the same value would bury the real conditional in noise.
pub fn select(cond: E, a: E, b: E) -> E {
    if a == b {
        return a;
    }
    Rc::new(Expr::Select(cond, a, b))
}

/// Renders `e` as arithmetic, parenthesised only where precedence needs it.
pub fn render(e: &Expr) -> String {
    let mut out = String::new();
    write_expr(&mut out, e, 0);
    out
}

fn write_expr(out: &mut String, e: &Expr, parent: u8) {
    match e {
        Expr::Num(v) => {
            if v.fract() == 0.0 && v.abs() < 1e15 {
                let _ = write!(out, "{}", *v as i64);
            } else {
                let _ = write!(out, "{v}");
            }
        }
        Expr::Load(place) => {
            let _ = write!(out, "{place}");
        }
        Expr::Neg(inner) => {
            let _ = write!(out, "-");
            write_expr(out, inner, 3);
        }
        Expr::Abs(inner) => {
            let _ = write!(out, "abs(");
            write_expr(out, inner, 0);
            let _ = write!(out, ")");
        }
        Expr::Sqrt(inner) => {
            let _ = write!(out, "sqrt(");
            write_expr(out, inner, 0);
            let _ = write!(out, ")");
        }
        Expr::Fun(name, args) => {
            let _ = write!(out, "{name}(");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    let _ = write!(out, ", ");
                }
                write_expr(out, arg, 0);
            }
            let _ = write!(out, ")");
        }
        Expr::Test(cmp, a, b) => {
            write_expr(out, a, 1);
            let _ = write!(out, " {} ", cmp.symbol());
            write_expr(out, b, 1);
        }
        Expr::Select(cond, a, b) => {
            let _ = write!(out, "if ");
            write_expr(out, cond, 0);
            let _ = write!(out, " then ");
            write_expr(out, a, 0);
            let _ = write!(out, " else ");
            write_expr(out, b, 0);
        }
        Expr::Bin(op, a, b) => {
            let precedence = op.precedence();
            let wrap = precedence < parent;
            if wrap {
                let _ = write!(out, "(");
            }
            write_expr(out, a, precedence);
            let _ = write!(out, " {} ", op.symbol());
            // The right operand of a subtraction or division binds tighter:
            // `a - (b - c)` must keep its brackets.
            write_expr(out, b, precedence + u8::from(matches!(op, Op::Sub | Op::Div)));
            if wrap {
                let _ = write!(out, ")");
            }
        }
    }
}

