use crate::schema::csaf2_0::schema::ProductGroup as ProductGroup20;
use crate::schema::csaf2_1::schema::ProductGroup as ProductGroup21;
use std::ops::Deref;

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Returns the unique identifier of the product group.
    fn get_group_id(&self) -> &String;

    /// Retrieves a vector of product IDs contained within the product group.
    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_;
}

impl ProductGroupTrait for ProductGroup20 {
    fn get_group_id(&self) -> &String {
        self.group_id.deref()
    }

    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_ {
        self.product_ids.iter().map(|id| id.deref())
    }
}

impl ProductGroupTrait for ProductGroup21 {
    fn get_group_id(&self) -> &String {
        self.group_id.deref()
    }

    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_ {
        self.product_ids.iter().map(|id| id.deref())
    }
}
