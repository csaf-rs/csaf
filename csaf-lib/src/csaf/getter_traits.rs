use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::helpers::resolve_product_groups;
use std::collections::BTreeSet;

pub trait CsafTrait {
    type VulnerabilityType: VulnerabilityTrait;
    type ProductTreeType: ProductTreeTrait;

    fn get_product_tree(&self) -> Option<Self::ProductTreeType>;

    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType>;
}

pub trait VulnerabilityTrait {
    type RemediationType: RemediationTrait;

    fn get_remediations(&self) -> Vec<Self::RemediationType>;
}

pub trait RemediationTrait {
    fn get_category(&self) -> CategoryOfTheRemediation;

    fn get_product_ids(&self) -> Option<Vec<&String>>;

    fn get_group_ids(&self) -> Option<Vec<&String>>;

    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<BTreeSet<String>> {
        if self.get_product_ids().is_none() && self.get_group_ids().is_none() {
            None
        } else {
            let mut product_set: BTreeSet<String> = match self.get_product_ids() {
                Some(product_ids) => product_ids.iter().map(|p| p.to_string()).collect(),
                None => BTreeSet::new(),
            };
            if let Some(product_groups) = self.get_group_ids() {
                if let Some(product_ids) = resolve_product_groups(doc, product_groups) {
                    product_set.extend(product_ids.iter().map(|p| p.to_string()));
                }
            }
            Some(product_set)
        }
    }
}

pub trait ProductTreeTrait {
    type ProductGroupType: ProductGroupTrait;

    fn get_product_groups(&self) -> Vec<Self::ProductGroupType>;
}

pub trait ProductGroupTrait {
    fn get_group_id(&self) -> &String;

    fn get_product_ids(&self) -> Vec<&String>;
}
