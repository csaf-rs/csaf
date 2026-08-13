use crate::csaf::types::csaf_product_id_helper_number::CsafModelNumber;
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_multiple_stars_model_number_error(number: &CsafModelNumber, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Model number '{number}' must not contain multiple unescaped asterisks (stars)"),
        instance_path: format!("{path}/product_identification_helper/model_numbers/{index}"),
    }
}

pub fn test_6_1_43_multiple_stars_in_model_number(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(model_numbers) = helper.get_model_numbers()
            {
                for (index, model_number) in model_numbers.iter().enumerate() {
                    if model_number.count_unescaped_stars() > 1 {
                        errors
                            .get_or_insert_default()
                            .push(create_multiple_stars_model_number_error(model_number, path, index));
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_43,
    test_6_1_43_multiple_stars_in_model_number
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_43 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_43() {
        TESTS_2_1.test_6_1_43.expect(ExpectedResults {
            case_01: // One model number with two unescaped stars
            Err(vec![create_multiple_stars_model_number_error(
                &CsafModelNumber::from("P*A*"),
                "/product_tree/full_product_names/0",
                0,
            )]),
            case_02: // One model number with one escaped and two unescaped stars
            Err(vec![create_multiple_stars_model_number_error(
                &CsafModelNumber::from("*P*\\*?*"),
                "/product_tree/full_product_names/0",
                0,
            )]),
            case_11: Ok(()), // 5 model numbers, all end with one unescaped star (and some '?' in between)
            case_12: Ok(()), // 1 model number, starts with unescaped star, 3 escaped stars
            case_13: Ok(()), // 1 model number, 2 escaped stars, one escaped backslash
        });
    }
}
