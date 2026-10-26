use std::collections::{BTreeSet, HashMap};

use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::cycle_detection::find_cycle;

fn create_circular_reference_error(cycle: &[String], instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Found circular reference of action IDs: `{}`", cycle.join(" -> ")),
        instance_path: instance_path.to_owned(),
    })
}

/// 6.1.64.11 Circular Reference of Action ID
///
/// For each action containing at least one referenced action id (type `$['$defs'].action_id_t`)
/// it SHALL be tested that the action does not end up in a circle.
/// This also includes actions containing their own `action_id` in the `referenced_action_ids`.
pub fn test_6_1_64_11_circular_reference_of_action_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let mut relation_map = HashMap::<String, HashMap<String, String>>::new();
    let mut referenced_action_ids = BTreeSet::new();

    for (action_index, action) in involvement.get_actions().enumerate() {
        let action_id = action.get_action_id().to_owned();

        if let Some(references) = action.get_referenced_action_ids() {
            let outgoing_edges = relation_map.entry(action_id).or_default();
            for referenced_action_id in references {
                referenced_action_ids.insert(referenced_action_id.to_owned());
                outgoing_edges.insert(
                    referenced_action_id.to_owned(),
                    format!("/document/involvement/actions/{action_index}"),
                );
            }
        }
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    for action_id in referenced_action_ids {
        if let Some((cycle, instance_path)) = find_cycle(&relation_map, &action_id) {
            findings
                .get_or_insert_default()
                .push(create_circular_reference_error(&cycle, &instance_path));
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
                "/document/involvement/actions/0",
            )]),
            case_02: Err(vec![
                create_circular_reference_error(
                    &[
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                    ],
                    "/document/involvement/actions/0",
                ),
                create_circular_reference_error(
                    &[
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                    ],
                    "/document/involvement/actions/1",
                ),
                create_circular_reference_error(
                    &[
                        "ACN-3".to_string(),
                        "ACN-2".to_string(),
                        "ACN-1".to_string(),
                        "ACN-3".to_string(),
                    ],
                    "/document/involvement/actions/2",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
