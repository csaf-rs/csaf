use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::{
    SsvcNamespaceResultAndPath, create_generic_namespace_finding_data, iter_ssvc_namespaces,
};
use ssvc::NamespaceError;

fn create_namespace_extension_in_tlp_clear_warning(namespace: &str, instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Usage of SSVC decision point namespace with an extension in a TLP:CLEAR document: `{namespace}`"
        ),
        instance_path: instance_path.to_owned(),
    })
}

/// 6.2.36 Usage of SSVC Decision Point Namespace with Extension in TLP:CLEAR Document
///
/// For each SSVC decision point given under `selections`, it MUST be tested that the namespace
/// does not use an extension if the document is labeled TLP:CLEAR. Namespaces reserved for
/// special purpose MUST be treated as per their definition.
pub fn test_6_2_36_usage_of_ssvc_decision_point_namespace_with_extension_in_tlp_clear_document(
    doc: &impl CsafTrait,
    allow_test_namespaces: bool,
) -> Result<(), Vec<TestFinding>> {
    // This test only applies to TLP:CLEAR documents
    // We can hard-code CSAF 2.1 distribution here, SSVC does not exist on CSAF 2.0, so this
    // will not be back-ported.
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;
    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() != LabelOfTlp::Clear {
        return Ok(());
    }

    let mut errors: Option<Vec<TestFinding>> = None;

    for SsvcNamespaceResultAndPath { instance_path, result } in iter_ssvc_namespaces(doc, allow_test_namespaces) {
        match result {
            // check if an extension exists on a valid namespace
            Ok(parsed_namespace) if parsed_namespace.extensions.is_some() => {
                errors
                    .get_or_insert_default()
                    .push(create_namespace_extension_in_tlp_clear_warning(
                        &parsed_namespace.to_string(),
                        &instance_path,
                    ))
            },
            // reserved forbidden namespaces "invalid" or "test" are used
            Err(err)
                if matches!(
                    err,
                    NamespaceError::ReservedForbiddenNamespace { .. } | NamespaceError::ReservedTestNamespace { .. }
                ) =>
            {
                errors
                    .get_or_insert_default()
                    .push(TestFinding::Warning(create_generic_namespace_finding_data(
                        &err,
                        &instance_path,
                    )));
            },
            // there is no extension / all other namespace errors
            Ok(_) | Err(_) => continue,
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_36
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<TestFinding>> {
        test_6_2_36_usage_of_ssvc_decision_point_namespace_with_extension_in_tlp_clear_document(doc, false)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_36 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::ssvc::ssvc_selection_namespace_path;

    #[test]
    fn test_test_6_2_36() {
        let case_01_extension_in_tlp_clear = Err(vec![create_namespace_extension_in_tlp_clear_warning(
            "ssvc//.example.test#refined-technical-impacts",
            &ssvc_selection_namespace_path(0, 0, 0),
        )]);
        let case_02_extension_in_tlp_clear_unregistered_ns =
            Err(vec![create_namespace_extension_in_tlp_clear_warning(
                "x_example.unregistered#some-decision-point-collection//.example.test#refined-technical-impacts",
                &ssvc_selection_namespace_path(0, 0, 0),
            )]);

        // Case 11: TLP:CLEAR, namespace without extension
        // Case 12: TLP:GREEN, namespace with extension (test does not apply)
        // Case 13: TLP:AMBER, reserved "test" namespace with extension (test does not apply)

        TESTS_2_1.test_6_2_36.expect(ExpectedResults {
            case_01: case_01_extension_in_tlp_clear,
            case_02: case_02_extension_in_tlp_clear_unregistered_ns,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
