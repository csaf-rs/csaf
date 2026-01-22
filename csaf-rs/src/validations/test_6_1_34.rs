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
    if let Some(tree) = doc.get_product_tree().as_ref()
        && let Some(branches) = tree.get_branches()
    {
        for (i, branch) in branches.iter().enumerate() {
            if let Some(path) = branch.find_excessive_branch_depth(MAX_DEPTH) {
                return Err(vec![create_excessive_branch_depth_error(i, &path)]);
            }
        }
    }
    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_34
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_34_branches_recursion_depth(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_34() {
        // Only CSAF 2.1 has this test with 3 test cases
        TESTS_2_1.test_6_1_34.expect(
            Err(vec![create_excessive_branch_depth_error(
                0,
                "/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0",
            )]),
            Err(vec![create_excessive_branch_depth_error(
                0,
                "/branches/0/branches/1/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0",
            )]),
            Ok(()),
        );
    }
}
