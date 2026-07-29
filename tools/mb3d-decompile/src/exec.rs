//! Symbolic execution of a formula's x87 code.
//!
//! The FPU is a stack machine, which is the whole reason this is practical:
//! every instruction's operands are wherever the last few instructions left
//! them, so running the stack with expressions instead of numbers reconstructs
//! the expression tree directly. There is no register allocation to undo, and
//! no aliasing to reason about — the two things that make decompiling ordinary
//! integer code hard.
//!
//! Straight-line code only. Anything with a branch is refused by name rather
//! than being silently linearised into a formula that is not what MB3D
//! computes.

use std::collections::HashMap;

use iced_x86::{Code, Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};

use crate::abi;
use crate::expr::{self, E, Expr, Op, Place};

/// What a general-purpose register is pointing at.
#[derive(Clone, PartialEq, Debug)]
enum Ptr {
    Var(char),
    /// The iteration record, displaced by however much has been added to it.
    Record(i64),
    /// The parameter block.
    PVar,
    Unknown,
}

pub struct Decompiled {
    /// Assignments in the order the formula makes them.
    pub stores: Vec<(Place, E)>,
    /// Why the decode stopped short, if it did.
    pub bailed: Option<String>,
}

/// Runs `code` symbolically.
pub fn run(code: &[u8]) -> Decompiled {
    let mut machine = Machine::default();
    let mut decoder = Decoder::with_ip(32, code, 0, DecoderOptions::NONE);
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        if let Err(reason) = machine.step(&instruction) {
            machine.bailed = Some(reason);
            break;
        }
    }
    Decompiled {
        stores: machine.stores,
        bailed: machine.bailed,
    }
}

#[derive(Default)]
struct Machine {
    /// The x87 stack. The last element is `st0`.
    fpu: Vec<E>,
    /// What each general register points at.
    regs: HashMap<Register, Ptr>,
    stores: Vec<(Place, E)>,
    bailed: Option<String>,
}

impl Machine {
    fn step(&mut self, ins: &Instruction) -> Result<(), String> {
        use Mnemonic::*;

        // Delphi's `register` convention hands the first three var parameters
        // in eax, edx and ecx. Seeded lazily so a formula that never touches
        // one does not carry a phantom.
        self.regs.entry(Register::EAX).or_insert(Ptr::Var('x'));
        self.regs.entry(Register::EDX).or_insert(Ptr::Var('y'));
        self.regs.entry(Register::ECX).or_insert(Ptr::Var('z'));

        match ins.mnemonic() {
            // Prologue, epilogue and stack housekeeping carry no arithmetic.
            Push | Pop | Nop | Ret | Leave | Enter | Int3 => Ok(()),
            // The comparison idiom: `fcom` sets flags, `fnstsw`/`sahf` move
            // them, and a `jcc` acts on them. Refused as a unit below, so
            // these are only reached when nothing branches on them.
            Fnstsw | Sahf | Wait | Fclex | Fninit | Fldcw | Fnstcw => Ok(()),
            Mov => self.mov(ins),
            Add => self.add_reg(ins),
            Fld => self.fld(ins),
            Fld1 => self.push(expr::num(1.0)),
            Fldz => self.push(expr::num(0.0)),
            Fldpi => self.push(expr::num(std::f64::consts::PI)),
            Fldl2e => self.push(expr::num(std::f64::consts::LOG2_E)),
            Fldl2t => self.push(expr::num(std::f64::consts::LOG2_10)),
            Fldlg2 => self.push(expr::num(std::f64::consts::LOG10_2)),
            Fldln2 => self.push(expr::num(std::f64::consts::LN_2)),
            Fst | Fstp => self.store(ins, ins.mnemonic() == Fstp),
            Fxch => self.fxch(ins),
            Fabs => self.map(expr::abs),
            Fchs => self.map(expr::neg),
            Fsqrt => self.map(expr::sqrt),
            Fadd | Faddp => self.arith(ins, Op::Add, false),
            Fmul | Fmulp => self.arith(ins, Op::Mul, false),
            Fsub | Fsubp => self.arith(ins, Op::Sub, false),
            Fsubr | Fsubrp => self.arith(ins, Op::Sub, true),
            Fdiv | Fdivp => self.arith(ins, Op::Div, false),
            Fdivr | Fdivrp => self.arith(ins, Op::Div, true),
            Ffree | Ffreep => self.free(ins),
            Fincstp => {
                self.pop().ok();
                Ok(())
            }
            // Integer housekeeping. Only reachable in straight-line code,
            // where it is stack alignment and counters rather than anything
            // the formula's value depends on — except when it moves a pointer
            // this model is tracking, which `sub` is checked for.
            Sub => self.sub_reg(ins),
            And | Or | Xor | Test | Cmp | Inc | Dec | Lea | Shl | Shr | Neg | Not | Movsx
            | Movzx | Cdq | Xchg => Ok(()),
            Fnop => Ok(()),
            // Transcendentals. Named rather than expanded: what matters is
            // that the operation survives into the output intact.
            Fsin => self.map(|e| expr::call("sin", vec![e])),
            Fcos => self.map(|e| expr::call("cos", vec![e])),
            Frndint => self.map(|e| expr::call("round", vec![e])),
            F2xm1 => self.map(|e| expr::call("exp2m1", vec![e])),
            // st1 = atan2(st1, st0), then pop.
            Fpatan => self.combine(|y, x| expr::call("atan2", vec![y, x])),
            // st1 = st1 * log2(st0), then pop.
            Fyl2x => self.combine(|y, x| {
                expr::bin(Op::Mul, y, expr::call("log2", vec![x]))
            }),
            Fyl2xp1 => self.combine(|y, x| {
                expr::bin(
                    Op::Mul,
                    y,
                    expr::call("log2", vec![expr::bin(Op::Add, x, expr::num(1.0))]),
                )
            }),
            Fscale => self.combine_keep(|x, n| {
                expr::bin(Op::Mul, x, expr::call("exp2", vec![n]))
            }),
            Fprem | Fprem1 => self.combine_keep(|a, b| expr::call("fmod", vec![a, b])),
            // Sine into st0 and cosine pushed above it.
            Fsincos => {
                let top = self.st(0)?;
                self.set_st(0, expr::call("sin", vec![top.clone()]))?;
                self.push(expr::call("cos", vec![top]))
            }
            Fptan => {
                let top = self.st(0)?;
                self.set_st(0, expr::call("tan", vec![top]))?;
                self.push(expr::num(1.0))
            }
            other => Err(format!("{other:?} at {:04x}", ins.ip())),
        }
    }

