use crate::csaf::enums::product_status::ProductStatus;
use crate::schema::csaf2_1::schema::ProductStatus as ProductStatus2_1;
use std::ops::Deref;

/// Helper macro to implement a getter that returns an optional iterator over product IDs.
macro_rules! impl_product_status_getter {
    ($method:ident, $field:ident) => {
        fn $method(&self) -> Option<impl Iterator<Item = &String> + '_> {
            self.$field.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
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

    /// Returns all product IDs grouped by their [`ProductStatus`]. The original index is
    /// implicit in the Vec<String> index.
    fn get_products_by_status(&self) -> Vec<(ProductStatus, Vec<String>)> {
        fn collect(products: Option<impl Iterator<Item = impl AsRef<str>>>) -> Vec<String> {
            products
                .map(|iter| iter.map(|s| s.as_ref().to_owned()).collect())
                .unwrap_or_default()
        }

        vec![
            (ProductStatus::FirstAffected, collect(self.get_first_affected())),
            (ProductStatus::LastAffected, collect(self.get_last_affected())),
            (ProductStatus::KnownAffected, collect(self.get_known_affected())),
            (ProductStatus::KnownNotAffected, collect(self.get_known_not_affected())),
            (ProductStatus::Fixed, collect(self.get_fixed())),
            (ProductStatus::FirstFixed, collect(self.get_first_fixed())),
            (
                ProductStatus::UnderInvestigation,
                collect(self.get_under_investigation()),
            ),
            (ProductStatus::Unknown, collect(self.get_unknown())),
            (ProductStatus::Recommended, collect(self.get_recommended())),
        ]
    }

    /// Helper method to add product references with a given label to the result vector.
    fn extract_product_references<'a>(
        &self,
        ids: &mut Vec<(String, String)>,
        products: Option<impl Iterator<Item = &'a String> + 'a>,
        label: &str,
    ) {
        if let Some(iter) = products {
            for (x_i, x) in iter.enumerate() {
                ids.push((x.clone(), format!("product_status/{label}/{x_i}")));
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
}

impl ProductStatusTrait for crate::schema::csaf2_0::schema::ProductStatus {
    impl_product_status_getter!(get_first_affected, first_affected);
    impl_product_status_getter!(get_first_fixed, first_fixed);
    impl_product_status_getter!(get_fixed, fixed);
    impl_product_status_getter!(get_known_affected, known_affected);
    impl_product_status_getter!(get_known_not_affected, known_not_affected);
    impl_product_status_getter!(get_last_affected, last_affected);
    impl_product_status_getter!(get_recommended, recommended);
    impl_product_status_getter!(get_under_investigation, under_investigation);

    /// Not specified for CSAF 2.0, so `None`
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl ProductStatusTrait for ProductStatus2_1 {
    impl_product_status_getter!(get_first_affected, first_affected);
    impl_product_status_getter!(get_first_fixed, first_fixed);
    impl_product_status_getter!(get_fixed, fixed);
    impl_product_status_getter!(get_known_affected, known_affected);
    impl_product_status_getter!(get_known_not_affected, known_not_affected);
    impl_product_status_getter!(get_last_affected, last_affected);
    impl_product_status_getter!(get_recommended, recommended);
    impl_product_status_getter!(get_under_investigation, under_investigation);
    impl_product_status_getter!(get_unknown, unknown);
}
