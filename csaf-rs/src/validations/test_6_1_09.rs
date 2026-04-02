use crate::{
    csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait},
    cvss,
    validation::ValidationError,
};

/// 6.1.9 Invalid CVSS computation
///
/// Checks for invalid CVSS computation, with the vector taken as authoritative.
/// For CVSS v2.0, the base, temporal and environmental scores are checked.
/// For CVSS v3.0 and v3.1, the base, temporal and environmental scores and severities are checked.
/// For CVSS v4.0, the score and severity are checked
pub fn test_6_1_09_invalid_cvss_computation(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let instance_path = content.get_content_json_path(i_v, metric_index);

                cvss::validate_content_scores(content, &instance_path, &mut errors);
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_9
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_09_invalid_cvss_computation(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_9
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_09_invalid_cvss_computation(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::cvss::{ScoreType, create_score_mismatch_error, create_severity_mismatch_error};
    use cvss_rs::Severity;

    #[test]
    fn test_test_6_1_09() {
        let path_2_0 = "/vulnerabilities/0/scores/0";

        let case01_3_1_base_score_and_severity_wrong_csaf_2_0: Result<(), _> = Err(vec![
            create_score_mismatch_error(6.5, 10.0, ScoreType::Base, path_2_0),
            create_severity_mismatch_error(&Severity::Medium, &Severity::Low, ScoreType::Base, path_2_0),
        ]);
        let case02_3_0_base_score_and_severity_wrong_csaf_2_0: Result<(), _> = Err(vec![
            create_score_mismatch_error(6.5, 10.0, ScoreType::Base, path_2_0),
            create_severity_mismatch_error(&Severity::Medium, &Severity::High, ScoreType::Base, path_2_0),
        ]);
        let case03_2_0_base_score_wrong_csaf_2_0: Result<(), _> =
            Err(vec![create_score_mismatch_error(10.0, 6.5, ScoreType::Base, path_2_0)]);

        // Case 11: v3.1 correct
        // Case 12: v3.0 correct
        // Case 13: v2.0 correct

        TESTS_2_0.test_6_1_9.expect(
            case01_3_1_base_score_and_severity_wrong_csaf_2_0,
            case02_3_0_base_score_and_severity_wrong_csaf_2_0,
            case03_2_0_base_score_wrong_csaf_2_0,
            Ok(()),
            Ok(()),
            Ok(()),
        );

        let path_2_1 = "/vulnerabilities/0/metrics/0/content";

        let case01_3_1_base_score_and_severity_wrong_csaf_2_1: Result<(), _> = Err(vec![
            create_score_mismatch_error(6.5, 10.0, ScoreType::Base, path_2_1),
            create_severity_mismatch_error(&Severity::Medium, &Severity::Low, ScoreType::Base, path_2_1),
        ]);
        let case02_3_0_base_score_and_severity_wrong_csaf_2_1: Result<(), _> = Err(vec![
            create_score_mismatch_error(6.5, 10.0, ScoreType::Base, path_2_1),
            create_severity_mismatch_error(&Severity::Medium, &Severity::High, ScoreType::Base, path_2_1),
        ]);
        let case03_2_0_base_score_wrong_csaf_2_1: Result<(), _> =
            Err(vec![create_score_mismatch_error(10.0, 6.5, ScoreType::Base, path_2_1)]);

        let case04_4_0_base_score_wrong_csaf_2_1: Result<(), _> =
            Err(vec![create_score_mismatch_error(10.0, 9.3, ScoreType::Base, path_2_1)]);
        let case05_4_0_base_score_and_severity_wrong_csaf_2_1: Result<(), _> = Err(vec![
            create_score_mismatch_error(9.3, 10.0, ScoreType::Base, path_2_1),
            create_severity_mismatch_error(&Severity::Critical, &Severity::High, ScoreType::Base, path_2_1),
        ]);

        // Case 11: v3.1 correct
        // Case 12: v3.0 correct
        // Case 13: v2.0 correct
        // Case 14: v4.0 correct
        // Case 15: v4.0 correct
        // Case 16: v4.0 correct

        TESTS_2_1.test_6_1_9.expect(
            case01_3_1_base_score_and_severity_wrong_csaf_2_1,
            case02_3_0_base_score_and_severity_wrong_csaf_2_1,
            case03_2_0_base_score_wrong_csaf_2_1,
            case04_4_0_base_score_wrong_csaf_2_1,
            case05_4_0_base_score_and_severity_wrong_csaf_2_1,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
