use std::collections::{BTreeMap, BTreeSet};

use super::NodeName;

/// Maps nodes in the CSAF document to JSON paths
///
/// It also deals with node aliasing and filters out trivial nodes
#[derive(Default)]
pub struct JsonPathMapper {
    aliases: Vec<BTreeSet<NodeName>>,
    paths: BTreeMap<NodeName, Vec<JsonPath>>,
    trivial: BTreeSet<NodeName>,
}

/// A JSON path like `/document/distribution/tlp/label` or
/// `/document/vulnerabilities[*]/remediations[*]`
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct JsonPath {
    inner: Vec<JsonPathComponent>,
}

/// A JSON path component
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JsonPathComponent {
    Element,
    Field(String),
}

impl JsonPath {
    pub fn root() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, component: JsonPathComponent) {
        self.inner.push(component)
    }
}

impl JsonPathMapper {
    pub fn insert(&mut self, orig_name: &NodeName, path: JsonPath) {
        let name = self.resolve(orig_name);
        self.paths.entry(name).or_default().push(path);
    }

    pub fn get(&self, orig_name: &NodeName) -> &[JsonPath] {
        let name = self.resolve(orig_name);
        self.paths.get(&name).unwrap()
    }

    pub fn alias(&mut self, src: NodeName, dest: NodeName) {
        let src_index = self.aliases.iter().position(|set| set.contains(&src));
        let dest_index = self.aliases.iter().position(|set| set.contains(&dest));

        let (a, b) = match (src_index, dest_index) {
            // create new set
            (None, None) => {
                let mut set = BTreeSet::new();
                set.insert(src.clone());
                set.insert(dest.clone());
                self.aliases.push(set);

                (src, dest)
            },

            // add new ID into existing set
            (None, Some(dest)) => {
                let set = &mut self.aliases[dest];
                let new_dest = set.first().unwrap().clone();
                set.insert(src.clone());

                (src, new_dest)
            },
            (Some(src), None) => {
                let set = &mut self.aliases[src];
                let new_src = set.first().unwrap().clone();
                set.insert(dest.clone());

                (new_src, dest)
            },

            // merge two existing sets
            (Some(src), Some(dest)) => {
                let (min, max) = if src > dest { (dest, src) } else { (src, dest) };
                let set_a = self.aliases.swap_remove(max);
                let set_b = &mut self.aliases[min];
                let a = set_a.first().unwrap().clone();
                let b = set_b.first().unwrap().clone();
                set_b.extend(set_a);

                (a, b)
            },
        };

        let (min, max) = if a > b { (b, a) } else { (a, b) };

        if let Some(paths) = self.paths.remove(&max) {
            self.paths.entry(min).or_default().extend(paths);
        }
    }

    fn aliases_of(&self, name: &NodeName) -> Option<&BTreeSet<NodeName>> {
        self.aliases.iter().find(|set| set.contains(name))
    }

    /// "Resolves" a node name into the alias with the smallest value
    ///
    /// Or returns the input if the node has no aliases
    fn resolve(&self, name: &NodeName) -> NodeName {
        for set in &self.aliases {
            if set.contains(name) {
                return set.first().unwrap().clone();
            }
        }

        name.clone()
    }

    pub fn trivial(&mut self, name: NodeName) {
        self.trivial.insert(name);
    }

    pub fn non_trivial_paths(self) -> BTreeMap<JsonPath, BTreeSet<NodeName>> {
        // sanity check that all alias sets were fully merged and do not overlap
        for (index_a, a) in self.aliases.iter().enumerate() {
            for (index_b, b) in self.aliases.iter().enumerate() {
                if index_a == index_b {
                    continue;
                }

                assert_eq!(0, a.intersection(b).count());
            }
        }

        // sanity check that path sets were merged into the smallest index of each alias set
        for set in &self.aliases {
            let mut is_first = true;
            for name in set {
                if is_first {
                    continue;
                }
                is_first = false;

                assert!(self.paths[name].is_empty());
            }
        }

        let mut skip = BTreeSet::new();
        for trivial in &self.trivial {
            if let Some(others) = self.aliases_of(trivial) {
                skip.extend(others);
            } else {
                skip.insert(trivial);
            }
        }

        let mut paths_to_indices = BTreeMap::new();
        for (name, paths) in &self.paths {
            if skip.contains(name) {
                continue;
            }

            for path in paths {
                let indices = self.aliases_of(name).cloned().unwrap_or_else(|| {
                    let mut set = BTreeSet::new();
                    set.insert(name.clone());
                    set
                });
                paths_to_indices.insert(path.clone(), indices);
            }
        }
        paths_to_indices
    }
}
