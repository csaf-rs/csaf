use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::helpers::{DP_VAL_KEYS_LOOKUP, REGISTERED_SSVC_NAMESPACES, SSVC_DECISION_POINTS};
use crate::validation::ValidationError;
use std::ops::Deref;
#[allow(clippy::too_many_arguments)]
fn create_unknown_value_error(
    namespace: &str,
    dp_name: &str,
    version: &str,
    value_key: &str,
    i_v: usize,
    i_m: usize,
    i_s: usize,
    i_val: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The SSVC decision point '{}::{}' (version {}) doesn't have a value with key '{}'",
            namespace, dp_name, version, value_key
        ),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}/values/{}",
            i_v, i_m, i_s, i_val
        ),
    }
}

fn create_incorrect_order_error(
    namespace: &str,
    dp_name: &str,
    version: &str,
    i_v: usize,
    i_m: usize,
    i_s: usize,
    i_val: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The values for SSVC decision point '{}::{}' (version {}) are not in correct order",
            namespace, dp_name, version
        ),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}/values/{}",
            i_v, i_m, i_s, i_val
        ),
    }
}

fn create_unknown_decision_point_error(
    namespace: &str,
    key: &str,
    version: &str,
    i_v: usize,
    i_m: usize,
    i_s: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Unknown SSVC decision point '{}::{}' with version '{}'",
            namespace, key, version
        ),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/selections/{}",
            i_v, i_m, i_s
        ),
    }
}

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {}", error),
        instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2", i_v, i_m),
    }
}

pub fn test_6_1_48_ssvc_decision_points(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
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
                                        for (i_val, v_key) in selection.values.iter().map(|v| v.key.deref()).enumerate()
                                        {
                                            match reference_indices.get(v_key) {
                                                None => {
                                                    return Err(vec![create_unknown_value_error(
                                                        &namespace,
                                                        dp.name.deref(),
                                                        &version,
                                                        v_key,
                                                        i_v,
                                                        i_m,
                                                        i_s,
                                                        i_val,
                                                    )]);
                                                },
                                                Some(i_dp_val) => {
                                                    if last_index > *i_dp_val {
                                                        return Err(vec![create_incorrect_order_error(
                                                            &namespace,
                                                            dp.name.deref(),
                                                            &version,
                                                            i_v,
                                                            i_m,
                                                            i_s,
                                                            i_val,
                                                        )]);
                                                    } else {
                                                        last_index = *i_dp_val;
                                                    }
                                                },
                                            }
                                        }
                                    },
                                    None => {
                                        return Err(vec![create_unknown_decision_point_error(
                                            &namespace, &s_key, &version, i_v, i_m, i_s,
                                        )]);
                                    },
                                }
                            }
                        },
                        Err(err) => {
                            return Err(vec![create_invalid_ssvc_error(err, i_v, i_m)]);
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_48
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_48_ssvc_decision_points(doc)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;


    #[test]
    fn test_test_6_1_48() {
        let case_01 = Err(vec![create_unknown_value_error(
            "ssvc",
            "Mission Impact",
            "1.0.0",
            "D",
            0,
            0,
            0,
            1,
        )]);
        let case_02 = Err(vec![create_unknown_decision_point_error(
            "ssvc", "SIs", "2.0.0", 0, 0, 0,
        )]);
        let case_03 = Err(vec![create_incorrect_order_error(
            "ssvc",
            "Safety Impact",
            "2.0.0",
            0,
            0,
            0,
            1,
        )]);
        let case_04 = Err(vec![create_unknown_decision_point_error(
            "ssvc", "SI", "1.9.7", 0, 0, 0,
        )]);
        let case_05 = Err(vec![create_unknown_value_error(
            "cvss",
            "Attack Complexity",
            "3.0.1",
            "E",
            0,
            0,
            0,
            0,
        )]);
        let case_06 = Err(vec![create_unknown_decision_point_error("cvss", "E", "3.0.1", 0, 0, 0)]);

        // Only CSAF 2.1 has this test with 20 test cases (6 error cases, 14 success cases)
        // Note: Cases 07, 08, 09, 21 deal with complex SSVC namespace rules, currently skipped
        // Note: Case 16 has no Exploit Maturity (E) decision point version 3.0.1, currently skipped
        // Note: Case 31 has erroneous JSON field "description", currently skipped

        TESTS_2_1.test_6_1_48.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            Ok(()), // case_07 - complex SSVC namespace rules, skipped
            Ok(()), // case_08 - complex SSVC namespace rules, skipped
            Ok(()), // case_09 - complex SSVC namespace rules, skipped
            Ok(()), // case_21 - complex SSVC namespace rules, skipped
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16 - no Exploit Maturity E v3.0.1, skipped
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
            Ok(()), // case_31 - erroneous JSON, skipped
        );
    }
}
*/
