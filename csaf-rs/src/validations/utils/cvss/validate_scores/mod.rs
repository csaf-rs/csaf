use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::cvss::errors::ScoreType;

pub mod validation;

/// Compares an actual score against a calculated score and adds a validation error if they differ.
pub fn check_score_mismatch(
    actual: f64,
    calculated: f64,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    // compare scores as scaled integers
    if (actual * 10.0).round() as i8 != (calculated * 10.0).round() as i8 {
        errors.get_or_insert_default().push(create_score_mismatch_error(
            calculated,
            actual,
            score_type,
            instance_path,
        ));
    }
}

pub fn create_score_mismatch_error(
    calculated: f64,
    actual: f64,
    score_type: ScoreType,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "{score_type} score does not match the expected value calculated from the vector. \
             Expected: {calculated}, found: {actual}"
        ),
        instance_path: instance_path.to_string(),
    })
}
