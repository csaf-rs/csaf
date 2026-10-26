use std::collections::{HashMap, HashSet};

/// A directed graph whose edges retain the JSON path of their definitions.
///
/// The outer map contains a node and its outgoing edges. Each inner-map key is
/// the destination node, and its value is the JSON path where that edge is
/// defined.
pub struct RelationMap {
    relations: HashMap<String, HashMap<String, String>>,
}

impl RelationMap {
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, source: &str, target: &str, json_path: String) {
        self.relations
            .entry(source.to_owned())
            .or_default()
            .insert(target.to_owned(), json_path);
    }

    pub fn referenced_nodes(&self) -> HashSet<&String> {
        self.relations
            .values()
            .flat_map(|relations| relations.keys())
            .collect::<HashSet<_>>()
    }
}

/// Finds the first cycle reachable from `node` in a direct graph, provided as a [`RelationMap`].
///
/// The returned tuple contains the first cycle, with its starting node repeated at
/// the end, and the path of the edge that starts / closes the cycle. If
/// `node` is not part of a reachable cycle, `None` is returned.
pub fn find_cycle(relation_map: &RelationMap, node: &str) -> Option<(Vec<String>, String)> {
    find_cycle_inner(&relation_map.relations, node, &mut HashSet::new())
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
    fn test_referenced_nodes() {
        let mut relation_map = RelationMap::new();
        relation_map.add_relation("A", "B", "0".to_string());
        relation_map.add_relation("A", "C", "1".to_string());
        relation_map.add_relation("D", "B", "2".to_string());
        relation_map.add_relation("E", "E", "3".to_string());

        let referenced_nodes = relation_map.referenced_nodes();
        let expected_nodes = ["B".to_string(), "C".to_string(), "E".to_string()];

        assert_eq!(referenced_nodes.len(), 3);
        assert_eq!(referenced_nodes, expected_nodes.iter().collect());
    }

    #[test]
    fn test_find_cycle() {
        let mut relation_map = RelationMap::new();
        relation_map.add_relation("A", "B", "0".to_string());
        relation_map.add_relation("B", "C", "1".to_string());
        relation_map.add_relation("B", "E", "2".to_string());
        relation_map.add_relation("C", "D", "3".to_string());
        relation_map.add_relation("C", "F", "4".to_string());
        relation_map.add_relation("D", "B", "5".to_string());
        relation_map.add_relation("E", "F", "6".to_string());
        relation_map.add_relation("F", "G", "7".to_string());

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

    #[test]
    fn test_find_self_cycle() {
        let mut relation_map = RelationMap::new();
        relation_map.add_relation("A", "A", "0".to_string());

        let result = find_cycle(&relation_map, "A");
        assert_eq!(result, Some((vec!["A".into(), "A".into()], "0".into())));
    }
}
