use jsonschema::Validator;
use serde_json::{Map, Value};

use crate::{
    csaf_traits::{
        ContentTrait, CsafTrait, MetricTrait, VulnerabilityMetric, VulnerabilityTrait, get_metric_prop_name,
    },
    validation::ValidationError,
};

/// 6.1.8 Invalid CVSS
/// Invalid CVSS object according to scheme
pub fn test_6_1_08_invalid_cvss(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let cvss20_validator = create_validator(include_str!("../../assets/cvss-v2.0.json"));
    let cvss30_validator = create_validator(include_str!("../../assets/cvss-v3.0.json"));
    let cvss31_validator = create_validator(include_str!("../../assets/cvss-v3.1.json"));
    let cvss40_validator = create_validator(include_str!("../../assets/cvss-v4.0.1.json"));

    let mut errors: Vec<ValidationError> = Vec::new();

    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let instance_prefix = content.get_content_json_path(i_v, metric_index);
                if let Some(cvss2) = content.get_cvss_v2() {
                    evaluate_cvss(
                        cvss2,
                        &cvss20_validator,
                        &instance_prefix,
                        VulnerabilityMetric::CvssV2,
                        &mut errors,
                    );
                }
                if let Some(cvss3) = content.get_cvss_v3() {
                    // Use as_str because otherwise additional quotation marks would be included
                    if let Some(version) = cvss3.get("version").and_then(|v| v.as_str()) {
                        let metric_type = VulnerabilityMetric::CvssV3(version.to_string());
                        if version == "3.0" {
                            evaluate_cvss(cvss3, &cvss30_validator, &instance_prefix, metric_type, &mut errors);
                        } else if version == "3.1" {
                            evaluate_cvss(cvss3, &cvss31_validator, &instance_prefix, metric_type, &mut errors);
                        }
                    }
                }
                if let Some(cvss4) = content.get_cvss_v4() {
                    evaluate_cvss(
                        cvss4,
                        &cvss40_validator,
                        &instance_prefix,
                        VulnerabilityMetric::CvssV4,
                        &mut errors,
                    );
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn create_validator(schema_str: &str) -> Validator {
    let parsed_schema: Value = serde_json::from_str(schema_str).unwrap();
    jsonschema::validator_for(&parsed_schema).unwrap()
}

/// Run the CVSS through json schema validation, add every error during validation to `errors`
fn evaluate_cvss(
    cvss_value: &Map<String, Value>,
    validator: &Validator,
    base_path: &str,
    metric: VulnerabilityMetric,
    errors: &mut Vec<ValidationError>,
) {
    let value = serde_json::to_value(cvss_value).unwrap();
    let evaluation = validator.evaluate(&value);
    for error in evaluation.iter_errors() {
        errors.push(create_validation_error(
            error.error.to_string(),
            base_path,
            metric.clone(),
        ));
    }
}

fn create_validation_error(message: String, base: &str, metric: VulnerabilityMetric) -> ValidationError {
    ValidationError {
        message,
        instance_path: format!("{}/{}", base, get_metric_prop_name(metric)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_08() {
        run_csaf20_tests(
            "08",
            test_6_1_08_invalid_cvss,
            HashMap::from([
                (
                    "01",
                    vec![create_validation_error(
                        "\"baseSeverity\" is a required property".to_string(),
                        "/vulnerabilities/0/scores/0",
                        VulnerabilityMetric::CvssV3("3.1".to_string()),
                    )],
                ),
                (
                    "02",
                    vec![create_validation_error(
                        "\"baseSeverity\" is a required property".to_string(),
                        "/vulnerabilities/0/scores/0",
                        VulnerabilityMetric::CvssV3("3.0".to_string()),
                    )],
                ),
                (
                    "03",
                    vec![create_validation_error(
                        "\"version\" is a required property".to_string(),
                        "/vulnerabilities/0/scores/0",
                        VulnerabilityMetric::CvssV2,
                    )],
                ),
            ]),
        );
        // 2.1 tests are not valid at the moment
        run_csaf21_tests(
            "08",
            test_6_1_08_invalid_cvss,
            HashMap::from([
                (
                    "01",
                    vec![create_validation_error(
                        "\"baseSeverity\" is a required property".to_string(),
                        "/vulnerabilities/0/metrics/0/content",
                        VulnerabilityMetric::CvssV3("3.1".to_string()),
                    )],
                ),
                (
                    "02",
                    vec![create_validation_error(
                        "\"baseSeverity\" is a required property".to_string(),
                        "/vulnerabilities/0/metrics/0/content",
                        VulnerabilityMetric::CvssV3("3.0".to_string()),
                    )],
                ),
                (
                    "03",
                    vec![create_validation_error(
                        "\"version\" is a required property".to_string(),
                        "/vulnerabilities/0/metrics/0/content",
                        VulnerabilityMetric::CvssV2,
                    )],
                ),
                (
                    "04",
                    vec![create_validation_error(
                        "\"baseSeverity\" is a required property".to_string(),
                        "/vulnerabilities/0/metrics/0/content",
                        VulnerabilityMetric::CvssV4,
                    )],
                ),
                (
                    "05",
                    vec![
                        create_validation_error(
                            "Unevaluated properties are not allowed ('threatScore', 'threatSeverity' were unexpected)".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow \"CRITICAL\"".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow 9.3".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                    ],
                ),
                (
                    "06",
                    vec![
                        create_validation_error(
                            "Unevaluated properties are not allowed ('environmentalScore', 'environmentalSeverity', 'threatScore', 'threatSeverity' were unexpected)".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow \"CRITICAL\"".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow \"MEDIUM\"".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow 9.3".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                        create_validation_error(
                            "False schema does not allow 5.4".to_string(),
                            "/vulnerabilities/0/metrics/0/content",
                            VulnerabilityMetric::CvssV4,
                        ),
                    ],
                )
            ]),
        );
    }
}
