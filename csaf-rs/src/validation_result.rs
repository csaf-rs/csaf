use serde::Serialize;

/// This represents the result of a test run for a single document.
#[derive(Serialize)]
pub struct ValidationResult {
    /// overall result, either valid, invalid, or undetermined (e.g. if no tests were executed)
    pub document_status: DocumentStatus,
    /// the CSAF spec tested against
    pub csaf_version: String,
    /// overview over the individual tests (x passed, y failed etc.)
    pub summary: ValidationSummary,
    /// the individual results
    pub results: Vec<TestRunResult>,
}

/// The final status of the document after running all tests. This is determined by the individual test results and their severity.
#[derive(Serialize, PartialEq)]
pub enum DocumentStatus {
    /// Cannot determine validity (e.g. no tests executed)
    Undetermined,
    /// File is valid
    Valid,
    /// File is invalid
    Invalid,
}

/// Summary of the test results, including counts of total tests, passed, failed, skipped, and not found.
#[derive(Serialize)]
pub struct ValidationSummary {
    /// Total number of tests along with counts of errors, warnings, and info findings
    pub total: Statistic,
    /// Number of passed tests
    pub passed: usize,
    /// Number of failed tests
    pub failed: usize,
    /// Number of skipped tests
    pub skipped: usize,
    /// Number of tests that were not found
    pub not_found: usize,
}

/// Statistic for a group of tests, including the count of tests and counts of errors, warnings, and info findings.
#[derive(Serialize)]
pub struct Statistic {
    /// Total number of tests
    pub test_count: usize,
    /// Number of errors
    pub errors_count: usize,
    /// Number of warnings
    pub warnings_count: usize,
    /// Number of informational findings
    pub info_count: usize,
}

/// Result of a single test run, including the test ID and the result.
#[derive(Serialize)]
pub struct TestRunResult {
    /// The ID of the test that was run.
    pub test_id: String,
    /// The result of the test run.
    pub result: TestResult,
}

/// The result of a single test, which can be passed, failed, skipped, or not found. Failed tests include a list of findings with their severity and details.
#[derive(Serialize, PartialEq)]
pub enum TestResult {
    /// The test passed successfully.
    Passed(Passed),
    /// The test failed, along with a list of findings that explain the reasons for failure.
    Failed(Vec<TestFinding>),
    /// The test was skipped, along with the reason for skipping.
    Skipped(Skipped),
    /// The test was not found.
    NotFound,
}

/// Classification of a test finding.
#[derive(Serialize, PartialEq)]
#[serde(tag = "severity")]
pub enum TestFinding {
    /// An information indicates a failure in an informative test, which does not necessarily mean the document is invalid, 
    /// but may provide insights into common mistakes or bad practices.
    Infomation(TestFindingData),
    /// A warning indicates a failure in a recommended test, which does not necessarily mean the document is invalid. 
    /// However, it may indicate potential issues or areas for improvement in the document. 
    Warning(TestFindingData),
    /// An error indicates a failure in a mandatory test, which means the document is invalid. 
    Error(TestFindingData),
}

/// The reason why a test was skipped.
#[derive(Serialize, PartialEq)]
#[serde(tag = "reason")]
pub enum Skipped {
    /// The test was skipped because the document could not be deserialized, which is a prerequisite for running the test.
    DeserializationFailed { message: String },
    /// The test was skipped because a precondition for the test was not met, such as missing required data or an unsupported feature in the document.
    PreconditionFailed { test_id: String, message: String },
}

/// The reason why a test passed.
#[derive(Serialize, PartialEq)]
#[serde(tag = "reason")]
pub enum Passed {
    /// The test passed successfully without any issues.
    Success,
    /// The test passed because there was no data to test against, such as an empty list.
    NoData { message: String },
    /// The test passed because it is not applicable to the document being tested, such as only being relevant for another document category.
    NotApplicable { message: String },
}

/// The details of a test finding, including the message and the instance path where the finding occurred.
#[derive(Serialize, PartialEq)]
pub struct TestFindingData {
    /// A message describing the finding, which can provide details about the issue or information related to the test result.
    pub message: String,
    /// The instance path in the document where the finding occurred, which can help identify the specific location of the issue or information in the document structure.
    pub instance_path: String,
}
