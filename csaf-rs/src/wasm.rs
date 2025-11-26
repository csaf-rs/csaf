//! WASM bindings for CSAF validation
//!
//! This module provides WebAssembly bindings for validating CSAF documents in the browser.

use crate::csaf2_0::loader::load_document_from_str as load_document_from_str_2_0;
use crate::csaf2_1::loader::load_document_from_str as load_document_from_str_2_1;
use crate::validation::{ValidationPreset, ValidationResult, validate_by_preset};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Validate a CSAF document from JSON string
///
/// This function auto-detects the CSAF version from the document and validates it
/// according to the specified preset.
///
/// # Arguments
///
/// * `json_str` - The CSAF document as a JSON string
/// * `preset` - The validation preset to use ("basic", "extended", or "full")
///
/// # Returns
///
/// A `ValidationResult` containing the validation outcome and any errors
#[wasm_bindgen(js_name = validateCsaf)]
pub fn validate_csaf(json_str: &str, preset_str: &str) -> Result<ValidationResult, JsValue> {
    // Parse the preset
    let preset = preset_str
        .parse::<ValidationPreset>()
        .map_err(|_| JsValue::from_str(&format!("Invalid preset: {}", preset_str)))?;

    // First, try to detect the version by parsing the JSON
    let json_value: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

    // Try to get the CSAF version from the document
    let version = json_value
        .get("document")
        .and_then(|doc| doc.get("csaf_version"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            JsValue::from_str(
                "Could not detect CSAF version. Make sure the document has a 'document.csaf_version' field",
            )
        })?;

    // Validate based on version
    let result = match version {
        "2.0" => validate_2_0(json_str, preset),
        "2.1" => validate_2_1(json_str, preset),
        _ => Err(format!(
            "Unsupported CSAF version: {}. Supported versions: 2.0, 2.1",
            version
        )),
    };

    match result {
        Ok(validation_result) => Ok(validation_result),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

/// Validate a CSAF 2.0 document
fn validate_2_0(json_str: &str, preset: ValidationPreset) -> Result<ValidationResult, String> {
    let document =
        load_document_from_str_2_0(json_str).map_err(|e| format!("Failed to load CSAF 2.0 document: {}", e))?;

    Ok(validate_by_preset(&document, "2.0", preset))
}

/// Validate a CSAF 2.1 document
fn validate_2_1(json_str: &str, preset: ValidationPreset) -> Result<ValidationResult, String> {
    let document =
        load_document_from_str_2_1(json_str).map_err(|e| format!("Failed to load CSAF 2.1 document: {}", e))?;

    Ok(validate_by_preset(&document, "2.1", preset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_serialization() {
        let result = ValidationResult {
            success: true,
            version: "2.0".to_string(),
            num_errors: 0,
            preset: ValidationPreset::Basic,
            test_results: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"version\":\"2.0\""));
    }
}
