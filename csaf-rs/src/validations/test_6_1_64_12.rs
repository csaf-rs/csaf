use crate::csaf::traits::util::resolve_entity_groups::resolve_ref_groups_to_ref_ids;
use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::ActionCategory;
use crate::validation::{TestFinding, TestFindingData};

fn create_unmentioned_entity_error(action_index: usize, entity_id: &str, referenced_action_id: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Entity '{entity_id}' acting in a confirmation is not a receiving entity in referenced action \
             '{referenced_action_id}'"
        ),
        instance_path: format!("/document/involvement/actions/{action_index}"),
    })
}

/// 6.1.64.12 Confirmation from Unmentioned Entity
///
/// For each item in `$.document.involvement.actions` with the `category` value of `confirmation`
/// it SHALL be tested that all acting entities are mentioned as receiving entities in each action referenced through `referenced_action_ids`.
/// This takes indirect relations through entity groups into account.
///
/// TODO: The error message reported can further be improved by pointing out the group id -> entity id mapping.
/// TODO: For now report only the colliding entity ID and the action index after resolving.
/// TODO: Performance can also be improved by building maps.
/// TODO: We also ignore unresolvable groups for now.
pub fn test_6_1_64_12_confirmation_from_unmentioned_entity(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // Get the entity group ID to entity ID map.
    let entity_group_id_to_entity_id_map = involvement.get_entity_groups_map().unwrap_or_default();
    let actions: Vec<_> = involvement.get_actions().collect();

    let mut findings: Option<Vec<TestFinding>> = None;
    for (action_index, action) in actions.iter().enumerate() {
        if action.get_category() != ActionCategory::Confirmation {
            continue;
        }

        // resolve the acting entity ids, if there are none, nothing to check
        let Some(resolved_acting_entity_ids) =
            resolve_ref_groups_to_ref_ids(action.get_acting_entity_refs(), &entity_group_id_to_entity_id_map)
                .resolved_entity_ids
        else {
            continue;
        };

        // check if there are referenced actions
        let Some(referenced_action_ids) = action.get_referenced_action_ids() else {
            continue;
        };

        // if there are, for every referenced action id, resolve the action
        for referenced_action_id in referenced_action_ids {
            let Some(referenced_action) = actions
                .iter()
                .find(|referenced_action| referenced_action.get_action_id() == referenced_action_id)
            else {
                continue;
            };

            let resolved_receiving_entity_ids = referenced_action
                .get_receiving_entity_refs()
                .map(|refs| {
                    resolve_ref_groups_to_ref_ids(refs, &entity_group_id_to_entity_id_map)
                        .resolved_entity_ids
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            for entity_id in resolved_acting_entity_ids.difference(&resolved_receiving_entity_ids) {
                findings.get_or_insert_default().push(create_unmentioned_entity_error(
                    action_index,
                    entity_id,
                    referenced_action_id,
                ));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_12,
    test_6_1_64_12_confirmation_from_unmentioned_entity
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_12 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_12() {
        TESTS_2_1.test_6_1_64_12.expect(ExpectedResults {
            case_01: Err(vec![create_unmentioned_entity_error(2, "EID-0003", "ACN-1")]),
            case_02: Err(vec![create_unmentioned_entity_error(1, "EID-0003", "ACN-1")]),
            case_11: Ok(()),
            case_12: Ok(()),
            // TODO: Typo, ACN-3 is defined multiple times, which throws our impl off
            case_13: Err(vec![
                create_unmentioned_entity_error(3, "EID-0003", "ACN-1"),
                create_unmentioned_entity_error(3, "EID-0003", "ACN-2"),
                create_unmentioned_entity_error(3, "EID-0003", "ACN-3"),
            ]),
        });
    }
}
