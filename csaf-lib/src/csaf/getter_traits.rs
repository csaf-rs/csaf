use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::helpers::resolve_product_groups;
use std::collections::BTreeSet;

/// Trait representing an abstract Common Security Advisory Framework (CSAF) document.
///
/// The `CsafTrait` trait defines the key structure of a CSAF document, allowing
/// interaction with its vulnerabilities and product tree without tying to a
/// specific version of the CSAF schema.
pub trait CsafTrait {
    /// The associated type representing the type of vulnerabilities in this CSAF structure.
    type VulnerabilityType: VulnerabilityTrait;

    /// The associated type representing the type of product tree in this CSAF structure.
    type ProductTreeType: ProductTreeTrait;

    /// Returns the product tree of the CSAF document.
    fn get_product_tree(&self) -> Option<Self::ProductTreeType>;

    /// Retrieves all vulnerabilities in the CSAF document.
    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType>;
}

/// Trait representing an abstract vulnerability in a CSAF document.
///
/// The `VulnerabilityTrait` defines the structure of a vulnerability and includes
/// information about potential remediations.
pub trait VulnerabilityTrait {
    /// The associated type representing the type of remediations in a vulnerability.
    type RemediationType: RemediationTrait;

    /// Retrieves all remediations associated with the vulnerability.
    fn get_remediations(&self) -> Vec<Self::RemediationType>;
}

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait {
    /// Returns the category of the remediation.
    ///
    /// Categories are defined by the CSAF schema.
    fn get_category(&self) -> CategoryOfTheRemediation;

    /// Retrieves the product IDs directly affected by this remediation, if any.
    fn get_product_ids(&self) -> Option<Vec<&String>>;

    /// Retrieves the product group IDs related to this remediation, if any.
    fn get_group_ids(&self) -> Option<Vec<&String>>;

    /// Computes a set of all product IDs affected by this remediation, either
    /// directly or through product groups.
    ///
    /// # Arguments
    ///
    /// * `doc` - A reference to the CSAF document to resolve product groups.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<String>` containing all product IDs, or `None` if none exist.
    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<BTreeSet<String>> {
        if self.get_product_ids().is_none() && self.get_group_ids().is_none() {
            None
        } else {
            let mut product_set: BTreeSet<String> = match self.get_product_ids() {
                Some(product_ids) => product_ids.iter().map(|p| p.to_string()).collect(),
                None => BTreeSet::new(),
            };
            if let Some(product_groups) = self.get_group_ids() {
                if let Some(product_ids) = resolve_product_groups(doc, product_groups) {
                    product_set.extend(product_ids.iter().map(|p| p.to_string()));
                }
            }
            Some(product_set)
        }
    }
}

/// Trait representing an abstract product tree in a CSAF document.
///
/// The `ProductTreeTrait` defines the structure of a product tree and allows
/// access to its product groups.
pub trait ProductTreeTrait {
    /// The associated type representing the type of product groups in the product tree.
    type ProductGroupType: ProductGroupTrait;

    /// Retrieves all product groups in the product tree.
    fn get_product_groups(&self) -> Vec<Self::ProductGroupType>;
}

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Retrieves the group ID of the product group.
    fn get_group_id(&self) -> &String;

    /// Retrieves the product IDs contained within the product group.
    fn get_product_ids(&self) -> Vec<&String>;
}