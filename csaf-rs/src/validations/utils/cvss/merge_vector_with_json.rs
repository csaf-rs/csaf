use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;

/// A CVSS value whose metric properties combine JSON and vector-string data.
///
/// Missing vector metric values are filled from values deserialized from JSON.
pub struct ValuesMergedCvss<T>(pub T);

macro_rules! merge_metric_fields {
    ($vector:ident, $json:ident, $($field:ident),+ $(,)?) => {
        $(
            $vector.$field = $vector.$field.or($json.$field);
        )+
    };
}

fn merge_v2(mut cvss_from_vector: CvssV2, original_from_json: CvssV2) -> ValuesMergedCvss<CvssV2> {
    // we copy
    cvss_from_vector.severity = original_from_json.severity;
    cvss_from_vector.base_score = original_from_json.base_score;
    cvss_from_vector.temporal_score = original_from_json.temporal_score;
    cvss_from_vector.environmental_score = original_from_json.environmental_score;
    merge_metric_fields!(
        cvss_from_vector,
        original_from_json,
        access_vector,
        access_complexity,
        authentication,
        confidentiality_impact,
        integrity_impact,
        availability_impact,
        exploitability,
        remediation_level,
        report_confidence,
        collateral_damage_potential,
        target_distribution,
        confidentiality_requirement,
        integrity_requirement,
        availability_requirement,
    );
    ValuesMergedCvss(cvss_from_vector)
}

fn merge_v3(mut cvss_from_vector: CvssV3, original_from_json: CvssV3) -> ValuesMergedCvss<CvssV3> {
    cvss_from_vector.base_score = original_from_json.base_score;
    cvss_from_vector.base_severity = original_from_json.base_severity;
    cvss_from_vector.temporal_score = original_from_json.temporal_score;
    cvss_from_vector.temporal_severity = original_from_json.temporal_severity;
    cvss_from_vector.environmental_score = original_from_json.environmental_score;
    cvss_from_vector.environmental_severity = original_from_json.environmental_severity;
    merge_metric_fields!(
        cvss_from_vector,
        original_from_json,
        attack_vector,
        attack_complexity,
        privileges_required,
        user_interaction,
        scope,
        confidentiality_impact,
        integrity_impact,
        availability_impact,
        exploit_code_maturity,
        remediation_level,
        report_confidence,
        confidentiality_requirement,
        integrity_requirement,
        availability_requirement,
        modified_attack_vector,
        modified_attack_complexity,
        modified_privileges_required,
        modified_user_interaction,
        modified_scope,
        modified_confidentiality_impact,
        modified_integrity_impact,
        modified_availability_impact,
    );
    ValuesMergedCvss(cvss_from_vector)
}

