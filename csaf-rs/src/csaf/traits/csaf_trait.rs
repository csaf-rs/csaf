use crate::csaf::traits::document::csaf_21_involvement_trait::Csaf21InvolvementTrait;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{
    ActionTrait, ContentTrait, DocumentTrait, EpssTrait, ExtensionsTrait, FirstKnownExploitationDatesTrait,
    MetricTrait, ProductTrait, ProductTreeTrait, TrackingTrait, VulnerabilityTrait, WithDate, WithOptionalDate,
};
use crate::json::JsonPointer;
use crate::schema::csaf2_0::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework20,
    DocumentLevelMetaData as DocumentLevelMetaData20, ProductTree as ProductTree20, Vulnerability as Vulnerability20,
};
use crate::schema::csaf2_1::schema::{
    CommonSecurityAdvisoryFramework as CommonSecurityAdvisoryFramework21,
    DocumentLevelMetaData as DocumentLevelMetaData21, EntityRefsTItem, ExtensionsT as Extensions21,
    ProductTree as ProductTree21, Vulnerability as Vulnerability21,
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

    /// Visits every group reference with its absolute JSON pointer.
    fn visit_all_group_references(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::new();
        let mut path = JsonPointer::new(&mut buffer);
        {
            let mut document_path = path.push("document");
            self.get_document()
                .visit_all_group_references(&mut document_path, visitor);
            if let Some(involvement) = self.get_document().get_involvement() {
                involvement.visit_all_group_references(&mut document_path.push("involvement"), visitor);
            }
        }
        let mut vulnerabilities_path = path.push("vulnerabilities");
        for (index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
            vulnerability.visit_all_group_references(&mut vulnerabilities_path.push(index), visitor);
        }
    }

    /// Visits every product reference with its absolute JSON pointer.
    fn visit_all_product_references(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::new();
        let mut path = JsonPointer::new(&mut buffer);
        {
            let mut document_path = path.push("document");
            self.get_document()
                .visit_all_product_references(&mut document_path, visitor);
            if let Some(involvement) = self.get_document().get_involvement() {
                involvement.visit_all_product_references(&mut document_path.push("involvement"), visitor);
            }
        }
        {
            let mut vulnerabilities_path = path.push("vulnerabilities");
            for (index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
                vulnerability.visit_all_product_references(&mut vulnerabilities_path.push(index), visitor);
            }
        }
        if let Some(pt) = self.get_product_tree() {
            pt.visit_all_product_references(&mut path.push("product_tree"), visitor);
        }
    }

    /// Visits document-local vulnerability references in involvement actions.
    fn visit_all_document_local_vuln_id_references(&self, visitor: &mut impl FnMut(&str, &str)) {
        let Some(involvement) = self.get_document().get_involvement() else {
            return;
        };
        let mut buffer = String::from("/document/involvement/actions");
        let mut path = JsonPointer::new(&mut buffer);
        for (action_index, action) in involvement.get_actions().enumerate() {
            let mut action_path = path.push(action_index);
            let mut ids_path = action_path.push("dl_vuln_ids");
            for (id_index, id) in action.get_document_local_vuln_ids().into_iter().flatten().enumerate() {
                visitor(id, ids_path.push(id_index).as_str());
            }
        }
    }

    fn all_vulnerabilities_have_document_local_vuln_ids(&self) -> bool {
        self.get_vulnerabilities()
            .iter()
            .all(|v| v.get_document_local_vuln_id().is_some())
    }

    /// Visits all document-local vulnerability ID definitions, including duplicates.
    fn visit_all_document_local_vuln_ids(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::from("/vulnerabilities");
        let mut path = JsonPointer::new(&mut buffer);
        for (index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
            if let Some(id) = vulnerability.get_document_local_vuln_id() {
                let mut vulnerability_path = path.push(index);
                visitor(id, vulnerability_path.push("dl_vuln_id").as_str());
            }
        }
    }

    /// Visits entity-group references in acting and receiving entity lists.
    fn visit_all_entity_group_id_references(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::from("/document/involvement/actions");
        let mut path = JsonPointer::new(&mut buffer);
        if let Some(involvement) = self.get_document().get_involvement() {
            for (action_index, action) in involvement.get_actions().enumerate() {
                let mut action_path = path.push(action_index);
                {
                    let mut refs_path = action_path.push("acting_entity_refs");
                    for (index, entity_ref) in action.get_acting_entity_refs().enumerate() {
                        if let EntityRefsTItem::GroupIdT(group_id) = entity_ref {
                            visitor(group_id.as_str(), refs_path.push(index).as_str());
                        }
                    }
                }
                if let Some(entity_refs) = action.get_receiving_entity_refs() {
                    let mut refs_path = action_path.push("receiving_entity_refs");
                    for (index, entity_ref) in entity_refs.enumerate() {
                        if let EntityRefsTItem::GroupIdT(group_id) = entity_ref {
                            visitor(group_id.as_str(), refs_path.push(index).as_str());
                        }
                    }
                }
            }
        }
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType>;

    /// Visits every date with its absolute JSON pointer, preserving document order.
    fn visit_all_dates(&self, visitor: &mut impl FnMut(&CsafDateTime, &str)) {
        let mut buffer = String::new();
        let mut path = JsonPointer::new(&mut buffer);
        let document = self.get_document();

        let tracking = document.get_tracking();
        {
            let mut document_path = path.push("document");
            {
                let mut tracking_path = document_path.push("tracking");
                visitor(
                    &tracking.get_initial_release_date(),
                    tracking_path.push("initial_release_date").as_str(),
                );
                visitor(
                    &tracking.get_current_release_date(),
                    tracking_path.push("current_release_date").as_str(),
                );
                if let Some(generator) = tracking.get_generator()
                    && let Some(date) = generator.get_date()
                {
                    let mut generator_path = tracking_path.push("generator");
                    visitor(&date, generator_path.push("date").as_str());
                }
                let mut revisions_path = tracking_path.push("revision_history");
                for (index, revision) in tracking.get_revision_history().iter().enumerate() {
                    let mut revision_path = revisions_path.push(index);
                    visitor(&revision.get_date(), revision_path.push("date").as_str());
                }
            }
            if let Some(involvement) = document.get_involvement() {
                let mut involvement_path = document_path.push("involvement");
                let mut actions_path = involvement_path.push("actions");
                for (index, action) in involvement.get_actions().enumerate() {
                    let mut action_path = actions_path.push(index);
                    visitor(&action.get_date(), action_path.push("date").as_str());
                }
            }
        }

        let mut vulnerabilities_path = path.push("vulnerabilities");
        for (index, vuln) in self.get_vulnerabilities().iter().enumerate() {
            let mut vulnerability_path = vulnerabilities_path.push(index);
            if let Some(date) = vuln.get_disclosure_date() {
                visitor(
                    &date,
                    vulnerability_path
                        .push(vuln.get_disclosure_date_property_name())
                        .as_str(),
                );
            }
            if let Some(date) = vuln.get_discovery_date() {
                visitor(&date, vulnerability_path.push("discovery_date").as_str());
            }
            if let Some(flags) = vuln.get_flags() {
                let mut flags_path = vulnerability_path.push("flags");
                for (index, flag) in flags.iter().enumerate() {
                    if let Some(date) = flag.get_date() {
                        let mut flag_path = flags_path.push(index);
                        visitor(&date, flag_path.push("date").as_str());
                    }
                }
            }
            if let Some(involvements) = vuln.get_involvements() {
                let mut involvements_path = vulnerability_path.push("involvements");
                for (index, involvement) in involvements.iter().enumerate() {
                    if let Some(date) = involvement.get_date() {
                        let mut involvement_path = involvements_path.push(index);
                        visitor(&date, involvement_path.push("date").as_str());
                    }
                }
            }
            {
                let mut remediations_path = vulnerability_path.push("remediations");
                for (index, remediation) in vuln.get_remediations().iter().enumerate() {
                    if let Some(date) = remediation.get_date() {
                        let mut remediation_path = remediations_path.push(index);
                        visitor(&date, remediation_path.push("date").as_str());
                    }
                }
            }
            {
                let mut threats_path = vulnerability_path.push("threats");
                for (index, threat) in vuln.get_threats().iter().enumerate() {
                    if let Some(date) = threat.get_date() {
                        let mut threat_path = threats_path.push(index);
                        visitor(&date, threat_path.push("date").as_str());
                    }
                }
            }
            if let Some(first_known_exploitation_dates) = vuln.get_first_known_exploitation_dates() {
                let mut dates_path = vulnerability_path.push("first_known_exploitation_dates");
                for (index, date) in first_known_exploitation_dates.iter().enumerate() {
                    let mut date_path = dates_path.push(index);
                    visitor(&date.get_date(), date_path.push("date").as_str());
                    visitor(
                        &date.get_exploitation_date(),
                        date_path.push("exploitation_date").as_str(),
                    );
                }
            }
            if let Some(metrics) = vuln.get_metrics() {
                let mut metrics_path = vulnerability_path.push(vuln.get_metrics_prop_name());
                for (index, metric) in metrics.iter().enumerate() {
                    let mut metric_path = metrics_path.push(index);
                    let mut content_path = metric_path.push("content");
                    if let Some(epss) = metric.get_content().get_epss() {
                        let mut epss_path = content_path.push("epss");
                        visitor(&epss.get_timestamp(), epss_path.push("timestamp").as_str());
                    }
                    if let Some(ssvc_map) = metric.get_content().get_ssvc_v2_raw()
                        && let Some(timestamp) = ssvc_map.get("timestamp").and_then(serde_json::Value::as_str)
                    {
                        let mut ssvc_path = content_path.push("ssvc_v2");
                        visitor(&CsafDateTime::from(timestamp), ssvc_path.push("timestamp").as_str());
                    }
                }
            }
        }
    }

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
