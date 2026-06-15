use crate::result::{self, ResultJson, ResultT, ValidationMessageT};
use anyhow::Result;
use csaf::validation::TestResultStatus::{Failure, NotFound, Skipped, Success};
use csaf::validation::{TestResult, ValidationError, ValidationResult};

/// Build a [`ResultJson`] from a [`ValidationResult`] with the given primary test ID.
pub fn build_testresult_json(result: &ValidationResult, primary_test_id: &str) -> Result<ResultJson> {
    let primary_test = result
        .test_results
        .iter()
        .find(|r| r.test_id == primary_test_id)
        .ok_or_else(|| anyhow::anyhow!("Test '{primary_test_id}' not found in validation results"))?;

    let primary_result = convert_result(primary_test).ok_or_else(|| {
        anyhow::anyhow!("Cannot create result for test '{primary_test_id}': test was not found/skipped")
    })?;

    let secondary_results: Vec<ResultT> = result
        .test_results
        .iter()
        .filter(|r| r.test_id != primary_test_id)
        .filter_map(convert_result)
        .filter(|r| !r.passed)
        .collect();

    Ok(ResultJson {
        schema: result::RESULT_JSON_SCHEMA.to_string(),
        resultschema_version: "2.1".to_string(),
        overall_valid: result.success,
        primary_result,
        secondary_results: if secondary_results.is_empty() {
            None
        } else {
            Some(secondary_results)
        },
    })
}

/// Convert a [`TestResult`] to a [`ResultT`], returning `None` for tests that were not found or skipped.
fn convert_result(test_result: &TestResult) -> Option<ResultT> {
    match &test_result.status {
        Success => Some(ResultT {
            id: test_result.test_id.clone(),
            passed: true,
            errors: None,
            warnings: None,
            infos: None,
        }),
        Failure {
            errors,
            warnings,
            infos,
        } => Some(ResultT {
            id: test_result.test_id.clone(),
            passed: false,
            errors: convert_messages(errors),
            warnings: convert_messages(warnings),
            infos: convert_messages(infos),
        }),
        NotFound | Skipped => None,
    }
}

/// Convert a slice of [`ValidationError`]s to a [`Vec<ValidationMessageT>`], or `None` if empty.
fn convert_messages(errors: &[ValidationError]) -> Option<Vec<ValidationMessageT>> {
    if errors.is_empty() {
        return None;
    }
    let messages: Vec<ValidationMessageT> = errors
        .iter()
        .filter_map(|e| {
            Some(ValidationMessageT {
                instance_path: e.instance_path.clone(),
                message: e.message.parse().ok()?,
            })
        })
        .collect();
    if messages.is_empty() { None } else { Some(messages) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csaf::validation::{TestResult, TestResultStatus, ValidationError, ValidationResult};

    fn make_error(path: &str, msg: &str) -> ValidationError {
        ValidationError {
            instance_path: path.to_string(),
            message: msg.to_string(),
        }
    }

    // --- convert_messages ---

    #[test]
    fn convert_messages_empty_returns_none() {
        assert_eq!(convert_messages(&[]), None);
    }

    #[test]
    fn convert_messages_converts_correctly() {
        let errors = vec![make_error("/foo/bar", "something went wrong")];
        let result = convert_messages(&errors).expect("should produce Some");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].instance_path, "/foo/bar");
        assert_eq!(result[0].message, "something went wrong");
    }

    // --- convert_result ---

    #[test]
    fn convert_result_success() {
        let tr = TestResult {
            test_id: "6.1.1".to_string(),
            status: TestResultStatus::Success,
        };
        let result = convert_result(&tr).expect("Success should yield Some");
        assert_eq!(result.id, "6.1.1");
        assert!(result.passed);
        assert!(result.errors.is_none());
        assert!(result.warnings.is_none());
        assert!(result.infos.is_none());
    }

    #[test]
    fn convert_result_failure_with_errors() {
        let tr = TestResult {
            test_id: "6.1.2".to_string(),
            status: TestResultStatus::Failure {
                errors: vec![make_error("/product_tree", "missing field")],
                warnings: vec![],
                infos: vec![],
            },
        };
        let result = convert_result(&tr).expect("Failure should yield Some");
        assert_eq!(result.id, "6.1.2");
        assert!(!result.passed);
        let errors = result.errors.expect("errors should be Some");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].instance_path, "/product_tree");
    }

    #[test]
    fn convert_result_not_found_returns_none() {
        let tr = TestResult {
            test_id: "6.1.3".to_string(),
            status: TestResultStatus::NotFound,
        };
        assert!(convert_result(&tr).is_none());
    }

    #[test]
    fn convert_result_skipped_returns_none() {
        let tr = TestResult {
            test_id: "6.1.4".to_string(),
            status: TestResultStatus::Skipped,
        };
        assert!(convert_result(&tr).is_none());
    }

    // --- build_testresult_json ---

    #[test]
    fn build_testresult_json_primary_not_found_is_error() {
        let result = ValidationResult {
            success: true,
            version: "2.1".to_string(),
            test_results: vec![],
            num_errors: 0,
            num_warnings: 0,
            num_infos: 0,
            num_not_found: 0,
        };
        assert!(build_testresult_json(&result, "6.1.1").is_err());
    }

    #[test]
    fn build_testresult_json_success_no_secondary() {
        let result = ValidationResult {
            success: true,
            version: "2.1".to_string(),
            test_results: vec![TestResult {
                test_id: "6.1.1".to_string(),
                status: TestResultStatus::Success,
            }],
            num_errors: 0,
            num_warnings: 0,
            num_infos: 0,
            num_not_found: 0,
        };
        let json = build_testresult_json(&result, "6.1.1").expect("should succeed");
        assert!(json.overall_valid);
        assert_eq!(json.primary_result.id, "6.1.1");
        assert!(json.primary_result.passed);
        assert!(json.secondary_results.is_none());
    }

    #[test]
    fn build_testresult_json_failed_secondary_included() {
        let result = ValidationResult {
            success: false,
            version: "2.1".to_string(),
            test_results: vec![
                TestResult {
                    test_id: "6.1.1".to_string(),
                    status: TestResultStatus::Success,
                },
                TestResult {
                    test_id: "6.1.2".to_string(),
                    status: TestResultStatus::Failure {
                        errors: vec![make_error("/x", "bad")],
                        warnings: vec![],
                        infos: vec![],
                    },
                },
            ],
            num_errors: 1,
            num_warnings: 0,
            num_infos: 0,
            num_not_found: 0,
        };
        let json = build_testresult_json(&result, "6.1.1").expect("should succeed");
        assert!(!json.overall_valid);
        let secondaries = json.secondary_results.expect("should have secondaries");
        assert_eq!(secondaries.len(), 1);
        assert_eq!(secondaries[0].id, "6.1.2");
        assert!(!secondaries[0].passed);
    }

    #[test]
    fn build_testresult_json_passed_secondary_excluded() {
        let result = ValidationResult {
            success: true,
            version: "2.1".to_string(),
            test_results: vec![
                TestResult {
                    test_id: "6.1.1".to_string(),
                    status: TestResultStatus::Success,
                },
                TestResult {
                    test_id: "6.1.2".to_string(),
                    status: TestResultStatus::Success,
                },
            ],
            num_errors: 0,
            num_warnings: 0,
            num_infos: 0,
            num_not_found: 0,
        };
        let json = build_testresult_json(&result, "6.1.1").expect("should succeed");
        // Passing secondary results are excluded
        assert!(json.secondary_results.is_none());
    }
}
