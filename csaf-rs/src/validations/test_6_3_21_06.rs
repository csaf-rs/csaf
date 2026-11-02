use crate::csaf_traits::{CsafTrait, ProductPathTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_product_path_extension_info(product_path_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The product path uses a CSAF Extension.".to_string(),
        instance_path: format!("/product_tree/product_paths/{product_path_index}/full_product_name/x_extensions"),
    })
}

/// 6.3.21.6 Usage of Extension in Product Tree Product Paths Path
///
/// It SHALL be tested that the element
/// `$.product_tree.product_paths[*].full_product_name.x_extensions` does not exist.
pub fn test_6_3_21_6_usage_of_extension_in_product_tree_product_paths_path(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    // early return if no product tree is present
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // #407: wasSkipped
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    for (product_path_index, product_path) in product_tree.get_product_paths().iter().enumerate() {
        if product_path.get_full_product_name().get_extensions().is_some() {
            findings
                .get_or_insert_default()
                .push(create_product_path_extension_info(product_path_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_6,
    test_6_3_21_6_usage_of_extension_in_product_tree_product_paths_path
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_6 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_6() {
        // Case 11: extension at the root level, but none in product paths
        TESTS_2_1.test_6_3_21_6.expect(ExpectedResults {
            case_01: Err(vec![create_product_path_extension_info(0)]),
            case_11: Ok(()),
        });
    }
}
