use std::collections::BTreeMap;

use crate::csaf_traits::{Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityGroupTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_multiple_entity_group_id_error(id: &str, entity_group_index: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Multiple definition of entity group ID `{id}`"),
        instance_path: format!("/document/involvement/entity_groups/{entity_group_index}/entity_group_id"),
    })
}

/// 6.1.64.8 Multiple Definition of Entity Group ID
///
/// For each Entity Group ID (type: `$['$defs'].entity_group_id_t`) in Entity Group elements (`$.document.involvement.entity_groups[*]`)
/// it SHALL be tested that the `entity_group_id` was not already defined within the same document.
pub fn test_6_1_64_8_multiple_definition_of_entity_group_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // collect all entity group IDs and their paths
    let mut entity_group_id_to_indices_map: Option<BTreeMap<String, Vec<usize>>> = None;

    if let Some(entity_groups) = involvement.get_entity_groups() {
        for (entity_group_index, entity_group) in entity_groups.enumerate() {
            entity_group_id_to_indices_map
                .get_or_insert_default()
                .entry(entity_group.get_entity_group_id().to_owned())
                .or_default()
                .push(entity_group_index);
        }
    }

    let Some(entity_group_id_occurrence_map) = entity_group_id_to_indices_map else {
        return Ok(()); // no entity groups, no duplicates
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each defined group id
    for (entity_group_id, entity_group_indices) in entity_group_id_occurrence_map {
        // if there are multiple definitions, add an error for each occurrence
        if entity_group_indices.len() > 1 {
            for entity_group_index in entity_group_indices {
                findings
                    .get_or_insert_default()
                    .push(create_multiple_entity_group_id_error(
                        &entity_group_id,
                        entity_group_index,
                    ));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_8,
    test_6_1_64_8_multiple_definition_of_entity_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_8 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_8() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_1_64_8.expect(ExpectedResults {
            case_01: Err(vec![
                create_multiple_entity_group_id_error("EGID-9001", 0),
                create_multiple_entity_group_id_error("EGID-9001", 1),
            ]),
            case_11: Ok(()),
        });
    }
}
