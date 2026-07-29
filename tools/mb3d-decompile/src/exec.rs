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

/// Runs `code` symbolically, following both sides of any branch.
pub fn run(code: &[u8]) -> Decompiled {
    run_with_constants(code, &[])
}

/// The same, with the `[CONSTANTS]` block resolved to numbers.
///
/// They sit at `PVar + 0` upwards, in declaration order, so a formula reading
/// `k2` is reading the third of them. Substituting turns `k2` into
/// `0.7071067811865475` — and that in turn lets the constant folding see that
/// two of them multiply to something simpler.
pub fn run_with_constants(code: &[u8], constants: &[f64]) -> Decompiled {
    let program = Program::decode(code);
    let mut machine = Machine {
        constants: constants.to_vec(),
        ..Machine::default()
    };
    let result = machine.run_range(&program, 0, program.instructions.len());
    if let Err(reason) = result {
        machine.bailed = Some(reason);
    }
    Decompiled {
        stores: machine.stores,
        bailed: machine.bailed,
    }
}

/// The decoded instructions, with the index of each one by address.
struct Program {
    instructions: Vec<Instruction>,
    /// Instruction address to its position in the list. Jump targets are
    /// addresses; everything else here works in positions.
    index_of: HashMap<u64, usize>,
}

impl Program {
    fn decode(code: &[u8]) -> Self {
        let mut decoder = Decoder::with_ip(32, code, 0, DecoderOptions::NONE);
        let mut instructions = Vec::new();
        let mut index_of = HashMap::new();
        let mut ins = Instruction::default();
        while decoder.can_decode() {
            decoder.decode_out(&mut ins);
            index_of.insert(ins.ip(), instructions.len());
            instructions.push(ins);
        }
        Self {
            instructions,
            index_of,
        }
    }

    fn position(&self, address: u64) -> Option<usize> {
        self.index_of.get(&address).copied()
    }

    /// Where the subroutine starting at `from` ends — just past its `ret`.
    fn subroutine_end(&self, from: usize) -> usize {
        for (at, ins) in self.instructions.iter().enumerate().skip(from) {
            if ins.mnemonic() == Mnemonic::Ret {
                return at + 1;
            }
        }
        self.instructions.len()
    }
}

/// The comparison a conditional jump tests, given the operands it was set up
/// with. x87 arranges its flags so the unsigned integer conditions read as the
/// float ones, which is why `jb` is "less than" rather than "below".
fn jump_condition(mnemonic: Mnemonic) -> Option<expr::Cmp> {
    use expr::Cmp;
    Some(match mnemonic {
        Mnemonic::Jb => Cmp::Lt,
        Mnemonic::Jbe => Cmp::Le,
        Mnemonic::Ja => Cmp::Gt,
        Mnemonic::Jae => Cmp::Ge,
        Mnemonic::Je => Cmp::Eq,
        Mnemonic::Jne => Cmp::Ne,
        Mnemonic::Jl => Cmp::Lt,
        Mnemonic::Jle => Cmp::Le,
        Mnemonic::Jg => Cmp::Gt,
        Mnemonic::Jge => Cmp::Ge,
        _ => return None,
    })
}

#[derive(Default)]
struct Machine {
    /// The x87 stack. The last element is `st0`.
    fpu: Vec<E>,
    /// What each general register points at.
    regs: HashMap<Register, Ptr>,
    stores: Vec<(Place, E)>,
    /// The current value of every place stored so far. Loads deliberately do
    /// *not* consult this — keeping the compiler's temporaries as names is
    /// what makes the output readable. It exists only to answer "what does
    /// this place hold?" when two paths rejoin and have to be reconciled.
    env: HashMap<Place, E>,
    /// The `[CONSTANTS]` block, indexed as the constant pool is.
    constants: Vec<f64>,
    /// The SSE2 registers, each a pair of doubles: low lane then high. MB3D
    /// compiles some formulas this way instead of to x87, working on (x, y)
    /// and (z, w) as packed pairs.
    xmm: HashMap<Register, [E; 2]>,
    /// The operands of the last comparison, waiting for the jump that reads
    /// its flags. x87 compares and branches are several instructions apart,
    /// with `fnstsw`/`sahf` shuttling the flags between them.
    pending_cmp: Option<(E, E)>,
    bailed: Option<String>,
}

