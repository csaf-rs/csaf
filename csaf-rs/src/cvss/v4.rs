use std::str::FromStr;

use cvss_rs::Version;
use cvss_rs::v4_0::CvssV4;

use super::{
    ScoreType, check_optional_field_mismatch, check_score_mismatch, check_severity_mismatch, create_vector_parse_error,
    map_score_to_severity,
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

/// Validates CVSS v4 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_consistency(cvss4: &CvssV4, instance_path: &str, errors: &mut Option<Vec<ValidationError>>) {
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

    // Base metrics
    check_optional_field_mismatch(
        "attackVector",
        &cvss4.attack_vector,
        &parsed.attack_vector,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "attackComplexity",
        &cvss4.attack_complexity,
        &parsed.attack_complexity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "attackRequirements",
        &cvss4.attack_requirements,
        &parsed.attack_requirements,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "privilegesRequired",
        &cvss4.privileges_required,
        &parsed.privileges_required,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "userInteraction",
        &cvss4.user_interaction,
        &parsed.user_interaction,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "vulnConfidentialityImpact",
        &cvss4.vuln_confidentiality_impact,
        &parsed.vuln_confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "vulnIntegrityImpact",
        &cvss4.vuln_integrity_impact,
        &parsed.vuln_integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "vulnAvailabilityImpact",
        &cvss4.vuln_availability_impact,
        &parsed.vuln_availability_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "subConfidentialityImpact",
        &cvss4.sub_confidentiality_impact,
        &parsed.sub_confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "subIntegrityImpact",
        &cvss4.sub_integrity_impact,
        &parsed.sub_integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "subAvailabilityImpact",
        &cvss4.sub_availability_impact,
        &parsed.sub_availability_impact,
        instance_path,
        errors,
    );

    // Threat metric
    check_optional_field_mismatch(
        "exploitMaturity",
        &cvss4.exploit_maturity,
        &parsed.exploit_maturity,
        instance_path,
        errors,
    );

    // Environmental metrics
    check_optional_field_mismatch(
        "confidentialityRequirement",
        &cvss4.confidentiality_requirement,
        &parsed.confidentiality_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "integrityRequirement",
        &cvss4.integrity_requirement,
        &parsed.integrity_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "availabilityRequirement",
        &cvss4.availability_requirement,
        &parsed.availability_requirement,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAttackVector",
        &cvss4.modified_attack_vector,
        &parsed.modified_attack_vector,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAttackComplexity",
        &cvss4.modified_attack_complexity,
        &parsed.modified_attack_complexity,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedAttackRequirements",
        &cvss4.modified_attack_requirements,
        &parsed.modified_attack_requirements,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedPrivilegesRequired",
        &cvss4.modified_privileges_required,
        &parsed.modified_privileges_required,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedUserInteraction",
        &cvss4.modified_user_interaction,
        &parsed.modified_user_interaction,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedVulnConfidentialityImpact",
        &cvss4.modified_vuln_confidentiality_impact,
        &parsed.modified_vuln_confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedVulnIntegrityImpact",
        &cvss4.modified_vuln_integrity_impact,
        &parsed.modified_vuln_integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedVulnAvailabilityImpact",
        &cvss4.modified_vuln_availability_impact,
        &parsed.modified_vuln_availability_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedSubConfidentialityImpact",
        &cvss4.modified_sub_confidentiality_impact,
        &parsed.modified_sub_confidentiality_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedSubIntegrityImpact",
        &cvss4.modified_sub_integrity_impact,
        &parsed.modified_sub_integrity_impact,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "modifiedSubAvailabilityImpact",
        &cvss4.modified_sub_availability_impact,
        &parsed.modified_sub_availability_impact,
        instance_path,
        errors,
    );

    // Supplemental metrics
    // Safety, Automatable and Recovery are capitalized in the JSON schema
    check_optional_field_mismatch("Safety", &cvss4.safety, &parsed.safety, instance_path, errors);
    check_optional_field_mismatch(
        "Automatable",
        &cvss4.automatable,
        &parsed.automatable,
        instance_path,
        errors,
    );
    check_optional_field_mismatch("Recovery", &cvss4.recovery, &parsed.recovery, instance_path, errors);
    check_optional_field_mismatch(
        "valueDensity",
        &cvss4.value_density,
        &parsed.value_density,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "vulnerabilityResponseEffort",
        &cvss4.vulnerability_response_effort,
        &parsed.vulnerability_response_effort,
        instance_path,
        errors,
    );
    check_optional_field_mismatch(
        "providerUrgency",
        &cvss4.provider_urgency,
        &parsed.provider_urgency,
        instance_path,
        errors,
    );
}
