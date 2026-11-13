// Regression test for https://github.com/csaf-rs/csaf/issues/1487
//
// The published npm package failed during module initialization with
// `WebAssembly.instantiate(): Import ... function import requires a callable`
// because the wasm-bindgen import shims in uniffi_runtime.ts no longer matched
// the imports required by the compiled csaf_ffi.wasm.
//
// These tests load the package exactly like a consumer would (top-level import,
// which triggers WASM instantiation) and exercise the documented API.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { CsafFfi } from '../csaf_ffi.ts';

const FIXTURE_BASE_URL = new URL('../../oasis-csaf/csaf_2.0/test/validator/data/mandatory/', import.meta.url);
const FIXTURE_2_1_RECOMMENDED_URL = new URL(
  '../../oasis-csaf/csaf_2.1/test/validator/data/recommended/',
  import.meta.url,
);

function readFixture(name: string): string {
  return readFileSync(new URL(name, FIXTURE_BASE_URL), 'utf-8');
}

function readFixture21Recommended(name: string): string {
  return readFileSync(new URL(name, FIXTURE_2_1_RECOMMENDED_URL), 'utf-8');
}

test('package initializes and exposes the documented API', () => {
  assert.equal(typeof CsafFfi.validateCsaf, 'function');
  assert.equal(typeof CsafFfi.getTests, 'function');
});

test('getTests returns the 2.0 test catalogue', () => {
  const tests = CsafFfi.getTests('2.0');
  assert.ok(Array.isArray(tests));
  assert.ok(tests.length > 0, 'expected a non-empty test catalogue');
  assert.ok(
    tests.some((t) => t.name === '6.1.1'),
    'expected mandatory test 6.1.1 to be part of the catalogue',
  );
});

test('valid CSAF 2.0 document passes validation', () => {
  const json = readFixture('oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json');
  const result = CsafFfi.validateCsaf(json, 'basic');
  assert.equal(result.version, '2.0');
  assert.equal(result.success, true);
  assert.equal(result.numErrors, 0n);
});

test('invalid CSAF 2.0 document fails validation with errors', () => {
  const json = readFixture('oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json');
  const result = CsafFfi.validateCsaf(json, 'basic');
  assert.equal(result.success, false);
  assert.ok(result.numErrors > 0n, 'expected at least one validation error');
});

// Test 6.2.33 calls chrono's Utc::now(), which goes through the wasm-bindgen
// Date shims (`new Date()` + `getTime()`).
test('validation-time preset exercises the Date shims (6.2.33)', () => {
  const valid = CsafFfi.validateCsaf(readFixture21Recommended('oasis_csaf_tc-csaf_2_1-2024-6-2-33-11.json'), 'validation-time');
  assert.equal(valid.version, '2.1');
  assert.equal(valid.numErrors, 0n);

  const invalid = CsafFfi.validateCsaf(readFixture21Recommended('oasis_csaf_tc-csaf_2_1-2024-6-2-33-01.json'), 'validation-time');
  assert.ok(invalid.numWarnings + invalid.numInfos + invalid.numErrors > 0n, 'expected 6.2.33 to report a finding');
});

test('invalid JSON is rejected with a structured error', () => {
  assert.throws(() => CsafFfi.validateCsaf('not json', 'basic'), (err: unknown) => {
    assert.ok(err instanceof Error);
    assert.match(err.message, /InvalidJson/);
    return true;
  });
});
