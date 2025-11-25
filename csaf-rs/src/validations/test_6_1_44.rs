use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::helpers::count_unescaped_stars;
use crate::validation::ValidationError;

pub fn test_6_1_44_multiple_stars_in_serial_number(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(serial_numbers) = helper.get_serial_numbers() {
                    for (index, serial_number) in serial_numbers.enumerate() {
                        if count_unescaped_stars(serial_number) > 1 {
                            errors.get_or_insert_with(Vec::new).push(ValidationError {
                                message: "Serial number must not contain multiple unescaped asterisks (stars)"
                                    .to_string(),
                                instance_path: format!(
                                    "{}/product_identification_helper/serial_numbers/{}",
                                    path, index
                                ),
                            });
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
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_44::test_6_1_44_multiple_stars_in_serial_number;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_44() {
        let expected_error = ValidationError {
            message: "Serial number must not contain multiple unescaped asterisks (stars)".to_string(),
            instance_path: "/product_tree/full_product_names/0/product_identification_helper/serial_numbers/0"
                .to_string(),
        };

        run_csaf21_tests(
            "44",
            test_6_1_44_multiple_stars_in_serial_number,
            HashMap::from([
                ("01", vec![expected_error.clone()]),
                ("02", vec![expected_error.clone()]),
            ]),
        );
    }
}
