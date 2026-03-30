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
    fn get_product_tree(&self) -> &Option<Self::ProductTreeType>;

    fn get_product_tree_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        if let Some(product_tree) = self.get_product_tree() {
            ids.append(&mut product_tree.get_product_groups_product_references());
            ids.append(&mut product_tree.get_relationships_product_references());
        }

        ids
    }

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType>;

    /// Utility function to prepend a JSON path prefix to a list of (ID, path) tuples
    fn prepend_path(prefix: &str, idx: &usize, id_path_tuples: Vec<(String, String)>) -> Vec<(String, String)> {
        id_path_tuples
            .iter()
            .map(|(group_or_product_id, path)| (group_or_product_id.to_owned(), format!("/{prefix}/{idx}/{path}")))
            .collect()
    }

    /// Utility function to get all group IDs referenced in vulnerabilities along with their JSON paths
    fn get_vulnerability_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (vuln_index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
            let getters = [
                vulnerability.get_flags_group_references(),
                vulnerability.get_involvement_group_references(),
                vulnerability.get_notes_group_references(),
                vulnerability.get_remediations_group_references(),
                vulnerability.get_threats_group_references(),
                vulnerability.get_first_known_exploitation_dates_group_references(),
            ];
            for getter in getters {
                ids.append(&mut Self::prepend_path("vulnerabilities", &vuln_index, getter));
            }
        }
        ids
    }

    /// Utility function to get all product IDs referenced in vulnerabilities along with their JSON paths
    fn get_vulnerability_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (vuln_index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
            let getters = [
                vulnerability.get_flags_product_references(),
                vulnerability.get_threats_product_references(),
                vulnerability.get_remediations_product_references(),
                vulnerability.get_product_status_product_references(),
                vulnerability.get_metrics_product_references(),
                vulnerability.get_notes_product_references(),
                vulnerability.get_involvements_product_references(),
                vulnerability.get_first_known_exploitation_dates_product_references(),
            ];

            for getter in getters {
                ids.append(&mut Self::prepend_path("vulnerabilities", &vuln_index, getter));
            }
        }
        ids
    }
    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> &Self::DocumentType;

    /// Utility function to get all group IDs referenced in the document along with their JSON paths
    fn get_all_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        ids.append(&mut self.get_document().get_notes_group_references());
        ids.append(&mut self.get_vulnerability_group_references());
        ids
    }

    /// Utility function to get all product IDs referenced in the document along with their JSON paths
    fn get_all_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        ids.append(&mut self.get_document().get_notes_product_references());
        ids.append(&mut self.get_vulnerability_product_references());
        ids.append(&mut self.get_product_tree_product_references());
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

    fn get_product_tree(&self) -> &Option<Self::ProductTreeType> {
        &self.product_tree
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

    fn get_product_tree(&self) -> &Option<Self::ProductTreeType> {
        &self.product_tree
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }
}
