use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::ActionStatus;
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_receiving_entity_refs_error(action_index: usize, status: ActionStatus) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Action with status '{status}' must have a 'receiving_entity_refs' element present"),
        instance_path: format!("/document/involvement/actions/{action_index}/receiving_entity_refs"),
    })
}

/// 6.1.64.3 Missing Recipient on Attempt
///
/// For each item in `$.document.involvement.actions` with the status `attempted`
/// it SHALL be tested that the element `receiving_entity_refs` exists.
pub fn test_6_1_64_3_missing_recipient_on_attempt(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // if the action status is attempted and receiving_entity_refs is not present, add an error
        let status = action.get_status();
        if status == ActionStatus::Attempted && action.get_receiving_entity_refs().is_none() {
            findings
                .get_or_insert_default()
                .push(create_missing_receiving_entity_refs_error(action_index, status));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_3,
    test_6_1_64_3_missing_recipient_on_attempt
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_3 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_3() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_1_64_3.expect(ExpectedResults {
            case_01: Err(vec![create_missing_receiving_entity_refs_error(
                0,
                ActionStatus::Attempted,
            )]),
            case_11: Ok(()),
        });
    }
}
