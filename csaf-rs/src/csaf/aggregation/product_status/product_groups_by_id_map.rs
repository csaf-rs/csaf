use std::collections::hash_map::IntoIter;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use crate::csaf_traits::{ProductStatus, ProductStatusGroup, ProductStatusTrait};

/// Aggregation of product IDs, mapped to their [`ProductStatusGroup`], preserving
/// the original [`ProductStatus`] and index each product originated from.
#[derive(Debug, Clone)]
pub struct ProductGroupsByIdMap(HashMap<String, HashSet<ProductStatusGroupAndPath>>);

/// A [`ProductStatusGroup`] with its original [`ProductStatus`] and the index
/// within the original status list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductStatusGroupAndPath {
    pub status_group: ProductStatusGroup,
    pub status: ProductStatus,
    pub index: usize,
}

impl ProductStatusGroupAndPath {
    /// Returns the JSON path for this product status entry relative to a vulnerability.
    pub fn json_path(&self, vulnerability_index: usize) -> String {
        format!(
            "/vulnerabilities/{}/product_status/{}/{}",
            vulnerability_index, self.status, self.index
        )
    }
}

impl<T: ProductStatusTrait> From<&T> for ProductGroupsByIdMap {
    /// Construct a [`ProductGroupsByIdMap`] from anything implementing [`ProductStatusTrait`].
    fn from(ps: &T) -> Self {
        let mut result = Self(HashMap::new());

        for (status, products) in ps.get_products_by_status() {
            let group = ProductStatusGroup::from(&status);
            for (index, product_id) in products.into_iter().enumerate() {
                result.0
                    .entry(product_id)
                    .or_default()
                    .insert(ProductStatusGroupAndPath {
                        status_group: group.clone(),
                        status: status.clone(),
                        index,
                    });
            }
        }

        result
    }
}

impl Deref for ProductGroupsByIdMap {
    type Target = HashMap<String, HashSet<ProductStatusGroupAndPath>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProductGroupsByIdMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ProductGroupsByIdMap {
    type Item = (String, HashSet<ProductStatusGroupAndPath>);
    type IntoIter = IntoIter<String, HashSet<ProductStatusGroupAndPath>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
