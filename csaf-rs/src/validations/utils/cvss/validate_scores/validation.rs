use std::str::FromStr;

use crate::validation::TestFinding;
use crate::validations::utils::cvss::errors::{ScoreType, create_vector_parse_error};
use crate::validations::utils::cvss::validate_scores::utils::{
    check_score_mismatch, check_severity_mismatch, map_score_to_severity, to_unified,
};
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::{CvssV3};
use cvss_rs::v4_0::CvssV4;
use cvss_rs::{Version};
use serde_json::Value;

/// Validates CVSS v2 scores.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected scores,
/// which are then compared against the values declared in the JSON.
///
/// Checked fields: `baseScore`, `temporalScore`, `environmentalScore`
pub fn validate_v2_scores(
    cvss2: &CvssV2,
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
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
            if let Some(mismatch) = check_score_mismatch(actual, *calculated, *score_type, instance_path) {
                errors.get_or_insert_default().push(TestFinding::Error(mismatch));
            }
        }
    }
}

/// Validates CVSS v3 scores and severities.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected scores
/// and severities, which are then compared against the values declared in the JSON.
///
/// Checked score fields: `baseScore`, `temporalScore`, `environmentalScore`
/// Checked severity fields: `baseSeverity`, `temporalSeverity`, `environmentalSeverity`
pub fn validate_v3_scores(cvss3: &CvssV3, instance_path: &str, errors: &mut Option<Vec<TestFinding>>) {
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
        if let Some(mismatch) = check_score_mismatch(cvss3.base_score, calculated, ScoreType::Base, instance_path) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
    if let Some(actual) = cvss3.temporal_score
        && let Some(calculated) = calculated_temporal
    {
        if let Some(mismatch) = check_score_mismatch(actual, calculated, ScoreType::Temporal, instance_path) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
    if let Some(actual) = cvss3.environmental_score
        && let Some(calculated) = calculated_environmental
    {
        if let Some(mismatch) = check_score_mismatch(actual, calculated, ScoreType::Environmental, instance_path) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }

    // Validate severities
    if let Some(calculated) = map_score_to_severity(calculated_base) {
        if let Some(mismatch) = check_severity_mismatch(
            &to_unified(&cvss3.base_severity),
            &calculated,
            ScoreType::Base,
            instance_path,
        ) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
    if let Some(actual) = &cvss3.temporal_severity
        && let Some(calculated) = map_score_to_severity(calculated_temporal)
    {
        if let Some(mismatch) =
            check_severity_mismatch(&to_unified(actual), &calculated, ScoreType::Temporal, instance_path)
        {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
    if let Some(actual) = &cvss3.environmental_severity
        && let Some(calculated) = map_score_to_severity(calculated_environmental)
    {
        if let Some(mismatch) = check_severity_mismatch(
            &to_unified(actual),
            &calculated,
            ScoreType::Environmental,
            instance_path,
        ) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
}

/// Validates CVSS v4 base score and base severity.
///
/// The `vectorString` is taken as authoritative. It is used to calculate the expected score and severity,
/// which are then compared against the values declared in the JSON.
///
/// Checked fields: `baseScore`, `baseSeverity`
pub fn validate_v4_scores(cvss4: &CvssV4, instance_path: &str, errors: &mut Option<Vec<TestFinding>>) {
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
        if let Some(mismatch) = check_score_mismatch(cvss4.base_score, calculated, ScoreType::Base, instance_path) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }

    // Validate base severity using base_severity() which converts to unified Severity
    if let Some(calculated) = map_score_to_severity(calculated_full)
        && let Some(actual) = cvss4.base_severity()
    {
        if let Some(mismatch) = check_severity_mismatch(&actual, &calculated, ScoreType::Base, instance_path) {
            errors.get_or_insert_default().push(TestFinding::Error(mismatch));
        }
    }
}
