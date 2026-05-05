use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_invalid_ssvc_error(error_message: &str, vulnerability_index: usize, metric_index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {error_message}"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/metrics/{metric_index}/content/ssvc_v2"),
    }
}

pub fn test_6_1_46_invalid_ssvc(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                let content = m.get_content();
                if content.has_ssvc()
                    && let Err(e) = content.get_ssvc()
                {
                    errors
                        .get_or_insert_default()
                        .push(create_invalid_ssvc_error(&e.to_string(), i_v, i_m));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_46, test_6_1_46_invalid_ssvc);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_46() {
        // Case 01: selections object is missing
        // Case 02: key in selections object is missing
        // Case 11: minimal valid ssvc
        // Case 12: valid ssvc

        TESTS_2_1.test_6_1_46.expect(
            Err(vec![create_invalid_ssvc_error("missing field `selections`", 0, 0)]),
            Err(vec![create_invalid_ssvc_error("missing field `key`", 0, 0)]),
            Ok(()),
            Ok(()),
        );
    }
}
