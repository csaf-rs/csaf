use std::collections::HashSet;
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;

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
    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<HashSet<String>>;
}

pub trait ProductTreeTrait {
    type ProductGroupType: ProductGroupTrait;

    fn get_product_groups(&self) -> Vec<Self::ProductGroupType>;
}

pub trait ProductGroupTrait {
    fn get_group_id(&self) -> &String;
    fn get_product_ids(&self) -> Vec<&String>;
}
