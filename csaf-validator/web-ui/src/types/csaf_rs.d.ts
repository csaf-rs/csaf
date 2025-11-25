// Type definitions generated to match Rust `ValidationResult` structures
// These mirror the serde-serialized shapes produced by the Rust library.

export type ValidationPreset = 'basic' | 'extended' | 'full';

export interface ValidationError {
  message: string;
  instancePath: string;
}

export type TestResultStatus =
  | { kind: 'Success' }
  | { kind: 'NotFound' }
  | { kind: 'Failure'; errors: ValidationError[] };

export interface TestResult {
  testId: string;
  status: TestResultStatus;
}

export interface ValidationResult {
  success: boolean;
  version: string;
  preset: ValidationPreset;
  testResults: TestResult[];
  numErrors: number;
  // Backwards-compatible top-level errors array (may be absent)
  errors?: ValidationError[];
}

// low-level binding for the wasm init/exports
export function init(module_or_path?: string | URL | Request | Response): Promise<any>;
export function validateCsaf(jsonStr: string, preset: string): Promise<ValidationResult>;
