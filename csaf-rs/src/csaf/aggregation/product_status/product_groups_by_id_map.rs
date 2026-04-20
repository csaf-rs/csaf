use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::csaf_traits::{ProductStatusAndPath, ProductStatusGroup, ProductStatusTrait};

/// Aggregation of product IDs, mapped to their [`ProductStatusGroup`], preserving
/// the original [`ProductStatus`] and index each product originated from.
#[derive(Debug, Clone)]
pub struct ProductGroupsByIdMap(HashMap<String, HashMap<ProductStatusGroup, Vec<ProductStatusAndPath>>>);

impl<T: ProductStatusTrait> From<&T> for ProductGroupsByIdMap {
    /// Construct a [`ProductGroupsByIdMap`] from anything implementing [`ProductStatusTrait`].
    fn from(ps: &T) -> Self {
        let mut result = Self(HashMap::new());

        for (status, products) in ps.get_products_by_status() {
            let group = ProductStatusGroup::from(&status);
            for (index, product_id) in products.into_iter().enumerate() {
                result
                    .0
                    .entry(product_id)
                    .or_default()
                    .entry(group.clone())
                    .or_default()
                    .push(ProductStatusAndPath {
                        status: status.clone(),
                        index,
                    });
            }
        }

        result
    }
}

impl Deref for ProductGroupsByIdMap {
    type Target = HashMap<String, HashMap<ProductStatusGroup, Vec<ProductStatusAndPath>>>;

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
    type Item = (String, HashMap<ProductStatusGroup, Vec<ProductStatusAndPath>>);
    type IntoIter = IntoIter<String, HashMap<ProductStatusGroup, Vec<ProductStatusAndPath>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
