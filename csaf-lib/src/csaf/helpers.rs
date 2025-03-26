use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::BTreeSet;

pub fn resolve_product_groups<'a, I>(doc: &impl CsafTrait, product_groups: I) -> Option<BTreeSet<String>>
where
    I: IntoIterator<Item = &'a String>
{
    let product_groups: Vec<&String> = product_groups.into_iter().collect();

    doc.get_product_tree().as_ref().map(|product_tree| {
        product_tree
            .get_product_groups()
            .iter()
            .filter(|x| product_groups.iter().any(|g| *g == x.get_group_id()))
            .map(|x| x.get_product_ids().map(|p| p.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect()
    })
}