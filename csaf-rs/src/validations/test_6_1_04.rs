use crate::csaf_traits::{CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn generate_undefined_product_group_id_error(undefined_product_group_id: &str, instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of product_group_id: {undefined_product_group_id}"),
        instance_path: instance_path.to_owned(),
    })
}

/// 6.1.4 Missing Definition of Product Group ID
//
/// For each element of type `$['$defs'].product_group_id_t` which is not inside a Product Group
/// (`$.product_tree.product_groups[*]`) and therefore reference an element within the `product_tree`
/// it SHALL be tested that the Product Group element with the matching `group_id` exists.
///
/// The same applies for all items of elements of type `$['$defs'].product_groups_t`.
///
/// This test is functionally the same between CSAF 2.0 and CSAF 2.1.
/// It collects
/// a) the set of product group IDs defined in the product tree and
/// b) the set of product group IDs with the paths at which they are found from the entire CSAF document
/// and compares the two sets.
///
/// Due to the syntax changes between the two versions, the way these sets are derived differs, though.
/// This is abstracted through the [`ProductTreeTrait::get_all_product_group_ids`] and [`CsafTrait::get_all_group_references`]
/// functions, which allows the same implementation to work for both CSAF versions.
pub fn test_6_1_04_missing_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if there is a product tree
    let Some(tree) = doc.get_product_tree() else {
        return Ok(()); // #409 wasSkipped
    };

    // get all defined product group IDs from the product tree
    let defined_product_group_ids = tree.get_all_product_group_ids();

    let mut errors: Option<Vec<TestFinding>> = None;
    // iterate over all product group references
    let product_group_references = doc.get_all_group_references();
    for (referenced_product_group_id, referenced_product_group_path) in &product_group_references {
        // if a reference is not defined, generate an error
        if !defined_product_group_ids.contains(referenced_product_group_id) {
            errors
                .get_or_insert_default()
                .push(generate_undefined_product_group_id_error(
                    referenced_product_group_id,
                    referenced_product_group_path,
                ));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_4,
    test_6_1_04_missing_definition_of_product_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_4 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_4 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_04() {
        let case_threats = Err(vec![generate_undefined_product_group_id_error(
            "CSAFGID-1020301",
            "/vulnerabilities/0/threats/0/group_ids/0",
        )]);
        let case_flags = Err(vec![
            generate_undefined_product_group_id_error("CSAFGID-1020300", "/vulnerabilities/0/flags/0/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020301", "/vulnerabilities/1/flags/0/group_ids/0"),
        ]);
        let case_remediations = Err(vec![
            generate_undefined_product_group_id_error(
                "CSAFGID-1020300",
                "/vulnerabilities/0/remediations/0/group_ids/0",
            ),
            generate_undefined_product_group_id_error(
                "CSAFGID-1020301",
                "/vulnerabilities/1/remediations/0/group_ids/0",
            ),
        ]);
        let case_vulnerabilities = Err(vec![
            generate_undefined_product_group_id_error("CSAFGID-1020304", "/document/notes/0/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020302", "/document/involvement/actions/0/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020301", "/document/involvement/actions/1/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020301", "/document/involvement/actions/2/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020304", "/document/involvement/actions/3/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020303", "/vulnerabilities/0/flags/0/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020302", "/vulnerabilities/0/notes/0/group_ids/0"),
            generate_undefined_product_group_id_error(
                "CSAFGID-1020302",
                "/vulnerabilities/0/remediations/0/group_ids/0",
            ),
            generate_undefined_product_group_id_error("CSAFGID-1020301", "/vulnerabilities/0/threats/0/group_ids/0"),
            generate_undefined_product_group_id_error(
                "CSAFGID-1020301",
                "/vulnerabilities/0/first_known_exploitation_dates/0/group_ids/0",
            ),
        ]);
        let case_vuln_id = Err(vec![
            generate_undefined_product_group_id_error("CSAFGID-1020314", "/vulnerabilities/0/ids/0/group_ids/0"),
            generate_undefined_product_group_id_error("CSAFGID-1020313", "/vulnerabilities/0/ids/1/group_ids/2"),
            generate_undefined_product_group_id_error("CSAFPID-9080700", "/vulnerabilities/0/ids/2/group_ids/0"),
        ]);

        TESTS_2_0.test_6_1_4.expect(ExpectedResults_2_0 {
            case_01: case_threats.clone(),
            case_02: case_flags,
            case_s01: case_remediations,
            case_11: Ok(()),
            case_12: Ok(()),
        });
        TESTS_2_1.test_6_1_4.expect(ExpectedResults_2_1 {
            case_01: case_threats,
            case_02: case_vulnerabilities,
            case_03: case_vuln_id,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
