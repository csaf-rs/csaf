use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;

pub trait CsafTrait {
    type VulnerabilityType: VulnerabilityTrait;

    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType>;
}

pub trait VulnerabilityTrait {
    type RemediationType: RemediationTrait;

    fn get_remediations(&self) -> Vec<Self::RemediationType>;
}

pub trait RemediationTrait {
    fn get_category(&self) -> CategoryOfTheRemediation;
    fn get_product_ids(&self) -> Option<Vec<String>>;
}