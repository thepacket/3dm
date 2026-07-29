# mb3d-decompile

Recovers readable formulas from Mandelbulb 3D's compiled `.m3f` blobs.

MB3D ships **454 of its 460 formulas as hex-encoded 32-bit x86** inside a
`[CODE]` block. The files are ASCII, which is misleading — the payload is
machine code, and the Pascal it was compiled from is not in the repository.
Only about 40 formulas (the JIT ones, in `EM_JIT_M3Formulas/`) carry real
`[SOURCE]`.

MB3D's source is at `github.com/thargor6/mb3d` under **LGPL-2.1**, which §3
permits relicensing to GPL, so 3DM may use what this recovers.

## What an `.m3f` carries

`[OPTIONS]` — parameter declarations with their defaults and the engine
settings — then `[CONSTANTS]`, `[CODE]` and free prose after `[END]`. Across
the corpus: 457 have options, 454 have compiled code, **195 have constants**,
and 3 carry `[SOURCE]` instead.

The constants matter more than their number suggests. They sit at `PVar + 0`
upwards in declaration order, so a formula reading the third of them is reading
`PVar + 16`, and resolving that turns an opaque `k2` into `0.7071067811865475`.
`BenesiPine1`'s three are sqrt(2/3), sqrt(1/3) and sqrt(1/2) — exactly the
values its description names, in exactly its order. Nothing in the decode
consults the block and nothing in the block knows about the decode, so their
agreement is evidence for both.

## Why this is tractable

Not because decompilation is easy in general, but because of what this code
specifically is:

- **Tiny.** Median 210 bytes, max 2042.
- **x87, which is a stack machine.** An expression tree falls out of
  simulating the stack; there is no register allocation to undo.
- **A documented ABI.** `TIteration3Dext` in MB3D's `TypeDefinitions.pas`
  carries a byte offset in a comment against every field. `src/abi.rs` is that
  table transcribed, so a decode can be *checked* rather than guessed at.
- **Checkable output.** Around 94 of the 460 state their mathematics as
  pseudocode in the prose after `[END]`. That is a ground-truth set: a
  decompiler that reproduces those is trustworthy on the rest.

## What the corpus is

`cargo run -p mb3d-decompile -- /path/to/M3Formulas`

```
formulas with code:   457
instructions decoded: 48027
undecodable:          0
blob bytes:           min 13 median 210 max 2042
with any branch:      282
with a call:          28
120 distinct mnemonics
```

Concentrated: `fld` 8216, `fmul` 6689, `fstp` 4271, `faddp` 2880, then `mov`,
`fadd`, `fxch`, `fabs`, `fsub`, `fchs`. The tail is a minority of formulas
compiled to **SSE2** instead (`divsd`, `sqrtsd`, `unpcklpd`, `haddpd`) — a
second backend, not a long tail of exotica.

`--asm NAME` prints one formula annotated with ABI names and its shipped
description.

## State

`--decompile` runs a formula symbolically and prints its assignments;
`--decompile` with no name reports coverage across the corpus.

```
formulas attempted:      457
fully recovered:         304
ran but assigned nothing:7
bailed:                  146
```

Done: extraction, the ABI table, a corpus survey, annotated disassembly, a
symbolic x87 executor, control flow, and the SSE2 backend.

Branches are followed on both sides and reconciled where they meet, so a
conditional comes out as one. The corpus is forward-only — no formula loops
inside itself, the iteration loop is outside — which makes that a recursive
walk over nested intervals rather than general control-flow reconstruction.

Some formulas are compiled to packed doubles instead of x87, working on
(x, y) and (z, w) as pairs — which only works because `TIteration3Dext` stores
the four consecutively, and is why `[eax+8]` is `y`. SSE has no float negate or
absolute value, so those are bitwise operations against sign masks that MB3D
keeps in the same constant pool as the formula's own parameters.

## Calls, measured

A formula that folds the same way several times factors the fold out and calls
it. The helpers sit between the prologue and the body, which jumps over them on
the way in. Across the corpus:

| | sites |
|---|---|
| direct, target inside the blob | 54 |
| direct, target outside | 12 |
| indirect, through a pointer | 66 |
| backward jumps (all inside helpers) | 8 |

The internal ones are inlined: the x87 stack is how a helper takes its argument
and leaves its result, so it runs against the caller's own state and that is
the whole of the calling convention. The indirect ones go through `PMapFunc`
and its like — MB3D's own routines, not in the blob at all — and cannot be
recovered as arithmetic.

The backward jumps are all one idiom: an integer power by repeated squaring,
`shr eax,1` walking the exponent while `fmul st0` squares. Executing that needs
an exponent known at decompile time, which it is not.

Not done: a handful of integer-load instructions, the branches whose comparison
this model does not capture, and reading `.m3p` parameter files.

## Idioms

Delphi has no `pow`: it open-codes `x^n` as `exp(n * ln x)`, and the FPU has
neither `exp` nor `ln` either, so both are in turn built from `fldln2`,
`fldl2e`, `f2xm1` and `fscale`. Left alone, a decode of that comes back as a
page of logarithms that is arithmetically right and unreadable.

