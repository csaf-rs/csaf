use crate::csaf_traits::ProductStatusGroup;
use crate::schema::csaf2_1::schema::ProductStatus;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// Helper macro to add product status groups to a HashMap
macro_rules! add_product_status {
    ($result:ident, $status_group:expr, $getter:expr) => {
        if let Some(products) = $getter {
            $result
                .entry($status_group)
                .or_insert_with(HashSet::new)
                .extend(products);
        }
    };
}

/// Trait representing an abstract product status in a CSAF document.
pub trait ProductStatusTrait {
    /// Returns a reference to the list of first affected product IDs.
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of first fixed product IDs.
    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of fixed product IDs.
    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known affected product IDs.
    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known not-affected product IDs.
    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of last affected product IDs.
    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of recommended product IDs.
    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of product IDs currently under investigation.
    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Return a reference to the list of product IDs with unknown status.
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Helper method to add product references with a given label to the result vector.
    fn extract_product_references<'a>(
        &self,
        ids: &mut Vec<(String, String)>,
        products: Option<impl Iterator<Item = &'a String> + 'a>,
        label: &str,
    ) {
        if let Some(iter) = products {
            for (x_i, x) in iter.enumerate() {
                ids.push(((*x).to_owned(), format!("product_status/{label}/{x_i}")));
            }
        }
    }

    fn get_all_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        self.extract_product_references(&mut ids, self.get_first_affected(), "first_affected");
        self.extract_product_references(&mut ids, self.get_first_fixed(), "first_fixed");
        self.extract_product_references(&mut ids, self.get_fixed(), "fixed");
        self.extract_product_references(&mut ids, self.get_known_affected(), "known_affected");
        self.extract_product_references(&mut ids, self.get_known_not_affected(), "known_not_affected");
        self.extract_product_references(&mut ids, self.get_last_affected(), "last_affected");
        self.extract_product_references(&mut ids, self.get_recommended(), "recommended");
        self.extract_product_references(&mut ids, self.get_under_investigation(), "under_investigation");
        self.extract_product_references(&mut ids, self.get_unknown(), "unknown");
        ids
    }

    /// Returns a `HashMap` containing all product IDs grouped by their statuses.
    fn get_all_by_product_status(&self) -> HashMap<ProductStatusGroup, HashSet<&String>> {
        let mut result: HashMap<ProductStatusGroup, HashSet<&String>> = HashMap::new();

        // affected
        add_product_status!(result, ProductStatusGroup::Affected, self.get_first_affected());
        add_product_status!(result, ProductStatusGroup::Affected, self.get_last_affected());
        add_product_status!(result, ProductStatusGroup::Affected, self.get_known_affected());

        // not affected
        add_product_status!(result, ProductStatusGroup::NotAffected, self.get_known_not_affected());

        // fixed
        add_product_status!(result, ProductStatusGroup::Fixed, self.get_fixed());
        add_product_status!(result, ProductStatusGroup::Fixed, self.get_first_fixed());

        // under investigation
        add_product_status!(
            result,
            ProductStatusGroup::UnderInvestigation,
            self.get_under_investigation()
        );

        // unknown
        add_product_status!(result, ProductStatusGroup::Unknown, self.get_unknown());

        // recommended
        add_product_status!(result, ProductStatusGroup::Recommended, self.get_recommended());

        result
    }
}

impl ProductStatusTrait for crate::schema::csaf2_0::schema::ProductStatus {
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_not_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.last_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.recommended.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.under_investigation
            .as_ref()
            .map(|p| (*p).iter().map(|x| x.deref()))
    }

    /// Not specified for CSAF 2.0, so `None`
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl ProductStatusTrait for ProductStatus {
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_not_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.last_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.recommended.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.under_investigation
            .as_ref()
            .map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.unknown.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}
