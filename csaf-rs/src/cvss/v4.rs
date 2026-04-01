use std::str::FromStr;

use cvss_rs::Version;
use cvss_rs::v4_0::CvssV4;

use super::{
    ScoreType, check_score_mismatch, check_severity_mismatch, create_vector_parse_error, map_score_to_severity,
};
use crate::validation::ValidationError;

/// Validates CVSS v4 base score and base severity.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected score and severity,
/// which are then compared against the values declared in the JSON.
///
/// Checked fields: `baseScore`, `baseSeverity`
pub fn validate_scores(cvss4: &CvssV4, instance_path: &str, errors: &mut Option<Vec<ValidationError>>) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match CvssV4::from_str(&cvss4.vector_string) {
        Ok(p) => p,
        Err(e) => {
            errors.get_or_insert_default().push(create_vector_parse_error(
                &cvss4.vector_string,
                Version::V4,
                &e,
                instance_path,
            ));
            return;
        },
    };

    let calculated_full = parsed.calculated_full_score();

    // Validate base score (using full score which includes threat metric E if present)
    if let Some(calculated) = calculated_full {
        check_score_mismatch(cvss4.base_score, calculated, ScoreType::Base, instance_path, errors);
    }

    // Validate base severity using base_severity() which converts to unified Severity
    if let Some(calculated) = map_score_to_severity(calculated_full)
        && let Some(actual) = cvss4.base_severity()
    {
        check_severity_mismatch(&actual, &calculated, ScoreType::Base, instance_path, errors);
    }
}