fn merge_v4(original_from_json: CvssV4, mut cvss_from_vector: CvssV4) -> ValuesMergedCvss<CvssV4> {
    cvss_from_vector.base_score = original_from_json.base_score;
    cvss_from_vector.base_severity = original_from_json.base_severity;
    merge_metric_fields!(
        cvss_from_vector,
        original_from_json,
        attack_vector,
        attack_complexity,
        attack_requirements,
        privileges_required,
        user_interaction,
        vuln_confidentiality_impact,
        vuln_integrity_impact,
        vuln_availability_impact,
        sub_confidentiality_impact,
        sub_integrity_impact,
        sub_availability_impact,
        exploit_maturity,
        confidentiality_requirement,
        integrity_requirement,
        availability_requirement,
        modified_attack_vector,
        modified_attack_complexity,
        modified_attack_requirements,
        modified_privileges_required,
        modified_user_interaction,
        modified_vuln_confidentiality_impact,
        modified_vuln_integrity_impact,
        modified_vuln_availability_impact,
        modified_sub_confidentiality_impact,
        modified_sub_integrity_impact,
        modified_sub_availability_impact,
        safety,
        automatable,
        recovery,
        value_density,
        vulnerability_response_effort,
        provider_urgency,
    );
    ValuesMergedCvss(cvss_from_vector)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{merge_v2, merge_v3, merge_v4};
    use cvss_rs::v2_0::CvssV2;
    use cvss_rs::v3::CvssV3;
    use cvss_rs::v4_0::CvssV4;

    #[test]
    fn merge_v2_prefers_vector_metrics_and_retains_json_scores() {
        let json: CvssV2 = serde_json::from_str(
            r#"{
                "version": "2.0",
                "vectorString": "AV:L/AC:H/Au:M/C:P/I:P/A:P/E:H",
                "baseScore": 6.5,
                "accessVector": "LOCAL",
                "accessComplexity": "HIGH",
                "authentication": "MULTIPLE",
                "confidentialityImpact": "PARTIAL",
                "integrityImpact": "PARTIAL",
                "availabilityImpact": "PARTIAL",
                "exploitability": "HIGH"
            }"#,
        )
        .expect("valid CVSS v2 JSON");
        let vector = CvssV2::from_str("AV:N/AC:L/Au:N/C:C/I:C/A:C/E:U").expect("valid CVSS v2 vector");

        let merged = merge_v2(vector, json).0;

        // From vector
        assert_eq!(merged.vector_string, "AV:N/AC:L/Au:N/C:C/I:C/A:C/E:U");
        // From vector
        assert_eq!(merged.version, cvss_rs::version::VersionV2::V2_0);
        // From JSON
        assert_eq!(merged.base_score, 6.5);
        // From vector
        assert_eq!(merged.access_vector, Some(cvss_rs::v2_0::AccessVector::Network));
        // From vector
        assert_eq!(merged.access_complexity, Some(cvss_rs::v2_0::AccessComplexity::Low));
        // From vector
        assert_eq!(merged.authentication, Some(cvss_rs::v2_0::Authentication::None));
        // From vector
        assert_eq!(merged.confidentiality_impact, Some(cvss_rs::v2_0::Impact::Complete));
        // From vector
        assert_eq!(merged.integrity_impact, Some(cvss_rs::v2_0::Impact::Complete));
        // From vector
        assert_eq!(merged.availability_impact, Some(cvss_rs::v2_0::Impact::Complete));
        // From vector
        assert_eq!(merged.exploitability, Some(cvss_rs::v2_0::Exploitability::Unproven));
    }

    #[test]
    fn merge_v3_falls_back_to_json_metrics_when_vector_omits_them() {
        let json: CvssV3 = serde_json::from_str(
            r#"{
                "version": "3.1",
                "vectorString": "CVSS:3.1/AV:L/AC:H/PR:H/UI:R/S:C/C:N/I:N/A:N/E:F",
                "baseScore": 9.8,
                "baseSeverity": "CRITICAL",
                "attackVector": "LOCAL",
                "attackComplexity": "HIGH",
                "privilegesRequired": "HIGH",
                "userInteraction": "REQUIRED",
                "scope": "CHANGED",
                "confidentialityImpact": "NONE",
                "integrityImpact": "NONE",
                "availabilityImpact": "NONE",
                "exploitCodeMaturity": "FUNCTIONAL"
            }"#,
        )
        .expect("valid CVSS v3 JSON");
        let vector = CvssV3::from_str("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").expect("valid CVSS v3 vector");

        let merged = merge_v3(vector.clone(), json.clone()).0;

        // From vector
        assert_eq!(merged.vector_string, vector.vector_string);
        // From vector
        assert_eq!(merged.version, vector.version);
        // From JSON
        assert_eq!(merged.base_score, json.base_score);
        // From JSON
        assert_eq!(merged.base_severity, json.base_severity);
        // From vector
        assert_eq!(merged.attack_vector, vector.attack_vector);
        // From vector
        assert_eq!(merged.attack_complexity, vector.attack_complexity);
        // From vector
        assert_eq!(merged.privileges_required, vector.privileges_required);
        // From vector
        assert_eq!(merged.user_interaction, vector.user_interaction);
        // From vector
        assert_eq!(merged.scope, vector.scope);
        // From vector
        assert_eq!(merged.confidentiality_impact, vector.confidentiality_impact);
        // From vector
        assert_eq!(merged.integrity_impact, vector.integrity_impact);
        // From vector
        assert_eq!(merged.availability_impact, vector.availability_impact);
        // From JSON, as the vector does not have the value
        assert_eq!(merged.exploit_code_maturity, json.exploit_code_maturity);
    }

    #[test]
    fn merge_v4_retains_vector_not_defined_metrics() {
        let json: CvssV4 = serde_json::from_str(
            r#"{
                "version": "4.0",
                "vectorString": "CVSS:4.0/AV:L/AC:H/AT:P/PR:H/UI:A/VC:N/VI:N/VA:N/SC:N/SI:N/SA:N/E:A",
                "baseScore": 9.3,
                "baseSeverity": "CRITICAL",
                "attackVector": "LOCAL",
                "attackComplexity": "HIGH",
                "attackRequirements": "PRESENT",
                "privilegesRequired": "HIGH",
                "userInteraction": "ACTIVE",
                "vulnConfidentialityImpact": "NONE",
                "vulnIntegrityImpact": "NONE",
                "vulnAvailabilityImpact": "NONE",
                "subConfidentialityImpact": "NONE",
                "subIntegrityImpact": "NONE",
                "subAvailabilityImpact": "NONE",
                "exploitMaturity": "ATTACKED"
            }"#,
        )
        .expect("valid CVSS v4 JSON");
        let vector = CvssV4::from_str("CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N/E:X")
            .expect("valid CVSS v4 vector");

        let merged = merge_v4(json.clone(), vector.clone()).0;

        // From vector
        assert_eq!(merged.vector_string, vector.vector_string);
        // From vector
        assert_eq!(merged.version, vector.version);
        // From JSON
        assert_eq!(merged.base_score, json.base_score);
        // From JSON
        assert_eq!(merged.base_severity, json.base_severity);
        // From vector
        assert_eq!(merged.attack_vector, vector.attack_vector);
        // From vector
        assert_eq!(merged.attack_complexity, vector.attack_complexity);
        // From vector
        assert_eq!(merged.attack_requirements, vector.attack_requirements);
        // From vector
        assert_eq!(merged.privileges_required, vector.privileges_required);
        // From vector
        assert_eq!(merged.user_interaction, vector.user_interaction);
        // From vector
        assert_eq!(merged.vuln_confidentiality_impact, vector.vuln_confidentiality_impact);
        // From vector
        assert_eq!(merged.vuln_integrity_impact, vector.vuln_integrity_impact);
        // From vector
        assert_eq!(merged.vuln_availability_impact, vector.vuln_availability_impact);
        // From vector
        assert_eq!(merged.sub_confidentiality_impact, vector.sub_confidentiality_impact);
        // From vector
        assert_eq!(merged.sub_integrity_impact, vector.sub_integrity_impact);
        // From vector
        assert_eq!(merged.sub_availability_impact, vector.sub_availability_impact);
        // From vector, with "not defined" value
        assert_eq!(merged.exploit_maturity, vector.exploit_maturity);
    }
}
