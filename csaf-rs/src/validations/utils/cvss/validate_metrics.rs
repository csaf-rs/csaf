use super::deserialize_into_cvss::{create_deserialization_error, deserialize_cvss};
use crate::csaf_traits::ContentTrait;
use crate::validation::TestFinding;
use cvss_rs::{Cvss, Version};
use serde_json::Value;

/// Validates the CVSS scores (for v2, v3.x and v4) and severities (for v3.x and v4) of all CVSS metrics (v2, v3.x and v4) present
/// in the vulnerability scores / metric content object.
///
/// For each present metric, the scores and severities stated in the JSON object are compared
/// against the values calculated from its vector string. Versions that are absent are skipped.
///
/// # Arguments
///
/// * `content` - The vulnerability metric content holding the CVSS JSON objects.
/// * `instance_path` - JSON path of `content`, used for the location of reported findings.
/// * `errors` - Collector for findings; lazily initialized to `Some` when the first finding is added.
pub fn validate_content_scores(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    validate_content_metrics(content, instance_path, errors, validate_scores);
}

/// Validates the consistency of all CVSS metrics (v2, v3.x, v4) present in `content`.
///
/// For each present metric, the individual properties of the JSON object (e.g. base metrics) are
/// compared against the values parsed from its vector string. Versions that are absent are skipped.
///
/// # Arguments
///
/// * `content` - The vulnerability metric content holding the CVSS JSON objects.
/// * `instance_path` - JSON path of `content`, used for the location of reported findings.
/// * `errors` - Collector for findings; lazily initialized to `Some` when the first finding is added.
pub fn validate_content_consistency(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    validate_content_metrics(content, instance_path, errors, validate_consistency);
}

/// Dispatch validate function for each CVSS metric present.
fn validate_content_metrics(
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

/// Deserializes and extracts the expected CVSS variant, and validates scores and severities.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
fn validate_scores(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
) {
    let Some(cvss_deserialized) = deserialize_cvss(cvss_map, instance_path, errors) else {
        return;
    };
    match (expected_version, cvss_deserialized) {
        (Version::V2, Cvss::V2(cvss2)) => {
            super::validate_scores::validation::validate_v2_scores(&cvss2, cvss_map, instance_path, errors);
        },
        (Version::V3_0, Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3)) => {
            super::validate_scores::validation::validate_v3_scores(&cvss3, instance_path, errors);
        },
        (Version::V4, Cvss::V4(cvss4)) => {
            super::validate_scores::validation::validate_v4_scores(&cvss4, instance_path, errors);
        },
        (expected, found) => {
            errors.get_or_insert_default().push(create_deserialization_error(
                format!(
                    "Deserialized CVSS metric {} does not match expected version {expected}",
                    found.version()
                ),
                instance_path.to_string(),
            ));
        },
    }
}

/// Deserializes and extracts the expected CVSS variant, and validates the consistency of the JSON vs.
/// the CVSS vector.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
fn validate_consistency(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
) {
    let Some(cvss_deserialized) = deserialize_cvss(cvss_map, instance_path, errors) else {
        return;
    };
    match (expected_version, cvss_deserialized) {
        (Version::V2, Cvss::V2(cvss2)) => {
            super::validate_consistency::validation::validate_v2_consistency(&cvss2, instance_path, errors);
        },
        (Version::V3_0, Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3)) => {
            super::validate_consistency::validation::validate_v3_consistency(&cvss3, instance_path, errors);
        },
        (Version::V4, Cvss::V4(cvss4)) => {
            super::validate_consistency::validation::validate_v4_consistency(&cvss4, instance_path, errors);
        },
        (expected, found) => {
            errors.get_or_insert_default().push(create_deserialization_error(
                format!(
                    "Deserialized CVSS metric {} does not match expected version {expected}",
                    found.version()
                ),
                instance_path.to_string(),
            ));
        },
    }
}
