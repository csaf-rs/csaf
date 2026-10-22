use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_action_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of an action id (`action_id`): {id}"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.64.9 Missing Definition of Action ID
///
/// For each element of type `$['$defs'].action_id_t` which is not defining an action (`$.document.involvement.actions[*].action_id`)
/// and therefore references an action it SHALL be tested that the Action element with the matching `action_id` exists.
pub fn test_6_1_64_9_missing_definition_of_action_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // collect all defined action IDs
    let defined_action_ids = involvement.get_action_ids();

    let mut findings: Option<Vec<TestFinding>> = None;
    for (action_index, action) in involvement.get_actions().enumerate() {
        // for each referenced action ID in the current action
        if let Some(referenced_action_ids) = action.get_referenced_action_ids() {
            for (reference_index, action_id) in referenced_action_ids.enumerate() {
                if !defined_action_ids.contains(action_id) {
                    findings.get_or_insert_default().push(create_missing_action_id_error(
                        action_id,
                        &format!(
                            "/document/involvement/actions/{action_index}/referenced_action_ids/{reference_index}"
                        ),
                    ));
                }
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_9,
    test_6_1_64_9_missing_definition_of_action_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_9 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_9() {
        TESTS_2_1.test_6_1_64_9.expect(ExpectedResults {
            case_01: Err(vec![create_missing_action_id_error(
                "ACN-1",
                "/document/involvement/actions/0/referenced_action_ids/0",
            )]),
            case_11: Ok(()),
        });
    }
}
