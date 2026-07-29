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

pub fn call(name: &'static str, args: Vec<E>) -> E {
    Rc::new(Expr::Fun(name, args))
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

