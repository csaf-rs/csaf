use crate::csaf::csaf2_0::schema::{CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, ProductGroup, ProductTree, Remediation, Vulnerability};
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation as Remediation21;
use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait, RemediationTrait, VulnerabilityTrait};
use std::ops::Deref;

impl RemediationTrait for Remediation {

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
    fn get_category(&self) -> Remediation21 {
        match self.category {
            CategoryOfTheRemediation::Workaround => Remediation21::Workaround,
            CategoryOfTheRemediation::Mitigation => Remediation21::Mitigation,
            CategoryOfTheRemediation::VendorFix => Remediation21::VendorFix,
            CategoryOfTheRemediation::NoFixPlanned => Remediation21::NoFixPlanned,
            CategoryOfTheRemediation::NoneAvailable => Remediation21::NoneAvailable,
        }
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