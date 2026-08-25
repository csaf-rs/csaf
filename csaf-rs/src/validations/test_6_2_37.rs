use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::{create_generic_namespace_finding_data, ssvc_decision_point_resources_path};

fn create_missing_ssvc_resource_warning(namespace: &str, instance_path: &str) -> TestFinding {
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
    allow_test_namespaces: bool,
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
                let parsed = ssvc::validate_namespace(namespace, allow_test_namespaces);
                match parsed {
                    Ok(parsed_namespace) => {
                        // skip registered namespaces
                        if parsed_namespace.is_registered() {
                            continue;
                        }

                        // check if there is a resource containing the full namespace
                        let has_resource_with_full_namespace = resources.iter().any(|r| r.summary.contains(namespace));
                        // there should be at least one, push a finding if not
                        if !has_resource_with_full_namespace {
                            errors
                                .get_or_insert_default()
                                .push(create_missing_ssvc_resource_warning(namespace, &instance_path));
                        }
                    },
                    Err(err) => {
                        // Reserved forbidden namespaces "invalid" or "test" are used
                        if matches!(
                            err,
                            ssvc::NamespaceError::ReservedForbiddenNamespace { .. }
                                | ssvc::NamespaceError::ReservedTestNamespace { .. }
                        ) {
                            errors.get_or_insert_default().push(TestFinding::Warning(
                                create_generic_namespace_finding_data(&err, &instance_path),
                            ));
                        }
                    },
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_37
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<TestFinding>> {
        #[cfg(test)]
        let allow_test_namespaces = true;
        #[cfg(not(test))]
        let allow_test_namespaces = false;
        test_6_2_37_usage_of_unknown_ssvc_decision_point_namespace_without_resource(doc, allow_test_namespaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_37 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_37() {
        let resources_path = ssvc_decision_point_resources_path(0, 0);

        let case_01_resource_for_different_ns = Err(vec![create_missing_ssvc_resource_warning(
            "x_example.test#without-resource/de-DE",
            &resources_path,
        )]);
        // Case 12: two unregistered ns, one with resource, one without, one with reserved "test" namespace
        let case_02_multiple_including_test = Err(vec![
            create_missing_ssvc_resource_warning("x_example.test#without-resource/en-AU", &resources_path),
        ]);
        let case_03_no_resource = Err(vec![create_missing_ssvc_resource_warning(
            "x_example.test#without-resource/de-AT",
            &resources_path,
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
