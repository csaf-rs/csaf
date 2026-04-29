use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_invalid_ssvc_error(error_message: &str, vulnerability_index: usize, metric_index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {error_message}"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/metrics/{metric_index}/content/ssvc_v2"),
    }
}

pub fn test_6_1_46_invalid_ssvc(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // /vulnerabilities[]/metrics[]/content/ssvc_v2
    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                if m.get_content().has_ssvc() {
                    m.get_content()
                        .get_ssvc()
                        .map_err(|e| vec![create_invalid_ssvc_error(&e.to_string(), i_v, i_m)])?;
                }
            }
        }
    }

    Ok(())
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_46, test_6_1_46_invalid_ssvc);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_46() {
        // Only CSAF 2.1 has this test with 4 test cases (2 error cases, 2 success cases)
        TESTS_2_1.test_6_1_46.expect(
            Err(vec![create_invalid_ssvc_error("missing field `selections`", 0, 0)]),
            Err(vec![create_invalid_ssvc_error("missing field `key`", 0, 0)]),
            Ok(()),
            Ok(()),
        );
    }
}
