use std::str::FromStr;

use cvss_rs::v2_0::CvssV2;
use serde_json::Value;

use super::{ScoreType, check_optional_field_mismatch, check_score_mismatch, create_vector_parse_error};
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

/// Validates CVSS v2 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_consistency(
    cvss2: &CvssV2,
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

    // Compare all metric properties between the deserialized JSON object and the
    // vector-string-parsed object.
    check_optional_field_mismatch(
        "accessVector",
        &cvss2.access_vector,
        &parsed.access_vector,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "accessComplexity",
        &cvss2.access_complexity,
        &parsed.access_complexity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "authentication",
        &cvss2.authentication,
        &parsed.authentication,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "confidentialityImpact",
        &cvss2.confidentiality_impact,
        &parsed.confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "integrityImpact",
        &cvss2.integrity_impact,
        &parsed.integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "availabilityImpact",
        &cvss2.availability_impact,
        &parsed.availability_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "exploitability",
        &cvss2.exploitability,
        &parsed.exploitability,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "remediationLevel",
        &cvss2.remediation_level,
        &parsed.remediation_level,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "reportConfidence",
        &cvss2.report_confidence,
        &parsed.report_confidence,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "collateralDamagePotential",
        &cvss2.collateral_damage_potential,
        &parsed.collateral_damage_potential,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "targetDistribution",
        &cvss2.target_distribution,
        &parsed.target_distribution,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "confidentialityRequirement",
        &cvss2.confidentiality_requirement,
        &parsed.confidentiality_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "integrityRequirement",
        &cvss2.integrity_requirement,
        &parsed.integrity_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "availabilityRequirement",
        &cvss2.availability_requirement,
        &parsed.availability_requirement,
        instance_path,
        errors,
    );
}
