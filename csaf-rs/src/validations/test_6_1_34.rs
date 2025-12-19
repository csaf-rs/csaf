use crate::csaf_traits::{BranchTrait, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

static MAX_DEPTH: u32 = 30;

fn create_excessive_branch_depth_error(branch_index: usize, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Branches recursion depth too big (> {})", MAX_DEPTH),
        instance_path: format!("/product_tree/branches/{}{}", branch_index, path),
    }
}

pub fn test_6_1_34_branches_recursion_depth(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    if let Some(tree) = doc.get_product_tree().as_ref() {
        if let Some(branches) = tree.get_branches() {
            for (i, branch) in branches.iter().enumerate() {
                if let Some(path) = branch.find_excessive_branch_depth(MAX_DEPTH) {
                    return Err(vec![create_excessive_branch_depth_error(i, &path)]);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_34() {
        run_csaf21_tests(
            "34",
            test_6_1_34_branches_recursion_depth,
            HashMap::from([
                (
                    "01",
                    vec![create_excessive_branch_depth_error(
                        0,
                        "/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0",
                    )],
                ),
                (
                    "02",
                    vec![create_excessive_branch_depth_error(
                        0,
                        "/branches/0/branches/1/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0",
                    )],
                ),
            ]),
        );
    }
}
