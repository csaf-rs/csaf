use crate::csaf::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::csaf::helpers::{DP_VAL_KEYS_LOOKUP, REGISTERED_SSVC_NAMESPACES, SSVC_DECISION_POINTS};
use crate::csaf::validation::ValidationError;
use std::ops::Deref;

pub fn test_6_1_48_ssvc_decision_points(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                if m.get_content().has_ssvc() {
                    match m.get_content().get_ssvc() {
                        Ok(ssvc) => {
                            for (i_s, selection) in ssvc.selections.iter().enumerate() {
                                // Skip this test for unregistered namespaces
                                if !REGISTERED_SSVC_NAMESPACES.contains(selection.namespace.deref()) {
                                    continue;
                                }

                                // Create the key for lookup in CSAF_SSVC_DECISION_POINTS
                                let (namespace, s_key, version) = (
                                    selection.namespace.deref().to_owned(),
                                    selection.key.deref().to_owned(),
                                    selection.version.deref().to_owned(),
                                );
                                let dp_key = (namespace.clone(), s_key.clone(), version.clone());
                                match SSVC_DECISION_POINTS.get(&dp_key) {
                                    Some(dp) => {
                                        // Get value indices of decision point
                                        let reference_indices = DP_VAL_KEYS_LOOKUP.get(&dp_key).unwrap();
                                        // Index of last-seen value
                                        let mut last_index: i32 = -1;
                                        // Check if all values exist and are correctly ordered
                                        for (i_val, v_key) in selection.values.iter().map(|v| v.key.deref()).enumerate() {
                                            match reference_indices.get(v_key) {
                                                None => return Err(vec![ValidationError {
                                                    message: format!(
                                                        "The SSVC decision point '{}::{}' (version {}) doesn't have a value with key '{}'",
                                                        namespace, dp.name.deref(), version, v_key
                                                    ),
                                                    instance_path: format!(
                                                        "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}/values/{}",
                                                        i_v, i_m, i_s, i_val
                                                    ),
                                                }]),
                                                Some(i_dp_val) => {
                                                    if last_index > *i_dp_val {
                                                        return Err(vec![ValidationError {
                                                            message: format!(
                                                                "The values for SSVC decision point '{}::{}' (version {}) are not in correct order",
                                                                namespace, dp.name.deref(), version
                                                            ),
                                                            instance_path: format!(
                                                                "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}/values/{}",
                                                                i_v, i_m, i_s, i_val
                                                            ),
                                                        }])
                                                    } else {
                                                        last_index = *i_dp_val;
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    None => {
                                        return Err(vec![ValidationError {
                                            message: format!(
                                                "Unknown SSVC decision point '{}::{}' with version '{}'",
                                                namespace, s_key, version
                                            ),
                                            instance_path: format!(
                                                "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}",
                                                i_v, i_m, i_s
                                            ),
                                        }]);
                                    }
                                }
                            }
                        },
                        Err(err) => {
                            return Err(vec![ValidationError {
                                message: format!("Invalid SSVC object: {}", err),
                                instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2", i_v, i_m),
                            }]);
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests_with_excludes;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_48::test_6_1_48_ssvc_decision_points;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_48() {
        let instance_path = "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0".to_string();

        run_csaf21_tests_with_excludes(
            "48",
            test_6_1_48_ssvc_decision_points,
            &HashMap::from([
                ("01", &ValidationError {
                    message: "The SSVC decision point 'ssvc::Mission Impact' (version 1.0.0) doesn't have a value with key 'D'".to_string(),
                    instance_path: format!("{}/values/1", instance_path),
                }),
                ("02", &ValidationError {
                    message: "Unknown SSVC decision point 'ssvc::SIs' with version '2.0.0'".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("03", &ValidationError {
                    message: "The values for SSVC decision point 'ssvc::Safety Impact' (version 2.0.0) are not in correct order".to_string(),
                    instance_path: format!("{}/values/1", instance_path),
                }),
                ("04", &ValidationError {
                    message: "Unknown SSVC decision point 'ssvc::SI' with version '1.9.7'".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("05", &ValidationError {
                    message: "The SSVC decision point 'cvss::Attack Complexity' (version 3.0.1) doesn't have a value with key 'E'".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/values/0".to_string(),
                }),
                ("06", &ValidationError {
                    message: "Unknown SSVC decision point 'cvss::E' with version '3.0.1'".to_string(),
                    instance_path: instance_path.clone(),
                }),
            ]),
            // Tests 07, 08, 09, 21 deal with complex SSVC namespace rules, skipped for now.
            // Test 16: There seems to be no Exploit Maturity (E) decision point version 3.0.1 in the SSVC repository, skipped for now.
            // Test 31: Erroneous JSON field "description", skipped for now.
            &["07", "08", "09", "21", "16", "31"]
        );
    }
}