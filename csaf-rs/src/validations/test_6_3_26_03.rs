use std::collections::BTreeMap;

use crate::csaf_traits::{Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityGroupTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_multiple_entity_group_name_info(name: &str, entity_group_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Entity group name `{name}` is used multiple times"),
        instance_path: format!("/document/involvement/entity_groups/{entity_group_index}/name"),
    })
}

/// 6.3.26.5 Multiple Use of Same Entity Group Name
///
/// For each `name` in `$.document.involvement.entity_groups[*]` it SHALL be tested that the `name`
/// was not already used for another entity group within the same document.
pub fn test_6_3_26_5_multiple_use_of_same_entity_group_name(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // check if there are entity groups, if not, skip the test
    let Some(entity_groups) = involvement.get_entity_groups() else {
        return Ok(());
    };

    // collect all defined entity group names and their entity group indices
    let mut entity_group_name_entity_group_indices_map = BTreeMap::<&str, Vec<usize>>::new();
    for (entity_group_index, entity_group) in entity_groups.enumerate() {
        entity_group_name_entity_group_indices_map
            .entry(entity_group.get_name())
            .or_default()
            .push(entity_group_index);
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // check for multiple uses of the same entity group name
    for (name, indices) in entity_group_name_entity_group_indices_map {
        if indices.len() > 1 {
            for entity_group_index in indices {
                findings
                    .get_or_insert_default()
                    .push(create_multiple_entity_group_name_info(name, entity_group_index));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_26_3,
    test_6_3_26_5_multiple_use_of_same_entity_group_name
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_3_26_3, TESTS_2_1};

    #[test]
    fn test_test_6_3_26_5() {
        TESTS_2_1.test_6_3_26_3.expect(ExpectedResults_6_3_26_3 {
            case_01: Err(vec![
                create_multiple_entity_group_name_info("JJ Research", 0),
                create_multiple_entity_group_name_info("JJ Research", 1),
            ]),
            case_11: Ok(()),
        });
    }
}
