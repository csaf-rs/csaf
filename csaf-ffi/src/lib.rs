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
