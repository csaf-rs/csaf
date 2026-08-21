use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::ssvc::{
    SsvcNamespaceResultAndPath, create_generic_namespace_finding_data, iter_ssvc_namespaces,
};
use ssvc::NamespaceError;

fn create_unregistered_base_namespace_error(namespace: &str, instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Usage of unregistered SSVC decision point base namespace: `{namespace}`"),
        instance_path: instance_path.to_owned(),
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
    allow_test_namespaces: bool,
) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for SsvcNamespaceResultAndPath { instance_path, result } in iter_ssvc_namespaces(doc, allow_test_namespaces) {
        match result {
            // its unregistered (prefixed with x_)
            Ok(parsed) if parsed.is_unregistered() => {
                errors
                    .get_or_insert_default()
                    .push(create_unregistered_base_namespace_error(
                        &parsed.to_string(),
                        &instance_path,
                    ));
            },
            Err(err)
                if matches!(
                    err,
                    // not a "known" registered namespace to SSVC
                    NamespaceError::InvalidRegisteredNamespace { .. }
                        //reserved namespaces "invalid"
                        | NamespaceError::ReservedForbiddenNamespace { .. }
                        //reserved namespaces "test"
                        | NamespaceError::ReservedTestNamespace { .. }
                ) =>
            {
                errors
                    .get_or_insert_default()
                    .push(TestFinding::Warning(create_generic_namespace_finding_data(
                        &err,
                        &instance_path,
                    )));
            },
            // its registered / all other namespace errors
            Ok(_) | Err(_) => continue,
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_34
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<TestFinding>> {
        test_6_2_34_usage_of_unknown_ssvc_decision_point_base_namespace(doc, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_34 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::ssvc::ssvc_selection_namespace_path;

    #[test]
    fn test_test_6_2_34() {
        let case_01_unregistered_ns = Err(vec![create_unregistered_base_namespace_error(
            "x_example.unregistered#some-yet-unknown-or-maybe-private-namespace",
            &ssvc_selection_namespace_path(0, 0, 0),
        )]);
        let case_02_unregistered_ns_with_ext = Err(vec![create_unregistered_base_namespace_error(
            "x_example.test#also-unregistered-namespace//.example.other-test#some-extension",
            &ssvc_selection_namespace_path(0, 0, 0),
        )]);
        let case_03_reserved_forbidden_ns = Err(vec![TestFinding::Warning(create_generic_namespace_finding_data(
            &NamespaceError::ReservedForbiddenNamespace {
                namespace: "invalid".to_string(),
            },
            &ssvc_selection_namespace_path(0, 0, 0),
        ))]);

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
