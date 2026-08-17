use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
use std::collections::HashMap;

fn generate_multiple_group_id_definition_error(group_id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Duplicate definition for product group ID {group_id}"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.5 Multiple Definition of Product Group ID
/// Checks that all product group IDs defined in the document are unique.
pub fn test_6_1_05_multiple_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // Check if there is a product tree, if there isn't, this test can be skipped
    let Some(tree) = doc.get_product_tree() else {
        // This will be WasSkipped in the future
        return Ok(());
    };

    let product_groups = tree.get_product_groups();

    // Check if there are any product groups, if there aren't, this test can be skipped
    if product_groups.is_empty() {
        // This will be WasSkipped in the future
        return Ok(());
    }

    // Create a map of product group IDs to the JSON paths where they are defined
    let mut product_group_ids_with_paths = HashMap::<String, Vec<String>>::new();
    for (i_g, g) in product_groups.iter().enumerate() {
        product_group_ids_with_paths
            .entry(g.get_group_id().to_owned())
            .or_default()
            .push(format!("/product_tree/product_groups/{i_g}/group_id"));
    }

    // Generate an error for each product group ID that is defined more than once
    let mut errors: Option<Vec<TestFinding>> = None;
    for (group_id, paths) in &product_group_ids_with_paths {
        if paths.len() > 1 {
            for path in paths {
                errors
                    .get_or_insert_default()
                    .push(generate_multiple_group_id_definition_error(group_id, path));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_5,
    test_6_1_05_multiple_definition_of_product_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_5 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_5 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_05() {
        // Case 01: Two product groups with the same group_id
        let case_01 = Err(vec![
            generate_multiple_group_id_definition_error("CSAFGID-1020300", "/product_tree/product_groups/0/group_id"),
            generate_multiple_group_id_definition_error("CSAFGID-1020300", "/product_tree/product_groups/1/group_id"),
        ]);
        // Case S01: Three product groups with the same group_id
        let case_s01 = Err(vec![
            generate_multiple_group_id_definition_error("CSAFGID-1020300", "/product_tree/product_groups/0/group_id"),
            generate_multiple_group_id_definition_error("CSAFGID-1020300", "/product_tree/product_groups/1/group_id"),
            generate_multiple_group_id_definition_error("CSAFGID-1020300", "/product_tree/product_groups/2/group_id"),
        ]);
        // Case S11: Two product groups with different group_ids
        TESTS_2_0.test_6_1_5.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
            case_s01: case_s01.clone(),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_1_5.expect(ExpectedResults_2_1 {
            case_01,
            case_s01,
            case_s11: Ok(()),
        });
    }
}
