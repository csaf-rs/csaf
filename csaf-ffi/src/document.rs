//! CsafDocument — opaque UniFFI object wrapping a parsed CSAF document.
//!
//! Provides version-agnostic access to the full document model via trait dispatch.

use std::sync::{Arc, Mutex};

use csaf::csaf::raw::{HasParsed, RawDocument};
use csaf::csaf2_0::loader::load_document_from_str as load_2_0;
use csaf::csaf2_1::loader::load_document_from_str as load_2_1;
use csaf::csaf_traits::*;
use csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf20;
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf21;
use csaf::validation::{validate_by_preset, validate_by_test, validate_by_tests};

use crate::types::*;
// Explicitly resolve ambiguity with same-named types from csaf::csaf_traits::*
use crate::types::{CsafVersion, Cwe};
use crate::{CsafError, ValidationResult};

// ---------------------------------------------------------------------------
// Internal enum dispatch
// ---------------------------------------------------------------------------

enum DocumentInner {
    V20(RawDocument<Csaf20>),
    V21(RawDocument<Csaf21>),
}

// ---------------------------------------------------------------------------
// CsafDocument — the main UniFFI Object
// ---------------------------------------------------------------------------

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

// Macro to dispatch a trait method across both versions.
// Locks the Mutex, gets parsed, and invokes the body.
macro_rules! dispatch {
    ($self:expr, |$doc:ident| $body:expr) => {{
        let guard = $self.inner.lock().map_err(|_| CsafError::LoadError {
            message: "lock poisoned".into(),
        })?;
        match &*guard {
            DocumentInner::V20(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                let $doc = parsed;
                Ok($body)
            },
            DocumentInner::V21(raw) => {
                let parsed = raw
                    .get_parsed()
                    .as_ref()
                    .map_err(|e| CsafError::LoadError { message: e.clone() })?;
                let $doc = parsed;
                Ok($body)
            },
        }
    }};
}

#[uniffi::export]
impl CsafDocument {
    // -- Constructors -------------------------------------------------------

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

    // -- Validation ---------------------------------------------------------

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

    // -- Core accessors -----------------------------------------------------

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

    // -- Document-level metadata --------------------------------------------

    /// Document title.
    pub fn get_title(&self) -> Result<String, CsafError> {
        dispatch!(self, |doc| doc.get_document().get_tracking().get_id().clone())
    }

    /// Document category (e.g., csaf_vex, csaf_security_advisory).
    pub fn get_category(&self) -> Result<DocumentCategory, CsafError> {
        dispatch!(self, |doc| (&doc.get_document().get_category()).into())
    }

    /// Document language tag, if present.
    pub fn get_lang(&self) -> Result<Option<CsafLanguage>, CsafError> {
        dispatch!(self, |doc| doc.get_document().get_lang().as_ref().map(|l| l.into()))
    }

    /// Tracking ID.
    pub fn get_tracking_id(&self) -> Result<String, CsafError> {
        dispatch!(self, |doc| doc.get_document().get_tracking().get_id().clone())
    }

    /// Current release date.
    pub fn get_current_release_date(&self) -> Result<CsafDateTime, CsafError> {
        dispatch!(self, |doc| (&doc
            .get_document()
            .get_tracking()
            .get_current_release_date())
            .into())
    }

    /// Initial release date.
    pub fn get_initial_release_date(&self) -> Result<CsafDateTime, CsafError> {
        dispatch!(self, |doc| (&doc
            .get_document()
            .get_tracking()
            .get_initial_release_date())
            .into())
    }

    /// Publisher category as a string.
    pub fn get_publisher_category(&self) -> Result<String, CsafError> {
        dispatch!(self, |doc| format!(
            "{:?}",
            doc.get_document().get_publisher().get_category()
        ))
    }

    // -- Vulnerabilities ----------------------------------------------------

    /// Number of vulnerabilities in the document.
    pub fn get_vulnerability_count(&self) -> Result<u64, CsafError> {
        dispatch!(self, |doc| doc.get_vulnerabilities().len() as u64)
    }

    /// Get the CVE identifier for a vulnerability at the given index.
    pub fn get_vulnerability_cve(&self, index: u64) -> Result<Option<String>, CsafError> {
        dispatch!(self, |doc| doc
            .get_vulnerabilities()
            .get(index as usize)
            .and_then(|v| v.get_cve().cloned()))
    }

    /// Get CWE entries for a vulnerability at the given index.
    pub fn get_vulnerability_cwes(&self, index: u64) -> Result<Vec<Cwe>, CsafError> {
        dispatch!(self, |doc| doc
            .get_vulnerabilities()
            .get(index as usize)
            .and_then(|v| v.get_cwe())
            .unwrap_or_default()
            .iter()
            .map(|c| c.into())
            .collect())
    }

    /// Get vulnerability IDs (non-CVE) for a vulnerability at the given index.
    pub fn get_vulnerability_ids(&self, index: u64) -> Result<Vec<VulnerabilityId>, CsafError> {
        dispatch!(self, |doc| doc
            .get_vulnerabilities()
            .get(index as usize)
            .and_then(|v| v.get_ids())
            .map(|ids| {
                ids.iter()
                    .map(|id| VulnerabilityId {
                        system_name: id.get_system_name().clone(),
                        text: id.get_text().clone(),
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Get disclosure date for a vulnerability at the given index.
    pub fn get_vulnerability_disclosure_date(&self, index: u64) -> Result<Option<CsafDateTime>, CsafError> {
        dispatch!(self, |doc| {
            doc.get_vulnerabilities()
                .get(index as usize)
                .and_then(|v| v.get_disclosure_date())
                .as_ref()
                .map(|dt| dt.into())
        })
    }

    /// Get all product references across all vulnerabilities.
    pub fn get_all_product_references(&self) -> Result<Vec<ProductReference>, CsafError> {
        dispatch!(self, |doc| {
            doc.get_all_product_references().into_iter().map(Into::into).collect()
        })
    }

    /// Get all group references across all vulnerabilities.
    pub fn get_all_group_references(&self) -> Result<Vec<ProductReference>, CsafError> {
        dispatch!(self, |doc| doc
            .get_all_group_references()
            .into_iter()
            .map(Into::into)
            .collect())
    }

    // -- Product Tree -------------------------------------------------------

    /// Whether the document has a product tree.
    pub fn has_product_tree(&self) -> Result<bool, CsafError> {
        dispatch!(self, |doc| doc.get_product_tree().is_some())
    }

    /// Get all product IDs referenced in the document.
    pub fn get_all_product_ids(&self) -> Result<Vec<String>, CsafError> {
        dispatch!(self, |doc| doc.get_all_product_references_ids())
    }
}
