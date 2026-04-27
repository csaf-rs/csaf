use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::{BTreeSet, HashSet};

/// Resolves a set of product group IDs to the individual product IDs they contain.
///
/// Looks up each given group ID in the document's product tree and collects
/// all product IDs belonging to groups.
///
/// Returns `Some` with a deduplicated, sorted set of product IDs, or `None` if the document
/// has no product tree or if there were no product IDs resolved for the given group IDs.
/// The latter can be caused by the group IDs not existing in the product tree and / or the
/// group IDs not being associated with any product IDs.
///
/// # Arguments
///
/// * `doc` - A CSAF document implementing [`CsafTrait`], used to access the product tree
/// * `product_groups` - An iterator of product group ID strings to resolve
pub fn resolve_product_groups<'a, I>(doc: &impl CsafTrait, product_groups: I) -> Option<BTreeSet<String>>
where
    I: IntoIterator<Item = &'a String>,
{
    // early return if there isn't a product tree
    let product_tree = doc.get_product_tree()?;

    // collect requested group IDs into a hashset for O(1) lookup
    let product_groups: HashSet<&String> = product_groups.into_iter().collect();

    let product_ids: BTreeSet<String> = product_tree
        .get_product_groups()
        .iter()
        // filter out all non-queried group ids
        .filter(|x| product_groups.contains(x.get_group_id()))
        // resolve the group into its product ids
        .flat_map(|x| x.get_product_ids())
        .map(|p| p.to_string())
        .collect();

    if product_ids.is_empty() {
        None
    } else {
        Some(product_ids)
    }
}
