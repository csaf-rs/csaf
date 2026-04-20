use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusAndPath, ProductStatusGroup, ProductStatusGroupMap,
    VulnerabilityTrait,
};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn create_missing_cvss_v4_error(instance_path: String, cvss_versions: &[CsafVulnerabilityMetric]) -> ValidationError {
    let versions_str = cvss_versions
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    ValidationError {
        message: format!("The metric contains {versions_str} but does not include a CVSS v4.0 score."),
        instance_path,
    }
}

fn create_affected_product_not_covered_error(product_id: &str, instance_path: String) -> ValidationError {
    ValidationError {
        message: format!("Affected product {product_id} is not covered by any CVSS score."),
        instance_path,
    }
}

/// 6.3.12 Missing CVSS v4.0
///
/// For each item in the list of metrics that contains any CVSS object it MUST be tested that a
/// `cvss_v4` object is present.
///
/// The test MUST fail, if any Product ID (type `/$defs/product_id_t`) in the product status group
/// Affected is not covered by any CVSS object.
///
/// This is essentially two tests at once for each vulnerability. We generate separate error messages for both.
pub fn test_6_3_12_missing_cvss_v4(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // collect product IDs covered by any CVSS object across all metrics of this vulnerability
        let mut products_covered_by_cvss: HashSet<String> = HashSet::new();

        if let Some(metrics) = vulnerability.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                // aggregate if any cvss score is present
                let has_any_cvss = content.has_cvss_v2() || content.has_cvss_v3() || content.has_cvss_v4();

                if has_any_cvss {
                    // collect products covered by this CVSS metric
                    for product_id in metric.get_products() {
                        products_covered_by_cvss.insert(product_id.clone());
                    }

                    // check that cvss_v4 is present
                    if !content.has_cvss_v4() {
                        let path = content.get_content_json_path(v_i, m_i);
                        let cvss_types = content.get_cvss_metric_types();
                        errors
                            .get_or_insert_default()
                            .push(create_missing_cvss_v4_error(path, &cvss_types));
                    }
                }
            }
        }

        // check that all affected products (first_affected, known_affected, last_affected) are covered by at least one CVSS object
        if let Some(product_status) = vulnerability.get_product_status() {
            let status_map = ProductStatusGroupMap::from(product_status);
            if let Some(affected) = status_map.get(&ProductStatusGroup::Affected) {
                let mut affected_entries: Vec<&ProductStatusAndPath> = affected.iter().collect();
                // Sort by status and index to produce deterministic error ordering
                affected_entries.sort_by(|a, b| a.status.cmp(&b.status).then(a.index.cmp(&b.index)));

                for entry in affected_entries {
                    if !products_covered_by_cvss.contains(&entry.product_id) {
                        errors
                            .get_or_insert_default()
                            .push(create_affected_product_not_covered_error(
                                &entry.product_id,
                                entry.json_path(v_i),
                            ));
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_3_12, test_6_3_12_missing_cvss_v4);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_12() {
        let case_01_cvss_v3_1_only = Err(vec![create_missing_cvss_v4_error(
            "/vulnerabilities/0/metrics/0/content".to_string(),
            &[CsafVulnerabilityMetric::CvssV3("3.1".to_string())],
        )]);

        let case_02_cvss_v3_0_only = Err(vec![create_missing_cvss_v4_error(
            "/vulnerabilities/0/metrics/0/content".to_string(),
            &[CsafVulnerabilityMetric::CvssV3("3.0".to_string())],
        )]);

        let case_03_cvss_v2_only = Err(vec![create_missing_cvss_v4_error(
            "/vulnerabilities/0/metrics/0/content".to_string(),
            &[CsafVulnerabilityMetric::CvssV2("2.0".to_string())],
        )]);

        let case_04_multiple_vulns_two_without_cvss_v4 = Err(vec![
            create_missing_cvss_v4_error(
                "/vulnerabilities/0/metrics/0/content".to_string(),
                &[
                    CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                    CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                ],
            ),
            create_missing_cvss_v4_error(
                "/vulnerabilities/2/metrics/0/content".to_string(),
                &[
                    CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                    CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                ],
            ),
        ]);

        // Case 05:
        // Vuln 0: epss only
        // Vuln 1: v2,3,4 for 9080700, but affected 9080701 not covered
        // Vuln 2: v2,3 for 9080700, and affected 9080701 not covered
        let case_05_uncovered_affected = Err(vec![
            create_affected_product_not_covered_error(
                "CSAFPID-9080701",
                "/vulnerabilities/1/product_status/first_affected/1".to_string(),
            ),
            create_missing_cvss_v4_error(
                "/vulnerabilities/2/metrics/0/content".to_string(),
                &[
                    CsafVulnerabilityMetric::CvssV2("2.0".to_string()),
                    CsafVulnerabilityMetric::CvssV3("3.1".to_string()),
                ],
            ),
            create_affected_product_not_covered_error(
                "CSAFPID-9080701",
                "/vulnerabilities/2/product_status/known_affected/1".to_string(),
            ),
        ]);

        let case_s01_last_affected_not_covered = Err(vec![create_affected_product_not_covered_error(
            "CSAFPID-9080701",
            "/vulnerabilities/0/product_status/last_affected/1".to_string(),
        )]);

        // Case 11: v3.1 and v4
        // Case 12: v3.0 and v4
        // Case 13: v2 and v4
        // Case 14: multiple with v2,3,4
        // Case 15: only v4
        // Case 16: like 05, but
        // Vuln 1: with both products covered
        // Vuln 2: with 2 metrics, the second one covering both products

        TESTS_2_1.test_6_3_12.expect(
            case_01_cvss_v3_1_only,
            case_02_cvss_v3_0_only,
            case_03_cvss_v2_only,
            case_04_multiple_vulns_two_without_cvss_v4,
            case_05_uncovered_affected,
            case_s01_last_affected_not_covered,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
