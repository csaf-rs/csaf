use crate::csaf::getter_traits::{BranchTrait, CsafTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;

static MAX_DEPTH: u32 = 30;

pub fn test_6_1_34_branches_recursion_depth(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(tree) = doc.get_product_tree().as_ref() {
        if let Some(path) = find_excessive_branch_depth(tree.get_branches(), MAX_DEPTH) {
            return Err(ValidationError {
                message: format!("Branches recursion depth too big (> {})", MAX_DEPTH),
                instance_path: format!("/product_tree{}", path)
            });
        }
    }
    Ok(())
}

fn find_excessive_branch_depth(branches: Option<&Vec<impl BranchTrait>>, remaining_depth: u32) -> Option<String> {
    if let Some(branches) = branches {
        for (i, branch) in branches.iter().enumerate() {
            if let Some(subpath) = find_excessive_branch_depth_rec(branch, remaining_depth) {
                return Some(format!("/branches/{}{}", i, subpath));
            }
        }
    }
    None
}

fn find_excessive_branch_depth_rec(branch: &impl BranchTrait, remaining_depth: u32) -> Option<String> {
    if let Some(branches) = branch.get_branches() {
        // If we've reached depth limit and there are branches, we've found a violation
        if remaining_depth == 1 {
            return Some("/branches/0".to_string());
        }

        // Otherwise, check the branches with one less remaining depth
        return find_excessive_branch_depth(Some(branches), remaining_depth - 1);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_34::test_6_1_34_branches_recursion_depth;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_34() {
        run_csaf21_tests(
            "34",
            test_6_1_34_branches_recursion_depth,
            HashMap::from([
                ("01", &ValidationError {
                    message: "Branches recursion depth too big (> 30)".to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Branches recursion depth too big (> 30)".to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/1/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0".to_string(),
                }),
            ]),
        );
    }
}
