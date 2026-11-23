use crate::csaf::traits::util::reference::{IdList, IdReference, ReferenceLocation};
use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};

/// Iterates over the group IDs referenced by `items`, which are located at `key`.
///
/// Each reference borrows its ID from the document and is located at
/// `/{key}/{index}/group_ids/{group_index}`.
pub(crate) fn group_references<'a, T: WithOptionalGroupIds + 'a>(
    items: impl Iterator<Item = &'a T> + 'a,
    key: &'static str,
) -> impl Iterator<Item = IdReference<'a>> + 'a {
    items.enumerate().flat_map(move |(index, item)| {
        item.get_group_ids()
            .into_iter()
            .flatten()
            .enumerate()
            .map(move |(group_index, group_id)| {
                IdReference::new(
                    group_id,
                    ReferenceLocation::Field {
                        vulnerability: None,
                        field: key,
                        item: index,
                        list: IdList::GroupIds,
                        index: group_index,
                    },
                )
            })
    })
}

/// Iterates over the product IDs referenced by `items`, which are located at `key`.
///
/// Each reference borrows its ID from the document and is located at
/// `/{key}/{index}/product_ids/{product_index}`.
pub(crate) fn product_references<'a, T: WithOptionalProductIds + 'a>(
    items: impl Iterator<Item = &'a T> + 'a,
    key: &'static str,
) -> impl Iterator<Item = IdReference<'a>> + 'a {
    items.enumerate().flat_map(move |(index, item)| {
        item.get_product_ids()
            .into_iter()
            .flatten()
            .enumerate()
            .map(move |(product_index, product_id)| {
                IdReference::new(
                    product_id,
                    ReferenceLocation::Field {
                        vulnerability: None,
                        field: key,
                        item: index,
                        list: IdList::ProductIds,
                        index: product_index,
                    },
                )
            })
    })
}

/// Generates individual group/product reference iterators and aggregated
/// `get_all_group_references` / `get_all_product_references` iterators from a single
/// field declaration.
///
/// All generated methods return iterators of [`IdReference`]s borrowing from `self`, so no IDs
/// are cloned and paths are only rendered when needed. Locations are relative to `self`.
///
/// # Parameters
///
/// - `vec`: List of `(group_method, product_method, getter, "json_path")` tuples for
///   `&Vec<T>` getters.
/// - `optional_vec`: List of tuples for `Option<&Vec<T>>` getters.
/// - `iter`: List of tuples for `Iterator` getters.
/// - `optional_iter`: List of tuples for getters returning `Option<Iterator>`.
/// - `custom_group_extraction` (optional): List of already-implemented method names that return
///   iterators of group-only references with special handling. These are included in
///   `get_all_group_references` but **not** auto-generated.
/// - `custom_product_extraction` (optional): List of already-implemented method names that return
///   iterators of product-only references with special handling (e.g., metrics, product_status).
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
                fn $vec_group_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::group_references(
                        self.$getter().iter(),
                        $path,
                    )
                }

                /// Get all product references from this element
                fn $vec_product_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::product_references(
                        self.$getter().iter(),
                        $path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $optional_vec_group_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::group_references(
                        self.$optional_vec_getter().map(|items| items.iter()).into_iter().flatten(),
                        $optional_vec_path,
                    )
                }

                fn $optional_vec_product_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::product_references(
                        self.$optional_vec_getter().map(|items| items.iter()).into_iter().flatten(),
                        $optional_vec_path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $iter_group_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::group_references(
                        self.$iter_getter(),
                        $iter_path,
                    )
                }

                fn $iter_product_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::product_references(
                        self.$iter_getter(),
                        $iter_path,
                    )
                }
            )*
        )?

        $(
            $(
                fn $optional_iter_group_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::group_references(
                        self.$optional_iter_getter().into_iter().flatten(),
                        $optional_iter_path,
                    )
                }

                fn $optional_iter_product_method(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
                    $crate::csaf::traits::util::extract_references::product_references(
                        self.$optional_iter_getter().into_iter().flatten(),
                        $optional_iter_path,
                    )
                }
            )*
        )?

        /// Returns all group references across all elements
        fn get_all_group_references(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
            let refs = ::std::iter::empty();
            $( $( let refs = refs.chain(self.$vec_group_method()); )* )?
            $( $( let refs = refs.chain(self.$optional_vec_group_method()); )* )?
            $( $( let refs = refs.chain(self.$iter_group_method()); )* )?
            $( $( let refs = refs.chain(self.$optional_iter_group_method()); )* )?
            $( $( let refs = refs.chain(self.$custom_group_extraction_method()); )* )?
            refs
        }

        /// Returns all product references across all elements
        fn get_all_product_references(&self) -> impl Iterator<Item = $crate::csaf::traits::util::reference::IdReference<'_>> + '_ {
            let refs = ::std::iter::empty();
            $( $( let refs = refs.chain(self.$vec_product_method()); )* )?
            $( $( let refs = refs.chain(self.$optional_vec_product_method()); )* )?
            $( $( let refs = refs.chain(self.$iter_product_method()); )* )?
            $( $( let refs = refs.chain(self.$optional_iter_product_method()); )* )?
            $( $( let refs = refs.chain(self.$custom_product_extraction_method()); )* )?
            refs
        }
    };
}

pub(crate) use define_reference_accessors;
