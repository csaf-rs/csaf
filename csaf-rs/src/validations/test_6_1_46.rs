use std::sync::LazyLock;

use jsonschema::Validator;

use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use crate::validations::utils::validation_schemas::SSVC_2_SCHEMA;

static SSVC_VALIDATOR: LazyLock<Validator> = LazyLock::new(|| jsonschema::draft202012::new(&SSVC_2_SCHEMA).unwrap());

fn create_invalid_ssvc_error(
    error_message: &str,
    error_path: &str,
    vulnerability_index: usize,
    metric_index: usize,
) -> ValidationError {
    let path_info = if error_path.is_empty() {
        String::new()
    } else {
        format!(" at {error_path}")
    };
    ValidationError {
        message: format!("Invalid SSVC object{path_info}: {error_message}"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/metrics/{metric_index}/content/ssvc_v2{error_path}"),
    }
}

/// 6.1.46 Invalid SSVC
///
/// It MUST be tested that the given SSVC object is valid according to the referenced schema.
pub fn test_6_1_46_invalid_ssvc(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    // look for ssvc_v2 metrics in all vulnerabilities and metrics
    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                let content = m.get_content();
                // if there is a ssvc_v2 metric
                if let Some(ssvc) = content.get_ssvc_v2_raw() {
                    // schema validation
                    // depending on how we implement lenient parsing, this might need to be
                    // prefaced with json format check
                    for error in SSVC_VALIDATOR.iter_errors(&serde_json::Value::Object(ssvc.clone())) {
                        errors.get_or_insert_default().push(create_invalid_ssvc_error(
                            &error.to_string(),
                            error.instance_path().as_str(),
                            i_v,
                            i_m,
                        ));
                    }
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
            Err(vec![create_invalid_ssvc_error(
                "\"selections\" is a required property",
                "",
                0,
                0,
            )]),
            Err(vec![
                create_invalid_ssvc_error("\"key\" is a required property", "/selections/0", 0, 0),
                create_invalid_ssvc_error("\"key\" is a required property", "/selections/0/values/0", 0, 0),
            ]),
            Ok(()),
            Ok(()),
        );
    }
}
