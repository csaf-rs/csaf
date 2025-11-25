use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

pub fn test_6_1_05_multiple_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut product_group_ids_with_paths = HashMap::<String, Vec<String>>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        for (i_g, g) in tree.get_product_groups().iter().enumerate() {
            product_group_ids_with_paths
                .entry(g.get_group_id().to_owned())
                .or_insert_with(Vec::new)
                .push(format!("/product_tree/product_groups/{}/group_id", i_g));
        }
    }

    let errors: Vec<ValidationError> = product_group_ids_with_paths
        .iter()
        .filter(|(_, paths)| paths.len() > 1)
        .flat_map(|(group_id, paths)| {
            paths.iter().map(move |path| ValidationError {
                message: format!("Duplicate definition for product group ID {}", group_id),
                instance_path: path.clone(),
            })
        })
        .collect();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_05::test_6_1_05_multiple_definition_of_product_group_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_02() {
        let errors = HashMap::from([(
            "01",
            vec![
                ValidationError {
                    message: "Duplicate definition for product group ID CSAFGID-1020300".to_string(),
                    instance_path: "/product_tree/product_groups/0/group_id".to_string(),
                },
                ValidationError {
                    message: "Duplicate definition for product group ID CSAFGID-1020300".to_string(),
                    instance_path: "/product_tree/product_groups/1/group_id".to_string(),
                },
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
