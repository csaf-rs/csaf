use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_unregistered_base_namespace_error(namespace: &str, i_v: usize, i_m: usize, i_s: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Usage of unregistered SSVC decision point base namespace: `{namespace}`"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/selections/{i_s}/namespace"),
    })
}

fn create_unknown_invalid_base_namespace_error(reason: &str, i_v: usize, i_m: usize, i_s: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Usage of unknown or reserved registered SSVC decision point base namespace: {reason}"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/selections/{i_s}/namespace"),
    })
}

/// 6.2.34 Usage of Unknown SSVC Decision Point Base Namespace
///
/// For each SSVC decision point given under `selections`, it MUST be tested that the base
/// namespace is a registered one. Namespaces reserved for special purpose MUST be treated as per
/// their definition.
///
/// This test fails on unregistered namespaces (identified by the `x_` prefix) as well as on
/// registered-looking namespaces that are not (yet) registered/supported.
pub fn test_6_2_34_usage_of_unknown_ssvc_decision_point_base_namespace(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                let content = m.get_content();
                if !content.has_ssvc_v2() {
                    continue;
                }
                let Ok(selection_list) = content.get_ssvc_v2() else {
                    // Invalid SSVC objects are reported by test 6.1.46
                    continue;
                };

                // iterate all selection list items
                for (i_s, selection) in selection_list.selections.iter().enumerate() {
                    let namespace = selection.namespace.as_str();
                    // validate the selection list item's namespace
                    match ssvc::validate_namespace(namespace, false) {
                        // its unregistered (prefixed with x_)
                        Ok(parsed) if parsed.is_unregistered() => {
                            errors
                                .get_or_insert_default()
                                .push(create_unregistered_base_namespace_error(namespace, i_v, i_m, i_s));
                        },
                        // its registered, all good
                        Ok(_) => {},
                        // its otherwise malformed or invalid, e.g.:
                        // * reserved namespaces "invalid" or "test"
                        // * not a "known" registered namespace to SSVC
                        Err(err) => {
                            errors
                                .get_or_insert_default()
                                .push(create_unknown_invalid_base_namespace_error(
                                    err.to_string().as_str(),
                                    i_v,
                                    i_m,
                                    i_s,
                                ));
                        },
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_34,
    test_6_2_34_usage_of_unknown_ssvc_decision_point_base_namespace
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_34 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_34() {
        let case_01_unregistered_ns = Err(vec![create_unregistered_base_namespace_error(
            "x_example.unregistered#some-yet-unknown-or-maybe-private-namespace",
            0,
            0,
            0,
        )]);
        let case_02_unregistered_ns_with_ext = Err(vec![create_unregistered_base_namespace_error(
            "x_example.test#also-unregistered-namespace//.example.other-test#some-extension",
            0,
            0,
            0,
        )]);
        let case_03_reserved_forbidden_ns = Err(vec![create_unknown_invalid_base_namespace_error(
            "Reserved forbidden namespace 'invalid' must not be used",
            0,
            0,
            0,
        )]);

        // Case 11: registered namespace ("ssvc")
        // Case 12: registered namespace with an extension
        // Case 13: reserved "example" namespace, valid for documentation purposes

        TESTS_2_1.test_6_2_34.expect(ExpectedResults {
            case_01: case_01_unregistered_ns,
            case_02: case_02_unregistered_ns_with_ext,
            case_03: case_03_reserved_forbidden_ns,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
