use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::ops::{Deref, DerefMut};

use crate::csaf::aggregation::product_status::ProductStatusAndPath;
use crate::csaf_traits::{ProductStatusGroup, ProductStatusTrait};

/// Aggregation of product IDs grouped by their [`ProductStatusGroup`], preserving
/// the original [`ProductStatus`] and index each product originated from.
#[derive(Debug, Clone)]
pub struct ProductStatusGroupMap(HashMap<ProductStatusGroup, HashMap<String, Vec<ProductStatusAndPath>>>);

impl<T: ProductStatusTrait> From<&T> for ProductStatusGroupMap {
    /// Construct a [`ProductStatusGroupMap`] from anything implementing [`ProductStatusTrait`].
    fn from(ps: &T) -> Self {
        let mut result = Self(HashMap::new());

        for (status, products) in ps.get_products_by_status() {
            if products.is_empty() {
                continue;
            }
            let group = ProductStatusGroup::from(&status);
            let entry = result.0.entry(group).or_default();
            for (index, product_id) in products.into_iter().enumerate() {
                entry.entry(product_id).or_default().push(ProductStatusAndPath {
                    status: status.clone(),
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
            .is_some_and(|entries| entries.contains_key(product_id))
    }
}

impl Deref for ProductStatusGroupMap {
    type Target = HashMap<ProductStatusGroup, HashMap<String, Vec<ProductStatusAndPath>>>;

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
    type Item = (ProductStatusGroup, HashMap<String, Vec<ProductStatusAndPath>>);
    type IntoIter = IntoIter<ProductStatusGroup, HashMap<String, Vec<ProductStatusAndPath>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
