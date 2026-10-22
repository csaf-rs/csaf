use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::ActionCategory;
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_receiving_entity_refs_error(action_index: usize, category: ActionCategory) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Action with category '{category}' must have a 'receiving_entity_refs' element present"),
        instance_path: format!("/document/involvement/actions/{action_index}/receiving_entity_refs"),
    })
}

/// 6.1.64.1 Missing Receiving Entity Refs
///
/// For each item in `$.document.involvement.actions` which has one of the categories `confirmation`, `coordination`, `dispute` or `notification`
/// it SHALL be tested that the element `receiving_entity_refs` exists.
pub fn test_6_1_64_1_missing_receiving_entity_refs(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // skip if there are no involvements
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(()); // TODO #409: skipped because involvement is absent
    };

    let mut findings: Option<Vec<TestFinding>> = None;

    // for every involvement action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // check if the action category requires receiving_entity_refs to be present
        let requires_receiving_entity_refs = matches!(
            action.get_category(),
            ActionCategory::Confirmation
                | ActionCategory::Coordination
                | ActionCategory::Dispute
                | ActionCategory::Notification
        );

        // if receiving_entity_refs is required but not present, add an error to findings
        if requires_receiving_entity_refs && action.get_receiving_entity_refs().is_none() {
            findings
                .get_or_insert_default()
                .push(create_missing_receiving_entity_refs_error(
                    action_index,
                    action.get_category(),
                ));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_1,
    test_6_1_64_1_missing_receiving_entity_refs
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_1 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_1() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly
        TESTS_2_1.test_6_1_64_1.expect(ExpectedResults {
            case_01: Err(vec![
                create_missing_receiving_entity_refs_error(0, ActionCategory::Notification),
                create_missing_receiving_entity_refs_error(1, ActionCategory::Confirmation),
            ]),
            case_11: Ok(()),
        });
    }
}
