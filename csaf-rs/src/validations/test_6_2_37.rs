use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::create_other_namespace_error;

fn ssvc_decision_point_resources_path(vuln_index: usize, metric_index: usize) -> String {
    format!("/vulnerabilities/{vuln_index}/metrics/{metric_index}/content/ssvc_v2/decision_point_resources")
}

fn create_missing_resource_error(namespace: &str, instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Missing decision point resource for SSVC decision point with unregistered namespace: `{namespace}`"
        ),
        instance_path: instance_path.to_owned(),
    })
}

/// 6.2.37 Usage of Unknown SSVC Decision Point Namespace without Resource
///
/// For each SSVC object containing a decision point with a full namespace that is not registered,
/// it MUST be tested that a Decision Point Resource exists for each one that provides additional
/// context about the decision points from this namespace. Namespaces reserved for special purpose
/// MUST be treated as per their definition.
pub fn test_6_2_37_usage_of_unknown_ssvc_decision_point_namespace_without_resource(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (vuln_index, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        for (metric_index, metric) in vuln.get_metrics().into_iter().flatten().enumerate() {
            if !metric.get_content().has_ssvc_v2() {
                continue;
            }
            // Parse failures are reported by test 6.1.46
            let Ok(selection_list) = metric.get_content().get_ssvc_v2() else {
                continue;
            };

            let instance_path = ssvc_decision_point_resources_path(vuln_index, metric_index);
            let resources = &selection_list.decision_point_resources;

            for selection in &selection_list.selections {
                let namespace = selection.namespace.as_str();
                let parsed = ssvc::validate_namespace(namespace, false);
                match parsed {
                    Ok(parsed_namespace) => {
                        if !parsed_namespace.is_unregistered() {
                            continue;
                        }

                        let has_resource = resources.iter().any(|r| r.summary.contains(namespace));
                        if !has_resource {
                            errors
                                .get_or_insert_default()
                                .push(create_missing_resource_error(namespace, &instance_path));
                        }
                    },
                    Err(err) => {
                        // Reserved forbidden namespaces "invalid" or "test" are used
                        if matches!(
                            err,
                            ssvc::NamespaceError::ReservedForbiddenNamespace { .. }
                                | ssvc::NamespaceError::ReservedTestNamespace { .. }
                        ) {
                            errors
                                .get_or_insert_default()
                                .push(TestFinding::Warning(create_other_namespace_error(&err, &instance_path)));
                        }
                    },
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_37,
    test_6_2_37_usage_of_unknown_ssvc_decision_point_namespace_without_resource
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_37 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use ssvc::NamespaceError;

    fn resources_path() -> String {
        ssvc_decision_point_resources_path(0, 0)
    }

    #[test]
    fn test_test_6_2_37() {
        let case_01_resource_for_different_ns = Err(vec![create_missing_resource_error(
            "x_example.test#without-resource/de-DE",
            &resources_path(),
        )]);
        let case_02_multiple_including_test = Err(vec![
            create_missing_resource_error("x_example.test#without-resource/en-AU", &resources_path()),
            TestFinding::Warning(create_other_namespace_error(
                &NamespaceError::ReservedTestNamespace {
                    namespace: "test".to_string(),
                },
                &resources_path(),
            )),
        ]);
        let case_03_no_resource = Err(vec![create_missing_resource_error(
            "x_example.test#without-resource/de-AT",
            &resources_path(),
        )]);

        // Case 11: resource summary contains the full namespace
        // Case 12: all unregistered namespaces covered
        // Case 13: registered namespace "ssvc"

        TESTS_2_1.test_6_2_37.expect(ExpectedResults {
            case_01: case_01_resource_for_different_ns,
            case_02: case_02_multiple_including_test,
            case_03: case_03_no_resource,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
