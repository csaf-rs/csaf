use serde::Serialize;

/// This represents the result of a test run for a single document.
#[derive(Serialize)]
pub struct ValidationResult {
    /// overall result, either valid, invalid, or undetermined (e.g. if no tests were executed)
    pub document_status: DocumentStatus,
    /// the CSAF spec tested against
    pub csaf_version: String,
    /// overview over the individual tests (x passed, y failed, runtime etc.)
    pub stats: ValidationSummary,
    /// the individual results
    pub results: Vec<TestRunResult>,
}

/// The final status of the document after running all tests. This is determined by the individual test results and their severity.
#[derive(Serialize, PartialEq)]
pub enum DocumentStatus {
    Undetermined, // cannot determine validity (e.g. no tests executed)
    Valid,        // file is valid
    Invalid,      // file is invalid
}

/// Summary of the test results, including counts of total tests, passed, failed, skipped, and not found.
#[derive(Serialize)]
pub struct ValidationSummary {
    pub total: Statistic,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub not_found: usize,
}

/// Statistic for a group of tests, including the count of tests and counts of errors, warnings, and info findings.
#[derive(Serialize)]
pub struct Statistic {
    pub test_count: usize,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub info_count: usize,
}

/// Result of a single test run, including the test ID and the result.
#[derive(Serialize)]
pub struct TestRunResult {
    pub test_id: String,
    pub result: TestResult,
}

/// The result of a single test, which can be passed, failed, skipped, or not found. Failed tests include a list of findings with their severity and details.
#[derive(Serialize, PartialEq)]
pub enum TestResult {
    Passed(Passed),
    Failed(Vec<TestFinding>),
    Skipped(Skipped),
    NotFound,
}

/// Classification of a test finding.
#[derive(Serialize, PartialEq)]
#[serde(tag = "severity")]
pub enum TestFinding {
    Infomation(TestFindingData),
    Warning(TestFindingData),
    Error(TestFindingData),
}

/// The reason why a test was skipped.
#[derive(Serialize, PartialEq)]
#[serde(tag = "reason")]
pub enum Skipped {
    DeserializationFailed { message: String },
    PreconditionFailed { test_id: String, message: String },
}

/// The reason why a test passed.
#[derive(Serialize, PartialEq)]
#[serde(tag = "reason")]
pub enum Passed {
    Success,
    NoData { message: String },
    NotApplicable { message: String },
}

/// The details of a test finding, including the message and the instance path where the finding occurred.
#[derive(Serialize, PartialEq)]
pub struct TestFindingData {
    pub message: String,
    pub instance_path: String,
}
