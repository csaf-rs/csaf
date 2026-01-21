use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use regex::Regex;
use std::sync::LazyLock;

static V_AS_VERSION_INDICATOR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[vV][0-9].*$").unwrap());

fn create_v_version_indicator_error(version: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Product version name {} starting with 'v' or 'V' as version indicator is not recommended",
            version
        ),
        instance_path: format!("{}/name", path),
    }
}

/// 6.3.11 Usage of V as Version Indicator
///
/// Tests that products in the product tree with the `product_version` branch category do not start
/// with a `v` or `V` before their version.
pub fn test_6_3_11_usage_of_v_as_version_indicator(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree().as_ref() {
        product_tree.visit_all_branches(&mut |branch, path| {
            if branch.get_category() == &CategoryOfTheBranch::ProductVersion
                && V_AS_VERSION_INDICATOR_REGEX.is_match(branch.get_name())
            {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_v_version_indicator_error(branch.get_name(), path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_3_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_11_usage_of_v_as_version_indicator(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_3_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_11_usage_of_v_as_version_indicator(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_11() {
        let case_01 = Err(vec![create_v_version_indicator_error(
            "v4.2",
            "/product_tree/branches/0/branches/0/branches/0",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_3_11.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_3_11.expect(case_01, Ok(()));
    }
}
