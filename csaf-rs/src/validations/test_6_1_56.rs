use std::collections::HashMap;

use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn generate_cvss_and_qualitative_error(
    product_id: &str,
    source: &Option<String>,
    v_i: usize,
    m_i: usize,
) -> ValidationError {
    let source_str = match source {
        Some(s) => format!("and source '{s}'"),
        None => "by author".to_string(),
    };

    ValidationError {
        message: format!(
            "Vulnerability has both a CVSS score and qualitative severity rating for product_id '{product_id}' {source_str}"
        ),
        // Metric instances paths are usually constructed using "ContentTrait::get_content_json_path".
        // We are not doing that here, as qualitative severity rating is CSAF 2.1 only.
        // So even if we implement running CSAF 2.1 tests for CSAF 2.0 docs, this test will always pass,
        // as the offending metric type does not exist on CSAF 2.0, and this will never print the wrong path.
        instance_path: format!("/vulnerabilities/{v_i}/metrics/{m_i}/content/qualitative_severity_rating",),
    }
}

/// 6.1.56 Use of CVSS and Qualitative Severity Rating
///
/// In each vulnerability, for each tuple of Product ID and source, there cannot be both a CVSS score
/// and a qualitative severity rating present.
pub fn test_6_1_56_cvss_and_qualitative_severity_rating(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    type ProductIdSourceTuple = (String, Option<String>);
    type HasRatingsTuple = (usize, bool, bool);
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each vulnerability
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // construct a hashmap of (product_id, Option<source>) to (has_any_cvss_score, has_qualitative_severity_rating)
        let mut ratings_map: Option<HashMap<ProductIdSourceTuple, Vec<HasRatingsTuple>>> = None;
        if let Some(metrics) = vulnerability.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                for product_id in metric.get_products() {
                    let product_id_source_tuple = (product_id.to_owned(), metric.get_source().map(|s| s.to_owned()));
                    let content = metric.get_content();
                    let has_ratings_tuple = (m_i, content.has_any_cvss(), content.has_qualitative_severity());
                    ratings_map
                        .get_or_insert_default()
                        .entry(product_id_source_tuple)
                        .or_default()
                        .push(has_ratings_tuple);
                }
            }
        }
        if let Some(ratings_map) = ratings_map {
            for ((product_id, source), ratings) in &ratings_map {
                let has_cvss_score = ratings.iter().any(|(_, has_cvss, _)| *has_cvss);
                if has_cvss_score {
                    for (m_i, _, has_qualitative) in ratings {
                        if *has_qualitative {
                            errors
                                .get_or_insert_default()
                                .push(generate_cvss_and_qualitative_error(product_id, source, v_i, *m_i));
                        }
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_56,
    test_6_1_56_cvss_and_qualitative_severity_rating
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_56() {
        let qualitative_in_second_metric_no_source = Err(vec![generate_cvss_and_qualitative_error(
            "CSAFPID-9080700",
            &None,
            0,
            1,
        )]);
        let case_02_qualitative_in_first_metric_no_source = Err(vec![generate_cvss_and_qualitative_error(
            "CSAFPID-9080700",
            &None,
            0,
            0,
        )]);

        let case_05_err = Err(vec![
            // First vulnerability has 2 metrics, same two metrics, same product id, same source
            // one has CVSS and the other qualitative
            generate_cvss_and_qualitative_error(
                "CSAFPID-9080700",
                &Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
                0,
                1,
            ),
            // 2nd vulnerability has 3 metrics, partially overlapping product ids, the second and third
            // with the same source and CVSS, but only the first without source has qualitative
            // 3rd vulnerability has 2 metrics, partially overlapping product ids, same source
            // one has CVSS and the other qualitative
            generate_cvss_and_qualitative_error(
                "CSAFPID-9080701",
                &Some("https://www.example.net/awesome-research-blog-post".to_string()),
                2,
                1,
            ),
            // 4th vulnerability has 2 metrics, partially overlapping product ids but different sources,
            // each metric has both CVSS and qualitative
            generate_cvss_and_qualitative_error(
                "CSAFPID-9080701",
                &Some("https://www.example.net/awesome-research-blog-post".to_string()),
                3,
                0,
            ),
            generate_cvss_and_qualitative_error(
                "CSAFPID-9080700",
                &Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
                3,
                1,
            ),
            generate_cvss_and_qualitative_error(
                "CSAFPID-9080701",
                &Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
                3,
                1,
            ),
        ]);

        // Case 11: 2 vulnerabilities, same product has CVSS in one vuln and qualitative in the other (cvss 3.1)
        // Case 12: 2 metrics, same product, one has CVSS and the other qualitative, but with different source
        // Case 13: multiple vulnerabilities, same product, qualitative is in a separate vulnerability
        // Case 14: 2 vulnerabilities, same product has CVSS in one vuln and qualitative in the other (cvss 2.0
        // Case 15: 2 vulnerabilities, same product has CVSS in one vuln and qualitative in the other (cvss 4.1)
        // Case 16: different products, one has CVSS and the other qualitative
        // Case 17: 2 metrics, same product, one has CVSS and the other qualitative, but with different source
        // Case 18: complex case, 4 vulnerabilities, all of them with 2 metrics, overlapping product,
        // different sources, one with CVSS and the other with qualitative, but differently ordered

        TESTS_2_1.test_6_1_56.expect(
            qualitative_in_second_metric_no_source.clone(),
            case_02_qualitative_in_first_metric_no_source,
            qualitative_in_second_metric_no_source.clone(),
            qualitative_in_second_metric_no_source,
            case_05_err,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
