use std::collections::{BTreeMap, BTreeSet};

use super::NodeIndex;

/// Maps nodes in the CSAF document to JSON paths
///
/// It also deals with node aliasing and filters out trivial nodes
#[derive(Default)]
pub struct JsonPathMapper {
    aliases: Vec<BTreeSet<NodeIndex>>,
    paths: BTreeMap<NodeIndex, Vec<JsonPath>>,
    trivial: BTreeSet<NodeIndex>,
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
    pub fn insert(&mut self, orig_index: NodeIndex, path: JsonPath) {
        let index = self.resolve(orig_index);
        self.paths.entry(index).or_default().push(path);
    }

    pub fn get(&self, orig_index: NodeIndex) -> &[JsonPath] {
        let index = self.resolve(orig_index);
        self.paths.get(&index).unwrap()
    }

    pub fn alias(&mut self, src: NodeIndex, dest: NodeIndex) {
        let src_index = self.aliases.iter().position(|set| set.contains(&src));
        let dest_index = self.aliases.iter().position(|set| set.contains(&dest));

        let (a, b) = match (src_index, dest_index) {
            // create new set
            (None, None) => {
                let mut set = BTreeSet::new();
                set.insert(src);
                set.insert(dest);
                self.aliases.push(set);

                (src, dest)
            },

            // add new ID into existing set
            (None, Some(dest)) => {
                let set = &mut self.aliases[dest];
                let new_dest = *set.first().unwrap();
                set.insert(src);

                (src, new_dest)
            },
            (Some(src), None) => {
                let set = &mut self.aliases[src];
                let new_src = *set.first().unwrap();
                set.insert(dest);

                (new_src, dest)
            },

            // merge two existing sets
            (Some(src), Some(dest)) => {
                let (min, max) = if src > dest { (dest, src) } else { (src, dest) };
                let set_a = self.aliases.swap_remove(max);
                let set_b = &mut self.aliases[min];
                let a = *set_a.first().unwrap();
                let b = *set_b.first().unwrap();
                set_b.extend(set_a);

                (a, b)
            },
        };

        let (min, max) = if a > b { (b, a) } else { (a, b) };

        if let Some(paths) = self.paths.remove(&max) {
            self.paths.entry(min).or_default().extend(paths);
        }
    }

    fn aliases_of(&self, index: NodeIndex) -> Option<&BTreeSet<NodeIndex>> {
        self.aliases.iter().find(|set| set.contains(&index))
    }

    /// "Resolves" a node index into the alias with the smallest value
    ///
    /// Or returns the input if the node has no aliases
    fn resolve(&self, index: NodeIndex) -> NodeIndex {
        for set in &self.aliases {
            if set.contains(&index) {
                return *set.first().unwrap();
            }
        }

        index
    }

    pub fn trivial(&mut self, index: NodeIndex) {
        self.trivial.insert(index);
    }

    pub fn non_trivial_paths(self) -> BTreeMap<JsonPath, BTreeSet<NodeIndex>> {
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
            for index in set {
                if is_first {
                    continue;
                }
                is_first = false;

                assert!(self.paths[index].is_empty());
            }
        }

        let mut skip = BTreeSet::new();
        for trivial in &self.trivial {
            if let Some(others) = self.aliases_of(*trivial) {
                skip.extend(others);
            } else {
                skip.insert(trivial);
            }
        }

        let mut paths_to_indices = BTreeMap::new();
        for (index, paths) in &self.paths {
            if skip.contains(index) {
                continue;
            }

            for path in paths {
                let indices = self.aliases_of(*index).cloned().unwrap_or_else(|| {
                    let mut set = BTreeSet::new();
                    set.insert(*index);
                    set
                });
                paths_to_indices.insert(path.clone(), indices);
            }
        }
        paths_to_indices
    }
}
