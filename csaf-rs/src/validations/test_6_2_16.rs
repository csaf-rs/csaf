use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;

/// 6.2.16 Missing Product Identification Helper
///
/// Each product in the product tree must include a `product_identification_helper` property.
///
/// As this property is not allowed to be empty in the schema, this ensures that at least
/// one product identification helper is provided for each product.
pub fn test_6_2_16_missing_product_identification_helper(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, path| {
            if fpn.get_product_identification_helper().is_none() {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_missing_product_identification_helper_error(path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

fn create_missing_product_identification_helper_error(instance_path: &str) -> ValidationError {
    ValidationError {
        message: "Product is missing 'product_identification_helper' property".to_string(),
        instance_path: instance_path.to_string(),
    }
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_2_16,
    test_6_2_16_missing_product_identification_helper
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_16() {
        let case_01 = Err(vec![create_missing_product_identification_helper_error(
            "/product_tree/full_product_names/0",
        )]);
        let case_02 = Err(vec![create_missing_product_identification_helper_error(
            "/product_tree/branches/0/branches/0/branches/0/product",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_16.expect(case_01.clone(), case_02.clone(), Ok(()));
        TESTS_2_1.test_6_2_16.expect(case_01, case_02, Ok(()));
    }
}
