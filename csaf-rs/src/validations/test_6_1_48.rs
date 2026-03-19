use ssvc::selection_list::SelectionList;
use ssvc::validation::SsvcError;
use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {error}"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2"),
    }
}

/// Test function for invocation by users, does not permit usage of the "test" namespace.
pub fn test_6_1_48_ssvc_decision_points(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    test_6_1_48_ssvc_decision_points_internal(doc, ssvc::validation::validate_selection_list)
}

/// Internal, actual test function allowing usage of a custom validation function, i.e.,
/// a function permitting the reserved "test" namespace for testing.
fn test_6_1_48_ssvc_decision_points_internal(
    doc: &impl CsafTrait,
    validation_fn: fn(&SelectionList) -> Result<(), Vec<SsvcError>>,
) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                if m.get_content().has_ssvc() {
                    match m.get_content().get_ssvc() {
                        Ok(ssvc) => {
                            if let Err(ssvc_errors) = validation_fn(&ssvc) {
                                let validation_errors: Vec<ValidationError> = ssvc_errors
                                    .into_iter()
                                    .map(|ssvc_error| {
                                        let path_suffix = ssvc_error.instance_path.join("/");
                                        ValidationError {
                                            message: ssvc_error.message,
                                            instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/{path_suffix}"),
                                        }
                                    })
                                    .collect();
                                return Err(validation_errors);
                            }
                        },
                        Err(err) => {
                            return Err(vec![create_invalid_ssvc_error(err, i_v, i_m)]);
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_48
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        // Use the internal validation function allowing usage of the "test" namespace.
        test_6_1_48_ssvc_decision_points_internal(doc, ssvc::validation::validate_selection_list_allow_test)
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validation::ValidationError;

    #[test]
    fn test_test_6_1_48() {
        let case_01 = Err(vec![ValidationError {
            message: "The SSVC decision point 'ssvc::Mission Impact' (version 1.0.0) doesn't have a value with key 'D'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/1".to_string(),
        }]);
        let case_02 = Err(vec![ValidationError {
            message: "Unknown SSVC decision point 'ssvc::SIs' with version '2.0.0'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0".to_string(),
        }]);
        let case_03 = Err(vec![ValidationError {
            message: "The values for SSVC decision point 'ssvc::Safety Impact' (version 2.0.0) are not in correct order".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/1".to_string(),
        }]);
        let case_04 = Err(vec![ValidationError {
            message: "Unknown SSVC decision point 'ssvc::SI' with version '1.9.7'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0".to_string(),
        }]);
        let case_05 = Err(vec![ValidationError {
            message: "The SSVC decision point 'cvss::Attack Complexity' (version 3.0.1) doesn't have a value with key 'E'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/0".to_string(),
        }]);
        let case_06 = Err(vec![ValidationError {
            message: "Unknown SSVC decision point 'cvss::E' with version '3.0.1'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0".to_string(),
        }]);
        let case_07 = Err(vec![ValidationError {
            message: "The SSVC decision point 'ssvc//.example.test#some-private-decision-point-collection::Safety Impact' (version 2.0.0) doesn't have a value with key 'S'".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/0".to_string(),
        }]);
        let case_08 = Err(vec![ValidationError {
            message: "The values for SSVC decision point 'ssvc//.example.test$en-GB::Safety Impact' (version 2.0.0) are not in correct order".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/2".to_string(),
        }]);
        let case_09 = Err(vec![ValidationError {
            message: "The values for SSVC decision point 'ssvc//.example.test$en-CA::Safety Impact' (version 2.0.0) are not in correct order".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/2".to_string(),
        }]);
        let case_21 = Err(vec![ValidationError {
            message: "Invalid SSVC namespace: Reserved namespace 'invalid' must not be used".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace".to_string(),
        }]);
        let case_16 = case_06.clone();
        let case_19 = Err(vec![ValidationError {
            message: "The values for SSVC decision point 'ssvc//.example.test$de-DE::Safety Impact' (version 2.0.0) are not in correct order".to_string(),
            instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/2".to_string(),
        }]);

        // Only CSAF 2.1 has this test, with 20 test cases (6 error cases, 14 success cases)
        TESTS_2_1.test_6_1_48.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            case_07,
            case_08,
            case_09,
            case_21,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            case_16, // no Exploit Maturity E v3.0.1
            Ok(()),
            Ok(()),
            case_19, // wrong order of translated keys "R" and "C"
            Ok(()),
        );
    }
}
