use crate::csaf_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use std::collections::BTreeMap;

/// Aggregation mapping:
/// resolved remediation product IDs -> remediation category -> vector of remediation indices of occurrences
pub struct ProductIdRemediationCategoriesMap(BTreeMap<String, BTreeMap<CategoryOfTheRemediation, Vec<usize>>>);

impl ProductIdRemediationCategoriesMap {
    pub fn aggregate(doc: &impl CsafTrait, vulnerability: &impl VulnerabilityTrait) -> Self {
        let mut map: BTreeMap<String, BTreeMap<CategoryOfTheRemediation, Vec<usize>>> = BTreeMap::new();
        for (remediation_index, remediation) in vulnerability.get_remediations().iter().enumerate() {
            // get the associated product ids, if there are none, continue
            let product_ids = match remediation.get_all_product_ids(doc) {
                Some(ids) => ids,
                None => continue,
            };

            // fill the map
            for product_id in product_ids {
                map.entry(product_id.clone())
                    .or_default()
                    .entry(remediation.get_category())
                    .or_default()
                    .push(remediation_index);
            }
        }
        Self(map)
    }
}

impl std::ops::Deref for ProductIdRemediationCategoriesMap {
    type Target = BTreeMap<String, BTreeMap<CategoryOfTheRemediation, Vec<usize>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
