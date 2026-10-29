use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Cvss;
use serde::Deserialize;
use serde_json::Value;

/// Attempts to deserialize a csaf-rs/csaf CVSS JSON map into a scm-rs/cvss-rs [Cvss] enum.
/// Returns `None` and adds a deserialization error if parsing fails.
pub fn deserialize_cvss(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) -> Option<Cvss> {
    match Cvss::deserialize(cvss_map) {
        Ok(cvss) => Some(cvss),
        Err(e) => {
            errors
                .get_or_insert_default()
                .push(create_deserialization_error(e.to_string(), instance_path.to_string()));
            None
        },
    }
}

pub fn create_deserialization_error(error_message: String, instance_path: String) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Error deserializing CVSS metric: {error_message}"),
        instance_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_cvss_returns_metric_without_findings_for_valid_map() {
        let value = json!({
            "version": "3.1",
            "vectorString": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
            "baseScore": 9.8,
            "baseSeverity": "CRITICAL"
        });
        let map = value.as_object().expect("CVSS metric is an object");
        let mut errors = None;

        let result = deserialize_cvss(map, "/dummy/cvss_v3", &mut errors);

        assert!(matches!(result, Some(Cvss::V3_1(_))));
        assert!(errors.is_none());
    }

    #[test]
    fn deserialize_cvss_records_finding_for_invalid_map() {
        let value = json!({
            "version": "3.1"
        });
        let map = value.as_object().expect("CVSS metric is an object");
        let mut errors = None;

        let result = deserialize_cvss(map, "/dummy/cvss_v3", &mut errors);

        assert!(result.is_none());
        let findings = errors.expect("deserialization should record a finding");
        assert_eq!(findings.len(), 1);
        assert!(matches!(
            &findings[0],
            TestFinding::Error(TestFindingData {
                message,
                instance_path
            }) if message.starts_with("Error deserializing CVSS metric: ")
                && instance_path == "/dummy/cvss_v3"
        ));
    }
}
