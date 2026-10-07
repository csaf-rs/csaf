use std::sync::LazyLock;

use jsonschema::Validator;
use serde_json::{Map, Value};

use crate::{
    csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait},
    validation::{TestFinding, TestFindingData},
};

#[allow(clippy::expect_used)]
fn create_validator(schema_str: &str) -> Validator {
    let schema = serde_json::from_str(schema_str).expect("embedded CVSS schema must be valid JSON");
    jsonschema::validator_for(&schema).expect("embedded CVSS schema must be a valid JSON schema")
}

#[allow(clippy::expect_used)]
fn create_draft_validator(schema_str: &str) -> Validator {
    let schema = serde_json::from_str(schema_str).expect("embedded CVSS schema must be valid JSON");
    jsonschema::draft202012::new(&schema).expect("embedded CVSS schema must be a valid draft 2020-12 JSON schema")
}

static CVSS_20_STRICT_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_validator(include_str!("../../assets/cvss-v2.0_strict.json")));
static CVSS_30_STRICT_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_validator(include_str!("../../assets/cvss-v3.0_strict.json")));
static CVSS_31_STRICT_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_validator(include_str!("../../assets/cvss-v3.1_strict.json")));
static CVSS40_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| create_draft_validator(include_str!("../../assets/cvss-v4.0.json")));

/// 6.1.8 Invalid CVSS
///
/// It SHALL be tested that the given CVSS object is valid according to the referenced schema.
///
///  This test runs for all CVSS properties in the `scores` items (CSAF 2.0) and `content` object of
/// each `metrics` item (CSAF 2.1) of each item in the `vulnerabilities` array.
/// It checks if the CVSS object matches the respective CVSS JSON schema using [`jsonschema`].
///
/// For this test, additional properties are not to be allowed. For CVSS v2, v3.0, and v3.1,
/// the strict schema provided as a referenced schema in the CSAF standard is used, as it does not allow additional properties.
///
/// For CVSS v3, the used validator is to be determined by the `version` property of the CVSS object.
/// If the version is `3.0`, the strict v3.0 schema is used, otherwise
/// (including for `3.1`, any other version like `3.2` or if the version property is missing) the strict v3.1 schema is used.
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
                if let Some(cvss_v2_raw) = content.get_cvss_v2_raw() {
                    evaluate_cvss_with_validator(
                        cvss_v2_raw,
                        &CVSS_20_STRICT_VALIDATOR,
                        &instance_prefix,
                        "cvss_v2",
                        &mut errors,
                    );
                }
                if let Some(cvss_v3_raw) = content.get_cvss_v3_raw() {
                    // use the CVSS v3.0 validator if the version is "3.0" , else, use the CVSS v3.1 validator
                    let validator = if cvss_v3_raw.get("version").and_then(Value::as_str) == Some("3.0") {
                        &CVSS_30_STRICT_VALIDATOR
                    } else {
                        &CVSS_31_STRICT_VALIDATOR
                    };
                    evaluate_cvss_with_validator(cvss_v3_raw, validator, &instance_prefix, "cvss_v3", &mut errors);
                }
                if let Some(cvss_v4_raw) = content.get_cvss_v4_raw() {
                    evaluate_cvss_with_validator(
                        cvss_v4_raw,
                        &CVSS40_VALIDATOR,
                        &instance_prefix,
                        "cvss_v4",
                        &mut errors,
                    );
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

/// Run the CVSS through json schema validation, add every error during validation to `errors`
fn evaluate_cvss_with_validator(
    cvss_value: &Map<String, Value>,
    validator: &Validator,
    base_path: &str,
    property_name: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    let value = Value::Object(cvss_value.clone());
    for error in validator.iter_errors(&value) {
        errors.get_or_insert_default().push(create_validation_error(
            error.to_string(),
            base_path,
            property_name,
            error.instance_path().as_str(),
        ));
    }
}

fn create_validation_error(
    message: String,
    csaf_base_path: &str,
    metric_prop_name: &str,
    cvss_inner_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message,
        instance_path: format!("{csaf_base_path}/{metric_prop_name}{cvss_inner_path}"),
    })
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_8, test_6_1_08_invalid_cvss);

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
                "false is not of type \"number\"",
                "/vulnerabilities/0/metrics/0/content/cvss_v3/baseScore"
            )]),
            case_s02: Err(vec![create_validation_error_with_path(
                "\"CVSS:3.1/FOO:BAR/AC:L/PR:H/UI:R/S:U/C:H/I:H/A:H\" does not match \"^CVSS:3[.]1/((AV:[NALP]|AC:[LH]|PR:[NLH]|UI:[NR]|S:[UC]|[CIA]:[NLH]|E:[XUPFH]|RL:[XOTWU]|RC:[XURC]|[CIA]R:[XLMH]|MAV:[XNALP]|MAC:[XLH]|MPR:[XNLH]|MUI:[XNR]|MS:[XUC]|M[CIA]:[XNLH])/)*(AV:[NALP]|AC:[LH]|PR:[NLH]|UI:[NR]|S:[UC]|[CIA]:[NLH]|E:[XUPFH]|RL:[XOTWU]|RC:[XURC]|[CIA]R:[XLMH]|MAV:[XNALP]|MAC:[XLH]|MPR:[XNLH]|MUI:[XNR]|MS:[XUC]|M[CIA]:[XNLH])$\"",
                "/vulnerabilities/0/metrics/0/content/cvss_v3/vectorString"
            )]),
            case_s03: Err(vec![create_validation_error_with_path(
                "\"3.2\" is not one of \"3.1\"",
                "/vulnerabilities/0/metrics/0/content/cvss_v3/version"
            )]),
            case_s04: Err(vec![create_validation_error_with_path(
                "\"version\" is a required property",
                "/vulnerabilities/0/metrics/0/content/cvss_v3",
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
