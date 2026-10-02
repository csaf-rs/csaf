use std::sync::LazyLock;

use jsonschema::Validator;
use serde_json::{Map, Value};

use crate::{
    csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait},
    validation::{TestFinding, TestFindingData},
};

static CVSS20_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_draft_validator(include_str!("../../assets/cvss-v2.0.strict.json")));
// TODO: We may not make this strict, otherwise the oneOf does not match
// TODO: But we will need to detect additional properties here.
static CVSS30_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_validator(include_str!("../../assets/cvss-v3.0.json")));
static CVSS31_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_validator(include_str!("../../assets/cvss-v3.1.json")));
static CVSS40_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_draft_validator(include_str!("../../assets/cvss-v4.0.json")));

/// 6.1.8 Invalid CVSS
///
/// It SHALL be tested that the given CVSS object is valid according to the referenced schema.
///
/// This test runs for all CVSS properties in the `scores` items (CSAF 2.0) and `content` object of
/// each `metrics` item (CSAF 2.1) of each item in the `vulnerabilities` array.
/// It checks if the CVSS object matches the respective CVSS JSON schema using [`jsonschema`].
///
/// For this test, additional properties are not to be allowed.
/// For the CVSS v2 schema, we "make the schema strict" by adding "unevaluatedProperties": false" and
/// constructing the validator as draft-2020-12.
///
/// It emits an error for each offense, including required properties being missing and unevaluated / additional
/// properties being present. The error message is taken from the [`jsonschema`] error.
pub fn test_6_1_08_invalid_cvss(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let instance_prefix = content.get_content_json_path(i_v, metric_index);
                let mut evaluate = |cvss_raw: &Map<String, Value>, validator: &Validator, property_name: &str| {
                    evaluate_cvss(cvss_raw, validator, &instance_prefix, property_name, &mut errors);
                };
                if let Some(cvss_v2_raw) = content.get_cvss_v2_raw() {
                    evaluate(cvss_v2_raw, &CVSS20_VALIDATOR, "cvss_v2");
                }
                if let Some(cvss_v3_raw) = content.get_cvss_v3_raw() {
                    // Use as_str because otherwise additional quotation marks would be included
                    if let Some(version) = cvss_v3_raw.get("version").and_then(|v| v.as_str()) {
                        if version == "3.0" {
                            evaluate(cvss_v3_raw, &CVSS30_VALIDATOR, "cvss_v3");
                        } else if version == "3.1" {
                            evaluate(cvss_v3_raw, &CVSS31_VALIDATOR, "cvss_v3");
                        }
                    }
                }
                if let Some(cvss_v4_raw) = content.get_cvss_v4_raw() {
                    evaluate(cvss_v4_raw, &CVSS40_VALIDATOR, "cvss_v4");
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_8, test_6_1_08_invalid_cvss);

fn create_validator(schema_str: &str) -> Validator {
    jsonschema::validator_for(&serde_json::from_str(schema_str).unwrap()).unwrap()
}

fn create_draft_validator(schema_str: &str) -> Validator {
    jsonschema::draft202012::new(&serde_json::from_str(schema_str).unwrap()).unwrap()
}

/// Run the CVSS through json schema validation, add every error during validation to `errors`
fn evaluate_cvss(
    cvss_value: &Map<String, Value>,
    validator: &Validator,
    base_path: &str,
    property_name: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    let value = serde_json::to_value(cvss_value).unwrap();
    for error in validator.iter_errors(&value) {
        errors
            .get_or_insert_default()
            .push(create_validation_error(error.to_string(), base_path, property_name));
    }
}

fn create_validation_error(message: String, base: &str, property_name: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message,
        instance_path: format!("{}/{}", base, property_name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_8 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_8 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    /// Like [`create_validation_error`], but takes the complete instance path directly.
    fn create_validation_error_with_path(message: impl Into<String>, instance_path: impl Into<String>) -> TestFinding {
        TestFinding::Error(TestFindingData {
            message: message.into(),
            instance_path: instance_path.into(),
        })
    }

    #[test]
    fn test_test_6_1_08() {
        TESTS_2_0.test_6_1_8.expect(ExpectedResults_2_0 {
            case_01: Err(vec![create_validation_error_with_path(
                "\"baseSeverity\" is a required property",
                "/vulnerabilities/0/scores/0/cvss_v3",
            )]),
            case_02: Err(vec![create_validation_error_with_path(
                "\"baseSeverity\" is a required property",
                "/vulnerabilities/0/scores/0/cvss_v3",
            )]),
            case_03: Err(vec![create_validation_error_with_path(
                "\"version\" is a required property",
                "/vulnerabilities/0/scores/0/cvss_v2",
            )]),
            case_s01: Err(vec![create_validation_error_with_path(
                "Unevaluated properties are not allowed ('severity' was unexpected)",
                "/vulnerabilities/0/scores/0/cvss_v2",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
        });

        TESTS_2_1.test_6_1_8.expect(ExpectedResults_2_1 {
            case_01: Err(vec![create_validation_error_with_path(
                "\"baseSeverity\" is a required property",
                "/vulnerabilities/0/metrics/0/content/cvss_v3",
            )]),
            case_02: Err(vec![create_validation_error_with_path(
                "\"baseSeverity\" is a required property",
                "/vulnerabilities/0/metrics/0/content/cvss_v3",
            )]),
            case_03: Err(vec![create_validation_error_with_path(
                "\"version\" is a required property",
                "/vulnerabilities/0/metrics/0/content/cvss_v2",
            )]),
            case_04: Err(vec![create_validation_error_with_path(
                "\"baseSeverity\" is a required property",
                "/vulnerabilities/0/metrics/0/content/cvss_v4",
            )]),
            case_05: Err(vec![create_validation_error_with_path(
                "Unevaluated properties are not allowed ('threatScore', 'threatSeverity' were unexpected)",
                "/vulnerabilities/0/metrics/0/content/cvss_v4",
            )]),
            case_06: Err(vec![create_validation_error_with_path(
                "Unevaluated properties are not allowed ('threatScore', 'threatSeverity', 'environmentalScore', 'environmentalSeverity' were unexpected)",
                "/vulnerabilities/0/metrics/0/content/cvss_v4",
            )]),
            case_s01: Err(vec![create_validation_error_with_path(
                "Unevaluated properties are not allowed ('severity' was unexpected)",
                "/vulnerabilities/0/metrics/0/content/cvss_v2",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
        });
    }
}
