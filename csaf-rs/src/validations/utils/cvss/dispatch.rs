use super::deserialize_json::deserialize_and_version_match_cvss_map;
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
    validate: impl FnOnce(Cvss, &str, &mut Option<Vec<TestFinding>>),
) {
    match deserialize_and_version_match_cvss_map(cvss_map, instance_path, expected_version) {
        Ok(VersionMatchedCvss(cvss)) => validate(cvss, instance_path, errors),
        Err(error) => {
            errors.get_or_insert_default().push(TestFinding::Error(error));
        },
    }
}
