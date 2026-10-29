use super::deserialize::deserialize_and_version_match_cvss_map;
use crate::csaf_traits::ContentTrait;
use crate::validation::TestFinding;
use cvss_rs::{Cvss, Version};
use serde_json::Value;

/// Wrapper type for a CVSS value whose version matches the expected CSAF score/metric version property.
#[derive(Debug)]
pub(crate) struct VersionMatchedCvss(pub Cvss);

impl VersionMatchedCvss {
    /// Returns the version of the contained CVSS value.
    #[cfg(test)]
    pub fn into_cvss(self) -> Cvss {
        self.0
    }
}

/// Dispatch validate function for each CVSS metric present (`cvss_v2`, `cvss_v3`, `cvss_v4`).
pub(crate) fn validate_content_metrics(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    validate: impl Fn(&serde_json::Map<String, Value>, &str, &mut Option<Vec<TestFinding>>, Version),
) {
    if let Some(cvss_map) = content.get_cvss_v2_raw() {
        validate(cvss_map, instance_path, errors, Version::V2);
    }
    if let Some(cvss_map) = content.get_cvss_v3_raw() {
        validate(cvss_map, instance_path, errors, Version::V3_0);
    }
    if let Some(cvss_map) = content.get_cvss_v4_raw() {
        validate(cvss_map, instance_path, errors, Version::V4);
    }
}

/// Try to dispatch the validate function for the expected version.
///
/// Will add an error to `errors` if the CVSS metric cannot be deserialized or if the deserialized version does not match the expected version.
pub(crate) fn validate_metric(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
    validate: impl FnOnce(Cvss, &serde_json::Map<String, Value>, &str, &mut Option<Vec<TestFinding>>),
) {
    match deserialize_and_version_match_cvss_map(cvss_map, instance_path, expected_version) {
        Ok(VersionMatchedCvss(cvss)) => validate(cvss, cvss_map, instance_path, errors),
        Err(error) => {
            errors.get_or_insert_default().push(TestFinding::Error(error));
        },
    }
}

/// Deserializes and extracts the expected CVSS variant, and validates scores and severities.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
pub(crate) fn validate_scores(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
) {
    validate_metric(
        cvss_map,
        instance_path,
        errors,
        expected_version,
        |cvss, cvss_map, instance_path, errors| match cvss {
            Cvss::V2(cvss2) => {
                super::validate_scores::validation::validate_v2_scores(&cvss2, cvss_map, instance_path, errors);
            },
            Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3) => {
                super::validate_scores::validation::validate_v3_scores(&cvss3, instance_path, errors);
            },
            Cvss::V4(cvss4) => {
                super::validate_scores::validation::validate_v4_scores(&cvss4, instance_path, errors);
            },
        },
    );
}

/// Deserializes and extracts the expected CVSS variant, and validates the consistency of the JSON vs.
/// the CVSS vector.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
pub(crate) fn validate_consistency(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
) {
    validate_metric(
        cvss_map,
        instance_path,
        errors,
        expected_version,
        |cvss, _, instance_path, errors| match cvss {
            Cvss::V2(cvss2) => {
                super::validate_consistency::validation::validate_v2_consistency(&cvss2, instance_path, errors);
            },
            Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3) => {
                super::validate_consistency::validation::validate_v3_consistency(&cvss3, instance_path, errors);
            },
            Cvss::V4(cvss4) => {
                super::validate_consistency::validation::validate_v4_consistency(&cvss4, instance_path, errors);
            },
        },
    );
}
