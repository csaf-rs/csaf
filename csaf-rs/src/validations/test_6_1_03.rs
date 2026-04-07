use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentTrait, ProductPathTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

fn generate_self_reference_product_error(version: &CsafVersion, path: &str) -> ValidationError {
    ValidationError {
        message: if version == &CsafVersion::X21 {
            "Product path references itself via 'beginning product reference'".to_string()
        } else {
            "Relationship references itself via 'product reference'".to_string()
        },
        instance_path: path.to_string(),
    }
}

fn generate_self_reference_relates_to_error(version: &CsafVersion, path: &str) -> ValidationError {
    ValidationError {
        message: if version == &CsafVersion::X21 {
            "Product path references itself via 'next product reference'".to_string()
        } else {
            "Relationship references itself via 'relates to product reference'".to_string()
        },
        instance_path: path.to_string(),
    }
}

fn generate_cycle_error(version: &CsafVersion, cycle: &[String], path: String) -> ValidationError {
    ValidationError {
        message: if version == &CsafVersion::X21 {
            format!("Found cycle in product path definitions: {}", cycle.join(" -> "))
        } else {
            format!("Found cycle in relationship definitions: {}", cycle.join(" -> "))
        },
        instance_path: path,
    }
}

/// Find the first cycle in the given `relation_map`, if any.
///
/// # Returns
/// - `Vec` of the product IDs forming the detected cycle
/// - Path of the CSAF relation / product path containing the product ID where the cycle was first detected
pub fn find_cycle<'a>(
    relation_map: &'a HashMap<String, HashMap<String, String>>,
    product_id: &'a str,
    visited: &mut HashSet<&'a str>,
) -> Option<(Vec<String>, String)> {
    if !visited.insert(product_id) {
        return Some((vec![product_id.to_string()], "".to_string()));
    }
    if let Some(next_vec) = relation_map.get(product_id) {
        for (next, r_i) in next_vec {
            match find_cycle(relation_map, next, visited) {
                None => {},
                Some((mut cycle, r_i_res)) => {
                    let first = cycle.first().unwrap();
                    if cycle.len() == 1 || first != cycle.last().unwrap() {
                        if first == product_id {
                            // Reverse the cycle when it is complete
                            cycle.push(product_id.to_string());
                            cycle.reverse();
                            return Some((cycle, r_i.clone()));
                        }
                        // Back-trace the cycle to the first node
                        cycle.push(product_id.to_string());
                    }
                    return Some((cycle, r_i_res));
                },
            }
        }
    }
    visited.remove(product_id);
    None
}

