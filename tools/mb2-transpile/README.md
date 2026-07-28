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

A formula counts only if **naga compiles it**. The rewriter accepting a formula
proves nothing: the first run "translated" 375 of 461, of which 33 actually
compiled. Anything the rewriter is unsure of is rejected with a reason rather
than emitted, so the report reflects shippable coverage.

Compiling is still a much weaker claim than rendering correctly.

## Gotcha

Mandelbulber is clang-formatted to a narrow column and wraps long statements.
Every line-based parser written against it has silently dropped data. Join input
into statements on `;` before parsing anything.
