# mb3d-decompile

Recovers readable formulas from Mandelbulb 3D's compiled `.m3f` blobs.

MB3D ships **454 of its 460 formulas as hex-encoded 32-bit x86** inside a
`[CODE]` block. The files are ASCII, which is misleading — the payload is
machine code, and the Pascal it was compiled from is not in the repository.
Only about 40 formulas (the JIT ones, in `EM_JIT_M3Formulas/`) carry real
`[SOURCE]`.

MB3D's source is at `github.com/thargor6/mb3d` under **LGPL-2.1**, which §3
permits relicensing to GPL, so 3DM may use what this recovers.

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
straight-line formulas:  173
fully recovered:         156
ran but assigned nothing:6
bailed:                  11
```

Done: extraction, the ABI table, a corpus survey, annotated disassembly, and a
symbolic x87 executor for straight-line code.

Not done: control flow for the 282 formulas with branches, the SSE2 path, a
handful of integer-load instructions (`fild`, `fimul`, `fistp`), and reading
`.m3p` parameter files.

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
