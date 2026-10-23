use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::ActionCategory;
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_referenced_action_ids_error(action_index: usize, category: ActionCategory) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Action with category '{category}' must have a 'referenced_action_ids' element present"),
        instance_path: format!("/document/involvement/actions/{action_index}"),
    })
}

/// 6.1.64.13 Missing Referenced Action IDs
///
/// For each item in `$.document.involvement.actions` which has one of the categories `confirmation`
/// or `dispute` it SHALL be tested that the element `referenced_action_ids` exists.
pub fn test_6_1_64_13_missing_referenced_action_ids(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each involvement action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // check if the action category is one of the relevant categories
        let requires_referenced_action_ids = matches!(
            action.get_category(),
            ActionCategory::Confirmation | ActionCategory::Dispute
        );

        // if it is, check if the referenced_action_ids element is present, if not, add an error
        if requires_referenced_action_ids && action.get_referenced_action_ids().is_none() {
            findings
                .get_or_insert_default()
                .push(create_missing_referenced_action_ids_error(
                    action_index,
                    action.get_category(),
                ));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_13,
    test_6_1_64_13_missing_referenced_action_ids
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_13 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_13() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_1_64_13.expect(ExpectedResults {
            case_01: Err(vec![create_missing_referenced_action_ids_error(
                1,
                ActionCategory::Confirmation,
            )]),
            case_02: Err(vec![create_missing_referenced_action_ids_error(
                1,
                ActionCategory::Dispute,
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
