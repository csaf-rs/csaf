use crate::csaf_traits::{BranchTrait, CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_branch_product_extension_info(branch_path: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The product tree branch uses a CSAF Extension.".to_string(),
        instance_path: format!("{branch_path}/product/x_extensions"),
    })
}

/// 6.3.21.4 Usage of Extension in Product Tree Branch Path
///
/// It SHALL be tested that the element `$.product_tree..branches[*].product.x_extensions` does not exist.
pub fn test_6_3_21_4_usage_of_extension_in_product_tree_branch_path(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    // early return if no product tree is present
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // #407: wasSkipped
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    product_tree.visit_all_branches(&mut |branch, branch_path| {
        if let Some(product) = branch.get_product()
            && product.get_extensions().is_some()
        {
            findings
                .get_or_insert_default()
                .push(create_branch_product_extension_info(branch_path));
        }
    });

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_4,
    test_6_3_21_4_usage_of_extension_in_product_tree_branch_path
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_4 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_4() {
        // Case 11: extension at the root level, but none in branches
        TESTS_2_1.test_6_3_21_4.expect(ExpectedResults {
            case_01: Err(vec![create_branch_product_extension_info(
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            case_11: Ok(()),
        });
    }
}
