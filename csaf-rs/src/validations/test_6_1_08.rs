use serde_json::Value;

use crate::{
    csaf_traits::{
        ContentTrait, CsafTrait, MetricTrait, VulnerabilityMetric, VulnerabilityTrait, get_metric_prop_name,
    },
    validation::ValidationError,
};

/// 6.1.8 Invalid CVSS
/// Invalid CVSS object according to scheme
pub fn test_6_1_08_invalid_cvss(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let cvss20_schema_str = include_str!("../../assets/cvss-v2.0.json");
    let parsed_cvss20_schema: Value = serde_json::from_str(cvss20_schema_str).unwrap();
    let cvss20_validator = jsonschema::validator_for(&parsed_cvss20_schema).unwrap();

    let cvss30_schema_str = include_str!("../../assets/cvss-v3.0.json");
    let parsed_cvss30_schema: Value = serde_json::from_str(cvss30_schema_str).unwrap();
    let cvss30_validator = jsonschema::validator_for(&parsed_cvss30_schema).unwrap();

    let cvss31_schema_str = include_str!("../../assets/cvss-v3.1.json");
    let parsed_cvss31_schema: Value = serde_json::from_str(cvss31_schema_str).unwrap();
    let cvss31_validator = jsonschema::validator_for(&parsed_cvss31_schema).unwrap();

    let cvss40_schema_str = include_str!("../../assets/cvss-v4.0.json");
    let parsed_cvss40_schema: Value = serde_json::from_str(cvss40_schema_str).unwrap();
    let cvss40_validator = jsonschema::validator_for(&parsed_cvss40_schema).unwrap();

    let mut errors: Vec<ValidationError> = Vec::new();

    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                if let Some(cvss2) = content.get_cvss_v2() {
                    let value = serde_json::to_value(cvss2).unwrap();
                    let prop_name = get_metric_prop_name(VulnerabilityMetric::CvssV2);
                    let evaluation = cvss20_validator.evaluate(&value);
                    for error in evaluation.iter_errors() {
                        errors.push(ValidationError {
                            message: error.error.to_string(),
                            instance_path: format!(
                                "{}/{}",
                                content.get_content_json_path(i_v, metric_index),
                                prop_name
                            ),
                        });
                    }
                }
                if let Some(cvss3) = content.get_cvss_v3() {
                    if let Some(version) = cvss3.get("version").and_then(|v| v.as_str()) {
                        // Use as_str because otherwise additional quotation marks would be included
                        let value = serde_json::to_value(cvss3).unwrap();
                        let prop_name = get_metric_prop_name(VulnerabilityMetric::CvssV3(version.to_string()));
                        if version == "3.0" {
                            let evaluation = cvss30_validator.evaluate(&value);
                            for error in evaluation.iter_errors() {
                                errors.push(ValidationError {
                                    message: error.error.to_string(),
                                    instance_path: format!(
                                        "{}/{}",
                                        content.get_content_json_path(i_v, metric_index),
                                        prop_name
                                    ),
                                });
                            }
                        } else if version == "3.1" {
                            let evaluation = cvss31_validator.evaluate(&value);
                            for error in evaluation.iter_errors() {
                                errors.push(ValidationError {
                                    message: error.error.to_string(),
                                    instance_path: format!(
                                        "{}/{}",
                                        content.get_content_json_path(i_v, metric_index),
                                        prop_name
                                    ),
                                });
                            }
                        }
                    }
                }
                if let Some(cvss4) = content.get_cvss_v4() {
                    let value = serde_json::to_value(cvss4).unwrap();
                    let evaluation = cvss40_validator.evaluate(&value);
                    let prop_name = get_metric_prop_name(VulnerabilityMetric::CvssV4);
                    for error in evaluation.iter_errors() {
                        errors.push(ValidationError {
                            message: error.error.to_string(),
                            instance_path: format!(
                                "{}/{}",
                                content.get_content_json_path(i_v, metric_index),
                                prop_name
                            ),
                        });
                    }
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_08::test_6_1_08_invalid_cvss;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_08() {
        run_csaf20_tests(
            "08",
            test_6_1_08_invalid_cvss,
            HashMap::from([
                (
                    "01",
                    vec![ValidationError {
                        message: "\"baseSeverity\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/scores/0/cvss_v3".to_string(),
                    }],
                ),
                (
                    "02",
                    vec![ValidationError {
                        message: "\"baseSeverity\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/scores/0/cvss_v3".to_string(),
                    }],
                ),
                (
                    "03",
                    vec![ValidationError {
                        message: "\"version\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/scores/0/cvss_v2".to_string(),
                    }],
                ),
            ]),
        );
        /*
        run_csaf21_tests(
            "08",
            test_6_1_08_invalid_cvss,
            HashMap::from([
                (
                    "01",
                    vec![ValidationError {
                        message: "\"baseSeverity\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v3".to_string(),
                    }],
                ),
                (
                    "02",
                    vec![ValidationError {
                        message: "\"baseSeverity\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v3".to_string(),
                    }],
                ),
                (
                    "03",
                    vec![ValidationError {
                        message: "\"version\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v2".to_string(),
                    }],
                ),
                (
                    "04",
                    vec![ValidationError {
                        message: "\"baseSeverity\" is a required property".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    }],
                ),
                (
                    "05",
                    vec![
                        // ToDo
                    ],
                ),
                (
                    "06",
                    vec![
                        // ToDo
                    ],
                )
            ]),
        );
         */
    }
}
