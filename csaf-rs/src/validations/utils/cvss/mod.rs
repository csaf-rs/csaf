pub mod deserialize;
pub mod dispatch;
pub mod errors;
pub mod validate_consistency;
pub mod validate_scores;


use crate::csaf_traits::ContentTrait;
use crate::validation::TestFinding;

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
    dispatch::validate_content_metrics(content, instance_path, errors, dispatch::validate_scores);
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
    dispatch::validate_content_metrics(content, instance_path, errors, dispatch::validate_consistency);
}