Four exact identities undo it: `2^x - 1` then `+ 1` is `2^x`, `2^a * 2^b` is
`2^(a+b)`, `ln 2 * log2 x` is `ln x`, and `2^(x * log2 e)` is `e^x` — after
which `e^(n * ln x)` is `x^n`. Constants are folded too, so
`log2(0.6931471805599453)` becomes a number rather than sitting mid-expression
as a computation.

The chain does not yet collapse all the way to `pow` on every formula: some
come back as `exp2(ln(...))` nested, which says the stack on entry to an
inlined helper is not always what the helper expects. That is an open bug, not
a finished feature.

## Why the parameter slots matter

Recovering the mathematics is only half of it. The reason to want MB3D's
formulas at all is the body of work its community has already made, shared as
`.m3p` parameter files — and a `.m3p` names an MB3D formula and carries values
in MB3D's own slot order. Map the slots wrongly and the file still renders;
it just renders something nobody made.

So a formula we skip because Mandelbulber already has an equivalent is not a
saving. It breaks every `.m3p` that references it.

The rule is `FormulaCompiler.pas`, which emits the accessor for every declared
value:

```pascal
COffset := 0;
VOffset := 16;
... := PDouble(Integer(PIteration3D^.PVar) + COffset)^;   // constants
... := PDouble(Integer(PIteration3D^.PVar) - VOffset)^;   // parameters
Inc(VOffset, JITValueDatatypeSize(Pair.Datatype));
```

Constants count up from `PVar + 0`; parameters count down from `PVar - 16`;
each steps by its datatype's size. `Kalisets1` confirms it exactly — two
declared parameters, `Scale` then `Fix`, read from `-16` and `-24`.

Declarations are **not** one per slot. MB3D's parser expands its three-angle
datatypes into X, Y and Z entries and its six-angle type into six, and
`.Boxscale` is two. `ABoxMod1` is the measurable case: seven declarations, six
of them plain `Double`, and its compiled code reads eight slots — so `Fold` is
`p3`, not `p2`, which is what the recovered arithmetic says too.

`--params` derives the slot count from the declarations and, independently,
the highest slot the code actually reads, then compares them:

```
slot table agrees:       254
  (of which unused tail: 79)
reads past declarations: 34
unknown datatypes:       0
```

The two sides come from completely different places, so agreement is evidence
rather than a tautology, and a disagreement names the formula. That is how
`.DRECI2` and `.SRECI2` — datatypes postdating the keyword list quoted above —
were sized at two slots each: at two the check agrees on 254 formulas, at one
on 247, and the seven that move are exactly the ones declaring them.

The 34 that still read past their declarations are mostly IFS formulas
overrunning by one to three slots. Those datatypes are still wrong.

## Auditing the whole corpus

Three bugs so far produced arithmetic that was wrong and looked entirely
reasonable: a conditional with its arms swapped, a compare against itself, and
an `fxch` that read as a no-op and turned every power in the corpus into
`log2(ln 2)`. None crashed. Each was found only by putting one formula's output
next to the maths its author wrote down — and the formulas checked that way are
exactly the ones already known to work.

`--audit` does it for everything at once. It compares *vocabulary* rather than
values: proving two expressions equal would need symbolic algebra and agreement
about names the two sides do not share, but a description saying "sqrt" against
a decode with no square root disagree about something real.

```
formulas stating their maths and decoding: 77
  agreeing on every operation:             27
  carrying un-collapsed scaffolding:       21
  using something undescribed:             21
  missing something described:             47

of those, explained by the open-coding bug:  14
```

Seventy-seven formulas is twenty-five times what was being checked by hand, and
the correlation is the useful part: fourteen of the disagreements are one bug,
not fourteen findings. A formula whose author wrote `^` and whose decode carries
`log2` is the reassembly chain failing, and fixing that fixes all of them.

Two known false-positive classes, both benign. A commented-out line is not a
description of what the formula does — `ABoxMod1` says
`// rr = pow(...) <- removed to speedup` — and those are skipped. An operation
applied to a constant folds away, so an author writing `sqrt(2)` leaves no
square root for the audit to find.

## Verified

Against formulas that published their own mathematics, in
`tests/against_shipped_maths.rs`. This is the only check that can catch the
failure that matters: a subtly wrong decode produces arithmetic every bit as
plausible as a correct one, with no crash and no way to tell by reading it.

`Kalisets1` recovers as

```
x = abs(x) * p1 / (abs(x) * abs(x) + abs(y) * abs(y) + abs(z) * abs(z) + p2) + J1
```

against a description of `m = Scale/(x*x + y*y + z*z + Fix)` then
`x = x*m + Cx`, after `x = abs(x)`.

`BenesiPine1` reproduces all eight lines of "Benesi fold 1", including the
reused temporary, with `k1` = sqrt(2/3), `k2` = sqrt(1/3), `k3` = sqrt(1/2)
recovered from the constant pool.

## What the output looks like, and why

Assignments in sequence, keeping the compiler's spill slots as named
temporaries, rather than one folded expression per output variable.

Substituting each stored value into its uses is arithmetically identical and
useless. MB3D's formulas reuse temporaries heavily, and inlining turns
`BenesiPine1`'s eight readable lines into a single expression of some ten
thousand characters. The slots the compiler spilled to *are* the temporaries
the original Pascal declared, so keeping them recovers its shape as well as
its value.
