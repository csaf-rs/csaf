#!/usr/bin/env bash
# Generate UniFFI Go bindings for csaf-ffi.
#
# Usage: ./scripts/publish/generate_go_bindings.sh [--skip-build]
#
# This script:
#   1. Builds csaf-ffi as a native release (dylib + static archive)
#   2. Runs uniffi-bindgen-go to generate go/csaf_ffi/csaf_ffi.go + csaf_ffi.h
#   3. Copies the static library to go/csaf_ffi/lib/<GOOS>_<GOARCH>/
#      so that the per-platform #cgo LDFLAGS in cgo_*.go can find it
set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# Ensure cargo-installed tools are on PATH
export PATH="$HOME/.cargo/bin:$PATH"

if [[ "${1:-}" != "--skip-build" ]]; then
  echo "Building csaf-ffi (native release)..."
  cargo build -p csaf-ffi --release --locked
fi

echo "Generating Go bindings..."
uniffi-bindgen-go \
  --library "$REPO_ROOT/target/release/libcsaf_ffi.dylib" \
  --out-dir "$REPO_ROOT/go/" 

# Copy the static archive into the per-platform lib directory so CGo can find
# it without needing CGO_LDFLAGS to be set manually.
GOOS="$(go env GOOS)"
GOARCH="$(go env GOARCH)"
LIB_DIR="$REPO_ROOT/go/csaf_ffi/lib/${GOOS}_${GOARCH}"
mkdir -p "$LIB_DIR"
cp "$REPO_ROOT/target/release/libcsaf_ffi.a" "$LIB_DIR/"
echo "Copied libcsaf_ffi.a → $LIB_DIR/"

echo "Done! Output in $REPO_ROOT/go/csaf_ffi/"
