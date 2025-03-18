use crate::csaf::csaf2_1::schema::{Branch, CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, FullProductNameT, Metric, ProductGroup, ProductStatus, ProductTree, Relationship, Remediation, Threat, Vulnerability};
use crate::csaf::getter_traits::{BranchTrait, CsafTrait, FullProductNameTrait, MetricTrait, ProductGroupTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, ThreatTrait, VulnerabilityTrait};
use std::ops::Deref;

impl RemediationTrait for Remediation {
    fn get_category(&self) -> CategoryOfTheRemediation {
        self.category.clone()
    }

    fn get_product_ids(&self) -> Option<Vec<&String>> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_group_ids(&self) -> Option<Vec<&String>> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()).collect())
    }
}

impl ProductStatusTrait for ProductStatus {
    fn get_first_affected(&self) -> Option<Vec<&String>> {
        self.first_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_first_fixed(&self) -> Option<Vec<&String>> {
        self.first_fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_fixed(&self) -> Option<Vec<&String>> {
        self.fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_known_affected(&self) -> Option<Vec<&String>> {
        self.known_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_known_not_affected(&self) -> Option<Vec<&String>> {
        self.known_not_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_last_affected(&self) -> Option<Vec<&String>> {
        self.last_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_recommended(&self) -> Option<Vec<&String>> {
        self.recommended.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }

    fn get_under_investigation(&self) -> Option<Vec<&String>> {
        self.under_investigation.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }
}

impl MetricTrait for Metric {
    fn get_products(&self) -> Vec<&String> {
        self.products.deref().iter().map(|p| p.deref()).collect()
    }
}

impl ThreatTrait for Threat {
    fn get_product_ids(&self) -> Option<Vec<&String>> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()).collect())
    }
}

impl VulnerabilityTrait for Vulnerability {
    type RemediationType = Remediation;
    type ProductStatusType = ProductStatus;
    type MetricType = Metric;
    type ThreatType = Threat;

    fn get_remediations(&self) -> Vec<Self::RemediationType> {
        self.remediations.clone()
    }

    fn get_product_status(&self) -> Option<Self::ProductStatusType> {
        self.product_status.clone()
    }

    fn get_metrics(&self) -> Option<Vec<Self::MetricType>> {
        self.metrics.clone()
    }

    fn get_threats(&self) -> Vec<Self::ThreatType> {
        self.threats.clone()
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
    type BranchType = Branch;
    type ProductGroupType = ProductGroup;
    type RelationshipType = Relationship;
    type FullProductNameType = FullProductNameT;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_ref().map(|branches| branches.deref())
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_relationships(&self) -> &Vec<Self::RelationshipType> {
        &self.relationships
    }

    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType> {
        &self.full_product_names
    }
}

impl BranchTrait for Branch {
    type BranchType = Branch;
    type FullProductNameType = FullProductNameT;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_ref().map(|branches| branches.deref())
    }

    fn get_product(&self) -> Option<&Self::FullProductNameType> {
        self.product.as_ref()
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

impl RelationshipTrait for Relationship {
    type FullProductNameType = FullProductNameT;

    fn get_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_relates_to_product_reference(&self) -> &String {
        self.relates_to_product_reference.deref()
    }

    fn get_full_product_name(&self) -> &Self::FullProductNameType {
        &self.full_product_name
    }
}

impl FullProductNameTrait for FullProductNameT {
    fn get_product_id(&self) -> &String {
        self.product_id.deref()
    }
}
