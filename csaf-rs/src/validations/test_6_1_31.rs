use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

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

/// Check the branch name for forbidden substrings and return a vector of found substrings
/// if any are found.
fn check_branch_name_for_forbidden_substrings(branch_name: &str) -> Option<Vec<&'static str>> {
    let mut forbidden_substrings: Option<Vec<&str>> = None;
    let mut branch_name = branch_name.to_lowercase();
    // check for `>=` first, then remove `>=`
    if branch_name.contains(FORBIDDEN_GREATER_EQUAL) {
        forbidden_substrings
            .get_or_insert_with(Vec::new)
            .push(FORBIDDEN_GREATER_EQUAL);
    }
    branch_name = branch_name.replace(FORBIDDEN_GREATER_EQUAL, "");
    // then check for `>`, then remove `>`
    if branch_name.contains(FORBIDDEN_GREATER) {
        forbidden_substrings
            .get_or_insert_with(Vec::new)
            .push(FORBIDDEN_GREATER);
    }
    branch_name = branch_name.replace(FORBIDDEN_GREATER, "");
    // check for `<=` first, then remove `<=`
    if branch_name.contains(FORBIDDEN_LESS_EQUAL) {
        forbidden_substrings
            .get_or_insert_with(Vec::new)
            .push(FORBIDDEN_LESS_EQUAL);
    }
    branch_name = branch_name.replace(FORBIDDEN_LESS_EQUAL, "");
    // check for `<`, then remove `<`
    if branch_name.contains(FORBIDDEN_LESS) {
        forbidden_substrings.get_or_insert_with(Vec::new).push(FORBIDDEN_LESS);
    }
    branch_name = branch_name.replace(FORBIDDEN_LESS, "");
    // check for the other keywords, tokenized by Unicode whitespace
    for token in branch_name.split_whitespace() {
        if let Some(&keyword) = FORBIDDEN_KEYWORDS.iter().find(|&&kw| kw == token) {
            forbidden_substrings.get_or_insert_with(Vec::new).push(keyword);
        }
    }
    forbidden_substrings
}

/// 6.1.31 Version Range in Product Version
/// All branches with type `product_version` in the product tree must not contain any of the substrings
/// `<, <=, >, >=, after, all, before, earlier, later, prior, versions` in their branch `name`.
/// `<=` and `>=` are prioritized before `<` and `>` respectively. The error contains all offending substrings.
pub fn test_6_1_31_version_range_in_product_version_branch_name(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    if let Some(product_tree) = doc.get_product_tree().as_ref() {
        product_tree.visit_all_branches(&mut |branch, path| {
            if branch.get_category() == &CategoryOfTheBranch::ProductVersion {
                // if there are any forbidden substrings found, create an error
                if let Some(forbidden_substrings) = check_branch_name_for_forbidden_substrings(branch.get_name()) {
                    errors
                        .get_or_insert_with(Vec::new)
                        .push(create_forbidden_strings_in_version_error(
                            branch.get_name(),
                            forbidden_substrings,
                            path,
                        ));
                }
            }
        })
    }

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
        let case_01 = Err(vec![create_forbidden_strings_in_version_error(
            "prior to 4.2",
            vec!["prior"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_02 = Err(vec![create_forbidden_strings_in_version_error(
            "<4.2",
            vec!["<"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_03 = Err(vec![create_forbidden_strings_in_version_error(
            "<=4.1",
            vec!["<="],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_04 = Err(vec![create_forbidden_strings_in_version_error(
            "<= 4.1",
            vec!["<="],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_05 = Err(vec![create_forbidden_strings_in_version_error(
            "4.1 and earlier",
            vec!["earlier"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_06 = Err(vec![create_forbidden_strings_in_version_error(
            "all",
            vec!["all"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_07 = Err(vec![create_forbidden_strings_in_version_error(
            "before 4.2",
            vec!["before"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_08 = Err(vec![create_forbidden_strings_in_version_error(
            "4.2 and later",
            vec!["later"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        let case_09 = Err(vec![create_forbidden_strings_in_version_error(
            "3.X versions",
            vec!["versions"],
            "/product_tree/branches/0/branches/0/branches/0",
        )]);
        TESTS_2_0.test_6_1_31.expect(
            case_01.clone(),
            case_02.clone(),
            case_03.clone(),
            case_04.clone(),
            case_05.clone(),
            case_06.clone(),
            case_07.clone(),
            case_08.clone(),
            case_09.clone(),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_31.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            case_07,
            case_08,
            case_09,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }

    #[test]
    fn test_check_branch_name_for_forbidden_substrings() {
        let test_cases = vec![
            ("Version 2.0", vec![]), // No forbidden substrings
            ("Version >= 2.0", vec![FORBIDDEN_GREATER_EQUAL]),
            ("Version > 2.0", vec![FORBIDDEN_GREATER]),
            ("Version <= 2.0", vec![FORBIDDEN_LESS_EQUAL]),
            ("Version < 2.0", vec![FORBIDDEN_LESS]),
            ("After 2.0", vec!["after"]),
            ("All", vec!["all"]),
            ("Before 2.0", vec!["before"]),
            ("Earlier than 2.0", vec!["earlier"]),
            ("Later than 2.0", vec!["later"]),
            ("Prior to 2.0", vec!["prior"]),
            ("3.X Versions", vec!["versions"]),
            (">=2.0 and <=3.0", vec![FORBIDDEN_GREATER_EQUAL, FORBIDDEN_LESS_EQUAL]),
            (">=2.0 and before 3.0", vec![FORBIDDEN_GREATER_EQUAL, "before"]),
            (
                ">=2.0 and <=3.0 and >4.1 and <5.1",
                vec![
                    FORBIDDEN_GREATER_EQUAL,
                    FORBIDDEN_GREATER,
                    FORBIDDEN_LESS_EQUAL,
                    FORBIDDEN_LESS,
                ],
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
