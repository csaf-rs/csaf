// Node 18 does not expose `globalThis.crypto` by default. The wasm module needs
// getRandomValues (ahash -> getrandom) on its first HashMap, so the runtime
// must fall back to `node:crypto`'s webcrypto.
//
// Kept in its own file: node:test runs each file in a separate process, so
// removing the global here cannot leak into the other tests.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

Object.defineProperty(globalThis, 'crypto', { value: undefined, configurable: true });

test('package works without a global Web Crypto', async () => {
  assert.equal(globalThis.crypto, undefined);
  const { CsafFfi } = await import('../csaf_ffi.ts');
  const json = readFileSync(
    new URL('../../oasis-csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json', import.meta.url),
    'utf-8',
  );
  const result = CsafFfi.validateCsaf(json, 'basic');
  assert.equal(result.success, true);
});
