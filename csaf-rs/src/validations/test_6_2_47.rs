use crate::csaf_traits::{ContentTrait, CsafTrait, DocumentTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn create_qualitative_severity_rating_error(content_path: &str) -> ValidationError {
    ValidationError {
        message: "A metric provided by the issuing party must not use `qualitative_severity_rating`.".to_string(),
        instance_path: format!("{content_path}/qualitative_severity_rating"),
    }
}

/// 6.2.47 Use of Qualitative Severity Rating by Issuing Party
///
/// For each item in metrics provided by the issuing party it MUST be tested that it does not use
/// the qualitative severity rating.
///
/// This covers all items in metrics that do not have a source property and those where the source
/// is equal to the canonical URL. It does not cover assessments made by third parties.
pub fn test_6_2_47_use_of_qualitative_severity_rating_by_issuing_party(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    if vulnerabilities.is_empty() {
        // wasSkipped later (#407)
        return Ok(());
    }

    // collect canonical URLs into a HashSet for O(1) lookups
    let canonical_urls: HashSet<&str> = doc.get_document().get_canonical_urls().into_iter().collect();

    let mut errors: Option<Vec<ValidationError>> = None;

    for (i_v, vulnerability) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                // check if metric is by issuing party
                let is_by_issuing_party = match metric.get_source() {
                    None => true,
                    Some(source) => canonical_urls.contains(source),
                };

                // if not, this metric is irrelevant to this test
                if !is_by_issuing_party {
                    continue;
                }

                // if metric has qualitative severity rating, generate an error
                let content = metric.get_content();
                if content.has_qualitative_severity() {
                    let content_path = content.get_content_json_path(i_v, i_m);
                    errors
                        .get_or_insert_default()
                        .push(create_qualitative_severity_rating_error(&content_path));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_47,
    test_6_2_47_use_of_qualitative_severity_rating_by_issuing_party
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_47 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_47() {
        let case_01_no_source = Err(vec![create_qualitative_severity_rating_error(
            "/vulnerabilities/0/metrics/0/content",
        )]);
        let case_02_source_is_canonical = Err(vec![create_qualitative_severity_rating_error(
            "/vulnerabilities/0/metrics/0/content",
        )]);

        // Case 11: metric without source and without qualitative severity
        // Case 12: metric with third-party URL as source and qualitative severity rating

        TESTS_2_1.test_6_2_47.expect(ExpectedResults {
            case_01: case_01_no_source,
            case_02: case_02_source_is_canonical,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
