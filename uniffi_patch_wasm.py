import pathlib, re, sys

if len(sys.argv) < 2:
    print('Usage: uniffi_patch_wasm.py <OUT_DIR>', file=sys.stderr)
    sys.exit(1)

out_dir = pathlib.Path(sys.argv[1])
rt_path = out_dir / 'uniffi_runtime.ts'
src = rt_path.read_text()

MARKER_BEGIN = '// >>> wasm-bindgen shim patch (uniffi_patch_wasm.py)'
MARKER_END = '// <<< wasm-bindgen shim patch (uniffi_patch_wasm.py)'

LOADWASM_START = 'async function loadWasm(wasmUrl: URL | string): Promise<WebAssembly.WebAssemblyInstantiatedSource> {'
if LOADWASM_START not in src:
    print('ERROR: Could not find loadWasm function to patch', file=sys.stderr)
    sys.exit(1)

# The pristine loadWasm exactly as emitted by uniffi-bindgen-js; needed to
# re-apply the patch to an already patched runtime (the patch block replaces
# the whole function, so it must be restored before patching again).
PRISTINE_LOADWASM = '''async function loadWasm(wasmUrl: URL | string): Promise<WebAssembly.WebAssemblyInstantiatedSource> {
  if (typeof process !== 'undefined' && process.versions?.node) {
    // Node.js: read file and instantiate
    const { readFile } = await import('node:fs/promises');
    const { fileURLToPath } = await import('node:url');
    const path = typeof wasmUrl === 'string' ? wasmUrl : fileURLToPath(wasmUrl);
    const bytes = await readFile(path);
    return WebAssembly.instantiate(bytes);
  } else {
    // Browser: prefer streaming compilation (compiles while downloading)
    const url = typeof wasmUrl === 'string' ? wasmUrl : wasmUrl.href;
    const resp = await fetch(url);
    if (typeof WebAssembly.instantiateStreaming === 'function') {
      try {
        // clone() so the original Response body remains available for the fallback
        return await WebAssembly.instantiateStreaming(resp.clone());
      } catch (_) {
        // Fallback if streaming fails (e.g., wrong MIME type)
      }
    }
    return WebAssembly.instantiate(await resp.arrayBuffer());
  }
}
'''

if LOADWASM_START not in src:
    print('ERROR: Could not find loadWasm function to patch', file=sys.stderr)
    sys.exit(1)

# Remove a previous patch application, restoring the pristine loadWasm so the
# script can be re-run on an already patched runtime.
if MARKER_BEGIN in src:
    match = re.search(re.escape(MARKER_BEGIN) + r'[\s\S]*?' + re.escape(MARKER_END) + r'\n', src)
    if match is None:
        print('ERROR: found patch start marker but no end marker', file=sys.stderr)
        sys.exit(1)
    src = src[:match.start()] + PRISTINE_LOADWASM + src[match.end():]

if LOADWASM_START not in src:
    print('ERROR: loadWasm function not found after unpatching', file=sys.stderr)
    sys.exit(1)

# Replace the whole generated loadWasm body: it ends at the first
# top-level closing brace ('\n}') after its start (inner braces are indented).
start = src.index(LOADWASM_START)
end = src.index('\n}\n', start) + len('\n}\n')

