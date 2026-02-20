use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_multiple_group_id_definition_error(group_id: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition for product group ID {group_id}"),
        instance_path: path.to_owned(),
    }
}

/// 6.1.5 Multiple Definition of Product Group ID
/// Checks that all product group IDs defined in the document are unique.
pub fn test_6_1_05_multiple_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Check if there is a product tree, if there isn't, this test can be skipped
    let Some(tree) = doc.get_product_tree().as_ref() else {
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
    let errors: Vec<ValidationError> = product_group_ids_with_paths
        .iter()
        .filter(|(_, paths)| paths.len() > 1)
        .flat_map(|(group_id, paths)| {
            paths
                .iter()
                .map(move |path| generate_multiple_group_id_definition_error(group_id, path))
        })
        .collect();

    // If there are no errors, the test passes, otherwise return the errors
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_05_multiple_definition_of_product_group_id(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_05_multiple_definition_of_product_group_id(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
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
        TESTS_2_0.test_6_1_5.expect(case_01.clone(), case_s01.clone(), Ok(()));
        TESTS_2_1.test_6_1_5.expect(case_01, case_s01, Ok(()));
    }
}
