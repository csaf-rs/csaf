use crate::csaf_traits::ProductTrait;
use crate::schema::csaf2_0::schema::Relationship;
use crate::schema::csaf2_1::schema::{FullProductNameT, ProductPath};
use std::ops::Deref;

/// Trait representing an abstract relationship or product path in a product tree.
pub trait ProductPathTrait<FPN: ProductTrait> {
    /// Retrieves the product reference identifier.
    fn get_beginning_product_reference(&self) -> &String;

    /// Retrieves the identifier of the related product.
    fn get_subpath_product_references(&self) -> Vec<&String>;

    /// Retrieves the full product name associated with the relationship.
    fn get_full_product_name(&self) -> &FPN;

    /// Retrieves the JSON path for a product path
    fn get_json_path_for_product_path(&self, pp_i: usize) -> String;

    /// Retrieves the JSON path for the product reference at the beginning of a product path relationship.
    fn get_json_path_for_product_path_beginning_product_reference(&self, pp_i: usize) -> String;

    /// Retrieves the JSON path for the product reference at a subpath of a product path relationship
    fn get_json_path_for_product_path_subpath_product_reference(&self, pp_i: usize, sp_i: usize) -> String;
}

impl ProductPathTrait<crate::schema::csaf2_0::schema::FullProductNameT> for Relationship {
    fn get_beginning_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_subpath_product_references(&self) -> Vec<&String> {
        vec![self.relates_to_product_reference.deref()]
    }

    fn get_full_product_name(&self) -> &crate::schema::csaf2_0::schema::FullProductNameT {
        &self.full_product_name
    }

    fn get_json_path_for_product_path(&self, rel_i: usize) -> String {
        format!("/product_tree/relationships/{rel_i}")
    }

    fn get_json_path_for_product_path_beginning_product_reference(&self, rel_i: usize) -> String {
        format!("/product_tree/relationships/{rel_i}/product_reference")
    }

    fn get_json_path_for_product_path_subpath_product_reference(&self, rel_i: usize, _: usize) -> String {
        format!("/product_tree/relationships/{rel_i}/relates_to_product_reference")
    }
}

impl ProductPathTrait<FullProductNameT> for ProductPath {
    fn get_beginning_product_reference(&self) -> &String {
        &self.beginning_product_reference
    }

    fn get_subpath_product_references(&self) -> Vec<&String> {
        self.subpaths
            .iter()
            .map(|subpath| subpath.next_product_reference.deref())
            .collect()
    }

    fn get_full_product_name(&self) -> &FullProductNameT {
        &self.full_product_name
    }

    fn get_json_path_for_product_path(&self, pp_i: usize) -> String {
        format!("/product_tree/product_paths/{pp_i}")
    }

    fn get_json_path_for_product_path_beginning_product_reference(&self, pp_i: usize) -> String {
        format!("/product_tree/product_paths/{pp_i}/beginning_product_reference")
    }

    fn get_json_path_for_product_path_subpath_product_reference(&self, pp_i: usize, sp_i: usize) -> String {
        format!("/product_tree/product_paths/{pp_i}/subpaths/{sp_i}/next_product_reference")
    }
}
