use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait, build_leaf_instance_path};
use crate::validation::ValidationError;

fn format_category_path(categories: &[&CategoryOfTheBranch]) -> String {
    categories
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn create_branch_categories_error(
    full_path: &[&CategoryOfTheBranch],
    relevant_categories: &[&CategoryOfTheBranch],
    instance_path: String,
) -> ValidationError {
    let full_display = format_category_path(full_path);
    let found_display = if relevant_categories.is_empty() {
        "(none)".to_string()
    } else {
        format_category_path(relevant_categories)
    };
    ValidationError {
        message: format!(
            "Branch path to product does not follow the recommended sequence of the categories 'vendor' -> 'product_name' -> 'product_version'. Along the categories of this path '{full_display}', the following sequence was found: '{found_display}'"
        ),
        instance_path,
    }
}

/// Required category sequence for branch paths.
const REQUIRED_CATEGORIES_ORDER: [&CategoryOfTheBranch; 3] = [
    &CategoryOfTheBranch::Vendor,
    &CategoryOfTheBranch::ProductName,
    &CategoryOfTheBranch::ProductVersion,
];

/// 6.3.9 Branch Categories
///
/// For each element of type `/$defs/full_product_name_t` in `/product_tree/branches` it must be tested
/// that ancestor nodes along the path exist which use the following branch categories
/// `vendor` -> `product_name` -> `product_version` in that order starting with the product tree node.
///
/// Other branch categories can be used before, after or between the aforementioned branch categories
/// without making the test invalid.
pub fn test_6_3_9_branch_categories(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // this will be a Passed::NoData later (#409)
    };

    // get all paths from root to leaves in the product tree
    let leaf_paths = product_tree.collect_leaf_paths();

    // for every path to a leaf node
    for (path, indices) in leaf_paths {
        // filter to only the categories relevant to this rule, preserving order
        let relevant: Vec<&CategoryOfTheBranch> = path
            .iter()
            .map(|b| b.get_category())
            .filter(|c| REQUIRED_CATEGORIES_ORDER.contains(c))
            .collect();

        // Check whether the filtered list matches the required sequence exactly
        if !relevant.iter().eq(REQUIRED_CATEGORIES_ORDER.iter()) {
            // collect all categories only when needed for the error message
            let all_categories: Vec<&CategoryOfTheBranch> = path.iter().map(|b| b.get_category()).collect();
            errors.get_or_insert_default().push(create_branch_categories_error(
                &all_categories,
                &relevant,
                build_leaf_instance_path(&indices),
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_9, test_6_3_9_branch_categories);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_9() {
        let case_01_missing_product_version = Err(vec![create_branch_categories_error(
            &[
                &CategoryOfTheBranch::Vendor,
                &CategoryOfTheBranch::ProductName,
                &CategoryOfTheBranch::PatchLevel,
            ],
            &[&CategoryOfTheBranch::Vendor, &CategoryOfTheBranch::ProductName],
            "/product_tree/branches/0/branches/0/branches/0/product".to_string(),
        )]);

        let case_02_missing_vendor = Err(vec![
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::ProductFamily,
                    &CategoryOfTheBranch::ProductName,
                    &CategoryOfTheBranch::ProductVersion,
                ],
                &[&CategoryOfTheBranch::ProductName, &CategoryOfTheBranch::ProductVersion],
                "/product_tree/branches/0/branches/0/branches/0/product".to_string(),
            ),
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::ProductFamily,
                    &CategoryOfTheBranch::ProductName,
                    &CategoryOfTheBranch::ProductVersion,
                ],
                &[&CategoryOfTheBranch::ProductName, &CategoryOfTheBranch::ProductVersion],
                "/product_tree/branches/0/branches/0/branches/1/product".to_string(),
            ),
        ]);

        let case_03_missing_vendor_wrong_order = Err(vec![
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::ProductFamily,
                    &CategoryOfTheBranch::ProductVersion,
                    &CategoryOfTheBranch::ProductName,
                ],
                &[&CategoryOfTheBranch::ProductVersion, &CategoryOfTheBranch::ProductName],
                "/product_tree/branches/0/branches/0/branches/0/product".to_string(),
            ),
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::ProductFamily,
                    &CategoryOfTheBranch::ProductVersion,
                    &CategoryOfTheBranch::ProductName,
                ],
                &[&CategoryOfTheBranch::ProductVersion, &CategoryOfTheBranch::ProductName],
                "/product_tree/branches/0/branches/0/branches/1/product".to_string(),
            ),
        ]);

        let case_04_categories: &[&CategoryOfTheBranch] = &[
            &CategoryOfTheBranch::Vendor,
            &CategoryOfTheBranch::ProductVersion,
            &CategoryOfTheBranch::ProductName,
        ];
        let case_04_wrong_order = Err(vec![
            create_branch_categories_error(
                case_04_categories,
                case_04_categories,
                "/product_tree/branches/0/branches/0/branches/0/product".to_string(),
            ),
            create_branch_categories_error(
                case_04_categories,
                case_04_categories,
                "/product_tree/branches/0/branches/0/branches/1/product".to_string(),
            ),
        ]);

        let case_05_full: &[&CategoryOfTheBranch] = &[
            &CategoryOfTheBranch::HostName,
            &CategoryOfTheBranch::Vendor,
            &CategoryOfTheBranch::ProductVersion,
            &CategoryOfTheBranch::Language,
            &CategoryOfTheBranch::ProductName,
            &CategoryOfTheBranch::Architecture,
            &CategoryOfTheBranch::ServicePack,
            &CategoryOfTheBranch::PatchLevel,
        ];
        let case_05_relevant = &[
            &CategoryOfTheBranch::Vendor,
            &CategoryOfTheBranch::ProductVersion,
            &CategoryOfTheBranch::ProductName,
        ];
        let case_05_wrong_order_deep_tree = Err(vec![
            create_branch_categories_error(
                case_05_full,
                case_05_relevant,
                "/product_tree/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/product".to_string(),
            ),
            create_branch_categories_error(
                case_05_full,
                case_05_relevant,
                "/product_tree/branches/0/branches/0/branches/0/branches/0/branches/0/branches/1/branches/0/branches/0/product".to_string(),
            ),
        ]);

        let case_06_missing_vendor_name_version = Err(vec![
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::HostName,
                    &CategoryOfTheBranch::Architecture,
                    &CategoryOfTheBranch::Language,
                ],
                &[],
                "/product_tree/branches/0/branches/0/branches/0/product".to_string(),
            ),
            create_branch_categories_error(
                &[
                    &CategoryOfTheBranch::HostName,
                    &CategoryOfTheBranch::Architecture,
                    &CategoryOfTheBranch::ServicePack,
                    &CategoryOfTheBranch::PatchLevel,
                ],
                &[],
                "/product_tree/branches/0/branches/1/branches/0/branches/0/product".to_string(),
            ),
        ]);

        // Case 11: Minimal passing example (vendor -> name -> version)
        // Case 12: vendor -> family -> name -> split to 2x version
        // Case 13: vendor -> family -> split to 2x name -> version
        // Case 14: vendor -> split to 2x name -> version
        // Case 15: Deep tree, split after 2x after name

        TESTS_2_0.test_6_3_9.expect(
            case_01_missing_product_version.clone(),
            case_02_missing_vendor.clone(),
            case_03_missing_vendor_wrong_order.clone(),
            case_04_wrong_order.clone(),
            case_05_wrong_order_deep_tree.clone(),
            case_06_missing_vendor_name_version.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_3_9.expect(
            case_01_missing_product_version,
            case_02_missing_vendor,
            case_03_missing_vendor_wrong_order,
            case_04_wrong_order,
            case_05_wrong_order_deep_tree,
            case_06_missing_vendor_name_version,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