impl Machine {
    /// Runs instructions `[from, to)`, splitting at any conditional jump and
    /// reconciling the two sides where they meet again.
    ///
    /// The corpus is forward-only — no formula loops inside itself, the
    /// iteration loop is outside — so a branch always rejoins at an address
    /// ahead of it, and the join is simply the earlier of the two paths'
    /// destinations. That makes the whole thing a recursive walk over nested
    /// intervals rather than a general control-flow reconstruction.
    fn run_range(&mut self, program: &Program, from: usize, to: usize) -> Result<(), String> {
        let mut at = from;
        while at < to {
            let ins = program.instructions[at];

            if ins.is_jmp_short_or_near() {
                // An unconditional jump forward is the tail of a `then` arm
                // skipping over the `else`; the caller has already accounted
                // for where it lands.
                let target = program
                    .position(ins.near_branch_target())
                    .ok_or("jump outside the blob")?;
                if target <= at {
                    return Err(format!("backward jump at {:04x}", ins.ip()));
                }
                at = target;
                continue;
            }

            if ins.is_jcc_short_or_near() {
                at = self.branch(program, at, to)?;
                continue;
            }

            // A formula that folds the same way several times factors the fold
            // out and calls it. The helpers live between the prologue and the
            // body, which jumps over them on the way in.
            if ins.mnemonic() == Mnemonic::Call {
                self.call(program, &ins)?;
                at += 1;
                continue;
            }

            // Returning ends this range. Without this a helper would run on
            // into whatever was laid out after it.
            if ins.mnemonic() == Mnemonic::Ret {
                return Ok(());
            }

            self.step(&ins)
                .map_err(|e| format!("{e} [{:?} at {:04x}]", ins.mnemonic(), ins.ip()))?;
            at += 1;
        }
        Ok(())
    }

    /// Runs an internal subroutine in place.
    ///
    /// The x87 stack is how these helpers take their argument and leave their
    /// result, so the callee runs against this machine's own state rather than
    /// a fresh one — inlining it is the whole of the "calling convention".
    fn call(&mut self, program: &Program, ins: &Instruction) -> Result<(), String> {
        if !ins.is_call_near() {
            // Through a pointer in the record — `PMapFunc` and its like, which
            // are MB3D's own routines and not in this blob at all.
            return Err(format!("indirect call at {:04x}", ins.ip()));
        }
        let target = program
            .position(ins.near_branch_target())
            .ok_or_else(|| format!("call outside the blob at {:04x}", ins.ip()))?;
        let end = program.subroutine_end(target);
        self.run_range(program, target, end)
    }

    /// Handles one conditional jump, returning where execution continues.
    fn branch(
        &mut self,
        program: &Program,
        at: usize,
        limit: usize,
    ) -> Result<usize, String> {
        let ins = program.instructions[at];
        let cmp = jump_condition(ins.mnemonic())
            .ok_or_else(|| format!("{:?} at {:04x}", ins.mnemonic(), ins.ip()))?;
        let (left, right) = self
            .pending_cmp
            .clone()
            .ok_or_else(|| format!("jump with no comparison at {:04x}", ins.ip()))?;

        let taken = program
            .position(ins.near_branch_target())
            .ok_or("branch outside the blob")?;
        let fallthrough = at + 1;
        if taken <= at {
            return Err(format!("backward branch at {:04x}", ins.ip()));
        }

        // The jump's condition describes the *taken* path, and the code
        // immediately after it is the path not taken — so the condition under
        // which the fallthrough runs is the negation. Getting this the wrong
        // way round swaps the arms of every conditional in the corpus and
        // still produces arithmetic that looks entirely reasonable.
        let condition = expr::test(cmp.negate(), left, right);

        // Where the two rejoin. The taken side starts later, so it is the
        // first candidate; if the fallthrough jumps somewhere further on, that
        // is the real join instead.
        let join = self.find_join(program, fallthrough, taken, limit)?;

        let mut then_branch = self.fork();
        then_branch.run_range(program, fallthrough, join.min(taken))?;
        let mut else_branch = self.fork();
        else_branch.run_range(program, taken, join)?;

        self.merge(&condition, then_branch, else_branch)?;
        Ok(join)
    }

