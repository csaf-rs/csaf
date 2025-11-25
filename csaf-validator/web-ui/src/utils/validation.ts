import type { ValidationResult, ValidationPreset } from '../types';
import init, { validateCsaf } from '/assets/pkg/csaf_rs.js';

let wasmInitialized = false;

/**
 * Initialize the WASM module
 */
export async function initWasm(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  await init();
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
  const result = await validateCsaf(jsonStr, preset);

  // Normalize older/newer shapes: ensure a top-level `errors` array exists
  if (!('errors' in result) || result.errors === undefined) {
    const collected: ValidationResult['errors'] = [];
    if (Array.isArray((result as any).testResults)) {
      for (const tr of (result as any).testResults) {
        const status = tr.status;
        if (status && status.errors && Array.isArray(status.errors)) {
          collected.push(...status.errors);
        }
      }
    }
    (result as any).errors = collected;
  }

  return result;
}
