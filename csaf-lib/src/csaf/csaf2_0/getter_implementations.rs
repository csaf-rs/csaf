use std::ops::Deref;
use std::str::FromStr;
use crate::csaf::csaf2_0::schema::{CommonSecurityAdvisoryFramework, Remediation, Vulnerability};
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation as Remediation21;
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};

impl RemediationTrait for Remediation {
    fn get_category(&self) -> Remediation21 {
        // Categories are identical, so this should never fail
        Remediation21::from_str(self.category.to_string().as_str()).unwrap()
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