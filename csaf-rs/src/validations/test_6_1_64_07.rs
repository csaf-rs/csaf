use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::EntityRefsTItem;
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_entity_group_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of entity group ID `{id}`"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.64.7 Missing Definition of Entity Group ID
///
/// For each element of type `$['$defs'].entity_group_id_t` which is not inside the list of entity groups (`$.document.involvement.entity_groups`)
/// and therefore references an item within the list of entity groups it SHALL be tested that the Entity Group element with the matching `entity_group_id` exists.
///
/// This test SHALL ignore elements of type `$['$defs'].entity_id_t` in items of elements of type `$['$defs'].entity_refs_id_t`.
pub fn test_6_1_64_7_missing_definition_of_entity_group_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    let defined_group_ids = involvement.get_entity_group_ids();
    let mut findings: Option<Vec<TestFinding>> = None;

    // for each involvement action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // for each acting entity reference
        for (acting_reference_index, acting_entity_ref) in action.get_acting_entity_refs().enumerate() {
            // if it is entity group ID
            if let EntityRefsTItem::GroupIdT(group_id) = acting_entity_ref {
                let group_id = group_id.to_string();
                // if there are defined group IDs and the current group ID is in the defined group IDs, continue
                if let Some(defined_group_ids) = &defined_group_ids
                    && defined_group_ids.contains(&group_id)
                {
                    continue;
                }

                // undefined group id, add an error
                findings
                    .get_or_insert_default()
                    .push(create_missing_entity_group_id_error(
                        &group_id,
                        &format!(
                            "/document/involvement/actions/{action_index}/acting_entity_refs/{acting_reference_index}"
                        ),
                    ));
            }
        }

        if let Some(receiving_entity_refs) = action.get_receiving_entity_refs() {
            for (reference_index, entity_ref) in receiving_entity_refs.enumerate() {
                if let EntityRefsTItem::GroupIdT(group_id) = entity_ref {
                    let group_id = group_id.to_string();
                    // if there are defined group IDs and the current group ID is in the defined group IDs, continue
                    if let Some(defined_group_ids) = &defined_group_ids
                        && defined_group_ids.contains(&group_id)
                    {
                        continue;
                    }

                    // undefined group id, add an error
                    findings
                        .get_or_insert_default()
                        .push(create_missing_entity_group_id_error(
                            &group_id,
                            &format!(
                                "/document/involvement/actions/{action_index}/receiving_entity_refs/{reference_index}"
                            ),
                        ));
                }
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_7,
    test_6_1_64_7_missing_definition_of_entity_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_7 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_7() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_1_64_7.expect(ExpectedResults {
            case_01: Err(vec![
                create_missing_entity_group_id_error(
                    "EGID-9011",
                    "/document/involvement/actions/0/acting_entity_refs/0",
                ),
                create_missing_entity_group_id_error(
                    "EGID-9011",
                    "/document/involvement/actions/1/receiving_entity_refs/0",
                ),
            ]),
            case_02: Err(vec![
                create_missing_entity_group_id_error(
                    "EGID-9001",
                    "/document/involvement/actions/0/acting_entity_refs/0",
                ),
                create_missing_entity_group_id_error(
                    "EGID-9002",
                    "/document/involvement/actions/0/receiving_entity_refs/1",
                ),
                create_missing_entity_group_id_error(
                    "EGID-9002",
                    "/document/involvement/actions/1/acting_entity_refs/1",
                ),
                create_missing_entity_group_id_error(
                    "EGID-9001",
                    "/document/involvement/actions/1/receiving_entity_refs/0",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
