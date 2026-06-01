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

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the category of the remediation.
    fn get_category(&self) -> CategoryOfTheRemediation21;

    /// Computes a set of all product IDs affected by this remediation, either
    /// directly or through product groups.
    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<BTreeSet<String>> {
        if self.get_product_ids().is_none() && self.get_group_ids().is_none() {
            None
        } else {
            let mut product_set: BTreeSet<String> = match self.get_product_ids() {
                Some(product_ids) => product_ids.map(|id| id.to_owned()).collect(),
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

crate::csaf::traits::impl_optional_ids!(Remediation20, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Remediation20, WithOptionalProductIds, ReturnsValues);
crate::csaf::traits::impl_with_optional_date!(Remediation20);

crate::csaf::traits::impl_optional_ids!(Remediation21, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Remediation21, WithOptionalProductIds, ReturnsValues);
crate::csaf::traits::impl_with_optional_date!(Remediation21);

impl RemediationTrait for Remediation20 {
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

impl RemediationTrait for Remediation21 {
    fn get_category(&self) -> CategoryOfTheRemediation21 {
        self.category
    }
}
