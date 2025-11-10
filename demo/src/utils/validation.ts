import init, { validateCsaf } from '/assets/csaf_rs.js';
import type { ValidationResult, ValidationPreset } from '../types';

let wasmInitialized = false;

/**
 * Initialize the WASM module
 */
export async function initWasm(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  await init('/assets/csaf_rs_bg.wasm');
  wasmInitialized = true;
}

/**
 * Validate a CSAF document
 * @param document - The CSAF document object
 * @param preset - The validation preset ('basic', 'extended', or 'full')
 * @returns Validation result
 */
export async function validateDocument(
  document: unknown,
  preset: ValidationPreset
): Promise<ValidationResult> {
  if (!wasmInitialized) {
    throw new Error('WASM module not initialized');
  }

  const jsonStr = JSON.stringify(document);
  return validateCsaf(jsonStr, preset);
}
