use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};

/// Central helper function for extracting group references.
///
/// This function implements the core logic for extracting group IDs and their JSON paths
/// from an iterator of items that implement `WithGroupIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement WithGroupIds
/// * `path_prefix` - A string representing the prefix for the JSON path (e.g., "flags", "notes")
///
/// # Returns
///
/// A vector of tuples containing (group_id, json_path) for each group reference found.
fn extract_group_id_impl<'a, T: WithOptionalGroupIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path_prefix: &str,
) -> Vec<(String, String)> {
    let mut ids: Vec<(String, String)> = Vec::new();
    for (index, item) in items.enumerate() {
        if let Some(group_ids) = item.get_group_ids() {
            for (group_index, group_id) in group_ids.enumerate() {
                ids.push((
                    group_id.to_owned(),
                    format!("{path_prefix}/{index}/group_ids/{group_index}"),
                ))
            }
        }
    }
    ids
}

/// Central helper function for extracting group references.
///
/// This function implements the core logic for extracting group IDs and their JSON paths
/// from an iterator of items that implement `WithProductIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement WithProductIds
/// * `path_prefix` - A string representing the prefix for the JSON path (e.g., "flags", "notes")
///
/// # Returns
///
/// A vector of tuples containing (product_id, json_path) for each product reference found.
fn extract_product_id_impl<'a, T: WithOptionalProductIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path_prefix: &str,
) -> Vec<(String, String)> {
    let mut ids: Vec<(String, String)> = Vec::new();
    for (index, item) in items.enumerate() {
        if let Some(product_ids) = item.get_product_ids() {
            for (product_index, product_id) in product_ids.enumerate() {
                ids.push((
                    product_id.to_owned(),
                    format!("{path_prefix}/{index}/product_ids/{product_index}"),
                ))
            }
        }
    }
    ids
}

/// Extension trait for extracting group references from collections where T implements WithOptionalGroupIds.
///
/// This trait provides a generic method to extract group IDs from collections of objects
/// that implement the `WithOptionalGroupIds` trait, returning them as tuples of (group_id, json_path).
///
/// Implemented for:
/// - `Option<&Vec<T>>`
/// - `&Option<Vec<T>>`
/// - `Vec<T>`
///
/// TODO: As already discussed, we should simplify / align our return params here.
/// It does not make sense to have the same functionality return either `Option<&Vec<T>>` or
/// `&Option<Vec<T>>` in some cases. When this is done, we can remove the unused case.
pub trait ExtractGroupReferences<T: WithOptionalGroupIds> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for Option<&Vec<T>> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter().flat_map(|x| x.iter()), path_prefix)
    }
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for &Option<Vec<T>> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter().flatten(), path_prefix)
    }
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for Vec<T> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter(), path_prefix)
    }
}

/// Extension trait for extracting product references from collections where T implements WithOptionalProductIds.
///
/// This trait provides a generic method to extract product IDs from collections of objects
/// that implement the `WithOptionalProductIds` trait, returning them as tuples of (product_id, json_path).
///
/// Implemented for:
/// - `Option<&Vec<T>>`
/// - `&Option<Vec<T>>`
/// - `Vec<T>`
///
/// TODO: As already discussed, we should simplify / align our return params here.
/// It does not make sense to have the same functionality return either `Option<&Vec<T>>` or
/// `&Option<Vec<T>>` in some cases. When this is done, we can remove the unused case.
pub trait ExtractProductReferences<T: WithOptionalProductIds> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for Option<&Vec<T>> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter().flat_map(|x| x.iter()), path_prefix)
    }
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for &Option<Vec<T>> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter().flatten(), path_prefix)
    }
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for Vec<T> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter(), path_prefix)
    }
}
