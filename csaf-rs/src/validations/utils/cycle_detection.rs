use std::collections::{HashMap, HashSet};

/// Finds the first cycle reachable from `node` in a directed graph.
///
/// The outer map contains a node and its outgoing edges. Each inner-map key is
/// the destination node, and its value is the JSON path where that edge is
/// defined.
/// The returned tuple contains the cycle, with its starting node repeated at
/// the end, and the path of the edge that starts / closes the cycle. If
/// `node` is not part of a reachable cycle, `None` is returned.
pub fn find_cycle<'a>(
    relation_map: &'a HashMap<String, HashMap<String, String>>,
    node: &'a str,
) -> Option<(Vec<String>, String)> {
    find_cycle_inner(relation_map, node, &mut HashSet::new())
}

fn find_cycle_inner<'a>(
    relation_map: &'a HashMap<String, HashMap<String, String>>,
    node: &'a str,
    visited: &mut HashSet<&'a str>,
) -> Option<(Vec<String>, String)> {
    if !visited.insert(node) {
        return Some((vec![node.to_string()], String::new()));
    }

    if let Some(next_nodes) = relation_map.get(node) {
        for (next, path) in next_nodes {
            match find_cycle_inner(relation_map, next, visited) {
                None => {},
                Some((mut cycle, detected_path)) => {
                    let (Some(first), Some(last)) = (cycle.first(), cycle.last()) else {
                        continue;
                    };

                    if cycle.len() == 1 || first != last {
                        if first == node {
                            cycle.push(node.to_string());
                            cycle.reverse();
                            return Some((cycle, path.clone()));
                        }
                        cycle.push(node.to_string());
                    }
                    return Some((cycle, detected_path));
                },
            }
        }
    }

    visited.remove(node);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cycle() {
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
        relation_map.insert("E".to_string(), HashMap::from([("F".to_string(), "6".to_string())]));
        relation_map.insert("F".to_string(), HashMap::from([("G".to_string(), "7".to_string())]));

        let result = find_cycle(&relation_map, "A");
        assert_eq!(
            result,
            Some((vec!["B".into(), "C".into(), "D".into(), "B".into()], "1".into()))
        );

        let result = find_cycle(&relation_map, "C");
        assert_eq!(
            result,
            Some((vec!["C".into(), "D".into(), "B".into(), "C".into()], "3".into()))
        );

        assert_eq!(find_cycle(&relation_map, "E"), None);

        assert_eq!(find_cycle(&relation_map, "Z"), None);
    }
}
