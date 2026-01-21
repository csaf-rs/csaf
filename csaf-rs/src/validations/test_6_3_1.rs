use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityMetric, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

fn create_cvss_v2_only_error(instance_path: String) -> ValidationError {
    ValidationError {
        message: "Vulnerability uses CVSS v2 as the only scoring system".to_string(),
        instance_path,
    }
}

/// 6.3.1 Use of CVSS v2 as the only Scoring System
///
/// For each vulnerability, tests if in the scores / metrics, CVSS v2 is not the only scoring system used.
pub fn test_6_3_1_use_of_cvss_v2_as_only_scoring_system(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each vuln
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // generate a map of each product to the set of vulnerability metrics used for it
        let mut product_metrics_map: HashMap<String, HashSet<VulnerabilityMetric>> =
            HashMap::<String, HashSet<VulnerabilityMetric>>::new();
        // generate a map of each product to the paths where it was encountered
        let mut product_path_map: HashMap<String, HashSet<String>> = HashMap::<String, HashSet<String>>::new();
        // for each metric and each product in it
        if let Some(metrics) = vuln.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                for product in metric.get_products() {
                    // add all vulnerability metrics of this metric to the product -> vulnerability metrics map
                    for vulnerability_metric in content.get_vulnerability_metric_types() {
                        product_metrics_map
                            .entry(product.to_string())
                            .or_default()
                            .insert(vulnerability_metric);
                    }
                    // add the path of this metric to the product -> paths map
                    product_path_map
                        .entry(product.to_string())
                        .or_default()
                        .insert(content.get_content_json_path(v_i, m_i));
                }
            }
        }
        // for each product that has only CVSS v2 as vulnerability metric,
        for (product, metrics_set) in product_metrics_map.iter() {
            if metrics_set.len() == 1 && metrics_set.contains(&VulnerabilityMetric::CvssV2) {
                // create an error for each path it was encountered at
                if let Some(paths) = product_path_map.get(product) {
                    for path in paths {
                        errors
                            .get_or_insert_with(Vec::new)
                            .push(create_cvss_v2_only_error(path.clone()));
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_3_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_1_use_of_cvss_v2_as_only_scoring_system(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_3_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_1_use_of_cvss_v2_as_only_scoring_system(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_1() {
        // CSAF 2.0 has 4 test cases and CSAF 2.1 has 8 test cases
        TESTS_2_0.test_6_3_1.expect(
            Err(vec![create_cvss_v2_only_error(
                "/vulnerabilities/0/scores/0".to_string(),
            )]),
            Err(vec![
                create_cvss_v2_only_error("/vulnerabilities/0/scores/0".to_string()),
                create_cvss_v2_only_error("/vulnerabilities/2/scores/0".to_string()),
            ]),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_3_1.expect(
            Err(vec![create_cvss_v2_only_error(
                "/vulnerabilities/0/metrics/0/content".to_string(),
            )]),
            Err(vec![
                create_cvss_v2_only_error("/vulnerabilities/0/metrics/0/content".to_string()),
                create_cvss_v2_only_error("/vulnerabilities/2/metrics/0/content".to_string()),
            ]),
            Err(vec![
                create_cvss_v2_only_error("/vulnerabilities/0/metrics/0/content".to_string()),
                create_cvss_v2_only_error("/vulnerabilities/3/metrics/0/content".to_string()),
            ]),
            Err(vec![create_cvss_v2_only_error(
                "/vulnerabilities/2/metrics/0/content".to_string(),
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
