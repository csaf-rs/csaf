//! CsafDocument — opaque UniFFI object wrapping a parsed CSAF document.
//!
//! Provides version-agnostic access to the full document model via trait dispatch.

use std::sync::{Arc, Mutex};

use csaf::csaf::raw::{HasParsed, RawDocument};
use csaf::csaf2_0::loader::load_document_from_str as load_2_0;
use csaf::csaf2_1::loader::load_document_from_str as load_2_1;
use csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf20;
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf21;
use csaf::validation::{validate_by_preset, validate_by_test, validate_by_tests};

// Explicitly resolve ambiguity with same-named types from csaf::csaf_traits::*
use crate::types::CsafVersion;
use crate::{CsafError, ValidationResult};

// ---------------------------------------------------------------------------
// Internal enum dispatch
// ---------------------------------------------------------------------------

enum DocumentInner {
    V20(RawDocument<Csaf20>),
    V21(RawDocument<Csaf21>),
}

/// A parsed CSAF document (2.0 or 2.1).
///
/// Create via [`CsafDocument::from_json`] (auto-detect) or
/// [`CsafDocument::from_json_2_0`] / [`CsafDocument::from_json_2_1`].
#[derive(uniffi::Object)]
pub struct CsafDocument {
    inner: Mutex<DocumentInner>,
    version_string: String,
    raw_json: String,
}

#[uniffi::export]
impl CsafDocument {
    /// Parse a CSAF document from JSON, auto-detecting the version.
    #[uniffi::constructor]
    pub fn from_json(json_str: String) -> Result<Arc<Self>, CsafError> {
        let value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(|e| CsafError::InvalidJson { message: e.to_string() })?;

        let version = value
            .get("document")
            .and_then(|d| d.get("csaf_version"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| CsafError::MissingVersion {
                message: "document.csaf_version field not found".into(),
            })?
            .to_string();

        match version.as_str() {
            "2.0" => Self::from_json_2_0(json_str),
            "2.1" => Self::from_json_2_1(json_str),
            other => Err(CsafError::UnsupportedVersion {
                version: other.to_string(),
            }),
        }
    }

    /// Parse a CSAF 2.0 document from JSON.
    #[uniffi::constructor]
    pub fn from_json_2_0(json_str: String) -> Result<Arc<Self>, CsafError> {
        let raw = load_2_0(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
        Ok(Arc::new(Self {
            inner: Mutex::new(DocumentInner::V20(raw)),
            version_string: "2.0".into(),
            raw_json: json_str,
        }))
    }

    /// Parse a CSAF 2.1 document from JSON.
    #[uniffi::constructor]
    pub fn from_json_2_1(json_str: String) -> Result<Arc<Self>, CsafError> {
        let raw = load_2_1(&json_str).map_err(|e| CsafError::LoadError { message: e.to_string() })?;
        Ok(Arc::new(Self {
            inner: Mutex::new(DocumentInner::V21(raw)),
            version_string: "2.1".into(),
            raw_json: json_str,
        }))
    }

    /// Run validation with the given preset ("basic", "extended", "full").
    pub fn validate(&self, preset: String) -> Result<ValidationResult, CsafError> {
        let guard = self.inner.lock().map_err(|_| CsafError::LoadError {
            message: "lock poisoned".into(),
        })?;
        let result = match &*guard {
            DocumentInner::V20(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_preset(parsed, &self.version_string, &preset)
            },
            DocumentInner::V21(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_preset(parsed, &self.version_string, &preset)
            },
        };
        Ok(result.into())
    }

    /// Run a single validation test by ID.
    pub fn run_test(&self, test_id: String) -> Result<crate::TestResult, CsafError> {
        let guard = self.inner.lock().map_err(|_| CsafError::LoadError {
            message: "lock poisoned".into(),
        })?;
        let result = match &*guard {
            DocumentInner::V20(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_test(parsed, &test_id)
            },
            DocumentInner::V21(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_test(parsed, &test_id)
            },
        };
        Ok((&result).into())
    }

    /// Run specific validation tests by their IDs.
    pub fn run_tests(&self, test_ids: Vec<String>) -> Result<ValidationResult, CsafError> {
        let refs: Vec<&str> = test_ids.iter().map(|s| s.as_str()).collect();
        let guard = self.inner.lock().map_err(|_| CsafError::LoadError {
            message: "lock poisoned".into(),
        })?;
        let result = match &*guard {
            DocumentInner::V20(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_tests(parsed, &self.version_string, &refs)
            },
            DocumentInner::V21(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                validate_by_tests(parsed, &self.version_string, &refs)
            },
        };
        Ok(result.into())
    }

    /// The CSAF version of this document.
    pub fn get_version(&self) -> CsafVersion {
        match self.version_string.as_str() {
            "2.0" => CsafVersion::V20,
            _ => CsafVersion::V21,
        }
    }

    /// The version string ("2.0" or "2.1").
    pub fn get_version_string(&self) -> String {
        self.version_string.clone()
    }

    /// The original JSON string used to create this document.
    pub fn to_json(&self) -> String {
        self.raw_json.clone()
    }
}
