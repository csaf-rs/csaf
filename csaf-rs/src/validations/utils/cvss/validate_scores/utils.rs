use crate::validation::TestFinding;
use crate::validation::TestFindingData;
use crate::validations::utils::cvss::ScoreType;
use cvss_rs::v3::Severity as V3Severity;
use cvss_rs::{Severity as UnifiedSeverity, Severity};

/// Maps a CVSS v3 / v4 score to its severity rating.
///
/// The severity ranges follow the CVSS v3/v4 specification.
pub fn map_score_to_severity(score: Option<f64>) -> Option<UnifiedSeverity> {
    let scaled = (score? * 10.0).round() as i8;
    Some(match scaled {
        0 => UnifiedSeverity::None,
        1..=39 => UnifiedSeverity::Low,
        40..=69 => UnifiedSeverity::Medium,
        70..=89 => UnifiedSeverity::High,
        90..=100 => UnifiedSeverity::Critical,
        _ => return None,
    })
}

/// Converts a version-specific CVSS v3 [V3Severity] to the unified [Severity].
/// The [CvssV3] implementation does not provide this only for base_score.
pub fn to_unified(severity: &V3Severity) -> UnifiedSeverity {
    match severity {
        V3Severity::None => UnifiedSeverity::None,
        V3Severity::Low => UnifiedSeverity::Low,
        V3Severity::Medium => UnifiedSeverity::Medium,
        V3Severity::High => UnifiedSeverity::High,
        V3Severity::Critical => UnifiedSeverity::Critical,
    }
}

pub fn check_optional_score_mismatch(
    actual: Option<f64>,
    calculated: Option<f64>,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    if let (Some(actual), Some(calculated)) = (actual, calculated)
        && (actual * 10.0).round() as i8 != (calculated * 10.0).round() as i8
    {
        errors
            .get_or_insert_default()
            .push(TestFinding::Error(create_score_mismatch_finding_data(
                calculated,
                actual,
                score_type,
                instance_path,
            )));
    }
}

pub fn create_score_mismatch_finding_data(
    calculated: f64,
    actual: f64,
    score_type: ScoreType,
    instance_path: &str,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "{score_type} score does not match the expected value calculated from the vector. \
             Expected: {calculated}, found: {actual}"
        ),
        instance_path: instance_path.to_string(),
    }
}

pub fn check_optional_severity_mismatch(
    actual: Option<UnifiedSeverity>,
    calculated: Option<UnifiedSeverity>,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    if let (Some(actual), Some(calculated)) = (actual, calculated)
        && actual != calculated
    {
        errors
            .get_or_insert_default()
            .push(TestFinding::Error(create_severity_mismatch_finding_data(
                &calculated,
                &actual,
                score_type,
                instance_path,
            )));
    }
}

pub fn create_severity_mismatch_finding_data(
    calculated: &Severity,
    actual: &Severity,
    score_type: ScoreType,
    instance_path: &str,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "{score_type} severity does not match the expected value calculated from the vector. \
             Expected: {calculated:?}, found: {actual:?}"
        ),
        instance_path: instance_path.to_string(),
    }
}
