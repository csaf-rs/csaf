use std::str::FromStr;

use cvss_rs::Severity;
use cvss_rs::Version;
use cvss_rs::v3::{CvssV3, Severity as V3Severity};

use super::{
    ScoreType, check_score_mismatch, check_severity_mismatch, create_vector_parse_error, map_score_to_severity,
};
use crate::validation::ValidationError;

/// Converts a version-specific CVSS v3 [V3Severity] to the unified [Severity].
/// The [CvssV3] implementation does not provide this only for base_score.
fn to_unified(severity: &V3Severity) -> Severity {
    match severity {
        V3Severity::None => Severity::None,
        V3Severity::Low => Severity::Low,
        V3Severity::Medium => Severity::Medium,
        V3Severity::High => Severity::High,
        V3Severity::Critical => Severity::Critical,
    }
}

/// Validates CVSS v3 scores and severities.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected scores
/// and severities, which are then compared against the values declared in the JSON.
///
/// Checked score fields: `baseScore`, `temporalScore`, `environmentalScore`
/// Checked severity fields: `baseSeverity`, `temporalSeverity`, `environmentalSeverity`
pub fn validate_scores(cvss3: &CvssV3, instance_path: &str, errors: &mut Option<Vec<ValidationError>>) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match CvssV3::from_str(&cvss3.vector_string) {
        Ok(p) => p,
        Err(e) => {
            errors.get_or_insert_default().push(create_vector_parse_error(
                &cvss3.vector_string,
                Version::V3_0,
                &e,
                instance_path,
            ));
            return;
        },
    };

    let calculated_base = parsed.calculated_base_score();
    let calculated_temporal = parsed.calculated_temporal_score();
    let calculated_environmental = parsed.calculated_environmental_score();

    // Validate scores
    if let Some(calculated) = calculated_base {
        check_score_mismatch(cvss3.base_score, calculated, ScoreType::Base, instance_path, errors);
    }
    if let Some(actual) = cvss3.temporal_score
        && let Some(calculated) = calculated_temporal
    {
        check_score_mismatch(actual, calculated, ScoreType::Temporal, instance_path, errors);
    }
    if let Some(actual) = cvss3.environmental_score
        && let Some(calculated) = calculated_environmental
    {
        check_score_mismatch(actual, calculated, ScoreType::Environmental, instance_path, errors);
    }

    // Validate severities
    if let Some(calculated) = map_score_to_severity(calculated_base) {
        check_severity_mismatch(
            &to_unified(&cvss3.base_severity),
            &calculated,
            ScoreType::Base,
            instance_path,
            errors,
        );
    }
    if let Some(actual) = &cvss3.temporal_severity
        && let Some(calculated) = map_score_to_severity(calculated_temporal)
    {
        check_severity_mismatch(
            &to_unified(actual),
            &calculated,
            ScoreType::Temporal,
            instance_path,
            errors,
        );
    }
    if let Some(actual) = &cvss3.environmental_severity
        && let Some(calculated) = map_score_to_severity(calculated_environmental)
    {
        check_severity_mismatch(
            &to_unified(actual),
            &calculated,
            ScoreType::Environmental,
            instance_path,
            errors,
        );
    }
}
