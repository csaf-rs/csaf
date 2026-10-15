use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::{
    SsvcNamespaceAndPath, create_generic_namespace_finding_data, iter_ssvc_namespace_strings,
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
/// For each SSVC decision point given under `selections`, it SHALL be tested that the `namespace`
/// does not use an extension if the document is labeled `TLP:CLEAR`. Namespaces reserved for
/// special purpose SHALL be treated as per their definition.
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

    for SsvcNamespaceAndPath {
        namespace,
        instance_path,
    } in iter_ssvc_namespace_strings(doc)
    {
        match ssvc::validate_namespace(&namespace, allow_test_namespaces) {
            // TODO: Revisit once reserved namespace handling is clarified: https://github.com/csaf-rs/ssvc/issues/111
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

            // The presence of '/' is sufficient to detect an extension, regardless of whether namespace validation succeeds
            _ if namespace.contains('/') => {
                errors
                    .get_or_insert_default()
                    .push(create_namespace_extension_in_tlp_clear_warning(
                        &namespace,
                        &instance_path,
                    ));
            },

            // No extension or relevant reserved namespace error
            _ => continue,
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
        let case_03_extension_in_tlp_clear = Err(vec![create_namespace_extension_in_tlp_clear_warning(
            "ssvc/en-US",
            &ssvc_selection_namespace_path(0, 0, 0),
        )]);

        // Case 11: TLP:CLEAR, namespace without extension
        // Case 12: TLP:GREEN, namespace with extension (test does not apply)
        // Case 13: TLP:AMBER, reserved "test" namespace with extension (test does not apply)

        TESTS_2_1.test_6_2_36.expect(ExpectedResults {
            case_01: case_01_extension_in_tlp_clear,
            case_02: case_02_extension_in_tlp_clear_unregistered_ns,
            case_03: case_03_extension_in_tlp_clear,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
