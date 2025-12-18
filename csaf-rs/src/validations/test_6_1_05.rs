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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_02() {
        let errors = HashMap::from([(
            "01",
            vec![
                generate_err_msg("CSAFGID-1020300", "/product_tree/product_groups/0/group_id"),
                generate_err_msg("CSAFGID-1020300", "/product_tree/product_groups/1/group_id"),
            ],
        )]);
        run_csaf20_tests(
            "05",
            test_6_1_05_multiple_definition_of_product_group_id,
            errors.clone(),
        );
        run_csaf21_tests("05", test_6_1_05_multiple_definition_of_product_group_id, errors);
    }
}
