use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::helpers::count_unescaped_stars;
use crate::validation::ValidationError;

fn create_validation_error(path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: "Model number must not contain multiple unescaped asterisks (stars)".to_string(),
        instance_path: format!("{}/product_identification_helper/model_numbers/{}", path, index),
    }
}

pub fn test_6_1_43_multiple_stars_in_model_number(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(model_numbers) = helper.get_model_numbers() {
                    for (index, model_number) in model_numbers.enumerate() {
                        if count_unescaped_stars(model_number) > 1 {
                            errors
                                .get_or_insert_with(Vec::new)
                                .push(create_validation_error(&path, index));
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use crate::validations::test_6_1_43::test_6_1_43_multiple_stars_in_model_number;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_43() {
        let expected_error = create_validation_error("/product_tree/full_product_names/0", 0);

        run_csaf21_tests(
            "43",
            test_6_1_43_multiple_stars_in_model_number,
            HashMap::from([
                ("01", vec![expected_error.clone()]),
                ("02", vec![expected_error.clone()]),
            ]),
        );
    }
}
