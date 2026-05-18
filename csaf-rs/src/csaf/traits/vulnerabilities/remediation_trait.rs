use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{
    CsafTrait, WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds, resolve_product_groups,
};
use crate::schema::csaf2_0::schema::{
    CategoryOfTheRemediation as CategoryOfTheRemediation20, Remediation as Remediation20,
};
use crate::schema::csaf2_1::schema::{
    CategoryOfTheRemediation as CategoryOfTheRemediation21, Remediation as Remediation21,
};
use std::collections::BTreeSet;
use std::ops::Deref;

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the category of the remediation.
    ///
    /// Categories are defined by the CSAF schema.
    fn get_category(&self) -> CategoryOfTheRemediation21;

    /// Computes a set of all product IDs affected by this remediation, either
    /// directly or through product groups.
    ///
    /// # Arguments
    ///
    /// * `doc` - A reference to the CSAF document to resolve product groups.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<String>` containing all product IDs, or `None` if none exist.
    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<BTreeSet<String>> {
        if self.get_product_ids().is_none() && self.get_group_ids().is_none() {
            None
        } else {
            let mut product_set: BTreeSet<String> = match self.get_product_ids() {
                Some(product_ids) => product_ids.cloned().collect(),
                None => BTreeSet::new(),
            };
            if let Some(product_groups) = self.get_group_ids()
                && let Some(product_ids) = resolve_product_groups(doc, product_groups)
            {
                product_set.extend(product_ids.iter().map(|id| id.to_owned()));
            }
            Some(product_set)
        }
    }
}

impl WithOptionalGroupIds for Remediation20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Remediation20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl RemediationTrait for Remediation20 {
    /// Normalizes the remediation categories from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the list of remediation categories was expanded, making it a superset of those
    /// in CSAF 2.0. This function ensures that the remediation category from a CSAF 2.0 remediation
    /// object is converted into the corresponding category defined in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `CategoryOfTheRemediation` that corresponds to the remediation category of the
    /// current object.
    fn get_category(&self) -> CategoryOfTheRemediation21 {
        match self.category {
            CategoryOfTheRemediation20::Workaround => CategoryOfTheRemediation21::Workaround,
            CategoryOfTheRemediation20::Mitigation => CategoryOfTheRemediation21::Mitigation,
            CategoryOfTheRemediation20::VendorFix => CategoryOfTheRemediation21::VendorFix,
            CategoryOfTheRemediation20::NoFixPlanned => CategoryOfTheRemediation21::NoFixPlanned,
            CategoryOfTheRemediation20::NoneAvailable => CategoryOfTheRemediation21::NoneAvailable,
        }
    }
}

impl WithOptionalDate for Remediation20 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalGroupIds for Remediation21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Remediation21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl RemediationTrait for Remediation21 {
    fn get_category(&self) -> CategoryOfTheRemediation21 {
        self.category
    }
}

impl WithOptionalDate for Remediation21 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}
