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

Done: extraction, the ABI table, a corpus survey, annotated disassembly.

Not done: the symbolic x87 executor that turns a decode into an expression
tree, control flow for the 282 formulas with branches, the SSE2 path, and the
mapping from a `PVar` slot back to a name in `[OPTIONS]`.

That last one is unsolved and worth stating plainly. Parameters live at
decreasing offsets from `PVar` and MB3D's comment says `PVar-8` is always the
constant `0.5`, so slot *n* sits at `-(8n+8)`. But the slots a formula uses are
**not** its `[OPTIONS]` entries in order — `ABoxMod1` declares seven and uses
slots 1, 4, 5, 6, 7, 8, skipping 2 and 3. The rule is in MB3D's loader and has
not been read yet. Until it is, recovered formulas would have correct
mathematics with unnamed parameters.

## Verified

`ABoxMod1` decodes, by hand against this tool's output, to

```
Fold - abs(abs(x + FoldXMod) - Fold) - abs(FoldXMod)
```

which is exactly what its shipped description says. The approach produces the
real formula, not a plausible-looking one — which is the only failure mode that
matters at 454 formulas, because nothing about the output would look wrong.
