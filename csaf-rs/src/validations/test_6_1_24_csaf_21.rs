use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::{ActionCategory, ActionStatus};
use crate::validation::{TestFinding, TestFindingData};
use std::collections::{BTreeMap, BTreeSet};

/// Representation of an action for duplicate detection.
///
/// Reference arrays are represented as `BTreeSet`s so their order and repeated
/// values do not affect equality.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct ActionKey {
    date: String,
    category: ActionCategory,
    status: ActionStatus,
    summary: Option<String>,
    acting_entity_refs: BTreeSet<String>,
    dl_vuln_ids: BTreeSet<String>,
    group_ids: BTreeSet<String>,
    product_ids: BTreeSet<String>,
    receiving_entity_refs: BTreeSet<String>,
    referenced_action_ids: BTreeSet<String>,
}

/// Creates the comparison key for an action, excluding its `action_id`.
///
/// Optional reference arrays that are absent are treated as empty sets.
fn action_key(action: &impl ActionTrait) -> ActionKey {
    ActionKey {
        date: action.get_date().get_raw_string().to_owned(),
        category: action.get_category(),
        status: action.get_status(),
        summary: action.get_summary().map(str::to_owned),
        acting_entity_refs: action.get_acting_entity_refs().map(ToString::to_string).collect(),
        dl_vuln_ids: action
            .get_document_local_vuln_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        group_ids: action
            .get_group_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        product_ids: action
            .get_product_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        receiving_entity_refs: action
            .get_receiving_entity_refs()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        referenced_action_ids: action
            .get_referenced_action_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
    }
}

fn duplicate_action_error(action_ids: &[&str], action_i: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The list of actions contains items with the same content when ignoring their action_id: {}",
            action_ids.join(", ")
        ),
        instance_path: format!("/document/involvement/actions/{action_i}"),
    })
}

/// Test 6.1.24: Multiple Definition in Actions
///
/// Items in `/document/involvement/actions` must differ in more values than
/// only their `action_id`.
pub fn test_6_1_24_multiple_definition_in_actions(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if there is a involvement section
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(()); // TODO #409 wasSkipped
    };

    // create a reverse lookup of action content to the action_ids with that content and their index in the actions array.
    let mut actions = BTreeMap::<ActionKey, Vec<(&str, usize)>>::new();
    for (action_i, action) in involvement.get_actions().enumerate() {
        actions
            .entry(action_key(action))
            .or_default()
            .push((action.get_action_id(), action_i));
    }

    // generate an error for each action that has the same content as another action
    let errors = actions
        .values()
        .filter(|actions| actions.len() > 1)
        .flat_map(|actions| {
            // collect the action_ids of the duplicate actions for the error message
            let action_ids = actions.iter().map(|(action_id, _)| *action_id).collect::<Vec<_>>();
            actions
                .iter()
                .map(|(_, action_i)| duplicate_action_error(&action_ids, *action_i))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_24,
    test_6_1_24_multiple_definition_in_actions
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_1_24, TESTS_2_1};

    #[test]
    fn test_test_6_1_24() {
        let action_ids = ["ACN-1", "ACN-2"];
        let duplicate_actions = Err(vec![
            duplicate_action_error(&action_ids, 0),
            duplicate_action_error(&action_ids, 1),
        ]);

        TESTS_2_1.test_6_1_24.expect(ExpectedResults_6_1_24 {
            case_01: duplicate_actions,
            case_11: Ok(()),
        });
    }
}
