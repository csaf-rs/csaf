use crate::csaf::traits::vulnerabilities_trait::collect_references;
use crate::csaf_traits::{DocumentTrait, ProductTreeTrait, VulnerabilityTrait};
use crate::schema::csaf2_0::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework20,
    DocumentLevelMetaData as DocumentLevelMetaData20, ProductTree as ProductTree20, Vulnerability as Vulnerability20,
};
use crate::schema::csaf2_1::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework21,
    DocumentLevelMetaData as DocumentLevelMetaData21, ProductTree as ProductTree21, Vulnerability as Vulnerability21,
};

/// Trait representing an abstract Common Security Advisory Framework (CSAF) document.
///
/// The `CsafTrait` trait defines the key structure of a CSAF document, allowing
/// interaction with its vulnerabilities and product tree without tying to a
/// specific version of the CSAF schema.
pub trait CsafTrait {
    /// The associated type representing the type of vulnerabilities in this CSAF structure.
    type VulnerabilityType: VulnerabilityTrait;

    /// The associated type representing the type of the product tree in this CSAF structure.
    type ProductTreeType: ProductTreeTrait;

    /// The associated type representing the type of document meta in this CSAF structure.
    type DocumentType: DocumentTrait;

    /// Returns the product tree of the CSAF document, if available.
    fn get_product_tree(&self) -> Option<&Self::ProductTreeType>;

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType>;

    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> &Self::DocumentType;

    /// Returns all group IDs referenced in the document along with their JSON paths.
    fn get_all_group_references(&self) -> Vec<(String, String)> {
        let mut ids = self.get_document().get_all_group_references();
        ids.extend(collect_references(self.get_vulnerabilities(), |v| {
            v.get_all_group_references()
        }));
        ids
    }

    /// Returns all product IDs referenced in the document along with their JSON paths.
    fn get_all_product_references(&self) -> Vec<(String, String)> {
        let mut ids = self.get_document().get_all_product_references();
        ids.extend(collect_references(self.get_vulnerabilities(), |v| {
            v.get_all_product_references()
        }));
        if let Some(pt) = self.get_product_tree() {
            ids.extend(pt.get_all_product_references());
        }
        ids
    }

    /// Utility function to get all product IDs referenced (expect those explicitly defined) in the document.
    fn get_all_product_references_ids(&self) -> Vec<String> {
        self.get_all_product_references()
            .iter()
            .map(|(id, _)| id.to_owned())
            .collect()
    }
}

impl CsafTrait for CommonSecurityAdvisoryFramework20 {
    type VulnerabilityType = Vulnerability20;
    type ProductTreeType = ProductTree20;
    type DocumentType = DocumentLevelMetaData20;

    fn get_product_tree(&self) -> Option<&Self::ProductTreeType> {
        self.product_tree.as_ref()
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }
}

impl CsafTrait for CommonSecurityAdvisoryFramework21 {
    type VulnerabilityType = Vulnerability21;
    type ProductTreeType = ProductTree21;
    type DocumentType = DocumentLevelMetaData21;

    fn get_product_tree(&self) -> Option<&Self::ProductTreeType> {
        self.product_tree.as_ref()
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }
}
