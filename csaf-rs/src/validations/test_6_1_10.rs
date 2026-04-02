use crate::{
    csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait},
    cvss,
    validation::ValidationError,
};

/// 6.1.10 Inconsistent CVSS
///
/// Checks for inconsistency between the vector and the values in the JSON, taking the vector as
/// authoritative.
///
/// Generates an error if a CVSS 2.0,3.x,4.0 metric differs in value between the JSON and vector or is
/// missing in either the JSON or vector and present in the other. For the later comparison, the "NotDefined"
/// values are normalized to be "not present".
pub fn test_6_1_10_inconsistent_cvss(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let instance_path = content.get_content_json_path(i_v, metric_index);

                cvss::validate_content_consistency(content, &instance_path, &mut errors);
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_10
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_10_inconsistent_cvss(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_10
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_10_inconsistent_cvss(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::cvss::create_field_value_mismatch_error;

    #[test]
    fn test_test_6_1_10() {
        let path_2_0 = "/vulnerabilities/0/scores/0";

        let case_01_3_1_mismatch_csaf_2_0 = Err(vec![
            create_field_value_mismatch_error("attackVector", &"L", &"N", path_2_0),
            create_field_value_mismatch_error("scope", &"C", &"U", path_2_0),
            create_field_value_mismatch_error("availabilityImpact", &"L", &"H", path_2_0),
        ]);

        TESTS_2_0.test_6_1_10.expect(case_01_3_1_mismatch_csaf_2_0);

        let path_2_1 = "/vulnerabilities/0/metrics/0/content";

        let case_01_3_1_mismatch_csaf_2_1 = Err(vec![
            create_field_value_mismatch_error("attackVector", &"L", &"N", path_2_1),
            create_field_value_mismatch_error("scope", &"C", &"U", path_2_1),
            create_field_value_mismatch_error("availabilityImpact", &"L", &"H", path_2_1),
        ]);

        let case_02_3_0_mismatch_csaf_2_1: Result<(), _> = Err(vec![
            create_field_value_mismatch_error("attackComplexity", &"H", &"L", path_2_1),
            create_field_value_mismatch_error("privilegesRequired", &"H", &"N", path_2_1),
            create_field_value_mismatch_error("userInteraction", &"R", &"N", path_2_1),
            create_field_value_mismatch_error("confidentialityImpact", &"N", &"H", path_2_1),
        ]);

        let case_03_2_0_mismatch_csaf_2_1: Result<(), _> = Err(vec![
            create_field_value_mismatch_error("accessComplexity", &"H", &"L", path_2_1),
            create_field_value_mismatch_error("authentication", &"M", &"N", path_2_1),
            create_field_value_mismatch_error("confidentialityImpact", &"P", &"C", path_2_1),
            create_field_value_mismatch_error("integrityImpact", &"N", &"C", path_2_1),
            create_field_value_mismatch_error("exploitability", &"F", &"POC", path_2_1),
        ]);

        let case_04_4_0_mismatch_csaf_2_1: Result<(), _> = Err(vec![
            create_field_value_mismatch_error("attackVector", &"L", &"N", path_2_1),
            create_field_value_mismatch_error("attackComplexity", &"H", &"L", path_2_1),
            create_field_value_mismatch_error("attackRequirements", &"P", &"N", path_2_1),
            create_field_value_mismatch_error("userInteraction", &"P", &"N", path_2_1),
            create_field_value_mismatch_error("vulnConfidentialityImpact", &"L", &"H", path_2_1),
            create_field_value_mismatch_error("vulnIntegrityImpact", &"N", &"H", path_2_1),
            create_field_value_mismatch_error("subConfidentialityImpact", &"L", &"H", path_2_1),
            create_field_value_mismatch_error("subAvailabilityImpact", &"N", &"H", path_2_1),
        ]);

        // Case 11: v3.1 correct
        // Case 12: v3.0 correct
        // Case 13: v2.0 correct
        // Case 14: v4.0 correct

        TESTS_2_1.test_6_1_10.expect(
            case_01_3_1_mismatch_csaf_2_1,
            case_02_3_0_mismatch_csaf_2_1,
            case_03_2_0_mismatch_csaf_2_1,
            case_04_4_0_mismatch_csaf_2_1,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
