use crate::csaf_traits::{BranchTrait, CsafTrait, ProductTreeTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheBranch;
use crate::validation::ValidationError;

fn create_forbidden_strings_in_version_error(
    product_name: &str,
    forbidden_substring: &str,
    product_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product version '{}' contains forbidden substring '{}'",
            product_name, forbidden_substring
        ),
        instance_path: format!("{}/name", product_path),
    }
}

/// This order implicitly ensures that <= will be found before <, which will also match.
const FORBIDDEN_SUBSTRINGS: &[&str] = &["<=", "<", ">=", ">"];
const FORBIDDEN_KEYWORDS: &[&str] = &["after", "all", "before", "earlier", "later", "prior", "versions"];

/// 6.1.31 Version Range in Product Version
/// All branches with type `product_version` in the product tree must not contain any of the substrings
/// `<, <=, >, >=, after, all, before, earlier, later, prior, versions` in their branch `name`.
/// `<=` and `>=` are prioritized before `<` and `>` respectively. This only returns the first error,
/// i.e. `all versions before versions 4.x` would only throw an error for the `all` substring.
pub fn test_6_1_31_version_range_in_product_version_branch_name(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    if let Some(product_tree) = doc.get_product_tree().as_ref() {
        if let Some(branches) = product_tree.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(&format!("/product_tree/branches/{}", i), &mut |branch, path| {
                    if branch.get_category() == &CategoryOfTheBranch::ProductVersion {
                        let branch_name = branch.get_name().to_lowercase();
                        for forbidden in FORBIDDEN_SUBSTRINGS {
                            if branch_name.contains(forbidden) {
                                errors
                                    .get_or_insert_with(Vec::new)
                                    .push(create_forbidden_strings_in_version_error(
                                        &branch.get_name(),
                                        forbidden,
                                        path,
                                    ));
                                break;
                            }
                        }
                        let product_name_tokenized = branch_name.split(' ').collect::<Vec<&str>>();
                        for token in product_name_tokenized {
                            if FORBIDDEN_KEYWORDS.contains(&token) {
                                errors
                                    .get_or_insert_with(Vec::new)
                                    .push(create_forbidden_strings_in_version_error(
                                        &branch.get_name(),
                                        token,
                                        path,
                                    ));
                                break;
                            }
                        }
                    }
                });
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_31() {
        let errors = HashMap::from([
            (
                "01",
                vec![create_forbidden_strings_in_version_error(
                    "prior to 4.2",
                    "prior",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "02",
                vec![create_forbidden_strings_in_version_error(
                    "<4.2",
                    "<",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "03",
                vec![create_forbidden_strings_in_version_error(
                    "<=4.1",
                    "<=",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "04",
                vec![create_forbidden_strings_in_version_error(
                    "<= 4.1",
                    "<=",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "05",
                vec![create_forbidden_strings_in_version_error(
                    "4.1 and earlier",
                    "earlier",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "06",
                vec![create_forbidden_strings_in_version_error(
                    "all",
                    "all",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "07",
                vec![create_forbidden_strings_in_version_error(
                    "before 4.2",
                    "before",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "08",
                vec![create_forbidden_strings_in_version_error(
                    "4.2 and later",
                    "later",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
            (
                "09",
                vec![create_forbidden_strings_in_version_error(
                    "3.X versions",
                    "versions",
                    "/product_tree/branches/0/branches/0/branches/0",
                )],
            ),
        ]);
        run_csaf20_tests(
            "31",
            test_6_1_31_version_range_in_product_version_branch_name,
            errors.clone(),
        );
        run_csaf21_tests("31", test_6_1_31_version_range_in_product_version_branch_name, errors);
    }
}
