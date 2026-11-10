use crate::validation::TestFinding;
use crate::validations::utils::cvss::dispatch::validate_metric;
use cvss_rs::{Cvss, Version};
use serde_json::Value;

pub mod utils;
pub mod validation;

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
        |cvss, instance_path, errors| match cvss {
            Cvss::V2(cvss2) => {
                validation::validate_v2_consistency(&cvss2, instance_path, errors);
            },
            Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3) => {
                validation::validate_v3_consistency(&cvss3, instance_path, errors);
            },
            Cvss::V4(cvss4) => {
                validation::validate_v4_consistency(&cvss4, instance_path, errors);
            },
        },
    );
}
