use crate::validation::TestFindingData;
use crate::validations::utils::cvss::errors::{
    create_deserialization_finding_data, create_deserialized_cvss_expected_version_mismatch_finding_data,
};
use cvss_rs::{Cvss, Version};
use serde::Deserialize;
use serde_json::{Map, Value};
use crate::validations::utils::cvss::dispatch::VersionMatchedCvss;

/// Attempts to deserialize a csaf-rs/csaf CVSS JSON map into a scm-rs/cvss-rs [`Cvss`] enum, checking
/// if the expected version defined by the CSAF property matches the deserialized version.
/// Returns [`VersionMatchedCvss`] if successful, or a [`TestFindingData`] error if deserialization fails or the version does not match.
pub(crate) fn deserialize_and_version_match_cvss_map(
    cvss_map: &Map<String, Value>,
    instance_path: &str,
    expected_version: Version,
) -> Result<VersionMatchedCvss, TestFindingData> {
    // Attempt to deserialize the CVSS map into a Cvss enum
    let cvss_deserialized = match Cvss::deserialize(cvss_map) {
        Ok(cvss) => cvss,
        Err(e) => {
            return Err(create_deserialization_finding_data(
                e.to_string(),
                instance_path.to_string(),
            ));
        },
    };
    // Check if the deserialized CVSS version matches the expected version
    let version_matches = matches!(
        (expected_version, &cvss_deserialized),
        (Version::V2, Cvss::V2(_))
            | (Version::V3_0, Cvss::V3_0(_))
            | (Version::V3_0, Cvss::V3_1(_))
            | (Version::V4, Cvss::V4(_))
    );

    if version_matches {
        Ok(VersionMatchedCvss(cvss_deserialized))
    } else {
        Err(create_deserialized_cvss_expected_version_mismatch_finding_data(
            cvss_deserialized.version(),
            expected_version,
            instance_path,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_and_version_match_returns_validated_cvss() {
        let value = json!({
            "version": "3.1",
            "vectorString": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
            "baseScore": 9.8,
            "baseSeverity": "CRITICAL"
        });
        let map = value.as_object().expect("CVSS metric is an object");

        let result = deserialize_and_version_match_cvss_map(map, "/dummy/cvss_v3", Version::V3_0)
            .expect("matching CVSS version should deserialize");

        assert!(matches!(result.into_cvss(), Cvss::V3_1(_)));
    }

    #[test]
    fn deserialize_and_version_match_rejects_mismatched_version() {
        let value = json!({
            "version": "4.0",
            "vectorString": "CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N"
        });
        let map = value.as_object().expect("CVSS metric is an object");

        let error = deserialize_and_version_match_cvss_map(map, "/dummy/cvss_v4", Version::V3_0)
            .expect_err("mismatched CVSS version should fail");

        assert_eq!(error.instance_path, "/dummy/cvss_v4");
        assert_eq!(
            error.message,
            "Deserialized CVSS metric 4.0 does not match expected version 3.0"
        );
    }

    #[test]
    fn deserialize_and_version_match_returns_error_for_invalid_map() {
        let value = json!({
            "version": "3.1"
        });
        let map = value.as_object().expect("CVSS metric is an object");

        let error = deserialize_and_version_match_cvss_map(map, "/dummy/cvss_v3", Version::V3_0)
            .expect_err("invalid CVSS map should fail");

        assert_eq!(error.instance_path, "/dummy/cvss_v3");
        assert!(error.message.starts_with("Error deserializing CVSS metric: "));
    }
}
