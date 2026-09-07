use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::schema::csaf2_0::schema::Score;
use crate::schema::csaf2_1::schema::{Content, Epss, QualitativeSeverityRating};
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;
use serde::Deserialize;
use serde::de::Error as SerdeError;
use serde_json::{Map, Value};
use ssvc::selection_list::SelectionList;

/// Helper function to extract the version string from a CVSS JSON object.
/// TODO: This will be replaced after the CVSS implementation (probably?)
fn get_cvss_version(cvss: &Map<String, Value>) -> Option<String> {
    cvss.get("version").and_then(|v| v.as_str()).map(|v| v.to_string())
}

/// Trait representing a "content holder" for actual metrics inside a "metric" object.
pub trait ContentTrait {
    /// Returns all CVSS metric types present.
    fn get_cvss_metric_types(&self) -> Vec<CsafVulnerabilityMetric> {
        let mut types: Vec<CsafVulnerabilityMetric> = Vec::new();
        if let Some(version) = self.get_cvss_v2().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV2(version));
        }
        if let Some(version) = self.get_cvss_v3().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV3(version));
        }
        if let Some(version) = self.get_cvss_v4().and_then(get_cvss_version) {
            types.push(CsafVulnerabilityMetric::CvssV4(version));
        }
        types
    }

    /// Returns all metric types present.
    fn get_vulnerability_metric_types(&self) -> Vec<CsafVulnerabilityMetric> {
        let mut types: Vec<CsafVulnerabilityMetric> = self.get_cvss_metric_types();
        if self.has_ssvc_v2() {
            types.push(CsafVulnerabilityMetric::SsvcV2);
        }
        if self.has_epss() {
            types.push(CsafVulnerabilityMetric::Epss);
        }
        if self.has_qualitative_severity() {
            types.push(CsafVulnerabilityMetric::QualitativeSeverityRating);
        }
        types
    }

    /// Returns whether this content contains a non-empty SSVC metric.
    fn has_ssvc_v2(&self) -> bool;

    /// Returns a parsed instance of the contained SSVC metric, or a `serde_json::Error`,
    /// encapsulated as a `Result`.
    fn get_ssvc_v2(&self) -> Result<SelectionList, serde_json::Error>;

    /// Returns a JSON representation of the contained SSVC v2 metric, if any.
    fn get_ssvc_v2_raw(&self) -> Option<&Map<String, Value>>;

    /// Returns a JSON representation of the contained CVSS 2.0 metric, if any.
    fn get_cvss_v2(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 2.0 metric.
    fn has_cvss_v2(&self) -> bool {
        self.get_cvss_v2().is_some()
    }

    /// Returns the contained CVSS 2.0 metric parsed into its typed representation, if any.
    fn get_cvss_v2_typed(&self) -> Option<Result<CvssV2, serde_json::Error>> {
        self.get_cvss_v2().map(CvssV2::deserialize)
    }

    /// Returns a JSON representation of the contained CVSS 3.0/3.1 metric, if any.
    fn get_cvss_v3(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 3.0/3.1 metric.
    fn has_cvss_v3(&self) -> bool {
        self.get_cvss_v3().is_some()
    }

    /// Returns the contained CVSS 3.0/3.1 metric parsed into its typed representation, if any.
    fn get_cvss_v3_typed(&self) -> Option<Result<CvssV3, serde_json::Error>> {
        self.get_cvss_v3().map(CvssV3::deserialize)
    }

    /// Returns a JSON representation of the contained CVSS 4.0 metric, if any.
    fn get_cvss_v4(&self) -> Option<&Map<String, Value>>;

    /// Returns whether this content contains a CVSS 4.0 metric.
    fn has_cvss_v4(&self) -> bool {
        self.get_cvss_v4().is_some()
    }

    /// Returns the contained CVSS 4.0 metric parsed into its typed representation, if any.
    fn get_cvss_v4_typed(&self) -> Option<Result<CvssV4, serde_json::Error>> {
        self.get_cvss_v4().map(CvssV4::deserialize)
    }

    /// Returns whether this content contains any CVSS metric (v2, v3, or v4).
    fn has_any_cvss(&self) -> bool {
        self.has_cvss_v2() || self.has_cvss_v3() || self.has_cvss_v4()
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
    fn has_ssvc_v2(&self) -> bool {
        false
    }

    fn get_ssvc_v2(&self) -> Result<SelectionList, serde_json::Error> {
        Err(SerdeError::custom("SSVC metrics are not implemented in CSAF 2.0"))
    }

    fn get_ssvc_v2_raw(&self) -> Option<&Map<String, Value>> {
        None
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
    fn has_ssvc_v2(&self) -> bool {
        !self.ssvc_v2.is_empty()
    }

    fn get_ssvc_v2(&self) -> Result<SelectionList, serde_json::Error> {
        SelectionList::deserialize(&self.ssvc_v2)
    }

    fn get_ssvc_v2_raw(&self) -> Option<&Map<String, Value>> {
        if self.ssvc_v2.is_empty() {
            None
        } else {
            Some(&self.ssvc_v2)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    /// Builds a CSAF 2.1 `Content` carrying `metric` under `key`.
    fn content_with(key: &str, metric: &Value) -> Content {
        let mut map = Map::new();
        map.insert(key.to_string(), metric.clone());
        serde_json::from_value(Value::Object(map)).expect("content deserializes")
    }

    /// Builds a CSAF 2.0 `Score` carrying `metric` under `key`.
    fn score_with(key: &str, metric: &Value) -> Score {
        let mut map = Map::new();
        map.insert("products".to_string(), json!(["CSAFPID-0001"]));
        map.insert(key.to_string(), metric.clone());
        serde_json::from_value(Value::Object(map)).expect("score deserializes")
    }

    /// Asserts the typed accessor matching `key` parses to `vector` and the other two
    /// return `None`.
    fn assert_only_typed(content: &impl ContentTrait, key: &str, vector: &str) {
        let (v2, v3, v4) = (
            content.get_cvss_v2_typed(),
            content.get_cvss_v3_typed(),
            content.get_cvss_v4_typed(),
        );
        match key {
            "cvss_v2" => {
                assert_eq!(v2.expect("present").expect("parses").vector_string, vector);
                assert!(v3.is_none());
                assert!(v4.is_none());
            },
            "cvss_v3" => {
                assert!(v2.is_none());
                assert_eq!(v3.expect("present").expect("parses").vector_string, vector);
                assert!(v4.is_none());
            },
            "cvss_v4" => {
                assert!(v2.is_none());
                assert!(v3.is_none());
                assert_eq!(v4.expect("present").expect("parses").vector_string, vector);
            },
            _ => unreachable!(),
        }
    }

    /// Asserts the typed accessor matching `key` reports a parse error.
    fn assert_typed_err(content: &impl ContentTrait, key: &str) {
        match key {
            "cvss_v2" => assert!(content.get_cvss_v2_typed().expect("present").is_err()),
            "cvss_v3" => assert!(content.get_cvss_v3_typed().expect("present").is_err()),
            "cvss_v4" => assert!(content.get_cvss_v4_typed().expect("present").is_err()),
            _ => unreachable!(),
        }
    }

    #[rstest]
    #[case::v2(
        "cvss_v2",
        json!({
            "version": "2.0",
            "vectorString": "AV:N/AC:L/Au:N/C:C/I:C/A:C",
            "baseScore": 10.0
        }),
        "AV:N/AC:L/Au:N/C:C/I:C/A:C"
    )]
    #[case::v3(
        "cvss_v3",
        json!({
            "version": "3.1",
            "vectorString": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
            "baseScore": 9.8,
            "baseSeverity": "CRITICAL"
        }),
        "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
    )]
    #[case::v4(
        "cvss_v4",
        json!({
            "version": "4.0",
            "vectorString": "CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N",
            "baseScore": 9.3,
            "baseSeverity": "CRITICAL"
        }),
        "CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N"
    )]
    fn typed_accessor_parses_and_the_others_are_none(#[case] key: &str, #[case] metric: Value, #[case] vector: &str) {
        assert_only_typed(&content_with(key, &metric), key, vector);
        // CSAF 2.0 has no cvss_v4 property; Score::get_cvss_v4 always returns None.
        if key != "cvss_v4" {
            assert_only_typed(&score_with(key, &metric), key, vector);
        }
    }

    #[rstest]
    #[case::v2("cvss_v2", "2.0")]
    #[case::v3("cvss_v3", "3.1")]
    #[case::v4("cvss_v4", "4.0")]
    fn typed_accessor_reports_a_nonconforming_map(#[case] key: &str, #[case] version: &str) {
        let metric = json!({ "version": version });
        assert_typed_err(&content_with(key, &metric), key);
        if key != "cvss_v4" {
            assert_typed_err(&score_with(key, &metric), key);
        }
    }
}
