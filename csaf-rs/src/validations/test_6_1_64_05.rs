use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityGroupTrait};
use crate::schema::csaf2_1::schema::EntityRefsTItem;
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_entity_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of entity ID `{id}`"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.64.5 Missing Definition of Entity ID
///
/// For each element of type `$['$defs'].entity_id_t` which is not inside the list of entities (`$.document.involvement.entities`)
/// and therefore references an item within the list of entities it SHALL be tested that the Entity element with the matching `entity_id` exists.
/// This test SHALL ignore elements of type `$['$defs'].entity_group_id_t` in items of elements of type `$['$defs'].entity_refs_id_t`.
pub fn test_6_1_64_5_missing_definition_of_entity_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // get all defined entity IDs
    let defined_entity_ids = involvement.get_entity_ids();
    let mut findings: Option<Vec<TestFinding>> = None;

    // for each involvement action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // for each acting entity reference
        for (reference_index, entity_ref) in action.get_acting_entity_refs().enumerate() {
            if let EntityRefsTItem::IdT(entity_id) = entity_ref {
                let entity_id = entity_id.to_string();
                if !defined_entity_ids.contains(&entity_id) {
                    findings.get_or_insert_default().push(create_missing_entity_id_error(
                        &entity_id,
                        &format!("/document/involvement/actions/{action_index}/acting_entity_refs/{reference_index}"),
                    ));
                }
            }
        }

        // for each receiving entity reference
        if let Some(receiving_entity_refs) = action.get_receiving_entity_refs() {
            for (reference_index, entity_ref) in receiving_entity_refs.enumerate() {
                if let EntityRefsTItem::IdT(entity_id) = entity_ref {
                    let entity_id = entity_id.to_string();
                    if !defined_entity_ids.contains(&entity_id) {
                        findings.get_or_insert_default().push(create_missing_entity_id_error(
                            &entity_id,
                            &format!(
                                "/document/involvement/actions/{action_index}/receiving_entity_refs/{reference_index}"
                            ),
                        ));
                    }
                }
            }
        }
    }

    // for each entity group, if there are entity groups
    if let Some(entity_groups) = involvement.get_entity_groups() {
        for (group_index, group) in entity_groups.enumerate() {
            // for each entity ID in the group
            for (entity_index, entity_id) in group.get_entity_ids().enumerate() {
                if !defined_entity_ids.contains(entity_id) {
                    findings.get_or_insert_default().push(create_missing_entity_id_error(
                        entity_id,
                        &format!("/document/involvement/entity_groups/{group_index}/entity_ids/{entity_index}"),
                    ));
                }
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_5,
    test_6_1_64_5_missing_definition_of_entity_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_5 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_5() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_1_64_5.expect(ExpectedResults {
            case_01: Err(vec![
                create_missing_entity_id_error("EID-0002", "/document/involvement/actions/0/receiving_entity_refs/0"),
                create_missing_entity_id_error("EID-0002", "/document/involvement/actions/1/acting_entity_refs/0"),
            ]),
            case_02: Err(vec![
                create_missing_entity_id_error("EID-0011", "/document/involvement/entity_groups/0/entity_ids/0"),
                create_missing_entity_id_error("EID-0011", "/document/involvement/actions/1/receiving_entity_refs/1"),
                create_missing_entity_id_error("EID-0013", "/document/involvement/actions/2/acting_entity_refs/0"),
                create_missing_entity_id_error("EID-0014", "/document/involvement/actions/2/receiving_entity_refs/1"),
                create_missing_entity_id_error("EID-0015", "/document/involvement/actions/2/receiving_entity_refs/2"),
                create_missing_entity_id_error("EID-0015", "/document/involvement/entity_groups/1/entity_ids/1"),
                create_missing_entity_id_error("EID-0016", "/document/involvement/entity_groups/1/entity_ids/2"),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
