use crate::csaf_traits::{CsafTrait, ProductPathTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
use std::collections::BTreeSet;

fn create_nested_product_path_info(instance_path: String) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Product path references a product defined by another product path.".to_string(),
        instance_path,
    })
}

/// 6.3.22 Nested Product Path
///
/// For each item in `$.product_tree.product_paths[*]` it SHALL be tested that all referenced
/// Product IDs are not defined under a `full_product_name_t` element within items of `$.product_tree.product_paths`.
pub fn test_6_3_22_nested_product_path(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // if there isn't a product tree, there is nothing to check
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let product_paths = product_tree.get_product_paths();
    // get all product IDs defined by product paths
    let product_ids_defined_by_product_path: BTreeSet<&str> = product_paths
        .iter()
        .map(|product_path| product_path.get_full_product_name().get_product_id())
        .collect();

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each product path
    for (product_path_index, product_path) in product_paths.iter().enumerate() {
        // check if ifs beginning product reference product id is defined by a product path, if so, add an error
        if product_ids_defined_by_product_path.contains(product_path.get_beginning_product_reference()) {
            findings.get_or_insert_default().push(create_nested_product_path_info(
                product_path.get_json_path_for_product_path_beginning_product_reference(product_path_index),
            ));
        }

        // check if one of its subpath product reference product ids is defined by a product path, if so, add an error
        for (subpath_index, product_reference) in product_path.get_subpath_product_references().iter().enumerate() {
            if product_ids_defined_by_product_path.contains(product_reference) {
                findings.get_or_insert_default().push(create_nested_product_path_info(
                    product_path
                        .get_json_path_for_product_path_subpath_product_reference(product_path_index, subpath_index),
                ));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_3_22, test_6_3_22_nested_product_path);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_22 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_22() {
        // TODO: Improve test coverage, next_product_reference is not exercised
        // TODO: Multiple findings in different product paths
        // TODO: Multiple offending next references in same path
        // TODO: Self-Reference (violates 6.1.3)
        // TODO: Trivial pass: no product tree / no product paths
        TESTS_2_1.test_6_3_22.expect(ExpectedResults {
            case_01: Err(vec![create_nested_product_path_info(
                "/product_tree/product_paths/1/beginning_product_reference".to_string(),
            )]),
            case_11: Ok(()),
        });
    }
}
