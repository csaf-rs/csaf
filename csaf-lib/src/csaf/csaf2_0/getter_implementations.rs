use crate::csaf::csaf2_0::schema::{CommonSecurityAdvisoryFramework, ProductGroup, ProductTree, Remediation, Vulnerability};
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation as Remediation21;
use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait, RemediationTrait, VulnerabilityTrait};
use std::ops::Deref;
use std::str::FromStr;

impl RemediationTrait for Remediation {
    fn get_category(&self) -> Remediation21 {
        // Categories are identical, so this should never fail
        Remediation21::from_str(self.category.to_string().as_str()).unwrap()
    }

    fn get_product_ids(&self) -> Option<Vec<&String>> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_group_ids(&self) -> Option<Vec<&String>> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()).collect())
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
    type ProductTreeType = ProductTree;

    fn get_product_tree(&self) -> Option<Self::ProductTreeType> {
        self.product_tree.clone()
    }

    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType> {
        self.vulnerabilities.clone()
    }
}

impl ProductTreeTrait for ProductTree {
    type ProductGroupType = ProductGroup;

    fn get_product_groups(&self) -> Vec<Self::ProductGroupType> {
        self.product_groups.clone()
    }
}

impl ProductGroupTrait for ProductGroup {
    fn get_group_id(&self) -> &String {
        self.group_id.deref()
    }

    fn get_product_ids(&self) -> Vec<&String> {
        self.product_ids.iter().map(|x| x.deref()).collect()
    }
}