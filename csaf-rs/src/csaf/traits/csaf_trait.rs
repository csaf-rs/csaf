use crate::csaf::traits::document::csaf_21_involvement_trait::Csaf21InvolvementTrait;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::vulnerabilities_trait::collect_references;
use crate::csaf_traits::{
    ContentTrait, DocumentTrait, ExtensionsTrait, MetricTrait, ProductTrait, ProductTreeTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_0::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework20,
    DocumentLevelMetaData as DocumentLevelMetaData20, ProductTree as ProductTree20, Vulnerability as Vulnerability20,
};
use crate::schema::csaf2_1::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework21,
    DocumentLevelMetaData as DocumentLevelMetaData21, ExtensionsT as Extensions21, ProductTree as ProductTree21,
    Vulnerability as Vulnerability21,
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

    type ExtensionsType: ExtensionsTrait;

    /// Returns the product tree of the CSAF document, if available.
    fn get_product_tree(&self) -> Option<&Self::ProductTreeType>;

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType>;

    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> &Self::DocumentType;

    /// Returns all group IDs referenced in the document along with their JSON paths.
    fn get_all_group_references(&self) -> Vec<(String, String)> {
        let mut ids = self.get_document().get_all_group_references();
        if let Some(involvement) = self.get_document().get_involvement() {
            ids.extend(involvement.get_all_group_references());
        }
        ids.extend(collect_references(self.get_vulnerabilities(), |v| {
            v.get_all_group_references()
        }));
        ids
    }

    /// Returns all product IDs referenced in the document along with their JSON paths.
    fn get_all_product_references(&self) -> Vec<(String, String)> {
        let mut ids = self.get_document().get_all_product_references();
        if let Some(involvement) = self.get_document().get_involvement() {
            ids.extend(involvement.get_all_product_references());
        }
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

    fn get_extensions(&self) -> Option<&Self::ExtensionsType>;

    /// Visits all extension containers present in the document.
    ///
    /// The provided visitor function is called for each extension trait implementation found,
    /// along with its corresponding JSON path.
    fn visit_all_extensions(&self, visitor: &mut impl FnMut(&Self::ExtensionsType, &str));
}

impl CsafTrait for CommonSecurityAdvisoryFramework20 {
    type VulnerabilityType = Vulnerability20;
    type ProductTreeType = ProductTree20;
    type DocumentType = DocumentLevelMetaData20;
    type ExtensionsType = NotPresentInCsaf20;

    fn get_product_tree(&self) -> Option<&Self::ProductTreeType> {
        self.product_tree.as_ref()
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        None
    }

    fn visit_all_extensions(&self, _visitor: &mut impl FnMut(&Self::ExtensionsType, &str)) {}
}

impl CsafTrait for CommonSecurityAdvisoryFramework21 {
    type VulnerabilityType = Vulnerability21;
    type ProductTreeType = ProductTree21;
    type DocumentType = DocumentLevelMetaData21;
    type ExtensionsType = Extensions21;

    fn get_product_tree(&self) -> Option<&Self::ProductTreeType> {
        self.product_tree.as_ref()
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        self.x_extensions.as_ref()
    }

    fn visit_all_extensions(&self, visitor: &mut impl FnMut(&Self::ExtensionsType, &str)) {
        // $.x_extensions[*]
        if let Some(extensions) = self.get_extensions() {
            visitor(extensions, "/x_extensions");
        }

        // $.document.x_extensions[*]
        if let Some(extensions) = self.get_document().get_extensions() {
            visitor(extensions, "/document/x_extensions");
        }

        let vulnerabilities = self.get_vulnerabilities();

        for (i, vuln) in vulnerabilities.iter().enumerate() {
            // $.vulnerabilities[*].x_extensions[*]
            if let Some(extensions) = vuln.get_extensions() {
                visitor(extensions, &format!("/vulnerabilities/{i}/x_extensions"));
            }

            // $.vulnerabilities[*].metrics[*].content.x_extensions[*]
            if let Some(metrics) = vuln.get_metrics() {
                for (m_i, metric) in metrics.iter().enumerate() {
                    if let Some(extensions) = metric.get_content().get_extensions() {
                        visitor(
                            extensions,
                            &format!("/vulnerabilities/{i}/metrics/{m_i}/content/x_extensions"),
                        );
                    }
                }
            }
        }

        // $.product_tree..branches[*].product.x_extensions[*]
        // $.product_tree.full_product_names[*].x_extensions[*]
        // $.product_tree.product_paths[*].full_product_name.x_extensions[*]
        if let Some(product_tree) = self.get_product_tree() {
            product_tree.visit_all_products_generic(&mut |product, path| {
                if let Some(extensions) = product.get_extensions() {
                    visitor(extensions, &format!("{path}/x_extensions"));
                }
            });
        }
    }
}
