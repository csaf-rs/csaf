pub mod check_nonzero;
pub mod deserialize_json;
pub mod deserialize_vector;
pub mod dispatch;
pub mod merge_vector_with_json;
pub mod validate_consistency;
pub mod validate_scores;

use crate::csaf_traits::ContentTrait;
use crate::validation::TestFinding;
use crate::validations::utils::cvss::dispatch::validate_content_metrics;
use crate::validations::utils::cvss::validate_consistency::validate_consistency;
use crate::validations::utils::cvss::validate_scores::validate_scores;
use cvss_rs::Version;
use strum::{AsRefStr, Display};

/// The type of CVSS score being validated, use for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
}

/// Validates the CVSS scores (for v2, v3.x and v4) and severities (for v3.x and v4) of all CVSS metrics (v2, v3.x and v4) present
/// in the vulnerability scores / metric content object.
///
/// For each present metric, the scores and severities stated in the JSON object are compared
/// against the values calculated from its vector string. Versions that are absent are skipped.
///
/// # Arguments
///
/// * `content` - The vulnerability metric content holding the CVSS JSON objects.
/// * `instance_path` - JSON path of `content`, used for the location of reported findings.
/// * `errors` - Collector for findings; lazily initialized to `Some` when the first finding is added.
pub fn validate_content_scores(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    validate_content_metrics(content, instance_path, errors, validate_scores);
}

/// Validates the consistency of all CVSS metrics (v2, v3.x, v4) present in `content`.
///
/// For each present metric, the individual properties of the JSON object (e.g. base metrics) are
/// compared against the values parsed from its vector string. Versions that are absent are skipped.
///
/// # Arguments
///
/// * `content` - The vulnerability metric content holding the CVSS JSON objects.
/// * `instance_path` - JSON path of `content`, used for the location of reported findings.
/// * `errors` - Collector for findings; lazily initialized to `Some` when the first finding is added.
pub fn validate_content_consistency(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    validate_content_metrics(content, instance_path, errors, validate_consistency);
}

pub enum CvssScoreNonZeroCheckResult {
    PropertyCvssScoreIsZero,
    PropertyCvssScoreNonZero(f64),
    CalculatedCvssScoreIsZero,
    CalculatedCvssScoreNonZero(f64),
    ParsingFailed,
    CalculationFailed,
}

pub fn get_content_nonzero(content: &impl ContentTrait) -> Vec<(Version, CvssScoreNonZeroCheckResult)> {
    let mut failing_check_results = Vec::new();

    for (metric_type, result) in [
        (
            Version::V2,
            content
                .get_cvss_v2_typed()
                .map(|result| result.map(check_nonzero::validation::cvss_v2_has_env_score_zero)),
        ),
        (
            Version::V3_0,
            content
                .get_cvss_v3_typed()
                .map(|result| result.map(check_nonzero::validation::cvss_v3_has_env_score_zero)),
        ),
        (
            Version::V4,
            content
                .get_cvss_v4_typed()
                .map(|result| result.map(check_nonzero::validation::cvss_v4_has_score_zero)),
        ),
    ] {
        let Some(result) = result else {
            continue;
        };
        // This unwrap_or_else handles cvss_vX_typed returning an error
        let results = result.unwrap_or_else(|_| vec![CvssScoreNonZeroCheckResult::ParsingFailed]);

        // map in the version
        failing_check_results.extend(results.into_iter().map(|result| (metric_type, result)));
    }

    failing_check_results
}
