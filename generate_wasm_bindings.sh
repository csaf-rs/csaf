#!/usr/bin/env bash
# Generate UniFFI WASM bindings for csaf-ffi.
#
# Usage: ./generate_wasm_bindings.sh [--skip-build]
#
# This script:
#   1. Builds csaf-ffi for wasm32-unknown-unknown (release)
#   2. Generates TypeScript + WASM bindings via uniffi-bindgen-js
#   3. Patches the runtime to provide wasm-bindgen polyfill imports
#      (required by transitive deps: chrono, getrandom, uuid)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_DIR="$SCRIPT_DIR/wasm"

# Ensure cargo-installed tools are on PATH
export PATH="$HOME/.cargo/bin:$PATH"

if [[ "${1:-}" != "--skip-build" ]]; then
  echo "Building csaf-ffi for wasm32-unknown-unknown..."
  cargo build --target wasm32-unknown-unknown --release --locked -p csaf-ffi
fi

echo "Generating TypeScript bindings..."
uniffi-bindgen-js generate \
  "$SCRIPT_DIR/target/wasm32-unknown-unknown/release/csaf_ffi.wasm" \
  --out-dir "$OUT_DIR"

echo "Patching uniffi_runtime.ts with wasm-bindgen polyfill imports..."
python3 uniffi_patch_wasm.py "$OUT_DIR"

echo "Done! Output in $OUT_DIR"
