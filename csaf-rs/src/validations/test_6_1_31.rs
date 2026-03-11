use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn create_forbidden_strings_in_version_error(
    product_name: &str,
    forbidden_substrings: Vec<&str>,
    product_path: &str,
) -> ValidationError {
    let forbidden_substrings_str: String = {
        if forbidden_substrings.len() > 1 {
            format!("substrings '{}'", forbidden_substrings.join("', '"))
        } else {
            format!("substring '{}'", forbidden_substrings[0])
        }
    };
    ValidationError {
        message: format!("Product version '{product_name}' contains forbidden {forbidden_substrings_str}"),
        instance_path: format!("{product_path}/name"),
    }
}
const FORBIDDEN_LESS_EQUAL: &str = "<=";
const FORBIDDEN_GREATER_EQUAL: &str = ">=";
const FORBIDDEN_LESS: &str = "<";
const FORBIDDEN_GREATER: &str = ">";
const FORBIDDEN_KEYWORDS: &[&str] = &["after", "all", "before", "earlier", "later", "prior", "versions"];

/// Check the branch name for forbidden substrings (operators and keywords).
/// Operators `<=` and `>=` are prioritized before `<` and `>` respectively.
/// For keywords, the string is tokenized by Unicode whitespace and scanned case-insensitive exact matches.
///
/// # Returns
///
/// * Some(Vec<&str>) - A vector of unique forbidden substrings found in the branch name, sorted alphabetically.
/// * None - If no forbidden substrings are found.
fn check_branch_name_for_forbidden_substrings(branch_name: &str) -> Option<Vec<&'static str>> {
    let mut forbidden_substrings: Option<HashSet<&'static str>> = None;
    let mut branch_name = branch_name.to_lowercase();
    // check for `>=` first, then remove `>=`
    if branch_name.contains(FORBIDDEN_GREATER_EQUAL) {
        forbidden_substrings
            .get_or_insert_default()
            .insert(FORBIDDEN_GREATER_EQUAL);
    }
    branch_name = branch_name.replace(FORBIDDEN_GREATER_EQUAL, "");
    // then check for `>`, then remove `>`
    if branch_name.contains(FORBIDDEN_GREATER) {
        forbidden_substrings.get_or_insert_default().insert(FORBIDDEN_GREATER);
    }
    branch_name = branch_name.replace(FORBIDDEN_GREATER, "");
    // check for `<=` first, then remove `<=`
    if branch_name.contains(FORBIDDEN_LESS_EQUAL) {
        forbidden_substrings
            .get_or_insert_default()
            .insert(FORBIDDEN_LESS_EQUAL);
    }
    branch_name = branch_name.replace(FORBIDDEN_LESS_EQUAL, "");
    // check for `<`, then remove `<`
    if branch_name.contains(FORBIDDEN_LESS) {
        forbidden_substrings.get_or_insert_default().insert(FORBIDDEN_LESS);
    }
    branch_name = branch_name.replace(FORBIDDEN_LESS, "");
    // check for the other keywords, tokenized by Unicode whitespace
    for token in branch_name.split_whitespace() {
        if let Some(&keyword) = FORBIDDEN_KEYWORDS.iter().find(|&&kw| kw == token) {
            forbidden_substrings.get_or_insert_default().insert(keyword);
        }
    }
    forbidden_substrings.map(|set| {
        let mut vec: Vec<&str> = set.into_iter().collect();
        vec.sort();
        vec
    })
}

