use std::collections::{BTreeSet, HashMap};

use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::EntityRefsTItem;
use crate::validation::{TestFinding, TestFindingData};

fn create_actor_and_recipient_entity_id_overlap_error(action_index: usize, entity_id: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Entity '{entity_id}' must not be both actor and recipient in an action"),
        instance_path: format!("/document/involvement/actions/{action_index}"),
    })
}

fn create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(
    action_index: usize,
    entity_group_id: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Unresolvable Entity Group Id '{entity_group_id}' must not be both actor and recipient in an action"
        ),
        instance_path: format!("/document/involvement/actions/{action_index}"),
    })
}

struct ResolvedEntityGroupIds {
    resolved_entity_ids: Option<BTreeSet<String>>,
    unresolvable_group_ids: Option<BTreeSet<String>>,
}

fn resolve_ref_groups_to_ref_ids<'a>(
    group_references: impl Iterator<Item = &'a EntityRefsTItem>,
    entity_groups: &HashMap<String, BTreeSet<String>>,
) -> ResolvedEntityGroupIds {
    let mut resolved_entity_ids: Option<BTreeSet<String>> = None;
    let mut unresolvable_group_ids: Option<BTreeSet<String>> = None;

    for group_ref in group_references {
        match group_ref {
            EntityRefsTItem::GroupIdT(group_id) => {
                let group_id_str = group_id.to_string();
                // if its a group ID, we need to resolve it to its corresponding entity IDs
                if let Some(entity_ids) = entity_groups.get(&group_id_str) {
                    // if the group ID is resolvable, we can add its entity IDs to the resolved entity IDs
                    resolved_entity_ids.get_or_insert_default().extend(entity_ids.clone());
                } else {
                    // if the group ID is not resolvable, we need to add it to the unresolvable group IDs
                    unresolvable_group_ids.get_or_insert_default().insert(group_id_str);
                }
            },
            // if its an entity ID, we can directly add it to the resolved entity IDs
            EntityRefsTItem::IdT(id) => {
                resolved_entity_ids.get_or_insert_default().insert(id.to_string());
            },
        }
    }

    ResolvedEntityGroupIds {
        resolved_entity_ids,
        unresolvable_group_ids,
    }
}

/// 6.1.64.4 Actor and Recipient
///
/// For each item in `$.document.involvement.actions` it SHALL be tested that
/// an entity is not actor and recipient. This includes indirect relations
/// through entity groups.
///
/// TODO: The error message reported can further be imrproved by pointing out the exact paths
/// TODO: of all offending entity references. For now report only the colliding entity ID and
/// TODO: the action index after resolving.
pub fn test_6_1_64_4_actor_and_recipient(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if there are involvements, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let entity_groups = involvement.get_entity_groups_map().unwrap_or_default();

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // skip if receiving_entity_refs is not present
        let Some(receiving_entity_refs) = action.get_receiving_entity_refs() else {
            continue;
        };
        // resolve the acting entity refs
        let actors = resolve_ref_groups_to_ref_ids(action.get_acting_entity_refs(), &entity_groups);
        // resolve the receiving entity refs
        let recipients = resolve_ref_groups_to_ref_ids(receiving_entity_refs, &entity_groups);

        // look for intersections between resolved entity IDs and create an error for each
        if let Some(resolved_actor_entity_ids) = actors.resolved_entity_ids
            && let Some(resolved_recipient_entity_ids) = recipients.resolved_entity_ids
        {
            for entity_id in resolved_actor_entity_ids.intersection(&resolved_recipient_entity_ids) {
                findings
                    .get_or_insert_default()
                    .push(create_actor_and_recipient_entity_id_overlap_error(
                        action_index,
                        entity_id,
                    ));
            }
        }

        // look for intersections between unresolvable group IDs and create an error for each
        if let Some(unresolvable_actor_entity_group_ids) = actors.unresolvable_group_ids
            && let Some(unresolvable_recipient_entity_group_ids) = recipients.unresolvable_group_ids
        {
            for entity_group_id in
                unresolvable_actor_entity_group_ids.intersection(&unresolvable_recipient_entity_group_ids)
            {
                findings.get_or_insert_default().push(
                    create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(
                        action_index,
                        entity_group_id,
                    ),
                );
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_64_4, test_6_1_64_4_actor_and_recipient);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_4 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_4() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly
        // TODO: This does not cover comparing the unresolvable group ids

        TESTS_2_1.test_6_1_64_4.expect(ExpectedResults {
            case_01: Err(vec![
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(0, "EID-0001"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(1, "EID-0001"),
            ]),
            case_02: Err(vec![
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(0, "EID-0001"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(1, "EID-0001"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(1, "EID-0002"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(1, "EID-0003"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(1, "EID-0004"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(2, "EID-0002"),
                create_actor_and_recipient_unresolvable_entity_group_id_overlap_error(2, "EID-0003"),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
