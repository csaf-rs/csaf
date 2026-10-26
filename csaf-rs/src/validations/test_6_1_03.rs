use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentTrait, ProductPathTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::cycle_detection::{RelationMap, find_cycle};

fn generate_self_reference_product_error(version: CsafVersion, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: if version == CsafVersion::X21 {
            "Product path references itself via 'beginning product reference'".to_string()
        } else {
            "Relationship references itself via 'product reference'".to_string()
        },
        instance_path: path.to_string(),
    })
}

fn generate_self_reference_relates_to_error(version: CsafVersion, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: if version == CsafVersion::X21 {
            "Product path references itself via 'next product reference'".to_string()
        } else {
            "Relationship references itself via 'relates to product reference'".to_string()
        },
        instance_path: path.to_string(),
    })
}

fn generate_cycle_error(version: CsafVersion, cycle: &[String], path: String) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: if version == CsafVersion::X21 {
            format!("Found cycle in product path definitions: {}", cycle.join(" -> "))
        } else {
            format!("Found cycle in relationship definitions: {}", cycle.join(" -> "))
        },
        instance_path: path,
    })
}

/// 6.1.3 Circular Definition of Product ID
///
/// For each new defined Product ID (type `$['$defs'].product_id_t`) in items of product paths (`$.product_tree.product_paths`) it
/// SHALL be tested that the `product_id` does not end up in a circle.
///
/// As this can be quite complex a program for large CSAF documents, a program could check first whether
/// a Product ID defined in a product path item is used as `beginning_product_reference` or `next_product_reference`.
/// Only for those which fulfill this condition it is necessary to run the full check following the references.
pub fn test_6_1_03_circular_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let version = doc.get_document().get_csaf_version();
    let mut errors: Option<Vec<TestFinding>> = None;
    if let Some(tree) = doc.get_product_tree() {
        let mut relation_map = RelationMap::new();
        for (pp_i, pp) in tree.get_product_paths().iter().enumerate() {
            let rel_prod_id = pp.get_full_product_name().get_product_id();
            if pp.get_beginning_product_reference() == rel_prod_id {
                errors
                    .get_or_insert_default()
                    .push(generate_self_reference_product_error(
                        version,
                        &pp.get_json_path_for_product_path_beginning_product_reference(pp_i),
                    ));
            } else if let Some(sp_i) = pp
                .get_subpath_product_references()
                .iter()
                .position(|next| *next == rel_prod_id)
            {
                errors
                    .get_or_insert_default()
                    .push(generate_self_reference_relates_to_error(
                        version,
                        &pp.get_json_path_for_product_path_subpath_product_reference(pp_i, sp_i),
                    ));
            } else {
                let path = pp.get_json_path_for_product_path(pp_i);
                relation_map.add_relation(rel_prod_id, pp.get_beginning_product_reference(), path.clone());
                pp.get_subpath_product_references().iter().for_each(|next| {
                    relation_map.add_relation(rel_prod_id, next, path.clone());
                });
            }
        }

        // Perform cycle check
        for product_id in relation_map.referenced_nodes() {
            if let Some((cycle, relation_index)) = find_cycle(&relation_map, product_id) {
                errors
                    .get_or_insert_default()
                    .push(generate_cycle_error(version, &cycle, relation_index));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_3, test_6_1_03_circular_definition_of_product_id);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_3 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_3 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_03() {
        let csaf_20_case_01_self_ref_via_relates_to_product_ref = Err(vec![generate_self_reference_relates_to_error(
            CsafVersion::X20,
            "/product_tree/relationships/0/relates_to_product_reference",
        )]);
        let csaf_20_case_s01_self_ref_via_product_ref = Err(vec![generate_self_reference_product_error(
            CsafVersion::X20,
            "/product_tree/relationships/0/product_reference",
        )]);
        let csaf_20_case_s02_cycle = Err(vec![
            generate_cycle_error(
                CsafVersion::X20,
                &[
                    "CSAFPID-9080701".to_string(),
                    "CSAFPID-9080702".to_string(),
                    "CSAFPID-9080701".to_string(),
                ],
                "/product_tree/relationships/0".to_string(),
            ),
            generate_cycle_error(
                CsafVersion::X20,
                &[
                    "CSAFPID-9080702".to_string(),
                    "CSAFPID-9080701".to_string(),
                    "CSAFPID-9080702".to_string(),
                ],
                "/product_tree/relationships/1".to_string(),
            ),
        ]);

        TESTS_2_0.test_6_1_3.expect(ExpectedResults_2_0 {
            case_01: csaf_20_case_01_self_ref_via_relates_to_product_ref,
            case_s01: csaf_20_case_s01_self_ref_via_product_ref,
            case_s02: csaf_20_case_s02_cycle,
        });

        let csaf_21_case_01_self_ref_via_product_path_next_ref = Err(vec![generate_self_reference_relates_to_error(
            CsafVersion::X21,
            "/product_tree/product_paths/0/subpaths/0/next_product_reference",
        )]);

        let csaf_21_case_02_cycle = Err(vec![
            generate_cycle_error(
                CsafVersion::X21,
                &[
                    "CSAFPID-9080701".to_string(),
                    "CSAFPID-9080702".to_string(),
                    "CSAFPID-9080701".to_string(),
                ],
                "/product_tree/product_paths/0".to_string(),
            ),
            generate_cycle_error(
                CsafVersion::X21,
                &[
                    "CSAFPID-9080702".to_string(),
                    "CSAFPID-9080701".to_string(),
                    "CSAFPID-9080702".to_string(),
                ],
                "/product_tree/product_paths/1".to_string(),
            ),
        ]);

        let csaf_21_case_s01_self_ref_via_product_path_beginning_ref =
            Err(vec![generate_self_reference_product_error(
                CsafVersion::X21,
                "/product_tree/product_paths/0/beginning_product_reference",
            )]);

        // Case 11: simple product path, no cycle
        // Case 12: multiple paths, no cycle
        // Case 13: multiple paths with different subpath categories, no cycle

        TESTS_2_1.test_6_1_3.expect(ExpectedResults_2_1 {
            case_01: csaf_21_case_01_self_ref_via_product_path_next_ref,
            case_02: csaf_21_case_02_cycle,
            case_s01: csaf_21_case_s01_self_ref_via_product_path_beginning_ref,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
