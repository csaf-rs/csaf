use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

/// Creates a ValidationError for invalid SSVC objects
fn create_invalid_ssvc_error(error_message: &str, vulnerability_index: usize, metric_index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {}", error_message),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2",
            vulnerability_index, metric_index
        ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_46() {
        run_csaf21_tests(
            "46",
            test_6_1_46_invalid_ssvc,
            HashMap::from([
                (
                    "01",
                    vec![create_invalid_ssvc_error("missing field `selections`", 0, 0)],
                ),
                ("02", vec![create_invalid_ssvc_error("missing field `key`", 0, 0)]),
            ]),
        );
    }
}
