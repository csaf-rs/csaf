use crate::csaf::csaf2_1::schema::{CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, Remediation, Vulnerability};
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use std::ops::Deref;

impl RemediationTrait for Remediation {
    fn get_category(&self) -> CategoryOfTheRemediation {
        self.category.clone()
    }

    fn get_product_ids(&self) -> Option<Vec<String>> {
        match &self.product_ids {
            None => None,
            Some(product_ids) => Some(product_ids.deref().iter().map(|x| x.to_string()).collect())
        }
    }
}

impl VulnerabilityTrait for Vulnerability {
    type RemediationType = Remediation;

    fn get_remediations(&self) -> Vec<Self::RemediationType> {
        self.remediations.clone()
    }
}

impl CsafTrait for CommonSecurityAdvisoryFramework {
    type VulnerabilityType = Vulnerability;

    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType> {
        self.vulnerabilities.clone()
    }
}