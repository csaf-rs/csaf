use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct RelationCycle {
    pub cycle: Vec<String>,
    pub relation_path: String,
}

/// A directed graph whose edges retain the JSON path of their definitions.
///
/// The outer map contains a node and its outgoing edges. Each inner-map key is
/// the destination node, and its value is the JSON path where that edge is
/// defined.
pub struct RelationMap(BTreeMap<String, BTreeMap<String, String>>);

impl RelationMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Adds a directed edge from `source` to `target`, with the given JSON path.
    pub fn add_relation(&mut self, source: &str, target: &str, json_path: String) {
        self.0
            .entry(source.to_owned())
            .or_default()
            .insert(target.to_owned(), json_path);
    }

    /// Returns a set of all nodes that are referenced by any other node.
    pub fn referenced_nodes(&self) -> BTreeSet<&String> {
        self.0
            .values()
            .flat_map(|relations| relations.keys())
            .collect::<BTreeSet<_>>()
    }

    /// Finds the first cycle reachable from `node`.
    ///
    /// The returned tuple contains the first cycle, with its starting node repeated at
    /// the end, and the path of the edge that starts / closes the cycle. If
    /// no cycle is reachable from `node`, `None` is returned.
    pub fn find_cycle(&self, node: &str) -> Option<RelationCycle> {
        self.find_cycle_inner(node, &mut HashSet::new())
    }

    /// Recursive helper function for `find_cycle`.
    fn find_cycle_inner<'a>(&'a self, node: &'a str, visited: &mut HashSet<&'a str>) -> Option<RelationCycle> {
        if !visited.insert(node) {
            return Some(RelationCycle {
                cycle: vec![node.to_string()],
                relation_path: String::new(),
            });
        }

        if let Some(next_nodes) = self.0.get(node) {
            for (next, path) in next_nodes {
                match self.find_cycle_inner(next, visited) {
                    None => {},
                    Some(RelationCycle {
                        mut cycle,
                        relation_path: detected_path,
                    }) => {
                        let (Some(first), Some(last)) = (cycle.first(), cycle.last()) else {
                            continue;
                        };

                        if cycle.len() == 1 || first != last {
                            if first == node {
                                cycle.push(node.to_string());
                                cycle.reverse();
                                return Some(RelationCycle {
                                    cycle,
                                    relation_path: path.clone(),
                                });
                            }
                            cycle.push(node.to_string());
                        }
                        return Some(RelationCycle {
                            cycle,
                            relation_path: detected_path,
                        });
                    },
                }
            }
        }

        visited.remove(node);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Graph:
    ///
    ///     A ──▶ B ◀── D
    ///     │
    ///     └──▶ C
    ///
    ///     E ──↺
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


    /// Graph:
    ///
    ///
    ///           ┌────────────────┐
    ///           ▼                │
    ///     A ──▶ B ──▶ C ──▶ D ───┘
    ///           │     │
    ///           ▼     └──▶ F ──▶ G
    ///           E ───────▶ F
    ///
    ///
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

        let result = relation_map.find_cycle("A");
        assert_eq!(
            result,
            Some(RelationCycle {
                cycle: vec!["B".into(), "C".into(), "D".into(), "B".into()],
                relation_path: "1".into(),
            })
        );

        let result = relation_map.find_cycle("C");
        assert_eq!(
            result,
            Some(RelationCycle {
                cycle: vec!["C".into(), "D".into(), "B".into(), "C".into()],
                relation_path: "3".into(),
            })
        );

        assert_eq!(relation_map.find_cycle("E"), None);

        assert_eq!(relation_map.find_cycle("Z"), None);
    }



    /// Graph:
    ///
    ///     A ──↺
    #[test]
    fn test_find_self_cycle() {
        let mut relation_map = RelationMap::new();
        relation_map.add_relation("A", "A", "0".to_string());

        let result = relation_map.find_cycle("A");
        assert_eq!(
            result,
            Some(RelationCycle {
                cycle: vec!["A".into(), "A".into()],
                relation_path: "0".into(),
            })
        );
    }
}