pub fn test_6_1_03_circular_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let version = doc.get_document().get_csaf_version();
    let mut errors: Option<Vec<ValidationError>> = None;
    if let Some(tree) = doc.get_product_tree() {
        let mut relation_map = HashMap::<String, HashMap<String, String>>::new();
        for (pp_i, pp) in tree.get_product_paths().iter().enumerate() {
            let rel_prod_id = pp.get_full_product_name().get_product_id();
            if pp.get_beginning_product_reference() == rel_prod_id {
                errors
                    .get_or_insert_with(Vec::new)
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
                    .get_or_insert_with(Vec::new)
                    .push(generate_self_reference_relates_to_error(
                        version,
                        &pp.get_json_path_for_product_path_subpath_product_reference(pp_i, sp_i),
                    ));
            } else {
                match relation_map.get_mut(rel_prod_id) {
                    Some(v) => {
                        v.insert(
                            pp.get_beginning_product_reference().to_string(),
                            pp.get_json_path_for_product_path(pp_i),
                        );
                        pp.get_subpath_product_references().iter().for_each(|next| {
                            v.insert(next.to_string(), pp.get_json_path_for_product_path(pp_i));
                        });
                    },
                    None => {
                        let mut v = HashMap::new();
                        v.insert(
                            pp.get_beginning_product_reference().to_string(),
                            pp.get_json_path_for_product_path(pp_i),
                        );
                        pp.get_subpath_product_references().iter().for_each(|next| {
                            v.insert(next.to_string(), pp.get_json_path_for_product_path(pp_i));
                        });
                        relation_map.insert(rel_prod_id.to_string(), v);
                    },
                }
            }
        }

        // Find all products that are part of any product path (either as beginning or next reference)
        let products_in_product_path: HashSet<&String> =
            relation_map.values().flat_map(|referenced| referenced.keys()).collect();

        // Perform cycle check
        for product_id in products_in_product_path {
            let mut visited = HashSet::new();
            if let Some((cycle, relation_index)) = find_cycle(&relation_map, product_id, &mut visited) {
                errors
                    .get_or_insert_with(Vec::new)
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
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_03() {
        TESTS_2_0.test_6_1_3.expect(
            Err(vec![generate_self_reference_relates_to_error(
                &CsafVersion::X20,
                "/product_tree/relationships/0/relates_to_product_reference",
            )]),
            Err(vec![generate_self_reference_product_error(
                &CsafVersion::X20,
                "/product_tree/relationships/0/product_reference",
            )]),
            Err(vec![
                generate_cycle_error(
                    &CsafVersion::X20,
                    &[
                        "CSAFPID-9080701".to_string(),
                        "CSAFPID-9080702".to_string(),
                        "CSAFPID-9080701".to_string(),
                    ],
                    "/product_tree/relationships/0".to_string(),
                ),
                generate_cycle_error(
                    &CsafVersion::X20,
                    &[
                        "CSAFPID-9080702".to_string(),
                        "CSAFPID-9080701".to_string(),
                        "CSAFPID-9080702".to_string(),
                    ],
                    "/product_tree/relationships/1".to_string(),
                ),
            ]),
        );
        TESTS_2_1.test_6_1_3.expect(
            Err(vec![generate_self_reference_relates_to_error(
                &CsafVersion::X21,
                "/product_tree/product_paths/0/subpaths/0/next_product_reference",
            )]),
            Err(vec![generate_self_reference_product_error(
                &CsafVersion::X21,
                "/product_tree/product_paths/0/beginning_product_reference",
            )]),
            Err(vec![
                generate_cycle_error(
                    &CsafVersion::X21,
                    &[
                        "CSAFPID-9080701".to_string(),
                        "CSAFPID-9080702".to_string(),
                        "CSAFPID-9080701".to_string(),
                    ],
                    "/product_tree/product_paths/0".to_string(),
                ),
                generate_cycle_error(
                    &CsafVersion::X21,
                    &[
                        "CSAFPID-9080702".to_string(),
                        "CSAFPID-9080701".to_string(),
                        "CSAFPID-9080702".to_string(),
                    ],
                    "/product_tree/product_paths/1".to_string(),
                ),
            ]),
        );
    }

    #[test]
    fn test_find_cycle() {
        // Create a relation map with a non-trivial cycle: B -> C -> D -> B
        let mut relation_map = HashMap::new();

        relation_map.insert("A".to_string(), HashMap::from([("B".to_string(), "0".to_string())]));
        relation_map.insert(
            "B".to_string(),
            HashMap::from([("C".to_string(), "1".to_string()), ("E".to_string(), "2".to_string())]),
        );
        relation_map.insert(
            "C".to_string(),
            HashMap::from([("D".to_string(), "3".to_string()), ("F".to_string(), "4".to_string())]),
        );
        relation_map.insert("D".to_string(), HashMap::from([("B".to_string(), "5".to_string())]));

        // Also add some nodes that aren't part of the cycle
        relation_map.insert("E".to_string(), HashMap::from([("F".to_string(), "6".to_string())]));
        relation_map.insert("F".to_string(), HashMap::from([("G".to_string(), "7".to_string())]));

        // Test cycle detection starting from the first node
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "A", &mut visited);
        assert!(result.is_some());
        let (cycle, relation_index) = result.unwrap();
        assert_eq!(cycle, vec!("B", "C", "D", "B"));
        assert_eq!(relation_index, "1");

        // Test starting from a node that's part of the cycle
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "C", &mut visited);
        assert!(result.is_some());
        let (cycle, relation_index) = result.unwrap();
        assert_eq!(cycle, vec!("C", "D", "B", "C"));
        assert_eq!(relation_index, "3");

        // Test starting from a node that's not part of any cycle
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "E", &mut visited);
        assert!(result.is_none());

        // Test with empty visited Set and starting from a node not in the map
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "Z", &mut visited);
        assert!(result.is_none());
    }
}
