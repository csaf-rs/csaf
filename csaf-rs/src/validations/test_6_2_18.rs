use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use regex::Regex;
use std::sync::LazyLock;

fn create_product_version_range_without_vers_error(version_range: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Product version range {} does not match vers syntax", version_range),
        instance_path: format!("{}/name", path),
    }
}

static VERS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^vers:[a-z.\-+][a-z0-9.\-+]*/.+").unwrap());

/// 6.2.18 Product Version Range without vers
///
/// Tests that in the product tree, all branches with the category `product_version_range` use vers
/// in their `name` property.
pub fn test_6_2_18_product_version_range_without_vers(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree().as_ref() {
        product_tree.visit_all_branches(&mut |branch, path| {
            if branch.get_category() == &CategoryOfTheBranch::ProductVersionRange
                && !VERS_REGEX.is_match(branch.get_name())
            {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_product_version_range_without_vers_error(branch.get_name(), path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_18
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_18_product_version_range_without_vers(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_18
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_18_product_version_range_without_vers(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_18() {
        let case_01 = Err(vec![create_product_version_range_without_vers_error(
            ">4.2",
            "/product_tree/branches/0/branches/0/branches/0",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_18.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_2_18.expect(case_01, Ok(()));
    }
}
