use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_err_msg(group_id: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition for product group ID {}", group_id),
        instance_path: path.to_owned(),
    }
}

pub fn test_6_1_05_multiple_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut product_group_ids_with_paths = HashMap::<String, Vec<String>>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        for (i_g, g) in tree.get_product_groups().iter().enumerate() {
            product_group_ids_with_paths
                .entry(g.get_group_id().to_owned())
                .or_default()
                .push(format!("/product_tree/product_groups/{}/group_id", i_g));
        }
    }

    let errors: Vec<ValidationError> = product_group_ids_with_paths
        .iter()
        .filter(|(_, paths)| paths.len() > 1)
        .flat_map(|(group_id, paths)| paths.iter().map(move |path| generate_err_msg(group_id, path)))
        .collect();

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
        let case_01 = Err(vec![
            generate_err_msg("CSAFGID-1020300", "/product_tree/product_groups/0/group_id"),
            generate_err_msg("CSAFGID-1020300", "/product_tree/product_groups/1/group_id"),
        ]);

        TESTS_2_0.test_6_1_5.expect(case_01.clone());
        TESTS_2_1.test_6_1_5.expect(case_01);
    }
}
