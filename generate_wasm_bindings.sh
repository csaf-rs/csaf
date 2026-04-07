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
  cargo build --target wasm32-unknown-unknown --release -p csaf-ffi
fi

echo "Generating TypeScript bindings..."
uniffi-bindgen-js generate \
  "$SCRIPT_DIR/target/wasm32-unknown-unknown/release/csaf_ffi.wasm" \
  --out-dir "$OUT_DIR"

echo "Patching uniffi_runtime.ts with wasm-bindgen polyfill imports..."
python3 -c "
import pathlib, sys
rt = pathlib.Path('$OUT_DIR/uniffi_runtime.ts')
src = rt.read_text()

old = 'async function loadWasm(wasmUrl: URL | string): Promise<WebAssembly.WebAssemblyInstantiatedSource> {'
if old not in src:
    print('ERROR: Could not find loadWasm function to patch', file=sys.stderr)
    sys.exit(1)

polyfill = '''// Polyfill imports required by transitive dependencies (chrono, getrandom,
// uuid) that compile with wasm-bindgen stubs on wasm32.  These are only
// needed to satisfy the linker — the hot paths get a real implementation;
// the rest are harmless no-ops.
function _wasmImports(): WebAssembly.Imports {
  function getRandomValues(_ptr: number, _len: number) {}
  return {
    __wbindgen_placeholder__: {
      __wbindgen_object_drop_ref: () => {},
      __wbg_getRandomValues_1c61fac11405ffdc: getRandomValues,
      __wbindgen_describe: () => {},
      __wbg___wbindgen_throw_be289d5034ed271b: (_ptr: number, _len: number) => {
        throw new Error(\"wbindgen_throw called from WASM\");
      },
    },
    __wbindgen_externref_xform__: {
      __wbindgen_externref_table_set_null: () => {},
      __wbindgen_externref_table_grow: () => 0,
    },
  };
}

'''

new_fn = old + '''
  const imports = _wasmImports();'''

src = src.replace(old, polyfill + new_fn)
src = src.replace('WebAssembly.instantiate(bytes)', 'WebAssembly.instantiate(bytes, imports)')
src = src.replace('WebAssembly.instantiateStreaming(resp.clone())', 'WebAssembly.instantiateStreaming(resp.clone(), imports)')
src = src.replace('WebAssembly.instantiate(await resp.arrayBuffer())', 'WebAssembly.instantiate(await resp.arrayBuffer(), imports)')
rt.write_text(src)
print('Patched successfully.')
"

echo "Done! Output in $OUT_DIR"
