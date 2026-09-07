use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

/// To implement this test it is deemed sufficient that the value of name does not match the following regex:
/// `^[vV][0-9].*$`. This function implements this trivial regex using regular string operations, which is significantly faster.
fn is_version_with_v_indicator(version: &str) -> bool {
    let mut chars = version.chars();
    matches!(chars.next(), Some('v') | Some('V')) && matches!(chars.next(), Some(c) if c.is_ascii_digit())
}

fn create_v_version_indicator_error(version: &str, path: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "Product version name {version} starting with 'v' or 'V' as version indicator is not recommended"
        ),
        instance_path: format!("{path}/name"),
    })
}

/// 6.3.11 Usage of V as Version Indicator
///
/// Tests that products in the product tree with the `product_version` branch category do not start
/// with a `v` or `V` before their version.
pub fn test_6_3_11_usage_of_v_as_version_indicator(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    if let Some(product_tree) = doc.get_product_tree().as_ref() {
        product_tree.visit_all_branches(&mut |branch, path| {
            if branch.get_category() == CategoryOfTheBranch::ProductVersion
                && is_version_with_v_indicator(branch.get_name())
            {
                errors
                    .get_or_insert_default()
                    .push(create_v_version_indicator_error(branch.get_name(), path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_11, test_6_3_11_usage_of_v_as_version_indicator);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_11 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_11 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use rstest::rstest;

    #[rstest]
    #[case("v1", true)]
    #[case("V1", true)]
    #[case("v4.2", true)]
    #[case("V4.2", true)]
    #[case("v1.0.0", true)]
    #[case("V1.0.0", true)]
    #[case("", false)]
    #[case("v", false)]
    #[case("V", false)]
    #[case("1", false)]
    #[case("4.2", false)]
    #[case("1.0.0", false)]
    #[case("vAlpha", false)]
    #[case("VAlpha", false)]
    #[case("version4.2", false)]
    fn test_is_version_with_v_indicator(#[case] version: &str, #[case] expected: bool) {
        assert_eq!(is_version_with_v_indicator(version), expected);
    }

    #[test]
    fn test_test_6_3_11() {
        let case_v_4_2 = Err(vec![create_v_version_indicator_error(
            "v4.2",
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_uppercase_v_4_2 = Err(vec![create_v_version_indicator_error(
            "V4.2",
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_multiple_parallel_branches = Err(vec![
            create_v_version_indicator_error("v4.2", "/product_tree/branches/0/branches/0/branches/0"),
            create_v_version_indicator_error("v6.2", "/product_tree/branches/0/branches/0/branches/2"),
        ]);
        // Note: Having stacked product version categories violates 6.1.57, making this test file mandatory invalid.
        let case_multiple_nested_branches = Err(vec![
            create_v_version_indicator_error("v4.2", "/product_tree/branches/0/branches/0/branches/0"),
            create_v_version_indicator_error(
                "v4.2.2-alpha",
                "/product_tree/branches/0/branches/0/branches/0/branches/0/branches/0",
            ),
        ]);

        // Case 11: product version "4.2"
        // Case S11: product version "vAlpha"
        // Case S12: architecture "v4.2"

        TESTS_2_0.test_6_3_11.expect(ExpectedResults_2_0 {
            case_01: case_v_4_2.clone(),
            case_s01: case_uppercase_v_4_2.clone(),
            case_s02: case_multiple_parallel_branches.clone(),
            case_s03: case_multiple_nested_branches.clone(),
            case_11: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });
        TESTS_2_1.test_6_3_11.expect(ExpectedResults_2_1 {
            case_01: case_v_4_2,
            case_s01: case_uppercase_v_4_2,
            case_s02: case_multiple_parallel_branches.clone(),
            case_s03: case_multiple_nested_branches.clone(),
            case_11: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });
    }
}
