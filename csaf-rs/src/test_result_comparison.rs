//! Helper functions for comparing test results
//!
//! This module contains logic for comparing actual validation results against
//! expected results, with support for comparing validation errors while ignoring order.

use crate::validation::ValidationError;

/// Compare actual and expected test results.
///
/// Returns Ok(()) if results match, or Err with a detailed description if they don't.
///
/// When both actual and expected are errors, this function compares the error lists
/// ignoring the order of errors, but ensuring all expected errors are present.
pub fn compare_test_results(
    actual: &Result<(), Vec<ValidationError>>,
    expected: &Result<(), Vec<ValidationError>>,
    test_id: &str,
    case_num: &str,
) -> Result<(), String> {
    match (actual, expected) {
        (Ok(()), Ok(())) => {
            // Both pass - good
            Ok(())
        },
        (Err(actual_errs), Err(expected_errs)) => {
            // Both fail - compare errors ignoring order
            let mut errors: Vec<String> = Vec::new();
            if actual_errs.len() != expected_errs.len() {
                errors.push(format!(
                    "Test {} case {}: Error count mismatch - expected {} error(s) but got {}",
                    test_id,
                    case_num,
                    expected_errs.len(),
                    actual_errs.len()
                ));
            }

            // Check that all expected errors exist in actual errors (ignoring order)
            for expected_err in expected_errs {
                if !actual_errs.iter().any(|actual_err| {
                    actual_err.message == expected_err.message && actual_err.instance_path == expected_err.instance_path
                }) {
                    errors.push(format!(
                        "Test {} case {}: Expected error not found: '{}', path: '{}'",
                        test_id, case_num, expected_err.message, expected_err.instance_path
                    ));
                }
            }
            for actual_err in actual_errs {
                if !expected_errs.iter().any(|expected_err| {
                    expected_err.message == actual_err.message && expected_err.instance_path == actual_err.instance_path
                }) {
                    errors.push(format!(
                        "Test {} case {}: Found not expected error: '{}', path: '{}'",
                        test_id, case_num, actual_err.message, actual_err.instance_path
                    ));
                }
            }
            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.join("\n"))
            }
        },
        (Ok(()), Err(expected_errors)) => {
            let mut errors: Vec<String> = Vec::new();
            errors.push(format!(
                "Test {test_id} case {case_num}: Expected failure but validation passed."
            ));
            for err in expected_errors {
                errors.push(format!(
                    "Test {} case {}: Expected error: '{}', path: '{}'",
                    test_id, case_num, err.message, err.instance_path
                ));
            }
            Err(errors.join("\n"))
        },
        (Err(actual_errs), Ok(())) => {
            let mut errors: Vec<String> = Vec::new();
            errors.push(format!(
                "Test {} case {}: Expected success but validation failed with {} error(s).",
                test_id,
                case_num,
                actual_errs.len()
            ));
            for err in actual_errs {
                errors.push(format!(
                    "Test {} case {}: Not expected error: '{}', path: '{}'",
                    test_id, case_num, err.message, err.instance_path
                ));
            }
            Err(errors.join("\n"))
        },
    }
}
