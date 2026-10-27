use crate::csaf::traits::document::csaf_21_involvement_trait::Csaf21InvolvementTrait;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::vulnerabilities_trait::collect_references;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{
    ActionTrait, ContentTrait, DocumentTrait, EpssTrait, ExtensionsTrait, FirstKnownExploitationDatesTrait,
    MetricTrait, ProductTrait, ProductTreeTrait, TrackingTrait, VulnerabilityTrait, WithDate, WithOptionalDate,
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
use std::collections::BTreeSet;

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

    /// Returns all document-local vulnerability IDs referenced by document involvement actions
    /// along with their JSON paths.
    fn get_all_document_local_vuln_id_references_with_path(&self) -> Vec<(String, String)> {
        let Some(involvement) = self.get_document().get_involvement() else {
            return Vec::new();
        };

        involvement
            .get_actions()
            .enumerate()
            .flat_map(|(action_index, action)| {
                action
                    .get_document_local_vuln_ids()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(move |(id_index, id)| {
                        (
                            id.to_owned(),
                            format!("/document/involvement/actions/{action_index}/dl_vuln_ids/{id_index}"),
                        )
                    })
            })
            .collect()
    }

    /// Returns all document-local vulnerability IDs defined by vulnerabilities in the document.
    fn get_all_document_local_vuln_ids(&self) -> BTreeSet<String> {
        self.get_vulnerabilities()
            .iter()
            .filter_map(VulnerabilityTrait::get_document_local_vuln_id)
            .map(ToOwned::to_owned)
            .collect()
    }

    fn all_vulnerabilities_have_document_local_vuln_ids(&self) -> bool {
        self.get_vulnerabilities()
            .iter()
            .all(|v| v.get_document_local_vuln_id().is_some())
    }

    /// Returns all document-local vulnerability ID definitions along with their JSON paths.
    fn get_all_document_local_vuln_ids_with_path(&self) -> Vec<(String, String)> {
        self.get_vulnerabilities()
            .iter()
            .enumerate()
            .filter_map(|(vulnerability_index, vulnerability)| {
                vulnerability.get_document_local_vuln_id().map(|id| {
                    (
                        id.to_owned(),
                        format!("/vulnerabilities/{vulnerability_index}/dl_vuln_id"),
                    )
                })
            })
            .collect()
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType>;

    fn get_all_dates(&self) -> Vec<(CsafDateTime, String)> {
        let mut dates = Vec::new();
        let document = self.get_document();

        let tracking = document.get_tracking();
        dates.push((
            tracking.get_initial_release_date(),
            "/document/tracking/initial_release_date".to_string(),
        ));
        dates.push((
            tracking.get_current_release_date(),
            "/document/tracking/current_release_date".to_string(),
        ));
        if let Some(generator) = tracking.get_generator()
            && let Some(date) = generator.get_date()
        {
            dates.push((date, "/document/tracking/generator/date".to_string()));
        }
        for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
            dates.push((
                revision.get_date().clone(),
                format!("/document/tracking/revision_history/{i_r}/date").to_string(),
            ));
        }
        if let Some(involvement) = document.get_involvement() {
            for (i_a, action) in involvement.get_actions().enumerate() {
                dates.push((
                    action.get_date().clone(),
                    format!("/document/involvement/actions/{i_a}/date").to_string(),
                ));
            }
        }

        let vulnerabilities = self.get_vulnerabilities();
        for (i_v, vuln) in vulnerabilities.iter().enumerate() {
            if let Some(date) = vuln.get_disclosure_date() {
                dates.push((
                    date.clone(),
                    format!("/vulnerabilities/{i_v}/{}", vuln.get_disclosure_date_property_name()).to_string(),
                ));
            }
            if let Some(date) = vuln.get_discovery_date() {
                dates.push((
                    date.clone(),
                    format!("/vulnerabilities/{i_v}/discovery_date").to_string(),
                ));
            }
            if let Some(flags) = vuln.get_flags() {
                for (i_f, flag) in flags.iter().enumerate() {
                    if let Some(date) = flag.get_date() {
                        dates.push((
                            date.clone(),
                            format!("/vulnerabilities/{i_v}/flags/{i_f}/date").to_string(),
                        ));
                    }
                }
            }
            if let Some(involvements) = vuln.get_involvements() {
                for (i_i, involvement) in involvements.iter().enumerate() {
                    if let Some(date) = involvement.get_date() {
                        dates.push((
                            date.clone(),
                            format!("/vulnerabilities/{i_v}/involvements/{i_i}/date").to_string(),
                        ));
                    }
                }
            }
            for (i_r, remediation) in vuln.get_remediations().iter().enumerate() {
                if let Some(date) = remediation.get_date() {
                    dates.push((
                        date.clone(),
                        format!("/vulnerabilities/{i_v}/remediations/{i_r}/date").to_string(),
                    ));
                }
            }
            for (i_t, threat) in vuln.get_threats().iter().enumerate() {
                if let Some(date) = threat.get_date() {
                    dates.push((
                        date.clone(),
                        format!("/vulnerabilities/{i_v}/threats/{i_t}/date").to_string(),
                    ));
                }
            }
            if let Some(first_known_exploitation_dates) = vuln.get_first_known_exploitation_dates() {
                for (i_d, date) in first_known_exploitation_dates.iter().enumerate() {
                    dates.push((
                        date.get_date().clone(),
                        format!("/vulnerabilities/{i_v}/first_known_exploitation_dates/{i_d}/date").to_string(),
                    ));
                    dates.push((
                        date.get_exploitation_date().clone(),
                        format!("/vulnerabilities/{i_v}/first_known_exploitation_dates/{i_d}/exploitation_date")
                            .to_string(),
                    ));
                }
            }
            if let Some(metrics) = vuln.get_metrics() {
                for (i_m, metric) in metrics.iter().enumerate() {
                    if let Some(epss) = metric.get_content().get_epss() {
                        dates.push((
                            epss.get_timestamp().clone(),
                            format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/epss/timestamp").to_string(),
                        ));
                    }
                    if let Some(ssvc_map) = metric.get_content().get_ssvc_v2_raw()
                        && let Some(timestamp) = ssvc_map.get("timestamp").and_then(serde_json::Value::as_str)
                    {
                        dates.push((
                            CsafDateTime::from(timestamp),
                            format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/timestamp").to_string(),
                        ))
                    }
                }
            }
        }

        dates
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
