use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_full_product_name_extension_info(full_product_name_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The full product name uses a CSAF Extension.".to_string(),
        instance_path: format!("/product_tree/full_product_names/{full_product_name_index}/x_extensions"),
    })
}

/// 6.3.21.5 Usage of Extension in Product Tree Full Product Names Path
///
/// It SHALL be tested that the element `$.product_tree.full_product_names[*].x_extensions` does not exist.
pub fn test_6_3_21_5_usage_of_extension_in_product_tree_full_product_names_path(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    // early return if no product tree is present
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // #407: wasSkipped
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    for (full_product_name_index, full_product_name) in product_tree.get_full_product_names().iter().enumerate() {
        if full_product_name.get_extensions().is_some() {
            findings
                .get_or_insert_default()
                .push(create_full_product_name_extension_info(full_product_name_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_5,
    test_6_3_21_5_usage_of_extension_in_product_tree_full_product_names_path
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_5 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_5() {
        // Case 11: extension at the root level, but none in full product names
        TESTS_2_1.test_6_3_21_5.expect(ExpectedResults {
            case_01: Err(vec![create_full_product_name_extension_info(0)]),
            case_11: Ok(()),
        });
    }
}