    // ---- the FPU stack -------------------------------------------------

    fn push(&mut self, value: E) -> Result<(), String> {
        if self.fpu.len() >= 8 {
            return Err("x87 stack overflow".to_owned());
        }
        self.fpu.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<E, String> {
        self.fpu.pop().ok_or_else(|| "x87 stack underflow".to_owned())
    }

    /// `st(i)` counted from the top, as the hardware numbers it.
    fn st(&self, i: usize) -> Result<E, String> {
        let len = self.fpu.len();
        if i >= len {
            return Err(format!("read of st{i} with {len} on the stack"));
        }
        Ok(self.fpu[len - 1 - i].clone())
    }

    fn set_st(&mut self, i: usize, value: E) -> Result<(), String> {
        let len = self.fpu.len();
        if i >= len {
            return Err(format!("write to st{i} with {len} on the stack"));
        }
        self.fpu[len - 1 - i] = value;
        Ok(())
    }

    fn map(&mut self, f: impl Fn(E) -> E) -> Result<(), String> {
        let top = self.st(0)?;
        self.set_st(0, f(top))
    }

    /// `st1 = f(st1, st0)`, then pop — the shape every two-operand
    /// transcendental takes.
    fn combine(&mut self, f: impl Fn(E, E) -> E) -> Result<(), String> {
        let (top, next) = (self.st(0)?, self.st(1)?);
        self.set_st(1, f(next, top))?;
        self.pop()?;
        Ok(())
    }

    /// The same, but leaving the stack depth alone: `fscale` and `fprem`
    /// consume nothing.
    fn combine_keep(&mut self, f: impl Fn(E, E) -> E) -> Result<(), String> {
        let (top, next) = (self.st(0)?, self.st(1)?);
        self.set_st(0, f(top, next))
    }

    /// `sub reg,imm` on a tracked pointer moves it, as `add` does.
    fn sub_reg(&mut self, ins: &Instruction) -> Result<(), String> {
        if ins.op0_kind() == OpKind::Register && ins.op1_kind() == OpKind::Immediate32 {
            let reg = ins.op0_register();
            if let Some(Ptr::Record(base)) = self.regs.get(&reg).cloned() {
                self.regs
                    .insert(reg, Ptr::Record(base - ins.immediate32() as i64));
            }
        }
        Ok(())
    }

    // ---- instructions --------------------------------------------------

    fn mov(&mut self, ins: &Instruction) -> Result<(), String> {
        if ins.op0_kind() != OpKind::Register {
            // A store to memory of something that is not a float: the
            // iteration counters MB3D keeps. No arithmetic in it.
            return Ok(());
        }
        let dst = ins.op0_register();
        let value = match ins.op1_kind() {
            OpKind::Register => self
                .regs
                .get(&ins.op1_register())
                .cloned()
                .unwrap_or(Ptr::Unknown),
            OpKind::Memory => {
                // `mov esi,[edi+30h]` — loading `PVar` out of the record is
                // the one pointer load that matters.
                match self.pointer(ins) {
                    Some((Ptr::Record(base), displacement))
                        if base + displacement == abi::PVAR_OFFSET =>
                    {
                        Ptr::PVar
                    }
                    // `[ebp+8]` is the fourth argument: the record itself.
                    None if ins.memory_base() == Register::EBP => Ptr::Record(0),
                    _ => Ptr::Unknown,
                }
            }
            _ => Ptr::Unknown,
        };
        self.regs.insert(dst, value);
        Ok(())
    }

    /// `add edi,80h` — the rebase every compiled formula applies so the far
    /// end of the record is reachable with a one-byte displacement.
    fn add_reg(&mut self, ins: &Instruction) -> Result<(), String> {
        if ins.op0_kind() == OpKind::Register && ins.op1_kind() == OpKind::Immediate32 {
            let reg = ins.op0_register();
            if let Some(Ptr::Record(base)) = self.regs.get(&reg).cloned() {
                self.regs
                    .insert(reg, Ptr::Record(base + ins.immediate32() as i64));
            }
        }
        Ok(())
    }

    fn fld(&mut self, ins: &Instruction) -> Result<(), String> {
        match ins.op0_kind() {
            OpKind::Register => {
                // `fld st(i)` duplicates, and the index is relative to the
                // stack *before* the push.
                let i = st_index(ins.op0_register())?;
                let value = self.st(i)?;
                self.push(value)
            }
            OpKind::Memory => {
                let place = self.place(ins)?;
                self.push(expr::load(place))
            }
            other => Err(format!("fld {other:?}")),
        }
    }

    fn store(&mut self, ins: &Instruction, pop: bool) -> Result<(), String> {
        let value = self.st(0)?;
        match ins.op0_kind() {
            OpKind::Memory => {
                let place = self.place(ins)?;
                self.stores.push((place, value));
            }
            OpKind::Register => {
                let i = st_index(ins.op0_register())?;
                // `fstp st(i)` writes before the pop, so the index still
                // counts from the current top.
                self.set_st(i, value)?;
            }
            other => return Err(format!("fstp {other:?}")),
        }
        if pop {
            self.pop()?;
        }
        Ok(())
    }

    fn fxch(&mut self, ins: &Instruction) -> Result<(), String> {
        let i = if ins.op_count() == 0 {
            1
        } else {
            st_index(ins.op0_register())?
        };
        let (a, b) = (self.st(0)?, self.st(i)?);
        self.set_st(0, b)?;
        self.set_st(i, a)
    }

    fn free(&mut self, ins: &Instruction) -> Result<(), String> {
        // `ffree` marks a slot empty without moving the top; treating it as a
        // pop is only right when it is the top being freed, which is how
        // Delphi emits it.
        if ins.op_count() > 0 && st_index(ins.op0_register())? == 0 {
            self.pop()?;
        }
        Ok(())
    }

    /// Every arithmetic form: with memory, with a register, and popping.
    ///
    /// The three-way split of x87's encodings is the fiddly part. `fsub st0,
    /// st(i)` and `fsubp st(i), st0` compute different things in different
    /// places, and `fsubr` swaps the operands of both.
    fn arith(&mut self, ins: &Instruction, op: Op, reversed: bool) -> Result<(), String> {
        let pops = matches!(
            ins.mnemonic(),
            Mnemonic::Faddp
                | Mnemonic::Fsubp
                | Mnemonic::Fsubrp
                | Mnemonic::Fmulp
                | Mnemonic::Fdivp
                | Mnemonic::Fdivrp
        );

        // Form 1: against memory. Always `st0 = st0 op mem`.
        if ins.op_count() > 0 && ins.op0_kind() == OpKind::Memory {
            let place = self.place(ins)?;
            let operand = expr::load(place);
            let top = self.st(0)?;
            let (a, b) = if reversed { (operand, top) } else { (top, operand) };
            return self.set_st(0, expr::bin(op, a, b));
        }

        // Form 2 and 3: between stack slots. With no operands the target is
        // st1, which is what the `p` forms use.
        let target = match ins.op_count() {
            0 => 1,
            _ => {
                let first = st_index(ins.op0_register())?;
                // `fadd st0, st(i)` targets st0; `faddp st(i), st0` targets
                // st(i). Which is which is decided by the first operand.
                first
            }
        };
        let other = match ins.op_count() {
            0 | 1 => 0,
            _ => st_index(ins.op1_register())?,
        };

        let (dst, src) = if target == 0 && !pops {
            (0usize, other)
        } else {
            (target, 0usize)
        };
        let (x, y) = (self.st(dst)?, self.st(src)?);
        // The destination is the left operand unless the encoding reverses it.
        let (a, b) = if reversed { (y, x) } else { (x, y) };
        let value = expr::bin(op, a, b);

        if pops {
            // The pop removes st0, so the destination's index shifts down by
            // one once it is gone.
            self.set_st(dst, value)?;
            self.pop()?;
        } else {
            self.set_st(dst, value)?;
        }
        Ok(())
    }

    // ---- naming memory -------------------------------------------------

    /// The base pointer and displacement of a memory operand, when the base
    /// register is one this model is tracking.
    fn pointer(&self, ins: &Instruction) -> Option<(Ptr, i64)> {
        let base = ins.memory_base();
        if base == Register::None {
            return None;
        }
        let ptr = self.regs.get(&base)?.clone();
        if ptr == Ptr::Unknown {
            return None;
        }
        Some((ptr, ins.memory_displacement64() as i32 as i64))
    }

    fn place(&self, ins: &Instruction) -> Result<Place, String> {
        // An indexed access is a table lookup, not a named field, and this
        // model has nothing useful to say about it.
        if ins.memory_index() != Register::None {
            return Err(format!("indexed memory at {:04x}", ins.ip()));
        }
        // The formula's own frame. `[ebp+8]` and above are the arguments, so
        // only what sits below the frame pointer — and anything esp-relative —
        // is a spill slot.
        let base = ins.memory_base();
        let displacement = ins.memory_displacement64() as i32 as i64;
        if (base == Register::EBP && displacement < 0) || base == Register::ESP {
            return Ok(Place::Local(displacement));
        }

        let Some((ptr, displacement)) = self.pointer(ins) else {
            return Ok(Place::Unknown(format!("{:04x}", ins.ip())));
        };
        Ok(match ptr {
            Ptr::Var(c) if displacement == 0 => Place::Var(c),
            Ptr::Var(c) => Place::Unknown(format!("{c}+{displacement}")),
            Ptr::PVar => abi::parameter_place(displacement),
            Ptr::Record(base) => match abi::field(base + displacement) {
                Some(name) => Place::Field(name),
                None => Place::Unknown(format!("rec{:+}", base + displacement)),
            },
            Ptr::Unknown => Place::Unknown(format!("{:04x}", ins.ip())),
        })
    }
}

fn st_index(reg: Register) -> Result<usize, String> {
    match reg {
        Register::ST0 => Ok(0),
        Register::ST1 => Ok(1),
        Register::ST2 => Ok(2),
        Register::ST3 => Ok(3),
        Register::ST4 => Ok(4),
        Register::ST5 => Ok(5),
        Register::ST6 => Ok(6),
        Register::ST7 => Ok(7),
        other => Err(format!("not an x87 register: {other:?}")),
    }
}

/// Whether a blob has any control flow, which this executor refuses.
pub fn has_branches(code: &[u8]) -> bool {
    let mut decoder = Decoder::with_ip(32, code, 0, DecoderOptions::NONE);
    let mut ins = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut ins);
        if ins.is_jcc_short_or_near()
            || ins.is_jmp_short_or_near()
            || ins.code() == Code::INVALID
            || ins.mnemonic() == Mnemonic::Call
        {
            return true;
        }
    }
    false
}

/// The assignments that describe the formula, in the order it makes them.
///
/// Printed as a sequence rather than folded into one expression per output.
/// Substituting each stored value into its later uses is correct but useless:
/// MB3D's formulas reuse their temporaries heavily, and inlining turns a
/// six-line Benesi fold into a single expression tens of thousands of
/// characters long. The spill slots the compiler chose *are* the temporaries
/// the original Pascal declared, so keeping them recovers its shape too.
///
/// A store of a bare load is a copy the compiler made for its own reasons and
/// carries no arithmetic, so it is dropped.
pub fn final_stores(stores: &[(Place, E)]) -> Vec<(Place, E)> {
    stores
        .iter()
        .filter(|(_, v)| !matches!(&**v, Expr::Load(_)))
        .cloned()
        .collect()
}
