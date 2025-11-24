use crate::csaf::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait, RelationshipTrait};
use crate::csaf::validation::ValidationError;
use std::collections::{HashMap, HashSet};

/// Find the first cycle in the given `relation_map`, if any.
///
/// # Returns
/// - `Vec` of the product IDs forming the detected cycle
/// - Index of the CSAF relation containing the product ID where the cycle was first detected
pub fn find_cycle<'a>(
    relation_map: &'a HashMap<String, HashMap<String, usize>>,
    product_id: &'a str,
    visited: &mut HashSet<&'a str>,
) -> Option<(Vec<String>, usize)> {
    if !visited.insert(product_id) {
        return Some((vec![product_id.to_string()], 0));
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
                            return Some((cycle, *r_i));
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
    let mut errors: Option<Vec<ValidationError>> = None;
    if let Some(tree) = doc.get_product_tree().as_ref() {
        let mut relation_map = HashMap::<String, HashMap<String, usize>>::new();

        for (i_r, r) in tree.get_relationships().iter().enumerate() {
            let rel_prod_id = r.get_full_product_name().get_product_id();
            if r.get_product_reference() == rel_prod_id {
                errors.get_or_insert_with(Vec::new).push(ValidationError {
                    message: "Relationship references itself via product_reference".to_string(),
                    instance_path: format!("/product_tree/relationships/{}/product_reference", i_r),
                });
            } else if r.get_relates_to_product_reference() == rel_prod_id {
                errors.get_or_insert_with(Vec::new).push(ValidationError {
                    message: "Relationship references itself via relates_to_product_reference".to_string(),
                    instance_path: format!("/product_tree/relationships/{}/relates_to_product_reference", i_r),
                });
            } else {
                match relation_map.get_mut(r.get_product_reference()) {
                    Some(v) => {
                        v.insert(r.get_relates_to_product_reference().to_owned(), i_r);
                    },
                    None => {
                        relation_map.insert(
                            r.get_product_reference().to_owned(),
                            HashMap::from([(r.get_relates_to_product_reference().to_owned(), i_r)]),
                        );
                    },
                }
            }
        }

        // Perform cycle check
        let mut visited = HashSet::new();
        for product_id in relation_map.keys() {
            if let Some((cycle, relation_index)) = find_cycle(&relation_map, product_id, &mut visited) {
                errors.get_or_insert_with(Vec::new).push(ValidationError {
                    message: format!("Found product relationship cycle: {}", cycle.join(" -> ")),
                    instance_path: format!("/product_tree/relationships/{}", relation_index),
                });
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_03::test_6_1_03_circular_definition_of_product_id;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_test_6_1_03() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "Relationship references itself via relates_to_product_reference".to_string(),
                instance_path: "/product_tree/relationships/0/relates_to_product_reference".to_string(),
            }],
        )]);
        run_csaf20_tests("03", test_6_1_03_circular_definition_of_product_id, errors.clone());
        run_csaf21_tests("03", test_6_1_03_circular_definition_of_product_id, errors);
    }

    #[test]
    fn test_find_cycle() {
        // Create a relation map with a non-trivial cycle: B -> C -> D -> B
        let mut relation_map = HashMap::new();

        relation_map.insert("A".to_string(), HashMap::from([("B".to_string(), 0)]));
        relation_map.insert(
            "B".to_string(),
            HashMap::from([("C".to_string(), 1), ("E".to_string(), 2)]),
        );
        relation_map.insert(
            "C".to_string(),
            HashMap::from([("D".to_string(), 3), ("F".to_string(), 4)]),
        );
        relation_map.insert("D".to_string(), HashMap::from([("B".to_string(), 5)]));

        // Also add some nodes that aren't part of the cycle
        relation_map.insert("E".to_string(), HashMap::from([("F".to_string(), 6)]));
        relation_map.insert("F".to_string(), HashMap::from([("G".to_string(), 7)]));

        // Test cycle detection starting from the first node
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "A", &mut visited);
        assert!(result.is_some());
        let (cycle, relation_index) = result.unwrap();
        assert_eq!(cycle, vec!("B", "C", "D", "B"));
        assert_eq!(relation_index, 1);

        // Test starting from a node that's part of the cycle
        let mut visited = HashSet::new();
        let result = super::find_cycle(&relation_map, "C", &mut visited);
        assert!(result.is_some());
        let (cycle, relation_index) = result.unwrap();
        assert_eq!(cycle, vec!("C", "D", "B", "C"));
        assert_eq!(relation_index, 3);

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