    /// Where the two sides of a branch meet again.
    fn find_join(
        &self,
        program: &Program,
        fallthrough: usize,
        taken: usize,
        limit: usize,
    ) -> Result<usize, String> {
        // An `if` with no `else` falls straight into the taken target.
        // An `if/else` ends its first arm with a jump past the second.
        for at in fallthrough..taken.min(limit) {
            let ins = program.instructions[at];
            if ins.is_jmp_short_or_near() {
                let target = program
                    .position(ins.near_branch_target())
                    .ok_or("jump outside the blob")?;
                if target > taken {
                    return Ok(target.min(limit));
                }
            }
        }
        Ok(taken.min(limit))
    }

    /// A copy of this machine's state, with the stores cleared.
    ///
    /// A branch's own stores are not emitted: only the values that survive to
    /// the join are, as one merged assignment each. Emitting both arms' stores
    /// unconditionally would state that things happened which happen only
    /// sometimes.
    fn fork(&self) -> Machine {
        Machine {
            fpu: self.fpu.clone(),
            constants: self.constants.clone(),
            regs: self.regs.clone(),
            xmm: self.xmm.clone(),
            env: self.env.clone(),
            stores: Vec::new(),
            pending_cmp: None,
            bailed: None,
        }
    }

    /// Reconciles two paths, emitting a `select` for everything they disagree
    /// about and nothing for what they do not.
    fn merge(
        &mut self,
        condition: &E,
        then_branch: Machine,
        else_branch: Machine,
    ) -> Result<(), String> {
        if then_branch.fpu.len() != else_branch.fpu.len() {
            return Err("branches left the x87 stack at different depths".to_owned());
        }
        self.fpu = then_branch
            .fpu
            .iter()
            .zip(else_branch.fpu.iter())
            .map(|(a, b)| expr::select(condition.clone(), a.clone(), b.clone()))
            .collect();

        // Every place either side wrote, in the order the `then` arm reached
        // them so the output keeps a sensible reading order.
        let mut places: Vec<Place> = then_branch.stores.iter().map(|(p, _)| p.clone()).collect();
        for (place, _) in &else_branch.stores {
            if !places.contains(place) {
                places.push(place.clone());
            }
        }
        places.dedup();

        for place in places {
            let before = || {
                self.env
                    .get(&place)
                    .cloned()
                    .unwrap_or_else(|| expr::load(place.clone()))
            };
            let taken_value = then_branch.env.get(&place).cloned().unwrap_or_else(before);
            let other_value = else_branch.env.get(&place).cloned().unwrap_or_else(before);
            let merged = expr::select(condition.clone(), taken_value, other_value);
            self.env.insert(place.clone(), merged.clone());
            self.stores.push((place, merged));
        }
        // Registers that differ between the arms are reconciled the same way
        // the stack is.
        for (reg, taken) in &then_branch.xmm {
            let other = else_branch.xmm.get(reg).cloned().unwrap_or(taken.clone());
            self.xmm.insert(
                *reg,
                [
                    expr::select(condition.clone(), taken[0].clone(), other[0].clone()),
                    expr::select(condition.clone(), taken[1].clone(), other[1].clone()),
                ],
            );
        }
        self.regs = then_branch.regs;
        self.pending_cmp = None;
        Ok(())
    }

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
            // The compare itself. Its result lives in the FPU status word
            // until an `fnstsw`/`sahf` pair moves it into the integer flags,
            // so the operands are held here until a jump asks for them.
            Fcom | Fcomp | Fcomi | Fcomip | Fucom | Fucomp | Fucomi | Fucomip => {
                self.compare(ins, false)
            }
            Fcompp | Fucompp => self.compare(ins, true),
            Ftst => {
                let top = self.st(0)?;
                self.pending_cmp = Some((top, expr::num(0.0)));
                Ok(())
            }
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
            Cmp => self.compare_int(ins),
            And | Or | Xor | Test | Inc | Dec | Lea | Shl | Shr | Neg | Not | Movsx | Movzx
            | Cdq | Xchg => Ok(()),
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
            // ---- SSE2 -------------------------------------------------
            Movupd | Movapd | Movdqa | Movdqu => self.sse_move(ins),
            Movsd => self.sse_move_scalar(ins, 0),
            Movlpd => self.sse_move_scalar(ins, 0),
            Movhpd => self.sse_move_scalar(ins, 1),
            Addpd => self.sse_arith(ins, Op::Add, true),
            Subpd => self.sse_arith(ins, Op::Sub, true),
            Mulpd => self.sse_arith(ins, Op::Mul, true),
            Divpd => self.sse_arith(ins, Op::Div, true),
            Addsd => self.sse_arith(ins, Op::Add, false),
            Subsd => self.sse_arith(ins, Op::Sub, false),
            Mulsd => self.sse_arith(ins, Op::Mul, false),
            Divsd => self.sse_arith(ins, Op::Div, false),
            Sqrtpd => self.sse_map(ins, true, expr::sqrt),
            Sqrtsd => self.sse_map(ins, false, expr::sqrt),
            Maxpd => self.sse_call(ins, true, "max"),
            Minpd => self.sse_call(ins, true, "min"),
            Maxsd => self.sse_call(ins, false, "max"),
            Minsd => self.sse_call(ins, false, "min"),
            // The sign-mask idioms. SSE has no float negate or absolute value
            // instruction: everything clears or flips the sign bit with a
            // bitwise operation against a constant, and MB3D keeps those
            // constants in the same pool as the formula's own.
            Andpd | Andps => self.sse_map(ins, true, expr::abs),
            Xorpd | Xorps => self.sse_xor(ins),
            Unpcklpd => self.sse_unpack(ins, false),
            Unpckhpd => self.sse_unpack(ins, true),
            Shufpd | Pshufd => self.sse_shuffle(ins),
            Haddpd => self.sse_hadd(ins),
            Ucomisd | Comisd => self.sse_compare(ins),
            other => Err(format!("{other:?} at {:04x}", ins.ip())),
        }
    }

    // ---- SSE2 ----------------------------------------------------------

    /// The pair a register or memory operand holds.
    fn sse_operand(&mut self, ins: &Instruction, which: u32) -> Result<[E; 2], String> {
        let kind = if which == 0 { ins.op0_kind() } else { ins.op1_kind() };
        match kind {
            OpKind::Register => {
                let reg = if which == 0 {
                    ins.op0_register()
                } else {
                    ins.op1_register()
                };
                Ok(self.xmm.get(&reg).cloned().unwrap_or([
                    expr::load(Place::Unknown(format!("{reg:?}.lo"))),
                    expr::load(Place::Unknown(format!("{reg:?}.hi"))),
                ]))
            }
            OpKind::Memory => {
                let low = self.place(ins)?;
                let high = self.place_at(ins, 8)?;
                Ok([expr::load(low), expr::load(high)])
            }
            other => Err(format!("SSE operand {other:?}")),
        }
    }

    fn sse_write(&mut self, ins: &Instruction, value: [E; 2], packed: bool) -> Result<(), String> {
        match ins.op0_kind() {
            OpKind::Register => {
                let reg = ins.op0_register();
                let mut pair = value;
                if !packed {
                    // A scalar operation leaves the upper lane alone.
                    if let Some(existing) = self.xmm.get(&reg) {
                        pair[1] = existing[1].clone();
                    }
                }
                self.xmm.insert(reg, pair);
                Ok(())
            }
            OpKind::Memory => {
                let low = self.place(ins)?;
                self.env.insert(low.clone(), value[0].clone());
                self.stores.push((low, value[0].clone()));
                if packed {
                    let high = self.place_at(ins, 8)?;
                    self.env.insert(high.clone(), value[1].clone());
                    self.stores.push((high, value[1].clone()));
                }
                Ok(())
            }
            other => Err(format!("SSE destination {other:?}")),
        }
    }

    fn sse_move(&mut self, ins: &Instruction) -> Result<(), String> {
        let value = self.sse_operand(ins, 1)?;
        self.sse_write(ins, value, true)
    }

    /// `movsd`, `movlpd` and `movhpd` move one lane.
    fn sse_move_scalar(&mut self, ins: &Instruction, lane: usize) -> Result<(), String> {
        if ins.op0_kind() == OpKind::Memory {
            let place = if lane == 0 {
                self.place(ins)?
            } else {
                self.place_at(ins, 8)?
            };
            let value = self.sse_operand(ins, 1)?[lane].clone();
            self.env.insert(place.clone(), value.clone());
            self.stores.push((place, value));
            return Ok(());
        }
        let source = self.sse_operand(ins, 1)?;
        let reg = ins.op0_register();
        let mut pair = self.xmm.get(&reg).cloned().unwrap_or([
            expr::num(0.0),
            expr::num(0.0),
        ]);
        // Loading a scalar from memory clears the upper lane; from another
        // register it leaves it.
        if ins.op1_kind() == OpKind::Memory && lane == 0 {
            pair[1] = expr::num(0.0);
        }
        pair[lane] = source[0].clone();
        self.xmm.insert(reg, pair);
        Ok(())
    }

    fn sse_arith(&mut self, ins: &Instruction, op: Op, packed: bool) -> Result<(), String> {
        let left = self.sse_operand(ins, 0)?;
        let right = self.sse_operand(ins, 1)?;
        let value = [
            expr::bin(op, left[0].clone(), right[0].clone()),
            if packed {
                expr::bin(op, left[1].clone(), right[1].clone())
            } else {
                left[1].clone()
            },
        ];
        self.sse_write(ins, value, packed)
    }

    fn sse_map(
        &mut self,
        ins: &Instruction,
        packed: bool,
        f: impl Fn(E) -> E,
    ) -> Result<(), String> {
        // `sqrtsd` and friends take their input from the second operand;
        // `andpd` masks the destination in place.
        let source = if ins.op_count() > 1 {
            self.sse_operand(ins, 1)?
        } else {
            self.sse_operand(ins, 0)?
        };
        let base = self.sse_operand(ins, 0)?;
        let value = [
            f(if matches!(ins.mnemonic(), Mnemonic::Andpd | Mnemonic::Andps) {
                base[0].clone()
            } else {
                source[0].clone()
            }),
            if packed {
                f(if matches!(ins.mnemonic(), Mnemonic::Andpd | Mnemonic::Andps) {
                    base[1].clone()
                } else {
                    source[1].clone()
                })
            } else {
                base[1].clone()
            },
        ];
        self.sse_write(ins, value, packed)
    }

    fn sse_call(&mut self, ins: &Instruction, packed: bool, name: &'static str) -> Result<(), String> {
        let left = self.sse_operand(ins, 0)?;
        let right = self.sse_operand(ins, 1)?;
        let value = [
            expr::call(name, vec![left[0].clone(), right[0].clone()]),
            if packed {
                expr::call(name, vec![left[1].clone(), right[1].clone()])
            } else {
                left[1].clone()
            },
        ];
        self.sse_write(ins, value, packed)
    }

    /// `xorpd` against itself is zero; against a constant it flips the sign.
    fn sse_xor(&mut self, ins: &Instruction) -> Result<(), String> {
        if ins.op0_kind() == OpKind::Register
            && ins.op1_kind() == OpKind::Register
            && ins.op0_register() == ins.op1_register()
        {
            self.xmm
                .insert(ins.op0_register(), [expr::num(0.0), expr::num(0.0)]);
            return Ok(());
        }
        self.sse_map(ins, true, expr::neg)
    }

    fn sse_unpack(&mut self, ins: &Instruction, high: bool) -> Result<(), String> {
        let left = self.sse_operand(ins, 0)?;
        let right = self.sse_operand(ins, 1)?;
        let lane = usize::from(high);
        self.sse_write(ins, [left[lane].clone(), right[lane].clone()], true)
    }

    /// Lane selection.
    ///
    /// `shufpd` takes its low lane from the destination and its high lane from
    /// the source, one bit of the immediate choosing which half of each — so
    /// with both operands the same register, an immediate of 1 is the swap
    /// that sets up a horizontal sum. `pshufd` shuffles four 32-bit lanes, of
    /// which only the three arrangements that keep doubles intact appear here.
    fn sse_shuffle(&mut self, ins: &Instruction) -> Result<(), String> {
        let selector = ins.immediate8();
        let source = self.sse_operand(ins, 1)?;

        if ins.mnemonic() == Mnemonic::Pshufd {
            let pair = match selector {
                0x4E => [source[1].clone(), source[0].clone()],
                0x44 => [source[0].clone(), source[0].clone()],
                0xEE => [source[1].clone(), source[1].clone()],
                other => return Err(format!("pshufd {other:#x} at {:04x}", ins.ip())),
            };
            return self.sse_write(ins, pair, true);
        }

        let destination = self.sse_operand(ins, 0)?;
        let low = destination[usize::from(selector & 1 != 0)].clone();
        let high = source[usize::from(selector & 2 != 0)].clone();
        self.sse_write(ins, [low, high], true)
    }

    fn sse_hadd(&mut self, ins: &Instruction) -> Result<(), String> {
        let left = self.sse_operand(ins, 0)?;
        let right = self.sse_operand(ins, 1)?;
        self.sse_write(
            ins,
            [
                expr::bin(Op::Add, left[0].clone(), left[1].clone()),
                expr::bin(Op::Add, right[0].clone(), right[1].clone()),
            ],
            true,
        )
    }

    fn sse_compare(&mut self, ins: &Instruction) -> Result<(), String> {
        let left = self.sse_operand(ins, 0)?;
        let right = self.sse_operand(ins, 1)?;
        self.pending_cmp = Some((left[0].clone(), right[0].clone()));
        Ok(())
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

    /// Records the operands of an x87 comparison for the jump that follows.
    fn compare(&mut self, ins: &Instruction, pop_two: bool) -> Result<(), String> {
        let left = self.st(0)?;
        let right = match ins.op0_kind() {
            OpKind::Memory => expr::load(self.place(ins)?),
            // A compare naming `st0` is the implicit form: `fcom` with no
            // operand decodes that way, and comparing the top of the stack
            // with itself would say nothing. The real operand is `st1`.
            OpKind::Register if is_st(ins.op0_register()) => {
                let i = st_index(ins.op0_register())?;
                self.st(if i == 0 { 1 } else { i })?
            }
            // `fcompp` and friends carry no explicit operand either.
            _ => self.st(1)?,
        };
        self.pending_cmp = Some((left, right));
        if matches!(ins.mnemonic(), Mnemonic::Fcomp | Mnemonic::Fucomp)
            || ins.mnemonic() == Mnemonic::Fcomip
            || ins.mnemonic() == Mnemonic::Fucomip
        {
            self.pop()?;
        }
        if pop_two {
            self.pop()?;
            self.pop()?;
        }
        Ok(())
    }

    /// The same for an integer compare, which is how the first-iteration
    /// guard is written: `cmp dword [rec+bFirstIt], 0`.
    fn compare_int(&mut self, ins: &Instruction) -> Result<(), String> {
        let left = match ins.op0_kind() {
            OpKind::Memory => expr::load(self.place(ins)?),
            OpKind::Register => expr::load(Place::Unknown(format!("{:?}", ins.op0_register()))),
            _ => return Ok(()),
        };
        let right = match ins.op1_kind() {
            OpKind::Immediate8 | OpKind::Immediate8to32 | OpKind::Immediate32 => {
                expr::num(ins.immediate32() as i32 as f64)
            }
            OpKind::Memory => expr::load(self.place(ins)?),
            _ => return Ok(()),
        };
        self.pending_cmp = Some((left, right));
        Ok(())
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
                self.push(self.value_of(place))
            }
            other => Err(format!("fld {other:?}")),
        }
    }

    fn store(&mut self, ins: &Instruction, pop: bool) -> Result<(), String> {
        let value = self.st(0)?;
        match ins.op0_kind() {
            OpKind::Memory => {
                let place = self.place(ins)?;
                self.env.insert(place.clone(), value.clone());
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
        // `fxch` with no operand means `fxch st1`, and decodes as naming st0.
        // Exchanging the top of the stack with itself is a no-op, so that
        // form can only be the implicit one — and reading it literally
        // silently drops the swap, which is how Delphi's `Ln` gets its
        // operands the right way round for `fyl2x`.
        let i = match ins.op_count() {
            0 => 1,
            _ if !is_st(ins.op0_register()) => 1,
            _ => match st_index(ins.op0_register())? {
                0 => 1,
                other => other,
            },
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
            let operand = self.value_of(place);
            let top = self.st(0)?;
            let (a, b) = if reversed { (operand, top) } else { (top, operand) };
            return self.set_st(0, expr::bin(op, a, b));
        }

        // Form 2 and 3: between stack slots. With no operands the target is
        // st1, which is what the `p` forms use.
        let target = match ins.op_count() {
            0 => 1,
            // Some encodings report operands that are not x87 registers; the
            // implicit `st1, st0` form is what they mean.
            _ if !is_st(ins.op0_register()) => 1,
            _ => {
                let first = st_index(ins.op0_register())?;
                // `fadd st0, st(i)` targets st0; `faddp st(i), st0` targets
                // st(i). Which is which is decided by the first operand.
                first
            }
        };
        let other = match ins.op_count() {
            0 | 1 => 0,
            _ if !is_st(ins.op1_register()) => 0,
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

    /// A load of `place`, resolved to a number where it names a constant the
    /// file supplies.
    fn value_of(&self, place: Place) -> E {
        if let Place::Const(index) = place
            && let Some(value) = self.constants.get(index)
        {
            return expr::num(*value);
        }
        expr::load(place)
    }

    /// The place `extra` bytes past this operand's address — the upper lane of
    /// a packed access.
    fn place_at(&self, ins: &Instruction, extra: i64) -> Result<Place, String> {
        let base = ins.memory_base();
        let displacement = ins.memory_displacement64() as i32 as i64 + extra;
        if (base == Register::EBP && displacement < 0) || base == Register::ESP {
            return Ok(Place::Local(displacement));
        }
        let Some((ptr, base_offset)) = self.pointer(ins) else {
            return Ok(Place::Unknown(format!("{:04x}+{extra}", ins.ip())));
        };
        let _ = base_offset;
        Ok(match ptr {
            Ptr::Var('x') if displacement == 8 => Place::Var('y'),
            Ptr::Var('z') if displacement == 8 => Place::Var('w'),
            Ptr::Var(c) => Place::Unknown(format!("{c}+{displacement}")),
            Ptr::PVar => abi::parameter_place(displacement),
            Ptr::Record(rebase) => match abi::field(rebase + displacement) {
                Some(name) => Place::Field(name),
                None => Place::Unknown(format!("rec{:+}", rebase + displacement)),
            },
            Ptr::Unknown => Place::Unknown(format!("{:04x}", ins.ip())),
        })
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
            // `TIteration3Dext` stores x, y, z and w as consecutive doubles,
            // so the pointer to x also reaches y and the pointer to z reaches
            // w. That adjacency is the whole reason the SSE2 formulas can work
            // on packed pairs.
            Ptr::Var(c) if displacement == 8 => match c {
                'x' => Place::Var('y'),
                'z' => Place::Var('w'),
                other => Place::Unknown(format!("{other}+8")),
            },
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

/// Whether a register is one of the x87 stack slots.
fn is_st(reg: Register) -> bool {
    matches!(
        reg,
        Register::ST0
            | Register::ST1
            | Register::ST2
            | Register::ST3
            | Register::ST4
            | Register::ST5
            | Register::ST6
            | Register::ST7
    )
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
