use crate::csaf::traits::util::{impl_str_field_getter, impl_str_iter_field_getter};
use crate::schema::csaf2_0::schema::ProductGroup as ProductGroup20;
use crate::schema::csaf2_1::schema::ProductGroup as ProductGroup21;

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Returns the unique identifier of the product group.
    fn get_group_id(&self) -> &str;

    /// Retrieves an iterator over product IDs contained within the product group.
    fn get_product_ids(&self) -> impl Iterator<Item = &str> + '_;
}

impl ProductGroupTrait for ProductGroup20 {
    impl_str_field_getter!(get_group_id, group_id);
    impl_str_iter_field_getter!(get_product_ids, product_ids);
}

impl ProductGroupTrait for ProductGroup21 {
    impl_str_field_getter!(get_group_id, group_id);
    impl_str_iter_field_getter!(get_product_ids, product_ids);
}
