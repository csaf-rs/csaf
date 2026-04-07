use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_product_version_range_error(path: &str) -> ValidationError {
    ValidationError {
        message: "Usage of 'product_version_range' branch category is not recommended".to_string(),
        instance_path: path.to_owned(),
    }
}

/// 6.3.10 Usage of Product Version Range
///
/// Tests that the `product_version_range` branch category is not used anywhere in the product tree.
pub fn test_6_3_10_usage_of_product_version_range(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_branches(&mut |branch, path| {
            if branch.get_category() == &CategoryOfTheBranch::ProductVersionRange {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_product_version_range_error(path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_10, test_6_3_10_usage_of_product_version_range);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_10() {
        let case_01 = Err(vec![create_product_version_range_error(
            "/product_tree/branches/0/branches/0/branches/0",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_3_10.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_3_10.expect(case_01, Ok(()));
    }
}
