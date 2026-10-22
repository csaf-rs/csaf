use std::collections::BTreeMap;

use crate::csaf_traits::{Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_multiple_entity_id_error(id: &str, entity_index: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Multiple definition for entity ID `{id}`"),
        instance_path: format!("/document/involvement/entities/{entity_index}/entity_id"),
    })
}

/// 6.1.64.6 Multiple Definition of Entity ID
///
/// For each Entity ID (type: `$['$defs'].entity_id_t`) in Entity elements (`$.document.involvement.entities[*]`)
/// it SHALL be tested that the `entity_id` was not already defined within the same document.
pub fn test_6_1_64_6_multiple_definition_of_entity_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // collect all defined entity IDs and their entity indices
    let mut entity_id_entity_index_map = BTreeMap::<String, Vec<usize>>::new();
    for (entity_index, entity) in involvement.get_entities().enumerate() {
        entity_id_entity_index_map
            .entry(entity.get_entity_id().to_owned())
            .or_default()
            .push(entity_index);
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // check for multiple definitions of entity IDs
    for (entity_id, entity_indices) in entity_id_entity_index_map {
        if entity_indices.len() > 1 {
            for entity_index in entity_indices {
                findings
                    .get_or_insert_default()
                    .push(create_multiple_entity_id_error(&entity_id, entity_index));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_6,
    test_6_1_64_6_multiple_definition_of_entity_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_64_6 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_64_6() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly
        
        TESTS_2_1.test_6_1_64_6.expect(ExpectedResults {
            case_01: Err(vec![
                create_multiple_entity_id_error("EID-0001", 0),
                create_multiple_entity_id_error("EID-0001", 2),
            ]),
            case_11: Ok(()),
        });
    }
}
