use std::str::FromStr;

use cvss_rs::Severity;
use cvss_rs::Version;
use cvss_rs::v3::{CvssV3, Severity as V3Severity};

use super::{
    ScoreType, check_optional_field_mismatch, check_score_mismatch, check_severity_mismatch, create_vector_parse_error,
    map_score_to_severity,
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

/// Validates CVSS v3 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_consistency(cvss3: &CvssV3, instance_path: &str, errors: &mut Option<Vec<ValidationError>>) {
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

    // Base metrics
    check_optional_field_mismatch(
        "attackVector",
        &cvss3.attack_vector,
        &parsed.attack_vector,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "attackComplexity",
        &cvss3.attack_complexity,
        &parsed.attack_complexity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "privilegesRequired",
        &cvss3.privileges_required,
        &parsed.privileges_required,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "userInteraction",
        &cvss3.user_interaction,
        &parsed.user_interaction,
        instance_path,
        errors,
    );
    check_optional_field_mismatch("scope", &cvss3.scope, &parsed.scope, instance_path, errors);
    check_optional_field_mismatch(
        "confidentialityImpact",
        &cvss3.confidentiality_impact,
        &parsed.confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "integrityImpact",
        &cvss3.integrity_impact,
        &parsed.integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "availabilityImpact",
        &cvss3.availability_impact,
        &parsed.availability_impact,
        instance_path,
        errors,
    );

    // Temporal metrics
    check_optional_field_mismatch(
        "exploitCodeMaturity",
        &cvss3.exploit_code_maturity,
        &parsed.exploit_code_maturity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "remediationLevel",
        &cvss3.remediation_level,
        &parsed.remediation_level,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "reportConfidence",
        &cvss3.report_confidence,
        &parsed.report_confidence,
        instance_path,
        errors,
    );

    // Environmental metrics
    check_optional_field_mismatch(
        "confidentialityRequirement",
        &cvss3.confidentiality_requirement,
        &parsed.confidentiality_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "integrityRequirement",
        &cvss3.integrity_requirement,
        &parsed.integrity_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "availabilityRequirement",
        &cvss3.availability_requirement,
        &parsed.availability_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAttackVector",
        &cvss3.modified_attack_vector,
        &parsed.modified_attack_vector,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAttackComplexity",
        &cvss3.modified_attack_complexity,
        &parsed.modified_attack_complexity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedPrivilegesRequired",
        &cvss3.modified_privileges_required,
        &parsed.modified_privileges_required,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedUserInteraction",
        &cvss3.modified_user_interaction,
        &parsed.modified_user_interaction,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedScope",
        &cvss3.modified_scope,
        &parsed.modified_scope,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedConfidentialityImpact",
        &cvss3.modified_confidentiality_impact,
        &parsed.modified_confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedIntegrityImpact",
        &cvss3.modified_integrity_impact,
        &parsed.modified_integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAvailabilityImpact",
        &cvss3.modified_availability_impact,
        &parsed.modified_availability_impact,
        instance_path,
        errors,
    );
}
