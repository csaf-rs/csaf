use crate::csaf_traits::{CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
fn generate_undefined_product_id_error(undefined_product_id: &str, instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of product_id: {undefined_product_id}"),
        instance_path: instance_path.to_string(),
    })
}

/// 6.1.1 Missing Definition of Product ID
///
/// For each element of type `$['$defs'].product_id_t` which is not inside a Full Product Name (type: `full_product_name_t`)
/// and therefore reference an element within the `product_tree` it SHALL be tested that the Full Product Name
/// element with the matching `product_id` exists.
///
/// The same applies for all items of elements of type `$['$defs'].products_t`.
///
/// This test is functionally the same between CSAF 2.0 and CSAF 2.1.
/// It collects
/// a) the set of product IDs defined in the product tree and
/// b) the set of product IDs with the paths at which they are found from the entire CSAF document
/// and compares the two sets.
///
/// Due to the syntax changes between the two versions, the way these sets are derived differs, though.
/// This is abstracted through the [`ProductTreeTrait::get_all_product_ids`] and [`ProductTreeTrait::get_all_product_references`]
/// functions, which allows the same implementation to work for both CSAF versions.
fn test_6_1_1_missing_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if there is a product tree
    let Some(tree) = doc.get_product_tree() else {
        return Ok(()); // TODO #409: There isn't a product tree, skip
    };
    // get all defined product ids from the product tree
    let defined_product_ids = tree.get_all_product_ids();

    let mut errors: Option<Vec<TestFinding>> = None;
    // iterate over all product references
    for (referenced_product_id, referenced_product_id_path) in &doc.get_all_product_references() {
        // if a reference is not defined, generate an error
        if !defined_product_ids.contains(referenced_product_id) {
            errors.get_or_insert_default().push(generate_undefined_product_id_error(
                referenced_product_id,
                referenced_product_id_path,
            ));
        }
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_1, test_6_1_1_missing_definition_of_product_id);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_1 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_1 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_6_1_01() {
        TESTS_2_0.test_6_1_1.expect(ExpectedResults_2_0 {
            case_01: Err(vec![
                generate_undefined_product_id_error("CSAFPID-9080700", "/product_tree/product_groups/0/product_ids/0"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/product_tree/product_groups/0/product_ids/1"),
            ]),
            case_02: Err(vec![
                generate_undefined_product_id_error("CSAFPID-9080701", "/vulnerabilities/0/flags/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/vulnerabilities/1/flags/0/product_ids/0"),
            ]),
            case_s01: Err(vec![
                generate_undefined_product_id_error(
                    "CSAFPID-9080701",
                    "/product_tree/relationships/0/product_reference",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080702",
                    "/product_tree/relationships/0/relates_to_product_reference",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/first_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/first_fixed/0",
                ),
                generate_undefined_product_id_error("CSAFPID-9080703", "/vulnerabilities/0/product_status/fixed/0"),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/known_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/known_not_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/last_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/recommended/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/under_investigation/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080704",
                    "/vulnerabilities/0/remediations/0/product_ids/0",
                ),
                generate_undefined_product_id_error("CSAFPID-9080705", "/vulnerabilities/0/scores/0/products/0"),
                generate_undefined_product_id_error("CSAFPID-9080706", "/vulnerabilities/0/threats/0/product_ids/0"),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });

        TESTS_2_1.test_6_1_1.expect(ExpectedResults_2_1 {
            case_01: Err(vec![
                generate_undefined_product_id_error("CSAFPID-9080700", "/product_tree/product_groups/0/product_ids/0"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/product_tree/product_groups/0/product_ids/1"),
            ]),
            case_02: Err(vec![
                generate_undefined_product_id_error("CSAFPID-9080701", "/document/notes/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/document/notes/0/product_ids/2"),
                generate_undefined_product_id_error("CSAFPID-9080703", "/document/notes/0/product_ids/3"),
                generate_undefined_product_id_error("CSAFPID-9080704", "/document/notes/0/product_ids/4"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/document/involvement/actions/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/document/involvement/actions/0/product_ids/2"),
                generate_undefined_product_id_error("CSAFPID-9080704", "/vulnerabilities/0/flags/0/product_ids/0"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/vulnerabilities/0/threats/0/product_ids/1"),
                generate_undefined_product_id_error(
                    "CSAFPID-9080701",
                    "/vulnerabilities/0/remediations/0/product_ids/1",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080702",
                    "/vulnerabilities/0/remediations/0/product_ids/2",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080710",
                    "/vulnerabilities/0/product_status/first_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/first_fixed/0",
                ),
                generate_undefined_product_id_error("CSAFPID-9080704", "/vulnerabilities/0/product_status/fixed/0"),
                generate_undefined_product_id_error(
                    "CSAFPID-9080701",
                    "/vulnerabilities/0/product_status/known_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080715",
                    "/vulnerabilities/0/product_status/known_not_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080702",
                    "/vulnerabilities/0/product_status/last_affected/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080704",
                    "/vulnerabilities/0/product_status/recommended/0",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080706",
                    "/vulnerabilities/0/product_status/under_investigation/0",
                ),
                generate_undefined_product_id_error("CSAFPID-9080707", "/vulnerabilities/0/product_status/unknown/0"),
                generate_undefined_product_id_error("CSAFPID-9080708", "/vulnerabilities/0/product_status/unknown/1"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/vulnerabilities/0/metrics/0/products/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/vulnerabilities/0/metrics/0/products/2"),
                generate_undefined_product_id_error("CSAFPID-9080701", "/vulnerabilities/0/notes/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/vulnerabilities/0/notes/0/product_ids/2"),
                generate_undefined_product_id_error(
                    "CSAFPID-9080701",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/product_ids/1",
                ),
                generate_undefined_product_id_error("CSAFPID-9080701", "/product_tree/product_groups/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080702", "/product_tree/product_groups/0/product_ids/2"),
                generate_undefined_product_id_error("CSAFPID-9080703", "/product_tree/product_groups/0/product_ids/3"),
                generate_undefined_product_id_error("CSAFPID-9080704", "/product_tree/product_groups/0/product_ids/4"),
                generate_undefined_product_id_error(
                    "CSAFPID-9080715",
                    "/product_tree/product_paths/0/subpaths/0/next_product_reference",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080701",
                    "/product_tree/product_paths/1/beginning_product_reference",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080702",
                    "/product_tree/product_paths/2/beginning_product_reference",
                ),
                generate_undefined_product_id_error(
                    "CSAFPID-9080715",
                    "/product_tree/product_paths/2/subpaths/0/next_product_reference",
                ),
            ]),
            case_03: Err(vec![
                generate_undefined_product_id_error("CSAFPID-9080710", "/vulnerabilities/0/ids/0/product_ids/0"),
                generate_undefined_product_id_error("CSAFPID-9080711", "/vulnerabilities/0/ids/0/product_ids/1"),
                generate_undefined_product_id_error("CSAFPID-9080712", "/vulnerabilities/0/ids/1/product_ids/0"),
                generate_undefined_product_id_error("CSAFPID-9080714", "/vulnerabilities/0/ids/1/product_ids/2"),
                generate_undefined_product_id_error("CSAFPID-9080715", "/vulnerabilities/0/ids/2/product_ids/0"),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_s11: Ok(()),
        });
    }
}
