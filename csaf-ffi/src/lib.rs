//! UniFFI bindings for the csaf-rs CSAF validation library.
//!
//! This crate provides a foreign-function interface (FFI) layer on top of `csaf-rs`,
//! enabling Go, WASM, and other language bindings via Mozilla UniFFI.

use csaf::csaf2_0::loader::load_document_from_str as load_document_from_str_2_0;
use csaf::csaf2_1::loader::load_document_from_str as load_document_from_str_2_1;
use csaf::validation::validate_by_preset;

pub mod document;
pub mod types;

uniffi::setup_scaffolding!();

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum CsafError {
    #[error("Invalid JSON: {message}")]
    InvalidJson { message: String },

    #[error("Missing CSAF version: {message}")]
    MissingVersion { message: String },

    #[error("Unsupported CSAF version: {version}")]
    UnsupportedVersion { version: String },

    #[error("Document load error: {message}")]
    LoadError { message: String },
}

// ---------------------------------------------------------------------------
// FFI-friendly mirror types for csaf::validation
// ---------------------------------------------------------------------------

/// A single validation error with a message and the JSON path where it occurred.
#[derive(Debug, Clone, uniffi::Record)]
pub struct ValidationError {
    pub message: String,
    pub instance_path: String,
}

/// Status of a single test execution.
#[derive(Debug, Clone, uniffi::Enum)]
pub enum TestResultStatus {
    Success,
    Failure {
        errors: Vec<ValidationError>,
        warnings: Vec<ValidationError>,
        infos: Vec<ValidationError>,
    },
    NotFound,
    Skipped,
}

/// Result of a single validation test.
#[derive(Debug, Clone, uniffi::Record)]
pub struct TestResult {
    pub test_id: String,
    pub status: TestResultStatus,
}

/// Overall result of a CSAF validation run.
#[derive(Debug, Clone, uniffi::Record)]
pub struct ValidationResult {
    /// Whether the validation was successful (no errors).
    pub success: bool,
    /// The detected CSAF version.
    pub version: String,
    /// Individual test results.
    pub test_results: Vec<TestResult>,
    /// Total number of errors.
    pub num_errors: u64,
    /// Total number of warnings.
    pub num_warnings: u64,
    /// Total number of informational findings.
    pub num_infos: u64,
    /// Total number of tests not found.
    pub num_not_found: u64,
}

// ---------------------------------------------------------------------------
// Conversion helpers: csaf-rs types → FFI types
// ---------------------------------------------------------------------------

impl From<&csaf::validation::ValidationError> for ValidationError {
    fn from(e: &csaf::validation::ValidationError) -> Self {
        Self {
            message: e.message.clone(),
            instance_path: e.instance_path.clone(),
        }
    }
}

impl From<&csaf::validation::TestResultStatus> for TestResultStatus {
    fn from(s: &csaf::validation::TestResultStatus) -> Self {
        match s {
            csaf::validation::TestResultStatus::Success => Self::Success,
            csaf::validation::TestResultStatus::Failure {
                errors,
                warnings,
                infos,
            } => Self::Failure {
                errors: errors.iter().map(Into::into).collect(),
                warnings: warnings.iter().map(Into::into).collect(),
                infos: infos.iter().map(Into::into).collect(),
            },
            csaf::validation::TestResultStatus::NotFound => Self::NotFound,
            csaf::validation::TestResultStatus::Skipped => Self::Skipped,
        }
    }
}

impl From<&csaf::validation::TestResult> for TestResult {
    fn from(r: &csaf::validation::TestResult) -> Self {
        Self {
            test_id: r.test_id.clone(),
            status: (&r.status).into(),
        }
    }
}

impl From<csaf::validation::ValidationResult> for ValidationResult {
    fn from(r: csaf::validation::ValidationResult) -> Self {
        Self {
            success: r.success,
            version: r.version,
            test_results: r.test_results.iter().map(Into::into).collect(),
            num_errors: r.num_errors as u64,
            num_warnings: r.num_warnings as u64,
            num_infos: r.num_infos as u64,
            num_not_found: r.num_not_found as u64,
        }
    }
}

// ---------------------------------------------------------------------------
// Exported functions
// ---------------------------------------------------------------------------

