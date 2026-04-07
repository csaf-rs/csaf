use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait, build_leaf_instance_path};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn create_stacked_categories_error(
    stacked_categories: &[(&CategoryOfTheBranch, &u64)],
    instance_path: String,
) -> ValidationError {
    // singular / plural of category
    let category_word = if stacked_categories.len() == 1 {
        "category"
    } else {
        "categories"
    };
    // generate list of "stacked category x (count), ..."
    let stacked_list: Vec<String> = stacked_categories
        .iter()
        .map(|(cat, count)| format!("{cat} ({count})"))
        .collect();
    let stacked_list_str = stacked_list.join(", ");
    ValidationError {
        message: format!("Stacked branch {category_word} found in path: {stacked_list_str}"),
        instance_path,
    }
}

/// 6.1.57 Stacked Branch Categories
///
/// In the product tree, the path from root to any FPN, all branch categories (except 'product_family')
/// are only allowed to occur once.
pub fn test_6_1_57_stacked_branch_categories(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // this will be a Passed::NoData later (#409)
    };

    // get all paths from root to leaves in the product tree
    let leaf_paths = product_tree.collect_leaf_paths();

    // for every path
    for (path, indices) in leaf_paths {
        // generate a hashmap of category occurrences
        let mut categories_in_path_map: HashMap<&CategoryOfTheBranch, u64> = HashMap::new();
        for branch in &path {
            if branch.get_category() == &CategoryOfTheBranch::ProductFamily {
                continue;
            }
            categories_in_path_map
                .entry(branch.get_category())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        // filter hashmap to only categories that occur more than once
        let mut stacked_categories: Vec<(&CategoryOfTheBranch, &u64)> = categories_in_path_map
            .iter()
            .filter(|(_, count)| **count > 1)
            .map(|(cat, count)| (*cat, count))
            .collect();
        // sort found categories deterministic
        stacked_categories.sort_by_key(|(cat, _)| *cat);

        // if there are any, create an error for this path
        if !stacked_categories.is_empty() {
            errors.get_or_insert_default().push(create_stacked_categories_error(
                &stacked_categories,
                build_leaf_instance_path(&indices),
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_57, test_6_1_57_stacked_branch_categories);

#[cfg(test)]
mod tests {
    use crate::csaf_traits::CategoryOfTheBranch;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::test_6_1_57::create_stacked_categories_error;

    #[test]
    fn test_6_1_57() {
        let case_01_simple = Err(vec![create_stacked_categories_error(
            &[(&CategoryOfTheBranch::Vendor, &2)],
            "/product_tree/branches/0/branches/0/branches/0/branches/0/product".to_string(),
        )]);

        let case_02_depth = Err(vec![
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::ProductName, &2)],
                "/product_tree/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/product".to_string()
            ),
        ]);

        let case_03_breadth = Err(vec![
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/0/branches/0/branches/0/branches/0/product"
                    .to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/0/branches/0/branches/0/branches/1/product"
                    .to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/0/branches/0/branches/1/branches/0/product"
                    .to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/0/branches/1/branches/0/branches/0/product"
                    .to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/0/branches/1/branches/1/branches/0/product"
                    .to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/1/branches/0/branches/0/product".to_string(),
            ),
            create_stacked_categories_error(
                &[(&CategoryOfTheBranch::Architecture, &3)],
                "/product_tree/branches/0/branches/0/branches/1/branches/1/branches/1/branches/0/product".to_string(),
            ),
        ]);

        // Case 11: no stacked categories
        // Case 12: 30 branches deep, but the only stacked category is product_family
        // Case 13: breadth with no stacked categories

        TESTS_2_1
            .test_6_1_57
            .expect(case_01_simple, case_02_depth, case_03_breadth, Ok(()), Ok(()), Ok(()))
    }
}
