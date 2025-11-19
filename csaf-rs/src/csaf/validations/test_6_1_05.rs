use crate::csaf::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_05_multiple_definition_of_product_group_id(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    // Map to store each key with all of its paths
    let mut conflicts = HashSet::<String>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        for (i_g, g) in tree.get_product_groups().iter().enumerate() {
            if conflicts.contains(g.get_group_id()) {
                return Err(vec![ValidationError {
                    message: format!("Duplicate definition for product group ID {}", g.get_group_id()),
                    instance_path: format!("/product_tree/product_groups/{}/group_id", i_g),
                }])
            } else {
                conflicts.insert(g.get_group_id().to_owned());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_05::test_6_1_05_multiple_definition_of_product_group_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_02() {
        let error01 = ValidationError {
            message: "Duplicate definition for product group ID CSAFGID-1020300".to_string(),
            instance_path: "/product_tree/product_groups/1/group_id".to_string(),
        };
        let errors = HashMap::from([
            ("01", &error01)
        ]);
        run_csaf20_tests("05", test_6_1_05_multiple_definition_of_product_group_id, &errors);
        run_csaf21_tests("05", test_6_1_05_multiple_definition_of_product_group_id, &errors);
    }
}
