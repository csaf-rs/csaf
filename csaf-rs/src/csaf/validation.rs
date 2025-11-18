use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use TestResultStatus::{*};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub message: String,
    #[serde(rename = "instancePath")]
    pub instance_path: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationError: {} at {}",
            self.message, self.instance_path
        )
    }
}

/// Result of executing a single test
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    /// The test ID that was executed
    pub test_id: String,

    /// The status of the test execution
    pub status: TestResultStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestResultStatus {
    Success,
    Failure { errors: Vec<ValidationError> },
    NotFound,
}

/// Result of a CSAF validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    fn validate_by_test<VersionedDocument>(&self, test_id: &str) -> TestResult;

    /// Validates this object according to specific test IDs and returns detailed results
    fn validate_by_tests(
        &self,
        version: &str,
        preset: ValidationPreset,
        test_ids: &[&str],
    ) -> ValidationResult;

    /// Validates this object according to a validation preset and returns detailed results
    fn validate_by_preset(&self, version: &str, preset: ValidationPreset) -> ValidationResult;
}

pub type Test<VersionedDocument> = fn(&VersionedDocument) -> Result<(), ValidationError>;

/// Represents something which is validatable according to the CSAF standard.
/// This trait MUST be implemented by the struct that represents a CSAF document
/// in the respective version.
///
/// It can then be used to validate documents with [validate_by_preset], [validate_by_tests],
/// or [validate_by_test].
pub trait Validatable<VersionedDocument> {
    /// Returns a hashmap containing the test ID per preset
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>>;

    /// Returns a hashmap containing the test function per test ID
    fn tests(&self) -> HashMap<&str, Test<VersionedDocument>>;

    fn doc(&self) -> &VersionedDocument;
}

/// Execute a single test and return the test result.
///
/// This function will check, whether the test_id exists in the Validatable's
/// tests. If it does, it will execute the test function and return the result.
/// If not, it will return a TestResult indicating that the test was not found.
pub fn validate_by_test<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    test_id: &str,
) -> TestResult {
    // Fetch tests from the validatable
    let tests = target.tests();

    // Try to find and execute the test specified by the test_id
    let status = if let Some(test_fn) = tests.get(test_id) {
        match test_fn(target.doc()) {
            Ok(()) => Success,
            Err(error) => Failure {
                errors: vec![error],
            },
        }
    } else {
        NotFound
    };

    TestResult {
        test_id: test_id.to_string(),
        status,
    }
}

/// Validate document with specific tests and return detailed results.
pub fn validate_by_tests<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    version: &str,
    preset: ValidationPreset,
    test_ids: &[&str],
) -> ValidationResult {
    let mut test_results = Vec::new();
    let mut success = true;
    let mut num_errors: usize = 0;

    // Loop through tests and gather all results and errors
    for test_id in test_ids {
        let test_result = validate_by_test(target, test_id);
        if let Failure { errors } = &test_result.status {
            success = false;
            num_errors += errors.len();            
        }
        test_results.push(test_result);
    }

    ValidationResult {
        success: success,
        version: version.to_string(),
        num_errors: num_errors,
        preset,
        test_results,
    }
}

/// Validate document with a preset and return detailed results.
pub fn validate_by_preset<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    version: &str,
    preset: ValidationPreset,
) -> ValidationResult {
    // Retrieve the test IDs for the given preset
    let test_ids: Vec<&str> = target
        .presets()
        .get(&preset)
        .map(|ids| ids.iter().copied().collect())
        .unwrap_or_default();

    // Forward them to validate_by_tests
    validate_by_tests(target, version, preset, &test_ids)
}