# ---------------------------------------------------------------------------
# wasm-bindgen shim layer.
#
# uniffi-bindgen-js only provides the UniFFI FFI glue; transitive Rust
# dependencies (chrono, getrandom, serde-wasm-bindgen, ...) additionally link
# wasm-bindgen bindings whose JS side must be supplied at instantiation time.
#
# ABI notes (raw cargo-emitted module, i.e. *before* wasm-bindgen CLI
# processing):
#   - JS values cross the boundary as u32 indices into a table owned by this
#     shim layer. wasm-bindgen reserves JSIDX_OFFSET (1024)..1027 for
#     undefined, null, true and false (INITIAL_HEAP_OFFSET / INITIAL_HEAP_VALUES
#     in wasm-bindgen's glue generator; keep in sync with wasm-bindgen's
#     src/rt/mod.rs).
#   - Fresh slots are allocated through the module's __externref_table_alloc
#     export (Rust-side free list, which grows the table through the
#     __wbindgen_externref_table_grow import below) and released through the
#     __externref_table_dealloc export.
#   - wasm-bindgen import names carry a signature hash suffix that changes
#     with dependency versions (e.g. __wbg_getRandomValues_<hash>), so shims
#     are matched by their stable base name and the module's actual import
#     list is verified at load time. The base-name switch below must cover
#     every wasm-bindgen import the compiled module may require.
# ---------------------------------------------------------------------------

