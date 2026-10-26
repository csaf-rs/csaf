use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::{BTreeSet, HashSet};

pub struct ResolvedProductGroupIds {
    pub resolved_product_ids: Option<BTreeSet<String>>,
    pub unresolvable_group_ids: Option<BTreeSet<String>>,
}

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
pub fn resolve_product_groups(
    doc: &impl CsafTrait,
    product_groups: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<BTreeSet<String>> {
    resolve_product_groups_with_unresolvable(doc, product_groups).resolved_product_ids
}

/// Resolves product group IDs while retaining group IDs that cannot be resolved.
///
/// TODO: Refactor to be more ergonomic and align this with the resolve_entity_groups function (#1384)
pub fn resolve_product_groups_with_unresolvable(
    doc: &impl CsafTrait,
    product_groups: impl IntoIterator<Item = impl AsRef<str>>,
) -> ResolvedProductGroupIds {
    let product_group_ids: HashSet<String> = product_groups.into_iter().map(|s| s.as_ref().to_owned()).collect();
    let Some(product_tree) = doc.get_product_tree() else {
        return ResolvedProductGroupIds {
            resolved_product_ids: None,
            unresolvable_group_ids: Some(product_group_ids.into_iter().collect()),
        };
    };

    let mut resolved_product_ids: Option<BTreeSet<String>> = None;
    let mut matched_group_ids = HashSet::new();
    for product_group in product_tree.get_product_groups() {
        if product_group_ids.contains(product_group.get_group_id()) {
            matched_group_ids.insert(product_group.get_group_id().to_owned());
            resolved_product_ids
                .get_or_insert_default()
                .extend(product_group.get_product_ids().map(str::to_owned));
        }
    }
    let unresolvable_group_ids: BTreeSet<String> = product_group_ids.difference(&matched_group_ids).cloned().collect();

    ResolvedProductGroupIds {
        resolved_product_ids,
        unresolvable_group_ids: (!unresolvable_group_ids.is_empty()).then_some(unresolvable_group_ids),
    }
}
