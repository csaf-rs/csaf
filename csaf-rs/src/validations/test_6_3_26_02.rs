use crate::csaf_traits::{Csaf21InvolvementTrait, CsafTrait, DocumentTrait, EntityGroupTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_unused_entity_group_id_info(id: &str, entity_group_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Entity group ID `{id}` is defined but not referenced in the document"),
        instance_path: format!("/document/involvement/entity_groups/{entity_group_index}/entity_group_id"),
    })
}

/// 6.3.26.2 Unused Definition of Entity Group ID
///
/// For each Entity Group ID (type: `$['$defs'].entity_group_id_t`) in Entity Group elements
/// (`$.document.involvement.entity_groups[*]`) it SHALL be tested that the `entity_group_id`
/// is referenced somewhere within the same document.
pub fn test_6_3_26_2_unused_definition_of_entity_group_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // check if there are entity groups defined, if not, skip the test
    let Some(entity_groups) = involvement.get_entity_groups() else {
        return Ok(());
    };

    // collect all referenced entity group IDs from the document
    let referenced_entity_group_ids = doc.get_all_entity_group_id_references();

    let mut findings: Option<Vec<TestFinding>> = None;

    // check for unused defined entity group IDs
    for (entity_group_index, entity_group) in entity_groups.enumerate() {
        let entity_group_id = entity_group.get_entity_group_id();
        if !referenced_entity_group_ids.contains(entity_group_id) {
            findings
                .get_or_insert_default()
                .push(create_unused_entity_group_id_info(entity_group_id, entity_group_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_26_2,
    test_6_3_26_2_unused_definition_of_entity_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_3_26_2, TESTS_2_1};

    #[test]
    fn test_test_6_3_26_2() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_3_26_2.expect(ExpectedResults_6_3_26_2 {
            case_01: Err(vec![create_unused_entity_group_id_info("EGID-9001", 0)]),
            case_02: Err(vec![create_unused_entity_group_id_info("EGID-9002", 1)]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
