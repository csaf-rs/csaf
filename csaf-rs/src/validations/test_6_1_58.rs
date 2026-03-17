use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_both_product_version_version_range_error(instance_path: String) -> ValidationError {
    ValidationError {
        message: "Path contains both branches with category 'product_version' and 'product_version_range'".to_string(),
        instance_path,
    }
}

/// 6.1.58 Use of product_version in one Path with product_version_range
///
/// In the product tree, the path from root to any FPN, the branches with category 'product_version'
/// and 'product_version_range' are not allowed to occur together.
pub fn test_6_1_58_product_version_and_product_version_range_in_one_path(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {

    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // this will be a Passed::NoData later (#409)
    };

    let mut errors: Option<Vec<ValidationError>> = None;

    // get all paths from root to leafs in the product tree
    let leaf_paths = product_tree.collect_leaf_paths();

    // for every path
    for (path, path_str) in leaf_paths {
        // check if it contains one of the relevant categories
        let contains_product_version = path
            .iter()
            .any(|b| b.get_category() == &CategoryOfTheBranch::ProductVersion);
        let contains_product_version_range = path
            .iter()
            .any(|b| b.get_category() == &CategoryOfTheBranch::ProductVersionRange);

        // if it contains both, add an error
        if contains_product_version && contains_product_version_range {
            errors.get_or_insert_default().push(create_both_product_version_version_range_error(path_str));
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_58
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_58_product_version_and_product_version_range_in_one_path(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::create_both_product_version_version_range_error;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_6_1_58() {
        // Case 01: Two path, both with version and version range
        let case_01 = Err(vec![
            create_both_product_version_version_range_error("/product_tree/branches/0/branches/0/branches/0/branches/0/product".to_string()),
            create_both_product_version_version_range_error("/product_tree/branches/0/branches/0/branches/1/branches/0/product".to_string()),
        ]);
        // Case 11: Only version range

        TESTS_2_1.test_6_1_58.expect(case_01, Ok(()))
    }
}
