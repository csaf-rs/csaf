use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_product_version_range_error(path: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Usage of 'product_version_range' branch category is not recommended".to_string(),
        instance_path: path.to_owned(),
    })
}

/// 6.3.10 Usage of Product Version Range
///
/// Tests that the `product_version_range` branch category is not used anywhere in the product tree.
pub fn test_6_3_10_usage_of_product_version_range(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // TODO #409 wasSkipped
    };

    let mut errors: Option<Vec<TestFinding>> = None;

    product_tree.visit_all_branches(&mut |branch, path| {
        if branch.get_category() == CategoryOfTheBranch::ProductVersionRange {
            errors
                .get_or_insert_default()
                .push(create_product_version_range_error(path));
        }
    });

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_10, test_6_3_10_usage_of_product_version_range);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_10 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_10 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_10() {
        let tree_with_product_version_range = Err(vec![create_product_version_range_error(
            "/product_tree/branches/0/branches/0/branches/0",
        )]);

        let tree_with_parallel_product_version_range = Err(vec![
            create_product_version_range_error("/product_tree/branches/0/branches/0/branches/0"),
            create_product_version_range_error("/product_tree/branches/0/branches/0/branches/1"),
        ]);

        // Stacked product categories violate 6.1.57 on CSAF 2.1, making this test file invalid there
        let tree_with_stacked_product_version_range = Err(vec![
            create_product_version_range_error("/product_tree/branches/0/branches/0/branches/0"),
            create_product_version_range_error("/product_tree/branches/0/branches/0/branches/0/branches/0"),
        ]);

        // Case 11: product tree without product version range
        // Case S11: no product tree

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_3_10.expect(ExpectedResults_2_0 {
            case_01: tree_with_product_version_range.clone(),
            case_s01: tree_with_parallel_product_version_range.clone(),
            case_s02: tree_with_stacked_product_version_range.clone(),
            case_11: Ok(()),
            case_s11: Ok(()), // TODO #409 wasSkipped
        });
        TESTS_2_1.test_6_3_10.expect(ExpectedResults_2_1 {
            case_01: tree_with_product_version_range,
            case_s01: tree_with_parallel_product_version_range,
            case_s02: tree_with_stacked_product_version_range,
            case_11: Ok(()),
            case_s11: Ok(()), // TODO #409 wasSkipped
        });
    }
}
