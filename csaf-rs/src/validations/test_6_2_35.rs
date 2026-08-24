use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::{SsvcNamespaceResultAndPath, iter_ssvc_namespaces, create_generic_namespace_finding_data};
use ssvc::NamespaceError;

fn create_unregistered_base_namespace_in_tlp_clear_warning(namespace: &str, instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Usage of unregistered SSVC decision point base namespace in a TLP:CLEAR document: `{namespace}`"
        ),
        instance_path: instance_path.to_owned(),
    })
}

/// 6.2.35 Usage of Unregistered SSVC Decision Point Base Namespace in TLP:CLEAR Document
///
/// For each SSVC decision point given under `selections`, it MUST be tested that the base
/// namespace is not an unregistered one if the document is labeled TLP:CLEAR. Namespaces
/// reserved for special purpose MUST be treated as per their definition.
pub fn test_6_2_35_usage_of_unregistered_ssvc_decision_point_base_namespace_in_tlp_clear_document(
    doc: &impl CsafTrait,
    allow_test_namespaces: bool,
) -> Result<(), Vec<TestFinding>> {
    // This test only applies to TLP:CLEAR documents
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;
    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() != LabelOfTlp::Clear {
        return Ok(());
    }

    let mut errors: Option<Vec<TestFinding>> = None;

    for SsvcNamespaceResultAndPath { instance_path, result } in iter_ssvc_namespaces(doc, allow_test_namespaces) {
        match result {
            // it's unregistered (prefixed with x_)
            Ok(parsed) if parsed.is_unregistered() => {
                errors
                    .get_or_insert_default()
                    .push(create_unregistered_base_namespace_in_tlp_clear_warning(
                        &parsed.to_string(),
                        &instance_path,
                    ));
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
                    .push(TestFinding::Warning(create_generic_namespace_finding_data(&err, &instance_path)));
            },
            // it's registered / all other namespace errors
            Ok(_) | Err(_) => continue,
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
for crate::csaf2_1::testcases::ValidatorForTest6_2_35
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<TestFinding>> {
        test_6_2_35_usage_of_unregistered_ssvc_decision_point_base_namespace_in_tlp_clear_document(doc, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_35 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::ssvc::ssvc_selection_namespace_path;

    #[test]
    fn test_test_6_2_35() {
        let case_01_unregistered_ns_in_tlp_clear = Err(vec![create_unregistered_base_namespace_in_tlp_clear_warning(
            "x_example.unregistered#namespace",
            &ssvc_selection_namespace_path(0, 0, 0),
        )]);

        // Case 11: TLP:CLEAR, registered namespace
        // Case 12: TLP:RED, unregistered namespace

        TESTS_2_1.test_6_2_35.expect(ExpectedResults {
            case_01: case_01_unregistered_ns_in_tlp_clear,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
