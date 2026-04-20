use std::collections::{HashMap, HashSet};
use std::collections::hash_map::IntoIter;
use std::ops::{Deref, DerefMut};

use crate::csaf::enums::product_status::ProductStatus;
use crate::csaf_traits::{ProductStatusGroup, ProductStatusTrait};

/// Aggregation of product IDs grouped by their [`ProductStatusGroup`], preserving
/// the original [`ProductStatus`] and index each product originated from.
#[derive(Debug, Clone)]
pub struct ProductStatusGroupMap(HashMap<ProductStatusGroup, HashSet<ProductStatusAndPath>>);

/// A product ID with its original [`ProductStatus`] and the index
/// within the original status list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductStatusAndPath {
    pub product_id: String,
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

impl<T: ProductStatusTrait> From<&T> for ProductStatusGroupMap {
    /// Construct a [`ProductStatusGroupMap`] from anything implementing [`ProductStatusTrait`].
    fn from(ps: &T) -> Self {
        let mut result = Self(HashMap::new());

        for (status, products) in ps.get_products_by_status() {
            let group = ProductStatusGroup::from(&status);
            let entry = result.0.entry(group).or_default();
            for (index, product_id) in products.into_iter().enumerate() {
                entry.insert(ProductStatusAndPath {
                    status: status.clone(),
                    product_id,
                    index,
                });
            }
        }

        result
    }
}

impl ProductStatusGroupMap {
    /// Returns `true` if the given `product_id` is present in the specified [`ProductStatusGroup`].
    pub fn contains(&self, group: &ProductStatusGroup, product_id: &str) -> bool {
        self.0
            .get(group)
            .is_some_and(|entries| entries.iter().any(|e| e.product_id == product_id))
    }
}


impl Deref for ProductStatusGroupMap {
    type Target = HashMap<ProductStatusGroup, HashSet<ProductStatusAndPath>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProductStatusGroupMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ProductStatusGroupMap {
    type Item = (ProductStatusGroup, HashSet<ProductStatusAndPath>);
    type IntoIter = IntoIter<ProductStatusGroup, HashSet<ProductStatusAndPath>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
