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
            if actual_errs.len() != expected_errs.len() {
                return Err(format!(
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
                    return Err(format!(
                        "Test {} case {}: Expected error not found - message: '{}', path: '{}'",
                        test_id, case_num, expected_err.message, expected_err.instance_path
                    ));
                }
            }
            Ok(())
        },
        (Ok(()), Err(_)) => Err(format!(
            "Test {} case {}: Expected failure but validation passed",
            test_id, case_num
        )),
        (Err(actual_errs), Ok(())) => Err(format!(
            "Test {} case {}: Expected success but validation failed with {} error(s)",
            test_id,
            case_num,
            actual_errs.len()
        )),
    }
}
