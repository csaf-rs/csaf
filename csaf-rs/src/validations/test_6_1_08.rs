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
                    let instance_path = create_instance_path(&instance_prefix, VulnerabilityMetric::CvssV2);
                    evaluate_cvss(cvss2, &cvss20_validator, instance_path.clone(), &mut errors);
                }
                if let Some(cvss3) = content.get_cvss_v3() {
                    // Use as_str because otherwise additional quotation marks would be included
                    if let Some(version) = cvss3.get("version").and_then(|v| v.as_str()) {
                        let instance_path =
                            create_instance_path(&instance_prefix, VulnerabilityMetric::CvssV3(version.to_string()));
                        if version == "3.0" {
                            evaluate_cvss(cvss3, &cvss30_validator, instance_path.clone(), &mut errors);
                        } else if version == "3.1" {
                            evaluate_cvss(cvss3, &cvss31_validator, instance_path.clone(), &mut errors);
                        }
                    }
                }
                if let Some(cvss4) = content.get_cvss_v4() {
                    let instance_path = create_instance_path(&instance_prefix, VulnerabilityMetric::CvssV4);
                    evaluate_cvss(cvss4, &cvss40_validator, instance_path.clone(), &mut errors);
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

fn create_instance_path(base: &str, metric: VulnerabilityMetric) -> String {
    let prop_name = get_metric_prop_name(metric);
    format!("{}/{}", base, prop_name)
}

fn evaluate_cvss(
    cvss_value: &Map<String, Value>,
    validator: &Validator,
    instance_path: String,
    errors: &mut Vec<ValidationError>,
) {
    let value = serde_json::to_value(cvss_value).unwrap();
    let evaluation = validator.evaluate(&value);
    for error in evaluation.iter_errors() {
        errors.push(ValidationError {
            message: error.error.to_string(),
            instance_path: instance_path.clone(),
        });
    }
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
        // 2.1 tests are not valid at the moment
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
                    vec![ValidationError {
                        message: "Unevaluated properties are not allowed ('threatScore', 'threatSeverity' were unexpected)".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow \"CRITICAL\"".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow 9.3".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    }],
                ),
                (
                    "06",
                    vec![ValidationError {
                        message: "Unevaluated properties are not allowed ('environmentalScore', 'environmentalSeverity', 'threatScore', 'threatSeverity' were unexpected)".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow \"CRITICAL\"".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow \"MEDIUM\"".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow 9.3".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    },
                    ValidationError {
                        message: "False schema does not allow 5.4".to_string(),
                        instance_path: "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
                    }],
                )
            ]),
        );
    }
}
