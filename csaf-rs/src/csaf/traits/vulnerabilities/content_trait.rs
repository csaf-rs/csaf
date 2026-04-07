use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::csaf2_1::ssvc_dp_selection_list::SelectionList;
use crate::schema::csaf2_0::schema::Score;
use crate::schema::csaf2_1::schema::{Content, Epss, QualitativeSeverityRating};
use serde::de::Error as SerdeError;
use serde_json::{Map, Value};

/// Helper function to extract the version string from a CVSS JSON object.
/// TODO: This will be replaced after the CVSS implementation (probably?)
fn get_cvss_version(cvss: &Map<String, Value>) -> Option<String> {
    cvss.get("version").and_then(|v| v.as_str()).map(|v| v.to_string())
}

/// Trait representing a "content holder" for actual metrics inside a "metric" object.
pub trait ContentTrait {
    fn get_vulnerability_metric_types(&self) -> Vec<CsafVulnerabilityMetric> {
        let mut types: Vec<CsafVulnerabilityMetric> = Vec::new();
        if self.has_ssvc() {
            types.push(CsafVulnerabilityMetric::SsvcV1);
        }
        if let Some(version) = self.get_cvss_v2().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV2(version));
        }
        if let Some(version) = self.get_cvss_v3().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV3(version));
        }
        if let Some(version) = self.get_cvss_v4().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV4(version));
        }
        if self.has_epss() {
            types.push(CsafVulnerabilityMetric::Epss);
        }
        types
    }

    /// Returns whether this content contains a non-empty SSVC metric.
    fn has_ssvc(&self) -> bool;

    /// Returns a parsed instance of the contained SSVC metric, or a `serde_json::Error`,
    /// encapsulated as a `Result`.
    fn get_ssvc(&self) -> Result<SelectionList, serde_json::Error>;

    /// Returns a JSON representation of the contained CVSS 2.0 metric, if any.
    fn get_cvss_v2(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 2.0 metric.
    fn has_cvss_v2(&self) -> bool {
        self.get_cvss_v2().is_some()
    }

    /// Returns a JSON representation of the contained CVSS 3.0/3.1 metric, if any.
    fn get_cvss_v3(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 3.0/3.1 metric.
    fn has_cvss_v3(&self) -> bool {
        self.get_cvss_v3().is_some()
    }

    /// Returns a JSON representation of the contained CVSS 4.0 metric, if any.
    fn get_cvss_v4(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 4.0 metric.
    fn has_cvss_v4(&self) -> bool {
        self.get_cvss_v4().is_some()
    }

    /// Returns a reference to the contained EPSS metric if it exists.
    fn get_epss(&self) -> Option<&Epss>;

    /// Returns whether this content contains an EPSS metric.
    fn has_epss(&self) -> bool {
        self.get_epss().is_some()
    }

    /// Returns a reference to the contained qualitative severity rating if it exists.
    fn get_qualitative_severity(&self) -> Option<&QualitativeSeverityRating>;

    /// Returns whether this content contains a qualitative severity rating.
    fn has_qualitative_severity(&self) -> bool {
        self.get_qualitative_severity().is_some()
    }

    /// This function constructs a JSON path string that can be used to locate the specific
    /// content object within a CSAF document's JSON structure. The path format varies between
    /// CSAF versions due to structural differences in how metrics and content are organized.
    ///
    /// # Parameters
    ///
    /// * `vulnerability_idx` - The zero-based index of the vulnerability in the document's
    ///   vulnerability array
    /// * `metric_idx` - The zero-based index of the metric within the vulnerability's metrics array
    ///
    /// # Returns
    ///
    /// A `String` containing the JSON path to the content object, formatted according to the
    /// appropriate CSAF version specification. The path can be used for validation error reporting,
    /// debugging, or programmatic access to the content location within the document.
    ///
    /// # Examples
    ///
    /// For CSAF 2.0, the path might look like:
    /// `/vulnerabilities/0/scores/0`
    ///
    /// For CSAF 2.1, the path might look like:
    /// `/vulnerabilities/0/metrics/0/content`
    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String;
}

impl ContentTrait for Score {
    fn has_ssvc(&self) -> bool {
        false
    }

    fn get_ssvc(&self) -> Result<SelectionList, serde_json::Error> {
        Err(SerdeError::custom("SSVC metrics are not implemented in CSAF 2.0"))
    }

    fn get_cvss_v2(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v2.is_empty() {
            None
        } else {
            Some(&self.cvss_v2)
        }
    }

    fn get_cvss_v3(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v3.is_empty() {
            None
        } else {
            Some(&self.cvss_v3)
        }
    }

    fn get_cvss_v4(&self) -> Option<&Map<String, Value>> {
        None
    }

    fn get_epss(&self) -> Option<&Epss> {
        None
    }

    fn get_qualitative_severity(&self) -> Option<&QualitativeSeverityRating> {
        None
    }

    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String {
        format!("/vulnerabilities/{vulnerability_idx}/scores/{metric_idx}")
    }
}

impl ContentTrait for Content {
    fn has_ssvc(&self) -> bool {
        !self.ssvc_v2.is_empty()
    }

    fn get_ssvc(&self) -> Result<SelectionList, serde_json::Error> {
        serde_json::from_value::<SelectionList>(Value::Object(self.ssvc_v2.clone()))
    }

    fn get_cvss_v2(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v2.is_empty() {
            None
        } else {
            Some(&self.cvss_v2)
        }
    }

    fn get_cvss_v3(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v3.is_empty() {
            None
        } else {
            Some(&self.cvss_v3)
        }
    }

    fn get_cvss_v4(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v4.is_empty() {
            None
        } else {
            Some(&self.cvss_v4)
        }
    }

    fn get_epss(&self) -> Option<&Epss> {
        self.epss.as_ref()
    }

    fn get_qualitative_severity(&self) -> Option<&QualitativeSeverityRating> {
        self.qualitative_severity_rating.as_ref()
    }

    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String {
        format!("/vulnerabilities/{vulnerability_idx}/metrics/{metric_idx}/content",)
    }
}
