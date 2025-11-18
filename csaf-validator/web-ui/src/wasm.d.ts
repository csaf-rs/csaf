declare module '/static/csaf_rs.js' {
  import type { ValidationResult } from './types';

  export default function init(wasmPath: string): Promise<void>;
  
  export function validateCsaf(jsonStr: string, preset: string): ValidationResult;
}
