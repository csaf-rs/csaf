use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_metrics_extension_info(vulnerability_index: usize, metric_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The metrics content uses a CSAF Extension.".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/metrics/{metric_index}/content/x_extensions"),
    })
}

/// 6.3.21.7 Usage of Extension in Vulnerabilities Metrics Path
///
/// It SHALL be tested that the element `x_extensions` does not exist in any path that starts with
/// `$.vulnerabilities[*].metrics`.
pub fn test_6_3_21_7_usage_of_extension_in_vulnerabilities_metrics_path(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        for (metric_index, metric) in vulnerability.get_metrics().into_iter().flatten().enumerate() {
            if metric.get_content().get_extensions().is_some() {
                findings
                    .get_or_insert_default()
                    .push(create_metrics_extension_info(vulnerability_index, metric_index));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_7,
    test_6_3_21_7_usage_of_extension_in_vulnerabilities_metrics_path
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_7 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_7() {
        // Case 11: extension at root, but not in vulnerabilities metrics
        TESTS_2_1.test_6_3_21_7.expect(ExpectedResults {
            case_01: Err(vec![create_metrics_extension_info(0, 0)]),
            case_11: Ok(()),
        });
    }
}