SHIMS = r'''
// >>> wasm-bindgen shim patch (uniffi_patch_wasm.py)
function instantiateWithShims(module: WebAssembly.Module): WebAssembly.WebAssemblyInstantiatedSource {
  const shims = new WasmBindgenShims();
  const instance = new WebAssembly.Instance(module, shims.buildImports(module));
  // The shims need the module exports (memory, allocator, externref free
  // list) which only exist after instantiation. No Rust code runs until the
  // first export is called, so binding here is always in time.
  shims.bind(instance);
  return { module, instance };
}

async function loadWasm(wasmUrl: URL | string): Promise<WebAssembly.WebAssemblyInstantiatedSource> {
  if (typeof process !== 'undefined' && process.versions?.node) {
    // Node.js: read file and compile
    const { readFile } = await import('node:fs/promises');
    const { fileURLToPath } = await import('node:url');
    const path = typeof wasmUrl === 'string' ? wasmUrl : fileURLToPath(wasmUrl);
    const bytes = await readFile(path);
    const module = await WebAssembly.compile(bytes);
    return instantiateWithShims(module);
  } else {
    // Browser: prefer streaming compilation (compiles while downloading)
    const url = typeof wasmUrl === 'string' ? wasmUrl : wasmUrl.href;
    const resp = await fetch(url);
    if (typeof WebAssembly.compileStreaming === 'function') {
      try {
        // clone() so the original Response body remains available for the fallback
        return await instantiateWithShims(await WebAssembly.compileStreaming(resp.clone()));
      } catch (_) {
        // Fallback if streaming fails (e.g., wrong MIME type)
      }
    }
    return instantiateWithShims(await WebAssembly.compile(await resp.arrayBuffer()));
  }
}

/**
 * JS side of the wasm-bindgen imports required by the compiled module.
 *
 * Import names are matched by their hash-free base name, so dependency
 * updates that only change the signature hash keep working. Any import the
 * table does not cover aborts with an explicit error instead of the cryptic
 * "function import requires a callable" LinkError.
 */
class WasmBindgenShims {
  /** wasm-bindgen reserves JSIDX_OFFSET (1024)..1027 for undefined/null/true/false. */
  private static readonly RESERVED_OFFSET = 1024;
  private static readonly RESERVED_COUNT = 4;

  private table: unknown[] = new Array(WasmBindgenShims.RESERVED_OFFSET).fill(null);
  private instance: WebAssembly.Instance | null = null;
  private dv: DataView | null = null;
  private u8: Uint8Array | null = null;
  private u8Buffer: ArrayBuffer | null = null;
  private wasmVectorLen = 0;

  constructor() {
    // undefined, null, true, false at JSIDX_OFFSET..JSIDX_OFFSET + 3
    this.table.push(undefined, null, true, false);
  }

  bind(instance: WebAssembly.Instance): void {
    this.instance = instance;
  }

  buildImports(module: WebAssembly.Module): WebAssembly.Imports {
    const placeholder: Record<string, (...args: never[]) => unknown> = {};
    const externref: Record<string, (...args: never[]) => unknown> = {};
    for (const imp of WebAssembly.Module.imports(module)) {
      if (imp.kind !== 'function') {
        throw new Error(`wasm-bindgen shim layer only supports function imports, got ${imp.kind}: ${imp.module}::${imp.name}`);
      }
      if (imp.module === '__wbindgen_placeholder__') {
        placeholder[imp.name] = this.placeholderShim(imp.name);
      } else if (imp.module === '__wbindgen_externref_xform__') {
        externref[imp.name] = this.externrefShim(imp.name);
      } else {
        throw new Error(`wasm requires an import from unexpected module '${imp.module}' (${imp.name}); extend WasmBindgenShims`);
      }
    }
    return {
      __wbindgen_placeholder__: placeholder,
      __wbindgen_externref_xform__: externref,
    } as WebAssembly.Imports;
  }

  /**
   * Map a wasm-bindgen import name to its implementation. The trailing
   * signature hash (16 hex chars) is stripped before dispatch; the base name
   * is the stable identity of the shim. Most shims take JS values as table
   * indices (i32) and return indices for JS-valued results.
   */
  private placeholderShim(name: string): (...args: never[]) => unknown {
    const base = name.replace(/_[0-9a-f]{16}$/, '');
    switch (base) {
      // --- object table management -----------------------------------------
      case '__wbindgen_object_drop_ref':
        return (idx: number) => { this.takeObject(idx); };
      case '__wbindgen_object_clone_ref':
        return (idx: number) => this.addObject(this.getObject(idx));
      // --- reflection stubs (only used by wasm-bindgen tooling) -------------
      case '__wbindgen_describe':
        return () => undefined;
      case '__wbindgen_describe_cast':
        // Never executed at runtime; the CLI rewrites its call sites.
        return (arg0: number) => arg0;
      // --- crypto (getrandom via ahash) -------------------------------------
      case '__wbg_getRandomValues':
        return (ptr: number, len: number) => {
          this.guardExceptions(() => {
            globalThis.crypto.getRandomValues(this.uint8View().subarray(ptr, ptr + len));
          });
        };
      // --- Date (chrono on wasm32) ------------------------------------------
      case '__wbg_getTime':
        return (idx: number) => (this.getObject(idx) as Date).getTime();
      case '__wbg_new_0':
        return () => this.addObject(new Date());
      // --- object construction (overloaded base: dispatch by arity) ---------
      case '__wbg_new':
        return (...args: unknown[]) => {
          if (args.length === 0) return this.addObject({}); // new Object()
          if (args.length === 1) {
            return this.addObject(new Uint8Array(this.getObject(args[0] as number) as ArrayLike<number>));
          }
          throw new Error(`unsupported arity ${args.length} for wasm-bindgen import '${name}'`);
        };
      // --- property access (serde-wasm-bindgen) ------------------------------
      case '__wbg_get_with_ref_key':
        return (obj: number, key: number) =>
          this.addObject((this.getObject(obj) as Record<PropertyKey, unknown>)[this.getObject(key) as PropertyKey]);
      case '__wbg_set':
        return (obj: number, key: number, value: number) => {
          (this.getObject(obj) as Record<PropertyKey, unknown>)[this.getObject(key) as PropertyKey] = this.getObject(value);
        };
      case '__wbg_length':
        return (idx: number) => (this.getObject(idx) as { length: number }).length;
      case '__wbg_isSafeInteger':
        return (idx: number) => (Number.isSafeInteger(this.getObject(idx)) ? 1 : 0);
      case '__wbg_Number':
        return (idx: number) => Number(this.getObject(idx));
      case '__wbg_Error':
        return (ptr: number, len: number) => this.addObject(new Error(this.getString(ptr, len)));
      case '__wbg_instanceof_ArrayBuffer':
        return (idx: number) => (this.safeInstanceof(this.getObject(idx), ArrayBuffer) ? 1 : 0);
      case '__wbg_instanceof_Uint8Array':
        return (idx: number) => (this.safeInstanceof(this.getObject(idx), Uint8Array) ? 1 : 0);
      case '__wbg_prototypesetcall':
        // Uint8Array.prototype.set.call(<wasm memory slice>, value)
        return (ptr: number, len: number, value: number) => {
          Uint8Array.prototype.set.call(this.uint8View().subarray(ptr, ptr + len), this.getObject(value) as ArrayLike<number>);
        };
      // --- JS value reflection ----------------------------------------------
      case '__wbg___wbindgen_number_get': {
        return (ptr: number, idx: number) => {
          const obj = this.getObject(idx);
          const value = typeof obj === 'number' ? obj : undefined;
          const dv = this.dataView();
          dv.setFloat64(ptr + 8, value === undefined ? 0 : value, true);
          dv.setInt32(ptr, value === undefined ? 0 : 1, true);
        };
      }
      case '__wbg___wbindgen_string_get': {
        return (ptr: number, idx: number) => {
          const obj = this.getObject(idx);
          const dv = this.dataView();
          if (typeof obj === 'string') {
            const dataPtr = this.passString(obj);
            dv.setInt32(ptr, dataPtr, true);
            dv.setInt32(ptr + 4, this.wasmVectorLen, true);
          } else {
            dv.setInt32(ptr, 0, true);
            dv.setInt32(ptr + 4, 0, true);
          }
        };
      }
      case '__wbg___wbindgen_boolean_get':
        return (idx: number) => {
          const value = this.getObject(idx);
          return typeof value === 'boolean' ? (value ? 1 : 0) : 0xFFFFFF;
        };
      case '__wbg___wbindgen_is_object':
        return (idx: number) => {
          const value = this.getObject(idx);
          return typeof value === 'object' && value !== null ? 1 : 0;
        };
      case '__wbg___wbindgen_is_undefined':
        return (idx: number) => (this.getObject(idx) === undefined ? 1 : 0);
      case '__wbg___wbindgen_jsval_loose_eq':
        // Loose equality is intentional: that is the upstream contract.
        // eslint-disable-next-line eqeqeq
        return (a: number, b: number) => (this.getObject(a) == this.getObject(b) ? 1 : 0);
      case '__wbg___wbindgen_in':
        return (a: number, b: number) =>
          this.getObject(a) as PropertyKey in (this.getObject(b) as object) ? 1 : 0;
      case '__wbg___wbindgen_debug_string': {
        return (ptr: number, idx: number) => {
          const dataPtr = this.passString(debugString(this.getObject(idx)));
          const dv = this.dataView();
          dv.setInt32(ptr, dataPtr, true);
          dv.setInt32(ptr + 4, this.wasmVectorLen, true);
        };
      }
      case '__wbg___wbindgen_throw':
        return (ptr: number, len: number) => {
          throw new Error(this.getString(ptr, len));
        };
      default:
        throw new Error(
          `wasm-bindgen import '${name}' (base '${base}') is not covered by WasmBindgenShims; `
          + 'add a shim in uniffi_patch_wasm.py and regenerate the bindings',
        );
    }
  }

  private externrefShim(name: string): (...args: never[]) => unknown {
    switch (name) {
      case '__wbindgen_externref_table_grow':
        // Mimic WebAssembly.Table.grow: return the previous length.
        return (delta: number) => {
          const old = this.table.length;
          for (let i = 0; i < delta; i++) this.table.push(null);
          return old;
        };
      case '__wbindgen_externref_table_set_null':
        return (idx: number) => {
          this.table[idx] = null;
        };
      default:
        throw new Error(`wasm-bindgen externref import '${name}' is not covered by WasmBindgenShims`);
    }
  }

  // --- table + memory access ------------------------------------------------

  private exports(): Record<string, (...args: never[]) => unknown> {
    if (!this.instance) throw new Error('wasm-bindgen shims used before bind()');
    return this.instance.exports as unknown as Record<string, (...args: never[]) => unknown>;
  }

  private addObject(value: unknown): number {
    const alloc = this.exports().__externref_table_alloc as () => number;
    const idx = alloc();
    this.table[idx] = value;
    return idx;
  }

  private takeObject(idx: number): unknown {
    const value = this.table[idx];
    const dealloc = this.exports().__externref_table_dealloc as (idx: number) => void;
    dealloc(idx); // frees the Rust-side slot and nulls our table entry
    return value;
  }

  private getObject(idx: number): unknown {
    return this.table[idx];
  }

  private dataView(): DataView {
    const buffer = this.memory().buffer;
    if (!this.dv || this.dv.buffer !== buffer) {
      this.dv = new DataView(buffer);
    }
    return this.dv;
  }

  private uint8View(): Uint8Array {
    const buffer = this.memory().buffer;
    if (!this.u8 || this.u8Buffer !== buffer) {
      this.u8 = new Uint8Array(buffer);
      this.u8Buffer = buffer;
    }
    return this.u8;
  }

  private memory(): WebAssembly.Memory {
    if (!this.instance) throw new Error('wasm-bindgen shims used before bind()');
    return this.instance.exports.memory as WebAssembly.Memory;
  }

  private getString(ptr: number, len: number): string {
    return _decodeUtf8(this.uint8View().subarray(ptr, ptr + len));
  }

  /** Copy a string into linear memory via the module allocator. */
  private passString(value: string): number {
    const bytes = _textEncoder.encode(value);
    const malloc = this.exports().__wbindgen_malloc as (len: number, align: number) => number;
    const ptr = malloc(bytes.length, 1) >>> 0;
    this.uint8View().set(bytes, ptr);
    this.wasmVectorLen = bytes.length;
    return ptr;
  }

  /** Route exceptions to the module, which converts them into a Rust panic. */
  private guardExceptions(fn: () => void): void {
    try {
      fn();
    } catch (err) {
      const exnStore = this.exports().__wbindgen_exn_store as (idx: number) => void;
      exnStore(this.addObject(err));
    }
  }

  private safeInstanceof(value: unknown, ctor: ArrayBufferConstructor | Uint8ArrayConstructor): boolean {
    try {
      return value instanceof ctor;
    } catch (_) {
      return false;
    }
  }
}

/** Mirrors wasm-bindgen's debugString helper for error/debug messages. */
function debugString(value: unknown): string {
  const type = typeof value;
  if (type === 'number' || type === 'boolean' || value == null) {
    return `${value}`;
  }
  if (type === 'string') {
    return `"${value}"`;
  }
  if (type === 'symbol') {
    const description = (value as symbol).description;
    return description == null ? 'Symbol' : `Symbol(${description})`;
  }
  if (type === 'function') {
    const name = (value as { name?: unknown }).name;
    return typeof name === 'string' && name.length > 0 ? `Function(${name})` : 'Function';
  }
  if (Array.isArray(value)) {
    return `[${value.map(debugString).join(', ')}]`;
  }
  const className = /\[object ([^\]]+)\]/.exec(Object.prototype.toString.call(value))?.[1];
  if (className === 'Object') {
    try {
      return 'Object(' + JSON.stringify(value) + ')';
    } catch (_) {
      return 'Object';
    }
  }
  if (value instanceof Error) {
    const err = value as Error;
    return `${err.name}: ${err.message}\n${err.stack}`;
  }
  return className ?? Object.prototype.toString.call(value);
}

// <<< wasm-bindgen shim patch (uniffi_patch_wasm.py)
'''

# The SHIMS block contains the new loadWasm, so the original generated
# function text (src[start:end]) is replaced entirely.
src = src[:start] + SHIMS.strip('\n') + '\n' + src[end:]

rt_path.write_text(src)
print('Patched successfully.')
