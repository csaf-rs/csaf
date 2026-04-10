use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_qualitative_severity_rating_error(instance_path: String) -> ValidationError {
    ValidationError {
        message: "The metric uses a qualitative severity rating. The use of qualitative severity ratings is generally discouraged.".to_string(),
        instance_path,
    }
}

/// 6.3.18 Use of Qualitative Severity Rating
///
/// For each item in metrics it MUST be tested that it does not use the qualitative severity rating.
pub fn test_6_3_18_use_of_qualitative_severity_rating(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                if content.has_qualitative_severity() {
                    let path = format!(
                        "{}/qualitative_severity_rating",
                        content.get_content_json_path(v_i, m_i)
                    );
                    errors
                        .get_or_insert_default()
                        .push(create_qualitative_severity_rating_error(path));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_18,
    test_6_3_18_use_of_qualitative_severity_rating
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_18() {
        let case_01 = Err(vec![create_qualitative_severity_rating_error(
            "/vulnerabilities/0/metrics/0/content/qualitative_severity_rating".to_string(),
        )]);

        TESTS_2_1.test_6_3_18.expect(case_01, Ok(()));
    }
}
