#!/usr/bin/env bash
# Generate UniFFI Go bindings for csaf-ffi.
#
# Usage: ./generate_go_bindings.sh [--skip-build]
#
# This script:
#   1. Builds csaf-ffi as a native release (dylib + static archive)
#   2. Runs uniffi-bindgen-go to generate go/csaf_ffi/csaf_ffi.go + csaf_ffi.h
#   3. Copies the static library to go/csaf_ffi/lib/<GOOS>_<GOARCH>/
#      so that the per-platform #cgo LDFLAGS in cgo_*.go can find it
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Ensure cargo-installed tools are on PATH
export PATH="$HOME/.cargo/bin:$PATH"

if [[ "${1:-}" != "--skip-build" ]]; then
  echo "Building csaf-ffi (native release)..."
  cargo build -p csaf-ffi --release
fi

echo "Generating Go bindings..."
uniffi-bindgen-go \
  --library "$SCRIPT_DIR/target/release/libcsaf_ffi.dylib" \
  --out-dir "$SCRIPT_DIR/go/" 

# Copy the static archive into the per-platform lib directory so CGo can find
# it without needing CGO_LDFLAGS to be set manually.
GOOS="$(go env GOOS)"
GOARCH="$(go env GOARCH)"
LIB_DIR="$SCRIPT_DIR/go/csaf_ffi/lib/${GOOS}_${GOARCH}"
mkdir -p "$LIB_DIR"
cp "$SCRIPT_DIR/target/release/libcsaf_ffi.a" "$LIB_DIR/"
echo "Copied libcsaf_ffi.a → $LIB_DIR/"

echo "Done! Output in $SCRIPT_DIR/go/csaf_ffi/"
