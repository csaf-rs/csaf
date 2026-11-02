use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::cycle_detection::{RelationCycle, RelationMap};

fn create_circular_reference_error(cycle: &[String], instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Found circular reference of action IDs: {}", cycle.join(" -> ")),
        instance_path: instance_path.to_owned(),
    })
}

/// Checks 6.1.64.11, Circular Reference of Action ID.
///
/// For each action containing at least one referenced action id (type `$['$defs'].action_id_t`)
/// it SHALL be tested that the action does not end up in a circle.
/// This also includes actions containing their own `action_id` in the `referenced_action_ids`.
///
/// The test reports the first detected cycle for each action and includes the circular path in the
/// message of the finding.
pub fn test_6_1_64_11_circular_reference_of_action_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // The test only applies when the optional involvement section is present.
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let mut relation_map = RelationMap::new();
    let mut action_ids_to_check = Vec::new();

    for (action_index, action) in involvement.get_actions().enumerate() {
        let action_id = action.get_action_id().to_owned();

        if let Some(references) = action.get_referenced_action_ids() {
            action_ids_to_check.push(action_id.clone());
            for (reference_index, referenced_action_id) in references.enumerate() {
                relation_map.add_relation(
                    &action_id,
                    referenced_action_id,
                    format!("/document/involvement/actions/{action_index}/referenced_action_ids/{reference_index}"),
                );
            }
        }
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    for action_id in action_ids_to_check {
        if let Some(RelationCycle { cycle, relation_path }) = relation_map.find_cycle(&action_id) {
            findings
                .get_or_insert_default()
                .push(create_circular_reference_error(&cycle, &relation_path));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_11,
    test_6_1_64_11_circular_reference_of_action_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_11 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_11() {
        TESTS_2_1.test_6_1_64_11.expect(ExpectedResults {
            case_01: Err(vec![create_circular_reference_error(
                &["ACN-1".to_string(), "ACN-1".to_string()],
                "/document/involvement/actions/0/referenced_action_ids/0",
            )]),
            case_02: Err(vec![
                create_circular_reference_error(
                    &[
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                    ],
                    "/document/involvement/actions/0/referenced_action_ids/0",
                ),
                create_circular_reference_error(
                    &[
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                    ],
                    "/document/involvement/actions/1/referenced_action_ids/0",
                ),
                create_circular_reference_error(
                    &[
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                    ],
                    "/document/involvement/actions/2/referenced_action_ids/0",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
