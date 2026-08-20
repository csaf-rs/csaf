use crate::csaf_traits::{
    ContentTrait, CsafTrait, DistributionTrait, DocumentTrait, MetricTrait, TlpTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::create_other_namespace_error;
use ssvc::NamespaceError;

fn create_namespace_extension_in_non_tlp_clear_error(
    namespace: &str,
    i_v: usize,
    i_m: usize,
    i_s: usize,
) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "Usage of SSVC decision point namespace with an extension in a non-TLP:CLEAR document: `{namespace}`"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/selections/{i_s}/namespace"),
    })
}

/// 6.3.15 Usage of SSVC Decision Point Namespace with Extension in Non-TLP:CLEAR Document
///
/// For each SSVC decision point given under `selections`, it MUST be tested that the namespace
/// does not use an extension if the document is not labeled TLP:CLEAR. Namespaces reserved for
/// special purpose MUST be treated as per their definition.
pub fn test_6_3_15_usage_of_ssvc_decision_point_namespace_with_extension_in_non_tlp_clear_document(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    // This test only applies to non-TLP:CLEAR documents
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;
    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() == LabelOfTlp::Clear {
        return Ok(());
    }

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

                for (i_s, selection) in selection_list.selections.iter().enumerate() {
                    let namespace = selection.namespace.as_str();
                    match ssvc::validate_namespace(namespace, false) {
                        // check if an extension exists on a valid namespace
                        Ok(parsed_namespace) if parsed_namespace.extensions.is_some() => errors
                            .get_or_insert_default()
                            .push(create_namespace_extension_in_non_tlp_clear_error(
                                namespace, i_v, i_m, i_s,
                            )),
                        // there is no extension
                        Ok(_) => continue,
                        // reserved forbidden namespaces "invalid" or "test" are used
                        Err(err)
                            if matches!(
                                err,
                                NamespaceError::ReservedForbiddenNamespace { .. }
                                    | NamespaceError::ReservedTestNamespace { .. }
                            ) =>
                        {
                            errors.get_or_insert_default().push(TestFinding::Information(
                                create_other_namespace_error(&err, i_v, i_m, i_s),
                            ));
                        },
                        // all other namespace errors
                        Err(_) => continue,
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_15,
    test_6_3_15_usage_of_ssvc_decision_point_namespace_with_extension_in_non_tlp_clear_document
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_15 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_15() {
        let case_01_extension_in_tlp_green = Err(vec![create_namespace_extension_in_non_tlp_clear_error(
            "ssvc//.example.test#refined-technical-impacts",
            0,
            0,
            0,
        )]);
        let case_02_extension_in_tlp_amber_unregistered_ns =
            Err(vec![create_namespace_extension_in_non_tlp_clear_error(
                "x_example.unregistered#some-decision-point-collection//.example.test#refined-technical-impacts",
                0,
                0,
                0,
            )]);

        // Case 11: TLP:GREEN, namespace without extension

        TESTS_2_1.test_6_3_15.expect(ExpectedResults {
            case_01: case_01_extension_in_tlp_green,
            case_02: case_02_extension_in_tlp_amber_unregistered_ns,
            case_11: Ok(()),
        });
    }
}
