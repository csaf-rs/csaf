use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

/// The actual test in run on a 3 level nested map.
/// The outer map maps product ids to a second map [ProductCsafVulnerabilityMetricMap], which maps to a third map [ProductCsafVulnerabilityMetricSourceMap].
type ProductMap = HashMap<String, ProductCsafVulnerabilityMetricMap>;
/// The second map maps a metric type (e.g., CVSS v3.1) to a third map
type ProductCsafVulnerabilityMetricMap = HashMap<CsafVulnerabilityMetric, ProductCsafVulnerabilityMetricSourceMap>;
/// The innermost map maps a source (None for CSAF 2.0 or if None is provided) to a list of JSON paths where this metric type is given for the product and source
type ProductCsafVulnerabilityMetricSourceMap = HashMap<Option<String>, Vec<String>>;
fn gather_product_metrics(vulnerability: &impl VulnerabilityTrait, vulnerability_index: usize) -> Option<ProductMap> {
    let metrics = vulnerability.get_metrics();

    metrics?;

    let mut product_metrics: ProductMap = HashMap::new();
    for (metric_index, metric) in metrics.unwrap().iter().enumerate() {
        let content = metric.get_content();
        let present_metric_types = content.get_vulnerability_metric_types();

        for product_id in metric.get_products() {
            for metric_type in present_metric_types.iter() {
                product_metrics
                    // First map: product id =>
                    .entry(product_id.to_owned())
                    .or_default()
                    // Second map: metric type =>
                    .entry(metric_type.to_owned())
                    .or_default()
                    // Third map: source (or none) =>
                    .entry(metric.get_source().as_ref().map(|s| s.to_owned()))
                    .or_default()
                    // vector with json paths that provide this metric type for this product and source
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
                for (m, metrics_map_2) in metrics_map.iter() {
                    for (s, paths) in metrics_map_2.iter() {
                        if paths.len() > 1 {
                            for path in paths {
                                errors.get_or_insert_with(Vec::new).push(create_validation_error(
                                    m,
                                    p,
                                    path.to_owned(),
                                    s.clone(),
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
        // CSAF 2.0 has 3 test cases (01, 11, 12)
        TESTS_2_0.test_6_1_7.expect(
            Err(vec![
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
            ]),
            Ok(()), // case_11
            Ok(()), // case_12
        );

        // CSAF 2.1 has 13 test cases (01-06, 11-18)
        TESTS_2_1.test_6_1_7.expect(
            // case 01
            Err(vec![
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
            ]),
            // case 02
            Err(vec![
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
            ]),
            // case 03
            Err(vec![
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
            ]),
            // case 04
            Err(vec![
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
            ]),
            // case 05
            Err(vec![
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
            ]),
            // case 06
            Err(vec![
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
            ]),
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
