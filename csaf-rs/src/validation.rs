use TestResultStatus::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tsify::Tsify;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub message: String,
    pub instance_path: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationError: {} at {}", self.message, self.instance_path)
    }
}

/// Result of executing a single test
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    /// The test ID that was executed
    pub test_id: String,

    /// The status of the test execution
    pub status: TestResultStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum TestResultStatus {
    Success,
    Failure {
        errors: Vec<ValidationError>,
        warnings: Vec<ValidationError>,
        infos: Vec<ValidationError>,
    },
    NotFound,
    Skipped,
}

/// Result of a CSAF validation
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ValidationResult {
    /// Whether the validation was successful (no errors)
    pub success: bool,
    /// The detected CSAF version
    pub version: String,
    /// The validation preset that was used
    pub preset: ValidationPreset,
    /// Individual test results with execution details
    pub test_results: Vec<TestResult>,
    /// The total number of errors found during validation
    pub num_errors: usize,
    /// The total number of warnings found during validation
    pub num_warnings: usize,
    /// The total number of infos found during validation
    pub num_infos: usize,
    /// The total number of tests not found
    pub num_not_found: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum ValidationPreset {
    Basic,
    Extended,
    Full,
}

impl FromStr for ValidationPreset {
    type Err = ();

    fn from_str(input: &str) -> Result<ValidationPreset, Self::Err> {
        match input {
            "basic" => Ok(ValidationPreset::Basic),
            "extended" => Ok(ValidationPreset::Extended),
            "full" => Ok(ValidationPreset::Full),
            _ => Err(()),
        }
    }
}

impl Display for ValidationPreset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Extended => write!(f, "extended"),
            Self::Full => write!(f, "full"),
        }
    }
}

pub trait Validate {
    /// Validates this object according to
    fn validate_by_test(&self, test_id: &str) -> TestResult;

    /// Validates this object according to specific test IDs and returns detailed results
    fn validate_by_tests(&self, version: &str, preset: ValidationPreset, test_ids: &[&str]) -> ValidationResult;

    /// Validates this object according to a validation preset and returns detailed results
    fn validate_by_preset(&self, version: &str, preset: ValidationPreset) -> ValidationResult;
}

/// Represents something which is validatable according to the CSAF standard.
/// This trait MUST be implemented by the struct that represents a CSAF document
/// in the respective version.
///
/// It can then be used to validate documents with [validate_by_preset] or [validate_by_tests].
pub trait Validatable {
    /// Returns the test IDs belonging to a preset
    fn tests_in_preset(preset: &ValidationPreset) -> Vec<&str>;

    /// Runs a test by test ID
    fn run_test(&self, test_id: &str) -> TestResult;
}

/// Execute a single test and return the test result.
///
/// This function will check, whether the test_id exists in the Validatable's
/// tests. If it does, it will execute the test function and return the result.
/// If not, it will return a TestResult indicating that the test was not found.
pub fn validate_by_test(target: &impl Validatable, test_id: &str) -> TestResult {
    // Try to execute the test specified by the test_id
    target.run_test(test_id)
}

/// Validate document with specific tests and return detailed results.
pub fn validate_by_tests(
    target: &impl Validatable,
    version: &str,
    preset: ValidationPreset,
    test_ids: &[&str],
) -> ValidationResult {
    let mut test_results = Vec::new();
    let mut num_errors: usize = 0;
    let mut num_warnings: usize = 0;
    let mut num_infos: usize = 0;
    let mut num_not_found: usize = 0;

    // Loop through tests and gather all results and errors
    for test_id in test_ids {
        let test_result = validate_by_test(target, test_id);
        match &test_result.status {
            Failure {
                errors,
                warnings,
                infos,
            } => {
                num_errors += errors.len();
                num_warnings += warnings.len();
                num_infos += infos.len();
            },
            NotFound => {
                num_not_found += 1;
            },
            _ => {},
        }
        test_results.push(test_result);
    }

    ValidationResult {
        success: num_errors == 0,
        version: version.to_string(),
        num_errors,
        num_warnings,
        num_infos,
        num_not_found,
        preset,
        test_results,
    }
}

/// Validate document with a preset and return detailed results.
pub fn validate_by_preset<V: Validatable>(target: &V, version: &str, preset: ValidationPreset) -> ValidationResult {
    // Retrieve the test IDs for the given preset
    let test_ids: Vec<&str> = V::tests_in_preset(&preset);

    // Forward them to validate_by_tests
    validate_by_tests(target, version, preset, &test_ids)
}
