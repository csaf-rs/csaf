use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_02_multiple_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Map to store each key with all of its paths
    let mut products = HashSet::<String>::new();

    let mut errors: Option<Vec<ValidationError>> = Option::None;
    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |product, path| {
            if products.contains(product.get_product_id()) {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(generate_err_msg(product.get_product_id(), path));
            } else {
                products.insert(product.get_product_id().to_owned());
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

fn generate_err_msg(product_id: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition for product ID {}", product_id),
        instance_path: format!("{}/product_id", path),
    }
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
            vec![generate_err_msg(
                "CSAFPID-9080700",
                "/product_tree/full_product_names/1",
            )],
        )]);
        run_csaf20_tests("02", test_6_1_02_multiple_definition_of_product_id, errors.clone());
        run_csaf21_tests("02", test_6_1_02_multiple_definition_of_product_id, errors);
    }
}
