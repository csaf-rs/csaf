use crate::csaf_traits::VulnerabilityMetric::{CvssV2, CvssV3, CvssV4, Epss, SsvcV1};
use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, VulnerabilityMetric, VulnerabilityTrait, get_metric_prop_name,
};
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

type ProductMetricsMap = HashMap<String, HashMap<(VulnerabilityMetric, Option<String>), Vec<String>>>;
fn gather_product_metrics(
    vulnerability: &impl VulnerabilityTrait,
    vulnerability_index: usize,
) -> Option<ProductMetricsMap> {
    let metrics = vulnerability.get_metrics();

    metrics?;

    let mut product_metrics: ProductMetricsMap = HashMap::new();
    for (metric_index, metric) in metrics.unwrap().iter().enumerate() {
        let content = metric.get_content();
        let mut present_metric_types = HashSet::<VulnerabilityMetric>::new();
        if content.has_ssvc() {
            present_metric_types.insert(SsvcV1);
        }
        if content.get_cvss_v2().is_some() {
            present_metric_types.insert(CvssV2);
        }
        if let Some(cvss_v3) = content.get_cvss_v3() {
            if let Some(version) = cvss_v3.get("version").and_then(|v| v.as_str()) {
                // Use as_str because otherwise additional quotation marks would be included
                present_metric_types.insert(CvssV3(version.to_owned()));
            }
        }
        if content.get_cvss_v4().is_some() {
            present_metric_types.insert(CvssV4);
        }
        if content.get_epss().is_some() {
            present_metric_types.insert(Epss);
        }

        for product_id in metric.get_products() {
            for metric_type in present_metric_types.iter() {
                product_metrics
                        .entry(product_id.to_owned())
                        .or_default()
                        // Distinguish by source and metric type to allow e.g., multiple CVSS scores from different sources
                        .entry((metric_type.to_owned(), metric.get_source().clone()))
                        .or_default()
                        .push(content.get_content_json_path(vulnerability_index, metric_index));
            }
        }
    }
    Some(product_metrics)
}

/// Test 6.1.7: Check for multiple identical metric types per vulnerability.
pub fn test_6_1_07_multiple_same_scores_per_product(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        let product_metrics = gather_product_metrics(vulnerability, vulnerability_index);
        if let Some(product_metrics) = product_metrics {
            for (p, metrics_map) in product_metrics.iter() {
                for ((metric_type, _), paths) in metrics_map.iter() {
                    if paths.len() > 1 {
                        for path in paths {
                            errors.get_or_insert_with(Vec::new).push(create_validation_error(
                                metric_type,
                                p,
                                path.to_owned(),
                            ));
                        }
                    }
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_07_multiple_same_scores_per_product(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_07_multiple_same_scores_per_product(doc)
    }
}

fn create_validation_error(score_type: &VulnerabilityMetric, product_id: &str, path: String) -> ValidationError {
    ValidationError {
        message: format!("Multiple {} scores are given for {}.", score_type, product_id),
        instance_path: format!("{}/{}", path, get_metric_prop_name(score_type.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_07() {
        // CSAF 2.0 case_01
        let case_01_v2_0 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/scores/0".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/scores/1".to_string(),
            ),
        ]);

        // CSAF 2.1 cases
        let case_01_v2_1 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
            ),
        ]);

        let case_02 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
            ),
        ]);

        let case_03 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV2,
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV2,
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
            ),
        ]);

        let case_04 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV4,
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV4,
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
            ),
        ]);

        let case_05 = Err(vec![
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.1".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/0/metrics/1/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/1/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV3("3.0".to_string()),
                "CSAFPID-9080700",
                "/vulnerabilities/1/metrics/2/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV2,
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV2,
                "CSAFPID-9080701",
                "/vulnerabilities/2/metrics/1/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV4,
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/0/content".to_string(),
            ),
            create_validation_error(
                &VulnerabilityMetric::CvssV4,
                "CSAFPID-9080701",
                "/vulnerabilities/3/metrics/1/content".to_string(),
            ),
        ]);

        // CSAF 2.0 has 3 test cases (01, 11, 12)
        TESTS_2_0.test_6_1_7.expect(
            case_01_v2_0,
            Ok(()), // case_11
            Ok(()), // case_12
        );

        // CSAF 2.1 has 13 test cases (01-05, 11-18)
        TESTS_2_1.test_6_1_7.expect(
            case_01_v2_1,
            case_02,
            case_03,
            case_04,
            case_05,
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
        );
    }
}
