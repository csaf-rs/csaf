use crate::csaf_traits::ProductTrait;
use crate::schema::csaf2_1::schema::{FullProductNameT, ProductPath};
use std::ops::Deref;

/// Trait representing an abstract relationship in a product tree.
pub trait RelationshipTrait<FPN: ProductTrait> {
    /// Retrieves the product reference identifier.
    fn get_product_reference(&self) -> &String;

    /// Retrieves the identifier of the related product.
    fn get_relates_to_product_reference(&self) -> &String;

    /// Retrieves the full product name associated with the relationship.
    fn get_full_product_name(&self) -> &FPN;
}

impl RelationshipTrait<crate::schema::csaf2_0::schema::FullProductNameT>
    for crate::schema::csaf2_0::schema::Relationship
{
    fn get_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_relates_to_product_reference(&self) -> &String {
        self.relates_to_product_reference.deref()
    }

    fn get_full_product_name(&self) -> &crate::schema::csaf2_0::schema::FullProductNameT {
        &self.full_product_name
    }
}

impl RelationshipTrait<FullProductNameT> for ProductPath {
    fn get_product_reference(&self) -> &String {
        &self.beginning_product_reference
    }

    fn get_relates_to_product_reference(&self) -> &String {
        todo!(
            "Refactor RelationshipTrait into ProductPathTrait, and convert CSAF 2.0 Relationships into ProductPaths, see issue #503"
        );
    }

    fn get_full_product_name(&self) -> &FullProductNameT {
        &self.full_product_name
    }
}
