use crate::csaf::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_02_multiple_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Map to store each key with all of its paths
    let mut conflicts = HashSet::<String>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |product, path| {
            if conflicts.contains(product.get_product_id()) {
                Err(vec![ValidationError {
                    message: format!("Duplicate definition for product ID {}", product.get_product_id()),
                    instance_path: format!("{}/product_id", path),
                }])
            } else {
                conflicts.insert(product.get_product_id().to_owned());
                Ok(())
            }
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_02::test_6_1_02_multiple_definition_of_product_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_02() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "Duplicate definition for product ID CSAFPID-9080700".to_string(),
                instance_path: "/product_tree/full_product_names/1/product_id".to_string(),
            }],
        )]);
        run_csaf20_tests("02", test_6_1_02_multiple_definition_of_product_id, errors.clone());
        run_csaf21_tests("02", test_6_1_02_multiple_definition_of_product_id, errors);
    }
}
