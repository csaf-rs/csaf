use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};

/// Central helper function for extracting group references.
///
/// This function implements the core logic for extracting group IDs and their JSON paths
/// from an iterator of items that implement `WithOptionalGroupIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement `WithOptionalGroupIds`
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
/// from an iterator of items that implement `WithOptionalProductIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement `WithOptionalProductIds`
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

/// Extension trait for extracting group references from iterators where T implements WithOptionalGroupIds.
///
/// This trait provides a generic method to extract group IDs from iterators over objects
/// that implement the `WithOptionalGroupIds` trait, returning them as tuples of (group_id, json_path).
///
pub trait ExtractGroupReferences<T: WithOptionalGroupIds> {
    fn extract_group_references(self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<'a, T: WithOptionalGroupIds + 'a, I: Iterator<Item = &'a T>> ExtractGroupReferences<T> for I {
    fn extract_group_references(self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self, path_prefix)
    }
}

/// Extension trait for extracting product references from iterators where T implements WithOptionalProductIds.
///
/// This trait provides a generic method to extract product IDs from iterators over objects
/// that implement the `WithOptionalProductIds` trait, returning them as tuples of (product_id, json_path).
///
pub trait ExtractProductReferences<T: WithOptionalProductIds> {
    fn extract_product_references(self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<'a, T: WithOptionalProductIds + 'a, I: Iterator<Item = &'a T>> ExtractProductReferences<T> for I {
    fn extract_product_references(self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self, path_prefix)
    }
}

/// Generates individual group/product reference extraction methods and aggregated
/// `get_all_group_references` / `get_all_product_references` methods from a single
/// field declaration.
///
/// # Parameters
///
/// - `vec`: List of `(group_method, product_method, getter, "json_path")` tuples for
///   `&Vec<T>` getters.
/// - `optional_vec`: List of tuples for `Option<&Vec<T>>` getters.
/// - `iter`: List of tuples for `Iterator` getters.
/// - `optional_iter`: List of tuples for getters returning `Option<Iterator>`.
/// - `custom_group_extraction` (optional): List of already-implemented method names that return
///   group-only references with special handling. These are included in
///   `get_all_group_references` but **not** auto-generated.
/// - `custom_product_extraction` (optional): List of already-implemented method names that return
///   product-only references with special handling (e.g., metrics, product_status).
///   These are included in `get_all_product_references` but **not** auto-generated.
///
/// # Example
///
/// ```ignore
/// define_reference_accessors! {
///     vec: [
///         (get_flags_group_references, get_flags_product_references,
///          get_flags, "flags"),
///     ],
///     optional_vec: [
///         (get_threats_group_references, get_threats_product_references,
///          get_threats, "threats"),
///     ],
///     iter: [],
///     optional_iter: [],
///     custom_group_extraction: [],
///     custom_product_extraction: [
///         get_metrics_product_references,
///     ],
/// }
/// ```
///
/// This generates:
/// - `get_flags_group_references()`, `get_flags_product_references()`
/// - `get_threats_group_references()`, `get_threats_product_references()`
/// - `get_all_group_references()` aggregating flags + threats group refs
/// - `get_all_product_references()` aggregating flags + threats product refs + metrics
macro_rules! define_reference_accessors {
    (
        $(
            vec: [
                $(
                    ($vec_group_method:ident, $vec_product_method:ident, $getter:ident, $path:literal)
                ),* $(,)?
            ],
        )?
        $(
            optional_vec: [
                $(
                    ($optional_vec_group_method:ident, $optional_vec_product_method:ident, $optional_vec_getter:ident, $optional_vec_path:literal)
                ),* $(,)?
            ],
        )?
        $(
            iter: [
                $(
                    ($iter_group_method:ident, $iter_product_method:ident, $iter_getter:ident, $iter_path:literal)
                ),* $(,)?
            ],
        )?
        $(
            optional_iter: [
                $(
                    ($optional_iter_group_method:ident, $optional_iter_product_method:ident, $optional_iter_getter:ident, $optional_iter_path:literal)
                ),* $(,)?
            ],
        )?
        $(
            custom_group_extraction: [ $( $custom_group_extraction_method:ident ),* $(,)? ],
        )?
        $(
            custom_product_extraction: [ $( $custom_product_extraction_method:ident ),* $(,)? ] $(,)?
        )?
    ) => {
        $(
            $(
                /// Get all group references from this element
                fn $vec_group_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractGroupReferences::extract_group_references(
                        self.$getter().iter(),
                        $path,
                    )
                }

                /// Get all product references from this element
                fn $vec_product_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractProductReferences::extract_product_references(
                        self.$getter().iter(),
                        $path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $optional_vec_group_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractGroupReferences::extract_group_references(
                        self.$optional_vec_getter().map(|items| items.iter()).into_iter().flatten(),
                        $optional_vec_path,
                    )
                }

                fn $optional_vec_product_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractProductReferences::extract_product_references(
                        self.$optional_vec_getter().map(|items| items.iter()).into_iter().flatten(),
                        $optional_vec_path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $iter_group_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractGroupReferences::extract_group_references(
                        self.$iter_getter(),
                        $iter_path,
                    )
                }

                fn $iter_product_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractProductReferences::extract_product_references(
                        self.$iter_getter(),
                        $iter_path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $optional_iter_group_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractGroupReferences::extract_group_references(
                        self.$optional_iter_getter().into_iter().flatten(),
                        $optional_iter_path,
                    )
                }

                fn $optional_iter_product_method(&self) -> Vec<(String, String)> {
                    $crate::csaf::traits::util::extract_references::ExtractProductReferences::extract_product_references(
                        self.$optional_iter_getter().into_iter().flatten(),
                        $optional_iter_path,
                    )
                }
            )*
        )?

        /// Returns all group references across all elements
        fn get_all_group_references(&self) -> Vec<(String, String)> {
            let mut refs = Vec::new();
            $( $( refs.extend(self.$vec_group_method()); )* )?
            $( $( refs.extend(self.$optional_vec_group_method()); )* )?
            $( $( refs.extend(self.$iter_group_method()); )* )?
            $( $( refs.extend(self.$optional_iter_group_method()); )* )?
            $( $( refs.extend(self.$custom_group_extraction_method()); )* )?
            refs
        }

        /// Returns all product references across all elements
        fn get_all_product_references(&self) -> Vec<(String, String)> {
            let mut refs = Vec::new();
            $( $( refs.extend(self.$vec_product_method()); )* )?
            $( $( refs.extend(self.$optional_vec_product_method()); )* )?
            $( $( refs.extend(self.$iter_product_method()); )* )?
            $( $( refs.extend(self.$optional_iter_product_method()); )* )?
            $( $( refs.extend(self.$custom_product_extraction_method()); )* )?
            refs
        }
    };
}

pub(crate) use define_reference_accessors;
