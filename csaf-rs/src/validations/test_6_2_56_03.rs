use std::collections::HashSet;

use crate::csaf_traits::{
    ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityGroupTrait, EntityTrait,
};
use crate::schema::csaf2_1::schema::EntityRefsTItem;
use crate::validation::{TestFinding, TestFindingData};

fn create_unused_entity_id_warning(entity_id: &str, entity_index: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Entity ID `{entity_id}` is defined but not referenced"),
        instance_path: format!("/document/involvement/entities/{entity_index}/entity_id"),
    })
}

/// 6.2.56.3 Unused Definition of Entity ID
///
/// For each Entity ID (type: `$['$defs'].entity_id_t`) in Entity elements (`$.document.involvement.entities[*]`)
/// it SHALL be tested that the `entity_id` is referenced somewhere within the same document.
pub fn test_6_2_56_3_unused_definition_of_entity_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let mut referenced_entity_ids: HashSet<String> = HashSet::new();

    // Collect all referenced entity IDs and entity group IDs from actions
    for action in involvement.get_actions() {
        for entity_ref in action.get_acting_entity_refs() {
            if let EntityRefsTItem::IdT(entity_id_ref) = entity_ref {
                referenced_entity_ids.insert(entity_id_ref.to_string());
            }
        }

        if let Some(entity_refs) = action.get_receiving_entity_refs() {
            for entity_ref in entity_refs {
                if let EntityRefsTItem::IdT(entity_id_ref) = entity_ref {
                    referenced_entity_ids.insert(entity_id_ref.to_string());
                }
            }
        }
    }

    // extend with all entity IDs referenced in entity group definitions
    if let Some(entity_groups) = involvement.get_entity_groups() {
        for entity_group in entity_groups {
            {
                referenced_entity_ids.extend(entity_group.get_entity_ids().map(str::to_owned));
            }
        }
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each defined entity id, check if it is referenced somewhere, if not, add a warning
    for (entity_index, entity) in involvement.get_entities().enumerate() {
        if !referenced_entity_ids.contains(entity.get_entity_id()) {
            findings
                .get_or_insert_default()
                .push(create_unused_entity_id_warning(entity.get_entity_id(), entity_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_56_3,
    test_6_2_56_3_unused_definition_of_entity_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_56_3 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_56_3() {
        // TODO: Improve test coverage
        // Case 01: 3 entities, 1 and 2 are in exchange, 3 is not mentioned
        // Case 02: 5 entities, 1 and 3 are a group, group + 4 are in exchange with 2, 5 is not mentioned
        // Case 11: 01 fixed, entity 3 is removed
        // Case 12: 02 fixed, entity 5 is now in a group with 4, which is not referenced, violating 6.2.56.2
        TESTS_2_1.test_6_2_56_3.expect(ExpectedResults {
            case_01: Err(vec![create_unused_entity_id_warning("EID-0003", 2)]),
            case_02: Err(vec![create_unused_entity_id_warning("EID-0005", 4)]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
