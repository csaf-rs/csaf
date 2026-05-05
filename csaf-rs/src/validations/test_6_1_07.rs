use crate::csaf::aggregation::product_cvss_metrics::aggregate_product_cvss_metrics;
use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::csaf_traits::CsafTrait;
use crate::validation::ValidationError;

/// Test 6.1.7 Multiple Scores with Same Version per Product
///
/// For each item in `/vulnerabilities` it MUST be tested that the same Product ID
/// is not a member of more than one CVSS vector with the same version and same source.
pub fn test_6_1_07_multiple_same_scores_per_product(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        let product_metrics = aggregate_product_cvss_metrics(vulnerability, vulnerability_index);
        // if there are any cvss metrics
        if let Some(product_metrics) = product_metrics {
            // traverse the metrics map
            for (product_id, cvss_metrics_map) in &product_metrics {
                for (metric_type, source_map) in cvss_metrics_map {
                    for (source, paths) in source_map {
                        // generate an error if the same metric, with same or no source, applies to the same product id
                        if paths.len() > 1 {
                            for path in paths {
                                errors.get_or_insert_default().push(create_validation_error(
                                    metric_type,
                                    product_id,
                                    path.to_owned(),
                                    source.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_7, test_6_1_07_multiple_same_scores_per_product);

fn create_validation_error(
    score_type: &CsafVulnerabilityMetric,
    product_id: &str,
    path: String,
    source: Option<String>,
) -> ValidationError {
    let source_info = source.map_or("by author".to_string(), |s| format!("for source: {s}"));
    ValidationError {
        message: format!("Multiple {score_type} scores are given for {product_id} {source_info}."),
        instance_path: format!("{}/{}", path, score_type.get_metric_prop_name()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_07() {
        // Case 01: two scores, same product, both CVSS v3.1
        let case_01_duplicate_cvss_v3_1_csaf_20 = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/scores/0".to_string(),
                None,
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/scores/1".to_string(),
                None,
            ),
        ]);
        let case_01_duplicate_cvss_v3_1_csaf_21 = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                None,
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                None,
            ),
        ]);
        // Case 02: two scores, same product, both CVSS v3.0
        let case_02_duplicate_cvss_v3_0_csaf_21 = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                None,
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                None,
            ),
        ]);
        // Case 03: two scores, some product, both CVSS v2
        let case_03_duplicate_cvss_v2_csaf_21 = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                None,
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                None,
            ),
        ]);
        // Case 04: two scores, some product, both CVSS v4
        let case_04_duplicate_cvss_v4_csaf_21 = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                None,
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                None,
            ),
        ]);
        // Case 05: two products, CVSS v2, v3.0, v3.1, v4 scores, with sources
        let case_05_duplicate_cvss_mixed_versions_with_sources = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/2/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/0/content".to_string(),
                Some("https://www.example.net/awesome-research-blog-post".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/1/content".to_string(),
                Some("https://www.example.net/awesome-research-blog-post".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/0/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
        ]);
        // Case 06: two products, invalid CVSS versions, also with sources
        let case_06_duplicate_cvss_invalid_versions_with_sources = Err(vec![
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.2".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.2".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.4".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV3("3.4".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/2/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.5".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/0/content".to_string(),
                Some("https://www.example.net/awesome-research-blog-post".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV2("2.5".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/1/content".to_string(),
                Some("https://www.example.net/awesome-research-blog-post".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0.1".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/0/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
            create_validation_error(
                &CsafVulnerabilityMetric::CvssV4("4.0.1".to_string()),
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/1/content".to_string(),
                Some("https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json".to_string()),
            ),
        ]);

        // Case 11: 2 vulns, 2 CVSS v3.1 scores, same product
        // Case 12: 1 product, CVSS v2 and CVSS v3.1 score
        // Case 13: 2 vulns, 2 CVSS v3.0 scores, same product
        // Case 14: 2 vulns, 2 CVSS v2 scores, same product
        // Case 15: 2 vulns, 2 CVSS v4 scores, same product
        // Case 16: 1 vuln, CVSS v2, v3.1, v4, same product
        // Case 17: 1 vuln, CVSS v2, v3.0, v4, same product
        // Case 18: like 05, but valid
        TESTS_2_0
            .test_6_1_7
            .expect(case_01_duplicate_cvss_v3_1_csaf_20, Ok(()), Ok(()));

        TESTS_2_1.test_6_1_7.expect(
            case_01_duplicate_cvss_v3_1_csaf_21,
            case_02_duplicate_cvss_v3_0_csaf_21,
            case_03_duplicate_cvss_v2_csaf_21,
            case_04_duplicate_cvss_v4_csaf_21,
            case_05_duplicate_cvss_mixed_versions_with_sources,
            case_06_duplicate_cvss_invalid_versions_with_sources,
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
