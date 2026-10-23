use std::collections::BTreeMap;

use crate::csaf_traits::{Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_multiple_entity_name_info(name: &str, entity_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Entity name `{name}` is used multiple times"),
        instance_path: format!("/document/involvement/entities/{entity_index}/name"),
    })
}

/// 6.3.26.4 Multiple Use of Same Entity Name
///
/// For each `name` in `$.document.involvement.entities[*]` it SHALL be tested that the `name` was
/// not already used for another entity within the same document.
pub fn test_6_3_26_4_multiple_use_of_same_entity_name(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // collect all defined entity names and their entity indices
    let mut entity_name_entity_indices_map = BTreeMap::<&str, Vec<usize>>::new();
    for (entity_index, entity) in involvement.get_entities().enumerate() {
        if let Some(name) = entity.get_name() {
            entity_name_entity_indices_map
                .entry(name)
                .or_default()
                .push(entity_index);
        }
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // check for multiple uses of the same entity name
    for (name, indices) in entity_name_entity_indices_map {
        if indices.len() > 1 {
            for entity_index in indices {
                findings
                    .get_or_insert_default()
                    .push(create_multiple_entity_name_info(name, entity_index));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_26_4,
    test_6_3_26_4_multiple_use_of_same_entity_name
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_3_26_4, TESTS_2_1};

    #[test]
    fn test_test_6_3_26_4() {
        TESTS_2_1.test_6_3_26_4.expect(ExpectedResults_6_3_26_4 {
            case_01: Err(vec![
                create_multiple_entity_name_info("Jane Researcher", 0),
                create_multiple_entity_name_info("Jane Researcher", 2),
            ]),
            case_11: Ok(()),
        });
    }
}
