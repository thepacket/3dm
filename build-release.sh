#!/usr/bin/env bash
# Production build of the 3DM web app into dist/.
#
# Trunk's own wasm-opt integration fails against Homebrew's binaryen (the child
# process exits immediately), so we let Trunk emit an unoptimised wasm and run
# wasm-opt over it ourselves. Sub-resource integrity is disabled because we
# rewrite the wasm after Trunk has hashed it.
set -euo pipefail

cd "$(dirname "$0")"

trunk build --release --no-sri

wasm=$(find dist -maxdepth 1 -name '*_bg.wasm' | head -1)
if [[ -z "$wasm" ]]; then
    echo "no wasm found in dist/ — did trunk succeed?" >&2
    exit 1
fi

if command -v wasm-opt >/dev/null 2>&1; then
    before=$(wc -c <"$wasm")
    wasm-opt -Oz --output "$wasm.opt" "$wasm"
    mv "$wasm.opt" "$wasm"
    after=$(wc -c <"$wasm")
    printf 'wasm-opt: %d KB -> %d KB\n' "$((before / 1024))" "$((after / 1024))"
else
    echo "wasm-opt not found (brew install binaryen) — shipping unoptimised wasm" >&2
fi

echo "dist/ ready — serve it with any static file server."
