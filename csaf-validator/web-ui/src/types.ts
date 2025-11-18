/**
 * A possible error encountered during validation.
 */
export interface ValidationError {
  message: string;
  instancePath: string;
}

/**
 * The preset used for validation, refers to the conformance targets 14-16
 * defines in [CSAF
 * 2.0](https://docs.oasis-open.org/csaf/csaf/v2.0/os/csaf-v2.0-os.html#91-conformance-targets).
 */
export type ValidationPreset = 'basic' | 'extended' | 'full';

/**
 * The result of a CSAF document validation.
 */
export interface ValidationResult {
  success: boolean;
  version: string;
  errors: ValidationError[];
  preset: ValidationPreset;
}
