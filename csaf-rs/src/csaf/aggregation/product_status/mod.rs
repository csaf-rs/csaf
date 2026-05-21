use crate::csaf_traits::ProductStatus;

pub mod product_groups_by_id_map;
pub mod product_status_group_map;

/// A product status entry with its original [`ProductStatus`] and the index
/// each product originated from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductStatusAndPath {
    pub status: ProductStatus,
    pub index: usize,
}

impl ProductStatusAndPath {
    /// Returns the JSON path for this product status entry relative to a vulnerability.
    pub fn json_path(&self, vulnerability_index: usize) -> String {
        format!(
            "/vulnerabilities/{}/product_status/{}/{}",
            vulnerability_index, self.status, self.index
        )
    }
}
