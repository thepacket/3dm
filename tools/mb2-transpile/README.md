# mb2-transpile

Translates [Mandelbulber2](https://github.com/buddhi1980/mandelbulber2)'s OpenCL
fractal formulas into the WGSL fragments 3DM's formula stack uses, and recovers
each parameter's default from Mandelbulber's C++ sources.

Mandelbulber2 is GPL-3.0, which is why 3DM is too.

## Getting the corpus

The transpiler reads a Mandelbulber2 checkout. Only three directories are
needed, so a sparse clone keeps it small:

```bash
git clone --depth 1 --filter=blob:none --sparse \
    https://github.com/buddhi1980/mandelbulber2.git /tmp/mb2
cd /tmp/mb2 && git sparse-checkout set \
    mandelbulber2/formula mandelbulber2/opencl mandelbulber2/src
```

It reads exactly four things:

| Path | Provides |
| --- | --- |
| `mandelbulber2/formula/opencl/*.cl` | the formula bodies |
| `mandelbulber2/opencl/fractal_cl.h` | parameter types and enumerators |
| `mandelbulber2/src/fractal.cpp` | struct path → settings name |
| `mandelbulber2/src/initparameters.cpp` | settings name → default value |
| `mandelbulber2/src/fractal.cpp` (`RecalculateFractalParams`) | derived parameters |
| `mandelbulber2/formula/definition/fractal_*.cpp` | estimator, bailout, `+ c` flag |

The estimator comes from `DEAnalyticFunction`, which is what Mandelbulber's
engine switches on — not the `DEFunctionType` two lines above it in the same
constructor. They disagree for much of the corpus, and the constants are spelled
differently too: `DEFunctionType` carries long-standing typos (`cutomDEFunction`,
`peudoKleinianDEFunction`) that `DEAnalyticFunction` does not.

That last one matters more than its size suggests. A `.cl` body says how to
transform `z` and nothing else — not which closed form turns the result into a
distance, and not whether the caller has to add the sampled point back each
iteration. Mandelbulber does that `+ c` outside the formula, so a transpiled
escape-time formula without the flag renders as a featureless sphere.

## Running

Coverage report only, writing nothing:

```bash
cargo run -p mb2-transpile -- /tmp/mb2 --report-only
```

Regenerate `src/formulas/generated.rs`:

```bash
cargo run -p mb2-transpile -- /tmp/mb2
```

Useful flags: `--errors` prints sample failures, `--dump <file.cl>` prints the
generated WGSL for one formula (or the naga diagnostic if it fails to compile),
which is the only practical way to debug a rewrite rule.

## How coverage is counted

A formula counts only if **naga parses it**. The rewriter accepting a formula
proves nothing: the first run "translated" 375 of 461, of which 33 actually
compiled. Anything the rewriter is unsure of is rejected with a reason rather
than emitted, so the report reflects shippable coverage.

Parsing is a weaker claim than compiling, which is a weaker claim than
rendering, and both gaps have bitten. `naga::front::wgsl::parse_str` — what this
tool's check uses — does not run naga's validator, so it accepted `i32 >= bool`
and a function with no `return` for as long as nobody looked.

The check also has to substitute parameters *exactly* as the renderer does, or
it measures a shader nobody builds. It did not, twice: it cast integer
parameters to `i32` where the renderer keeps f32, and it named its uniform
differently, hiding a formula whose local `u` shadowed the real binding. Any
rule added to `validate.rs` belongs in `three_dm::formulas::mb2` too.

Two examples in the main crate close the gap and report the honest numbers:

```bash
cargo run --example mb2_audit -- --errors
```

Parses *and* validates every emitted formula — going through `codegen`, so it
is the same shader wgpu gets. Currently 373 of 373.

```bash
cargo run --release --example mb2_sweep -- /tmp/sweep
```

Builds a real wgpu pipeline per formula, renders it, and classifies the result
as fractal detail / smooth blob / empty / shader error — then writes a contact
sheet, because whether a shape is *right* is not something a number can tell
you. Currently 302 render with detail, 34 as smooth blobs, 37 empty, and none
fail to build.

Of those 37 empties, 26 are `Transf*` formulas — transforms meant to be composed
into a hybrid, which genuinely draw nothing on their own:

```bash
cargo run --release --example mb2_sweep -- /tmp/hybrid --hybrid
```

stacks each formula onto a known-good Mandelbulb and asks whether the picture
changed, which is the only automatic way to judge a transform. Currently 286
reshape the bulb, 32 erase it, 24 have no effect and 19 fail to build — and the
report says *why* each inert one is inert, since an offset whose default is zero
is a no-op by design rather than a bug. `--rotate` additionally dials every
Euler rotation to 45 degrees, since rotation angles default to zero in
Mandelbulber too and a rotation transform is therefore *correctly* inert out of
the box — proving the matrix derivation works needs a non-zero angle.
`--stagger` runs the base over the first three iterations and the added formula
over the rest, which is how MB3D hybrids are usually built. It discriminates
*worse* than the overlapping default (148 reshape rather than 286), because a
formula applied only late in the loop often cannot move a shape the early
iterations already decided.

The 32 formulas that erase the base are not a fault: 24 of them are escape-time
formulas that add their own `c`, and two divergent maps applied on every
iteration send the orbit past the bailout immediately. That combination is
meaningless, not broken.

## Gotcha

Mandelbulber is clang-formatted to a narrow column and wraps long statements.
Every line-based parser written against it has silently dropped data. Join input
into statements on `;` before parsing anything.