/// 6.1.31 Version Range in Product Version
/// All branches with type `product_version` in the product tree must not contain any of the operators
/// `<, <=, >, >=` or keywords `after, all, before, earlier, later, prior, versions` when
/// tokenized by whitespace in their branch `name`.
/// `<=` and `>=` are prioritized before `<` and `>` respectively.
/// The error contains all unique offending operators and keywords.
pub fn test_6_1_31_version_range_in_product_version_branch_name(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree().as_ref() else {
        return Ok(());
    };

    let mut errors: Option<Vec<ValidationError>> = None;

    product_tree.visit_all_branches(&mut |branch, path| {
        if branch.get_category() == &CategoryOfTheBranch::ProductVersion {
            // if there are any forbidden substrings found, create an error
            if let Some(forbidden_substrings) = check_branch_name_for_forbidden_substrings(branch.get_name()) {
                errors
                    .get_or_insert_default()
                    .push(create_forbidden_strings_in_version_error(
                        branch.get_name(),
                        forbidden_substrings,
                        path,
                    ));
            }
        }
    });

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_31
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_31_version_range_in_product_version_branch_name(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_31
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_31_version_range_in_product_version_branch_name(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_31() {
        // Case 01: Keyword "prior"
        // Case 02: Operator "<"
        // Case 03: Operator "<="
        // Case 04: Operator "<=" with space
        // Case 05: Keyword "earlier"
        // Case 06: Keyword "all"
        // Case 07: Keyword "before"
        // Case 08: Keyword "later"
        // Case 09: Keyword "versions"
        // Case 11: Using product_version_range
        // Case 12: Keyword "after" as part of word "after-eight"
        // Case 13: Keyword "all" as part of word "overall"

        // Case S01: Keyword "ALL"
        // Case S02: Operator ">"
        // Case S03: Operator ">="
        // Case S04: Multiple operators and keywords ">=2.0 and <3.0 and after 4.1 and before 5.1"
        // Case S11: Keyword "all" as part of word "overall" (backport for CSAF 2.0, CSAF 2.1 has this as Case 13)
        // Case S12: Just a valid branch name "2.0"

        let case_01_prior = Err(vec![create_forbidden_strings_in_version_error(
            "prior to 4.2",
            vec!["prior"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_02_less_than = Err(vec![create_forbidden_strings_in_version_error(
            "<4.2",
            vec!["<"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_03_less_equal = Err(vec![create_forbidden_strings_in_version_error(
            "<=4.1",
            vec!["<="],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_04_less_equal_space = Err(vec![create_forbidden_strings_in_version_error(
            "<= 4.1",
            vec!["<="],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_05_earlier = Err(vec![create_forbidden_strings_in_version_error(
            "4.1 and earlier",
            vec!["earlier"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_06_all = Err(vec![create_forbidden_strings_in_version_error(
            "all",
            vec!["all"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_07_before = Err(vec![create_forbidden_strings_in_version_error(
            "before 4.2",
            vec!["before"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_08_later = Err(vec![create_forbidden_strings_in_version_error(
            "4.2 and later",
            vec!["later"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_09_versions = Err(vec![create_forbidden_strings_in_version_error(
            "3.X versions",
            vec!["versions"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_s01_all_uppercase = Err(vec![create_forbidden_strings_in_version_error(
            "ALL",
            vec!["all"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_s02_greater_than = Err(vec![create_forbidden_strings_in_version_error(
            ">4.2",
            vec![">"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_s03_greater_equal = Err(vec![create_forbidden_strings_in_version_error(
            ">=4.2",
            vec![">="],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_s04_multiple = Err(vec![create_forbidden_strings_in_version_error(
            ">=2.0 and <3.0 and after 4.1 and before 5.1",
            vec!["<", ">=", "after", "before"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);

        TESTS_2_0.test_6_1_31.expect(
            case_01_prior.clone(),
            case_02_less_than.clone(),
            case_03_less_equal.clone(),
            case_04_less_equal_space.clone(),
            case_05_earlier.clone(),
            case_06_all.clone(),
            case_07_before.clone(),
            case_08_later.clone(),
            case_09_versions.clone(),
            case_s01_all_uppercase.clone(),
            case_s02_greater_than.clone(),
            case_s03_greater_equal.clone(),
            case_s04_multiple.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_31.expect(
            case_01_prior,
            case_02_less_than,
            case_03_less_equal,
            case_04_less_equal_space,
            case_05_earlier,
            case_06_all,
            case_07_before,
            case_08_later,
            case_09_versions,
            case_s01_all_uppercase,
            case_s02_greater_than,
            case_s03_greater_equal,
            case_s04_multiple,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }

    // Additional tests for the helper function `check_branch_name_for_forbidden_substrings`.
    // These are not meant to be exhaustive, but to cover some additional edge cases that are not
    // covered by the upstream and supplementary test cases above.
    #[test]
    fn test_check_branch_name_for_forbidden_substrings() {
        let test_cases = vec![
            // more operators with spaces
            ("Version >= 2.0", vec![FORBIDDEN_GREATER_EQUAL]),
            ("Version > 2.0", vec![FORBIDDEN_GREATER]),
            ("Version < 2.0", vec![FORBIDDEN_LESS]),
            // more uppercase
            ("After 2.0", vec!["after"]),
            ("Before 2.0", vec!["before"]),
            ("Earlier than 2.0", vec!["earlier"]),
            ("Later than 2.0", vec!["later"]),
            ("Prior to 2.0", vec!["prior"]),
            ("3.X Versions", vec!["versions"]),
            // Priority of <= over < and >= over >
            (
                ">2.0 and <=3.0 and >=4.0",
                vec![FORBIDDEN_LESS_EQUAL, FORBIDDEN_GREATER, FORBIDDEN_GREATER_EQUAL],
            ),
            (
                "<2.0 and >=3.0 and <=4.0",
                vec![FORBIDDEN_LESS, FORBIDDEN_LESS_EQUAL, FORBIDDEN_GREATER_EQUAL],
            ),
            // multiple of the same should be unique
            (
                ">=2.0 and <3.0 and >=4.0 and <5.0",
                vec![FORBIDDEN_LESS, FORBIDDEN_GREATER_EQUAL],
            ),
            (
                "after 2.0 and before 3.0 and after 4.0 and before 5.0",
                vec!["after", "before"],
            ),
            (
                ">=2.0 and before 3.0 and >= 4.0 and before 5.",
                vec![FORBIDDEN_GREATER_EQUAL, "before"],
            ),
        ];

        for (branch_name, expected) in test_cases {
            let result = check_branch_name_for_forbidden_substrings(branch_name);
            match result {
                Some(found) => {
                    assert_eq!(found, expected, "Failed for branch name: {branch_name}");
                },
                None => {
                    assert!(
                        expected.is_empty(),
                        "Expected no forbidden substrings for branch name: {branch_name}"
                    );
                },
            }
        }
    }
}
