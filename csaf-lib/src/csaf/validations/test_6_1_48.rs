use crate::csaf::getter_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use std::ops::Deref;
use crate::csaf::helpers::{CSAF_SSVC_DECISION_POINTS, DP_VAL_LOOKUP};

pub fn test_6_1_48_ssvc_decision_points(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let vulnerabilities = doc.get_vulnerabilities();

    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                match m.get_content().get_ssvc_v1() {
                    Ok(ssvc) => {
                        for (i_s, selection) in ssvc.selections.iter().enumerate() {
                            // Create the key for lookup in CSAF_SSVC_DECISION_POINTS
                            let (name, version) = (selection.name.deref().to_owned(), selection.version.deref().to_owned());
                            let dp_key = (name.clone(), version.clone());
                            match CSAF_SSVC_DECISION_POINTS.get(&dp_key) {
                                Some(dp) => {
                                    // Decision point exists, check namespace
                                    if dp.namespace.deref() != selection.namespace.deref() {
                                        return Err(ValidationError {
                                            message: format!(
                                                "The selection has a namespace ({}) that differs from the SSVC decision point '{}' (version {}) namespace ({})",
                                                selection.namespace.deref(), name, version, dp.namespace.deref()
                                            ),
                                            instance_path: format!(
                                                "/vulnerabilities/{}/metrics/{}/content/ssvc_v1/selections/{}",
                                                i_v, i_m, i_s
                                            ),
                                        })
                                    }

                                    // Get value indices of decision point
                                    let reference_indices = DP_VAL_LOOKUP.get(&dp_key).unwrap();
                                    // Index of last seen value
                                    let mut last_index: i32 = -1;
                                    // Check if all values exist and are correctly ordered
                                    for (i_val, value) in selection.values.iter().map(|v| v.deref()).enumerate() {
                                        match reference_indices.get(value) {
                                            None => return Err(ValidationError {
                                                message: format!(
                                                    "The SSVC decision point '{}' (version {}) doesn't have the value '{}'",
                                                    name, version, value
                                                ),
                                                instance_path: format!(
                                                    "/vulnerabilities/{}/metrics/{}/content/ssvc_v1/selections/{}/values/{}",
                                                    i_v, i_m, i_s, i_val
                                                ),
                                            }),
                                            Some(i_dp_val) => {
                                                if last_index > *i_dp_val {
                                                    return Err(ValidationError {
                                                        message: format!(
                                                            "The values for SSVC decision point '{}' (version {}) are not in correct order",
                                                            name, version
                                                        ),
                                                        instance_path: format!(
                                                            "/vulnerabilities/{}/metrics/{}/content/ssvc_v1/selections/{}/values/{}",
                                                            i_v, i_m, i_s, i_val
                                                        ),
                                                    });
                                                } else {
                                                    last_index = *i_dp_val;
                                                }
                                            }
                                        }
                                    }
                                },
                                None => {
                                    return Err(ValidationError {
                                        message: format!(
                                            "Unknown SSVC decision point '{}' with version '{}'",
                                            name, version
                                        ),
                                        instance_path: format!(
                                            "/vulnerabilities/{}/metrics/{}/content/ssvc_v1/selections/{}",
                                            i_v, i_m, i_s
                                        ),
                                    });
                                }
                            }
                        }
                    },
                    Err(err) => {
                        return Err(ValidationError {
                            message: format!("Invalid SSVC object: {}", err),
                            instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1", i_v, i_m),
                        });
                    },
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_48::test_6_1_48_ssvc_decision_points;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_48() {
        let instance_path = "/vulnerabilities/0/metrics/0/content/ssvc_v1/selections/0".to_string();

        run_csaf21_tests(
            "48",
            test_6_1_48_ssvc_decision_points,
            HashMap::from([
                ("01", &ValidationError {
                    message: "The SSVC decision point 'Mission Impact' (version 1.0.0) doesn't have the value 'Degraded'".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1/selections/0/values/1".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Unknown SSVC decision point 'Safety Impacts' with version '1.0.0'".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("03", &ValidationError {
                    message: "The SSVC decision point 'Safety Impact' (version 1.0.0) doesn't have the value 'Critical'".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1/selections/0/values/1".to_string(),
                }),
                ("04", &ValidationError {
                    message: "Unknown SSVC decision point 'Safety Impact' with version '1.9.7'".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("05", &ValidationError {
                    message: "The SSVC decision point 'Attack Complexity' (version 3.0.1) doesn't have the value 'Easy'".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1/selections/0/values/0".to_string(),
                }),
                ("06", &ValidationError {
                    message: "The values for SSVC decision point 'Exploit Maturity' (version 2.0.0) are not in correct order".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1/selections/0/values/1".to_string(),
                }),
            ])
        );
    }
}