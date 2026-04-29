use crate::csaf_traits::{BranchTrait, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

static MAX_DEPTH: u32 = 30;

fn create_excessive_branch_depth_error(branch_index: usize, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Branches recursion depth too big (> {MAX_DEPTH})"),
        instance_path: format!("/product_tree/branches/{branch_index}{path}"),
    }
}

pub fn test_6_1_34_branches_recursion_depth(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // TODO This can be wasSkipped in the future
    let Some(branches) = doc.get_product_tree().and_then(|t| t.get_branches()) else {
        return Ok(());
    };

    let mut errors: Option<Vec<ValidationError>> = None;
    for (i, branch) in branches.iter().enumerate() {
        if let Some(path) = branch.find_excessive_branch_depth(MAX_DEPTH) {
            errors
                .get_or_insert_default()
                .push(create_excessive_branch_depth_error(i, &path));
        }
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_34, test_6_1_34_branches_recursion_depth);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_34() {
        // Case 01: One long branch structure with depth of 31
        // Case 02: More complex branch structure with 3 sub-branches, one of which has depth of 31
        // Case S01: Two long branch structures with depth of 31
        // Case 11: One long branch structure with depth of 30

        let one_too_long_branch_path = "/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0";
        let one_too_long_branch_error = Err(vec![create_excessive_branch_depth_error(0, one_too_long_branch_path)]);
        let two_too_long_branches_error = Err(vec![
            create_excessive_branch_depth_error(0, one_too_long_branch_path),
            create_excessive_branch_depth_error(1, one_too_long_branch_path),
        ]);
        let more_complex_too_long_branch_error = Err(vec![create_excessive_branch_depth_error(
            0,
            "/branches/0/branches/1/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0",
        )]);
        TESTS_2_1.test_6_1_34.expect(
            one_too_long_branch_error,
            more_complex_too_long_branch_error,
            two_too_long_branches_error,
            Ok(()),
        );
    }
}
