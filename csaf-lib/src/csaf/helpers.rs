use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::{BTreeSet, HashMap};

pub fn find_duplicates<T: std::hash::Hash + Eq + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut occurrences = HashMap::new();
    let mut duplicates = Vec::new();

    for item in vec.iter() {
        let count = occurrences.entry(item.clone()).or_insert(0);
        *count += 1;
    }

    for (item, count) in occurrences {
        if count > 1 {
            duplicates.push(item);
        }
    }

    duplicates
}

pub fn resolve_product_groups(doc: &impl CsafTrait, product_groups: Vec<&String>) -> Option<BTreeSet<String>> {
    doc.get_product_tree().map(|product_tree| {
        product_tree
            .get_product_groups()
            .iter()
            .filter(|x| product_groups.iter().any(|g| *g == x.get_group_id()))
            .map(|x| x.get_product_ids().iter().map(|p| p.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect()
    })
}