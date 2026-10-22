use std::collections::BTreeMap;

use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_multiple_action_id_error(id: &str, action_index: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Multiple definition of action ID `{id}`"),
        instance_path: format!("/document/involvement/actions/{action_index}/action_id"),
    })
}

/// 6.1.64.10 Multiple Definition of Action ID
///
/// For each element of type `$['$defs'].action_id_t` which is defining an action (`$.document.involvement.actions[*].action_id`)
/// it SHALL be tested that the `action_id` was not already defined within the same document.
pub fn test_6_1_64_10_multiple_definition_of_action_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // collect all defined action IDs and their action indices
    let mut action_id_indices_map = BTreeMap::<String, Vec<usize>>::new();
    for (action_index, action) in involvement.get_actions().enumerate() {
        action_id_indices_map
            .entry(action.get_action_id().to_owned())
            .or_default()
            .push(action_index);
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // check for multiple definitions of action IDs
    for (action_id, action_indices) in action_id_indices_map {
        if action_indices.len() > 1 {
            // for each action index with the same action ID, add an error
            for action_index in action_indices {
                findings
                    .get_or_insert_default()
                    .push(create_multiple_action_id_error(&action_id, action_index));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_10,
    test_6_1_64_10_multiple_definition_of_action_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_10 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_10() {
        TESTS_2_1.test_6_1_64_10.expect(ExpectedResults {
            // TODO: Evaluate test coverage
            // TODO: Document test cases properly

            case_01: Err(vec![
                create_multiple_action_id_error("ACN-1", 0),
                create_multiple_action_id_error("ACN-1", 1),
            ]),
            case_11: Ok(()),
        });
    }
}
