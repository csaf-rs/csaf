use crate::validation::TestFinding;

use crate::validations::utils::cvss::deserialize_vector::deserialize_vector;
use crate::validations::utils::cvss::validate_consistency::utils::{
    check_optional_field_mismatch, check_required_field_mismatch,
};
use cvss_rs::Version;
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;

/// Validates CVSS v2 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_v2_consistency(cvss2: &CvssV2, instance_path: &str, errors: &mut Option<Vec<TestFinding>>) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match deserialize_vector::<CvssV2>(&cvss2.vector_string, Version::V2, instance_path) {
        Ok(parsed) => parsed,
        Err(error) => {
            errors.get_or_insert_default().push(TestFinding::Error(error));
            return;
        },
    };

    // Compare all metric properties between the deserialized JSON object and the
    // vector-string-parsed object.
    check_required_field_mismatch(
        "accessVector",
        &cvss2.access_vector,
        &parsed.access_vector,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "accessComplexity",
        &cvss2.access_complexity,
        &parsed.access_complexity,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "authentication",
        &cvss2.authentication,
        &parsed.authentication,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "confidentialityImpact",
        &cvss2.confidentiality_impact,
        &parsed.confidentiality_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "integrityImpact",
        &cvss2.integrity_impact,
        &parsed.integrity_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
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

/// Validates CVSS v3 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_v3_consistency(cvss3: &CvssV3, instance_path: &str, errors: &mut Option<Vec<TestFinding>>) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match deserialize_vector::<CvssV3>(&cvss3.vector_string, Version::V3_0, instance_path) {
        Ok(parsed) => parsed,
        Err(error) => {
            errors.get_or_insert_default().push(TestFinding::Error(error));
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

/// Validates CVSS v4 property consistency between the JSON object and the vector string.
///
/// The `vectorString` is taken as authoritative. Each metric property declared in the JSON
/// is compared against the value parsed from the vector string, and mismatches are reported.
/// Mismatches include the value being present in either the JSON or vector string and missing in the other.
pub fn validate_v4_consistency(cvss4: &CvssV4, instance_path: &str, errors: &mut Option<Vec<TestFinding>>) {
    // Parse vector string to get a struct with populated metrics for calculation
    let parsed = match deserialize_vector::<CvssV4>(&cvss4.vector_string, Version::V4, instance_path) {
        Ok(parsed) => parsed,
        Err(error) => {
            errors.get_or_insert_default().push(TestFinding::Error(error));
            return;
        },
    };

    // Base metrics
    check_required_field_mismatch(
        "attackVector",
        &cvss4.attack_vector,
        &parsed.attack_vector,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "attackComplexity",
        &cvss4.attack_complexity,
        &parsed.attack_complexity,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "attackRequirements",
        &cvss4.attack_requirements,
        &parsed.attack_requirements,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "privilegesRequired",
        &cvss4.privileges_required,
        &parsed.privileges_required,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "userInteraction",
        &cvss4.user_interaction,
        &parsed.user_interaction,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "vulnConfidentialityImpact",
        &cvss4.vuln_confidentiality_impact,
        &parsed.vuln_confidentiality_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "vulnIntegrityImpact",
        &cvss4.vuln_integrity_impact,
        &parsed.vuln_integrity_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "vulnAvailabilityImpact",
        &cvss4.vuln_availability_impact,
        &parsed.vuln_availability_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "subConfidentialityImpact",
        &cvss4.sub_confidentiality_impact,
        &parsed.sub_confidentiality_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
        "subIntegrityImpact",
        &cvss4.sub_integrity_impact,
        &parsed.sub_integrity_impact,
        instance_path,
        errors,
    );
    check_required_field_mismatch(
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