/// Validate a CSAF document from a JSON string.
///
/// Auto-detects the CSAF version from the document's `document.csaf_version`
/// field and validates it according to the specified preset.
///
/// # Arguments
///
/// * `json_str` - The CSAF document as a JSON string.
/// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
///
/// # Returns
///
/// An `ValidationResult` containing the validation outcome and any findings.
#[uniffi::export]
pub fn validate_csaf(json_str: String, preset: String) -> Result<ValidationResult, CsafError> {
    let json_value: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| CsafError::InvalidJson { message: e.to_string() })?;

    let version = json_value
        .get("document")
        .and_then(|doc| doc.get("csaf_version"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| CsafError::MissingVersion {
            message: "document.csaf_version field not found".into(),
        })?
        .to_string();

    let result = match version.as_str() {
        "2.0" => {
            let doc =
                load_document_from_str_2_0(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
            validate_by_preset(&doc, "2.0", &preset)
        },
        "2.1" => {
            let doc =
                load_document_from_str_2_1(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
            validate_by_preset(&doc, "2.1", &preset)
        },
        other => {
            return Err(CsafError::UnsupportedVersion {
                version: other.to_string(),
            });
        },
    };

    Ok(result.into())
}

/// Validate a CSAF 2.0 document from a JSON string.
///
/// # Arguments
///
/// * `json_str` - The CSAF 2.0 document as a JSON string.
/// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
#[uniffi::export]
pub fn validate_csaf_2_0(json_str: String, preset: String) -> Result<ValidationResult, CsafError> {
    let doc = load_document_from_str_2_0(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
    Ok(validate_by_preset(&doc, "2.0", &preset).into())
}

/// Validate a CSAF 2.1 document from a JSON string.
///
/// # Arguments
///
/// * `json_str` - The CSAF 2.1 document as a JSON string.
/// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
#[uniffi::export]
pub fn validate_csaf_2_1(json_str: String, preset: String) -> Result<ValidationResult, CsafError> {
    let doc = load_document_from_str_2_1(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
    Ok(validate_by_preset(&doc, "2.1", &preset).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_csaf_invalid_json() {
        let result = validate_csaf("not json".into(), "basic".into());
        assert!(result.is_err());
        match result.unwrap_err() {
            CsafError::InvalidJson { .. } => {},
            other => panic!("Expected InvalidJson, got: {other:?}"),
        }
    }

    #[test]
    fn test_validate_csaf_missing_version() {
        let result = validate_csaf(r#"{"document": {}}"#.into(), "basic".into());
        assert!(result.is_err());
        match result.unwrap_err() {
            CsafError::MissingVersion { .. } => {},
            other => panic!("Expected MissingVersion, got: {other:?}"),
        }
    }

    #[test]
    fn test_validate_csaf_unsupported_version() {
        let result = validate_csaf(r#"{"document": {"csaf_version": "1.0"}}"#.into(), "basic".into());
        assert!(result.is_err());
        match result.unwrap_err() {
            CsafError::UnsupportedVersion { version } => assert_eq!(version, "1.0"),
            other => panic!("Expected UnsupportedVersion, got: {other:?}"),
        }
    }

    // -- CsafDocument tests -------------------------------------------------

    const CSAF_21_TEMPLATE: &str = r#"{
      "$schema": "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
      "document": {
        "category": "csaf_base",
        "csaf_version": "2.1",
        "distribution": { "tlp": { "label": "CLEAR" } },
        "publisher": {
          "category": "other",
          "name": "Test Publisher",
          "namespace": "https://example.com"
        },
        "title": "Test Advisory",
        "tracking": {
          "current_release_date": "2024-01-24T10:00:00.000Z",
          "id": "TEST-001",
          "initial_release_date": "2024-01-24T10:00:00.000Z",
          "revision_history": [
            { "date": "2024-01-24T10:00:00.000Z", "number": "1", "summary": "Initial." }
          ],
          "status": "final",
          "version": "1"
        }
      }
    }"#;

    const CSAF_20_TEMPLATE: &str = r#"{
      "document": {
        "category": "csaf_base",
        "csaf_version": "2.0",
        "distribution": { "tlp": { "label": "WHITE" } },
        "publisher": {
          "category": "other",
          "name": "Test Publisher",
          "namespace": "https://example.com"
        },
        "title": "Test Advisory 2.0",
        "tracking": {
          "current_release_date": "2024-01-24T10:00:00.000Z",
          "id": "TEST-002",
          "initial_release_date": "2024-01-24T10:00:00.000Z",
          "revision_history": [
            { "date": "2024-01-24T10:00:00.000Z", "number": "1", "summary": "Initial." }
          ],
          "status": "final",
          "version": "1"
        }
      }
    }"#;

    #[test]
    fn test_document_from_json_21() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        assert_eq!(doc.get_version_string(), "2.1");
        assert!(matches!(doc.get_version(), types::CsafVersion::V21));
    }

    #[test]
    fn test_document_from_json_20() {
        let doc = document::CsafDocument::from_json(CSAF_20_TEMPLATE.into()).unwrap();
        assert_eq!(doc.get_version_string(), "2.0");
        assert!(matches!(doc.get_version(), types::CsafVersion::V20));
    }

    #[test]
    fn test_document_tracking_id() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        assert_eq!(doc.get_tracking_id().unwrap(), "TEST-001");
    }

    #[test]
    fn test_document_category() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        assert!(matches!(doc.get_category().unwrap(), types::DocumentCategory::CsafBase));
    }

    #[test]
    fn test_document_current_release_date() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        let dt = doc.get_current_release_date().unwrap();
        match dt {
            types::CsafDateTime::Valid { raw_string, .. } => {
                assert!(raw_string.contains("2024"));
            },
            other => panic!("Expected Valid datetime, got: {other:?}"),
        }
    }

    #[test]
    fn test_document_validate() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        let result = doc.validate("basic".into()).unwrap();
        assert!(!result.test_results.is_empty());
    }

    #[test]
    fn test_document_run_test() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        let result = doc.run_test("6.1.1".into()).unwrap();
        assert_eq!(result.test_id, "6.1.1");
    }

    #[test]
    fn test_document_vulnerability_count() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        assert_eq!(doc.get_vulnerability_count().unwrap(), 0);
    }

    #[test]
    fn test_document_to_json_roundtrip() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        let json = doc.to_json();
        // Should be valid JSON that can be re-parsed
        let reparsed = document::CsafDocument::from_json(json).unwrap();
        assert_eq!(reparsed.get_tracking_id().unwrap(), "TEST-001");
    }

    #[test]
    fn test_document_product_tree() {
        let doc = document::CsafDocument::from_json(CSAF_21_TEMPLATE.into()).unwrap();
        assert!(!doc.has_product_tree().unwrap());
        assert!(doc.get_all_product_ids().unwrap().is_empty());
    }
}
