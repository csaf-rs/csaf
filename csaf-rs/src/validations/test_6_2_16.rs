use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

/// 6.2.16 Missing Product Identification Helper
///
/// Each product in the product tree must include a `product_identification_helper` property.
///
/// As this property is not allowed to be empty in the schema, this ensures that at least
/// one product identification helper is provided for each product.
pub fn test_6_2_16_missing_product_identification_helper(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    if let Some(tree) = doc.get_product_tree() {
        tree.visit_all_products(&mut |fpn, path| {
            if fpn.get_product_identification_helper().is_none() {
                errors
                    .get_or_insert_default()
                    .push(create_missing_product_identification_helper_error(path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

fn create_missing_product_identification_helper_error(instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: "Product is missing 'product_identification_helper' property".to_string(),
        instance_path: instance_path.to_string(),
    })
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_2_16,
    test_6_2_16_missing_product_identification_helper
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_16 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_16 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_16() {
        let case_01 = Err(vec![create_missing_product_identification_helper_error(
            "/product_tree/full_product_names/0",
        )]);
        let case_02 = Err(vec![create_missing_product_identification_helper_error(
            "/product_tree/branches/0/branches/0/branches/0/product",
        )]);

        let case_s01 = Err(vec![
            create_missing_product_identification_helper_error(
                "/product_tree/branches/0/branches/0/branches/1/product",
            ),
            create_missing_product_identification_helper_error(
                "/product_tree/branches/0/branches/0/branches/3/product",
            ),
        ]);

        let case_s02 = Err(vec![
            create_missing_product_identification_helper_error("/product_tree/full_product_names/1"),
            create_missing_product_identification_helper_error("/product_tree/full_product_names/3"),
        ]);

        let case_s03 = Err(vec![
            create_missing_product_identification_helper_error("/product_tree/product_paths/1/full_product_name"),
            create_missing_product_identification_helper_error("/product_tree/product_paths/3/full_product_name"),
        ]);

        let case_s04 = Err(vec![
            create_missing_product_identification_helper_error(
                "/product_tree/branches/0/branches/0/branches/0/product",
            ),
            create_missing_product_identification_helper_error(
                "/product_tree/branches/0/branches/0/branches/1/product",
            ),
            create_missing_product_identification_helper_error("/product_tree/full_product_names/0"),
            create_missing_product_identification_helper_error("/product_tree/full_product_names/1"),
            create_missing_product_identification_helper_error("/product_tree/product_paths/0/full_product_name"),
            create_missing_product_identification_helper_error("/product_tree/product_paths/1/full_product_name"),
        ]);

        TESTS_2_0.test_6_2_16.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
            case_02: case_02.clone(),
            case_11: Ok(()),
        });

        // Failing test cases:
        // 01  - missing product identification helper in full product names
        // 02  - missing product identification helper in nested branches
        // s01 - multiple products in nested branches with alternating present/missing product identification helpers
        // s02 - multiple full product names with alternating present/missing product identification helpers
        // s03 - multiple product paths with alternating present/missing product identification helpers
        // s04 - multiple missing product identification helpers across all three product tree locations

        // Valid test cases:
        // 11  - product identification helper present in full product names
        // s11 - no product tree
        // s12 - product tree without relevant full product name types

        TESTS_2_1.test_6_2_16.expect(ExpectedResults_2_1 {
            case_01,
            case_02,
            case_s01,
            case_s02,
            case_s03,
            case_s04,
            case_11: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });
    }
}
