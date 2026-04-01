use std::str::FromStr;

use cvss_rs::v2_0::CvssV2;
use serde_json::Value;

use super::{ScoreType, check_score_mismatch, create_vector_parse_error};
use crate::validation::ValidationError;
use cvss_rs::Version;

/// Validates CVSS v2 scores.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected scores,
/// which are then compared against the values declared in the JSON.
///
/// Checked fields: `baseScore`, `temporalScore`, `environmentalScore`
pub fn validate_scores(
    cvss2: &CvssV2,
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match CvssV2::from_str(&cvss2.vector_string) {
        Ok(p) => p,
        Err(e) => {
            errors.get_or_insert_default().push(create_vector_parse_error(
                &cvss2.vector_string,
                Version::V2,
                &e,
                instance_path,
            ));
            return;
        },
    };

    let score_checks: &[(&str, ScoreType, Option<f64>)] = &[
        ("baseScore", ScoreType::Base, parsed.calculated_base_score()),
        ("temporalScore", ScoreType::Temporal, parsed.calculated_temporal_score()),
        (
            "environmentalScore",
            ScoreType::Environmental,
            parsed.calculated_environmental_score(),
        ),
    ];

    for (json_key, score_type, calculated) in score_checks {
        if let Some(actual) = cvss_map.get(*json_key).and_then(|v| v.as_f64())
            && let Some(calculated) = calculated
        {
            check_score_mismatch(actual, *calculated, *score_type, instance_path, errors);
        }
    }
}
