# syntax=docker/dockerfile:1

# 3DM ships as a static bundle: the "server" only has to hand over an HTML file,
# a JS shim and a ~7 MB wasm blob. All the interesting work happens on the
# client's GPU, so the runtime image is just Caddy serving precompressed files.

########################  build  ########################
FROM rust:1.96-bookworm AS build

ARG TRUNK_VERSION=0.21.14
ARG BINARYEN_VERSION=130
# Supplied automatically by BuildKit; lets the same Dockerfile build on an
# Apple-silicon laptop and on Fly's amd64 builders.
ARG TARGETARCH

RUN apt-get update \
 && apt-get install -y --no-install-recommends brotli \
 && rm -rf /var/lib/apt/lists/*

# Prebuilt binaries — building Trunk and Binaryen from source would dominate
# the image build time.
RUN set -eux; \
    case "${TARGETARCH}" in \
      amd64) arch=x86_64 ;; \
      arm64) arch=aarch64 ;; \
      *) echo "unsupported TARGETARCH: ${TARGETARCH}" >&2; exit 1 ;; \
    esac; \
    curl -fsSL "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-${arch}-unknown-linux-gnu.tar.gz" \
      | tar -xz -C /usr/local/bin trunk; \
    curl -fsSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-${arch}-linux.tar.gz" \
      | tar -xz -C /opt; \
    ln -s "/opt/binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt" /usr/local/bin/wasm-opt

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Warm the dependency cache: wgpu and egui are a large tree and rebuild far more
# often than they change. The dummy sources are replaced by the real COPY below.
# Every workspace member's manifest has to be present or cargo cannot load the
# workspace at all, even to build one binary from the root package — and every
# package needs at least one target to exist, hence the stub sources. Neither
# tool is ever compiled here; both are developer tools that do not ship.
#
# Adding a workspace member means adding it here too, or this stage fails with
# "failed to load manifest for workspace member" before it compiles anything.
COPY Cargo.toml Cargo.lock ./
COPY tools/mb2-transpile/Cargo.toml tools/mb2-transpile/
COPY tools/mb3d-decompile/Cargo.toml tools/mb3d-decompile/
RUN mkdir -p src examples tools/mb2-transpile/src tools/mb3d-decompile/src \
 && echo 'fn main() {}' > src/main.rs \
 && echo '' > src/lib.rs \
 && echo 'fn main() {}' > examples/still.rs \
 && echo 'fn main() {}' > tools/mb2-transpile/src/main.rs \
 && echo 'fn main() {}' > tools/mb3d-decompile/src/main.rs \
 && echo '' > tools/mb3d-decompile/src/lib.rs \
 && cargo build --release --target wasm32-unknown-unknown --bin 3dm \
 && rm -rf src examples tools/mb2-transpile/src tools/mb3d-decompile/src

COPY . .
# Trunk fingerprints its output, so cargo must not reuse the dummy build's
# artefacts for our own crate.
RUN touch src/main.rs src/lib.rs

# `--no-sri` because we rewrite the wasm below, after Trunk has hashed it.
# Trunk's own wasm-opt step is disabled in index.html; we run it here instead.
RUN trunk build --release --no-sri

RUN set -eux; \
    wasm="$(find dist -maxdepth 1 -name '*_bg.wasm' | head -1)"; \
    test -n "$wasm"; \
    wasm-opt -Oz --output "${wasm}.opt" "$wasm"; \
    mv "${wasm}.opt" "$wasm"

# Precompress once at build time rather than burning CPU on every request.
# Caddy's `precompressed` serves these directly.
RUN find dist -type f \( -name '*.wasm' -o -name '*.js' -o -name '*.html' -o -name '*.css' \) \
      -exec gzip -9 -k {} \; \
      -exec brotli -9 -k {} \;

########################  runtime  ########################
FROM caddy:2-alpine

COPY --from=build /app/dist /srv
COPY Caddyfile /etc/caddy/Caddyfile

EXPOSE 8080
