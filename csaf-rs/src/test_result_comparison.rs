//! Helper functions for comparing test results
//!
//! This module contains logic for comparing actual validation results against
//! expected results, with support for comparing validation errors while ignoring order.

use crate::validation::TestFinding;

/// Compare actual and expected test results.
///
/// Returns Ok(()) if results match, or Err with a detailed description if they don't.
///
/// When both actual and expected are errors, this function compares the error lists
/// ignoring the order of errors, but ensuring all expected errors are present.
pub fn compare_test_results(
    actual: &Result<(), Vec<TestFinding>>,
    expected: &Result<(), Vec<TestFinding>>,
    csaf_version: &str,
    test_id: &str,
    case_num: &str,
) -> Result<(), String> {
    match (actual, expected) {
        (Ok(()), Ok(())) => {
            // Both pass - good
            Ok(())
        },
        (Err(actual_findings), Err(expected_findings)) => {
            // Both fail - compare errors ignoring order
            let mut errors: Vec<String> = Vec::new();
            if actual_findings.len() != expected_findings.len() {
                errors.push(format!(
                    "CSAF {csaf_version}: Test {test_id} case {case_num}: Error count mismatch - expected {} error(s) but got {}",
                    expected_findings.len(),
                    actual_findings.len()
                ));
            }

            // Check that all expected errors exist in actual errors (ignoring order)
            for expected_finding in expected_findings {
                if !actual_findings
                    .iter()
                    .any(|actual_finding| match (actual_finding, expected_finding) {
                        (TestFinding::Error(actual), TestFinding::Error(expected)) => actual == expected,
                        (TestFinding::Warning(actual), TestFinding::Warning(expected)) => actual == expected,
                        (TestFinding::Information(actual), TestFinding::Information(expected)) => actual == expected,
                        _ => false,
                    })
                {
                    errors.push(format!(
                        "CSAF {csaf_version}: Test {test_id} case {case_num}: Expected error not found: '{}', path: '{}'",
                        expected_finding.get_data().message, expected_finding.get_data().instance_path
                    ));
                }
            }
            for actual_finding in actual_findings {
                if !expected_findings
                    .iter()
                    .any(|expected_finding| match (expected_finding, actual_finding) {
                        (TestFinding::Error(expected), TestFinding::Error(actual)) => expected == actual,
                        (TestFinding::Warning(expected), TestFinding::Warning(actual)) => expected == actual,
                        (TestFinding::Information(expected), TestFinding::Information(actual)) => expected == actual,
                        _ => false,
                    })
                {
                    errors.push(format!(
                        "CSAF {csaf_version}: Test {test_id} case {case_num}: Found unexpected error: '{}', path: '{}'",
                        actual_finding.get_data().message,
                        actual_finding.get_data().instance_path
                    ));
                }
            }
            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.join("\n"))
            }
        },
        (Ok(()), Err(expected_findings)) => {
            let mut errors: Vec<String> = Vec::new();
            errors.push(format!(
                "CSAF {csaf_version}: Test {test_id} case {case_num}: Expected failure but validation passed."
            ));
            for finding in expected_findings {
                errors.push(format!(
                    "CSAF {csaf_version}: Test {test_id} case {case_num}: Expected error: '{}', path: '{}'",
                    finding.get_data().message,
                    finding.get_data().instance_path
                ));
            }
            Err(errors.join("\n"))
        },
        (Err(actual_findings), Ok(())) => {
            let mut errors: Vec<String> = Vec::new();
            errors.push(format!(
                "CSAF {csaf_version}: Test {test_id} case {case_num}: Expected success but validation failed with {} error(s).",
                actual_findings.len()
            ));
            for finding in actual_findings {
                errors.push(format!(
                    "CSAF {csaf_version}: Test {test_id} case {case_num}: Not expected error: '{}', path: '{}'",
                    finding.get_data().message,
                    finding.get_data().instance_path
                ));
            }
            Err(errors.join("\n"))
        },
    }
}
