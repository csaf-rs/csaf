use crate::csaf::traits::util::extract_references::{
    ExtractGroupReferences, ExtractProductReferences, define_reference_accessors,
};
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::vulnerabilities::product_status_trait::ProductStatusTrait;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{
    Cwe, FirstKnownExploitationDatesTrait, FlagTrait, InvolvementTrait, MetricTrait, NoteTrait, RemediationTrait,
    ThreatTrait, VulnerabilityIdTrait,
};
use crate::schema::csaf2_0::schema::{
    Flag as Flag20, Id as Id20, Involvement as Involvement20, Note as Note20, ProductStatus as ProductStatus20,
    Remediation as Remediation20, Score as Score20, Threat as Threat20, Vulnerability as Vulnerability20,
};
use crate::schema::csaf2_1::schema::{
    FirstKnownExploitationDate as FirstKnownExploitationDate21, Flag as Flag21, Id as Id21,
    Involvement as Involvement21, Metric as Metric21, Note as Note21, ProductStatus as ProductStatus21,
    Remediation as Remediation21, Threat as Threat21, Vulnerability as Vulnerability21,
};

/// Collects references from all vulnerabilities using the given extractor, prepending
/// each path with `/vulnerabilities/{index}/`.
pub(crate) fn collect_references<V: VulnerabilityTrait>(
    vulnerabilities: &[V],
    extractor: impl Fn(&V) -> Vec<(String, String)>,
) -> Vec<(String, String)> {
    vulnerabilities
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            extractor(v)
                .into_iter()
                .map(move |(id, path)| (id, format!("/vulnerabilities/{i}/{path}")))
        })
        .collect()
}

/// Trait representing an abstract vulnerability in a CSAF document.
///
/// The `VulnerabilityTrait` defines the structure of a vulnerability and includes
/// information about potential remediations.
pub trait VulnerabilityTrait {
    /// The associated type representing the type of remediations in a vulnerability.
    type RemediationType: RemediationTrait;

    /// The associated type representing the product status information.
    type ProductStatusType: ProductStatusTrait;

    /// The associated type representing the metric information.
    type MetricType: MetricTrait;

    /// The associated type representing the threat information.
    type ThreatType: ThreatTrait;

    /// The associated type representing a vulnerability flag.
    type FlagType: FlagTrait;

    /// The associated type representing a vulnerability involvement.
    type InvolvementType: InvolvementTrait;

    /// The associated type representing the vulnerability ID information.
    type VulnerabilityIdType: VulnerabilityIdTrait;

    /// The associated type representing vulnerability notes.
    type NoteType: NoteTrait;

    type FirstKnownExploitationDatesType: FirstKnownExploitationDatesTrait;

    /// Retrieves a list of remediations associated with the vulnerability.
    fn get_remediations(&self) -> &Vec<Self::RemediationType>;

    /// Retrieves the status of products affected by the vulnerability, if available.
    fn get_product_status(&self) -> &Option<Self::ProductStatusType>;

    fn get_product_status_product_references(&self) -> Vec<(String, String)> {
        if let Some(product_status) = self.get_product_status() {
            product_status.get_all_product_references()
        } else {
            Vec::new()
        }
    }

    /// Returns an optional vector of metrics related to the vulnerability.
    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>>;

    /// Return the path in the JSON document where the metrics are located, used for error reporting
    fn get_metrics_path(&self) -> String;

    /// Utility function to get all product IDs referenced in metrics along with their JSON paths
    fn get_metrics_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        if let Some(metrics) = self.get_metrics().as_ref() {
            let metric_path = self.get_metrics_path();
            for (metric_i, metric) in metrics.iter().enumerate() {
                for (x_i, x) in metric.get_products().enumerate() {
                    ids.push((x.to_owned(), format!("{metric_path}/{metric_i}/products/{x_i}")));
                }
            }
        }

        ids
    }

    /// Retrieves a list of potential threats related to the vulnerability.
    fn get_threats(&self) -> &Vec<Self::ThreatType>;

    /// Returns the date when this vulnerability was initially disclosed.
    fn get_disclosure_date(&self) -> Option<CsafDateTime>;

    /// Returns the date when this vulnerability was initially discovered.
    fn get_discovery_date(&self) -> Option<CsafDateTime>;

    /// Returns all flags associated with this vulnerability.
    fn get_flags(&self) -> Option<&Vec<Self::FlagType>>;

    /// Returns all involvements associated with this vulnerability.
    fn get_involvements(&self) -> Option<&Vec<Self::InvolvementType>>;

    /// Returns the CVE associated with the vulnerability.
    fn get_cve(&self) -> Option<&String>;

    /// Returns the CWE associated with the vulnerability.
    fn get_cwe(&self) -> Option<Vec<Cwe>>;

    /// Returns the vulnerability IDs associated with this vulnerability.
    fn get_ids(&self) -> Option<&Vec<Self::VulnerabilityIdType>>;

    /// Returns the notes associated with this vulnerability.
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;

    /// Returns the information about the first known exploitation dates of this vulnerability.
    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>>;

    define_reference_accessors! {
        both: [
            (get_remediations_group_references,                     get_remediations_product_references,                     get_remediations,                     "remediations"),
            (get_threats_group_references,                          get_threats_product_references,                          get_threats,                          "threats"),
            (get_flags_group_references,                            get_flags_product_references,                            get_flags,                            "flags"),
            (get_involvements_group_references,                     get_involvements_product_references,                     get_involvements,                     "involvements"),
            (get_notes_group_references,                            get_notes_product_references,                            get_notes,                            "notes"),
            (get_first_known_exploitation_dates_group_references,   get_first_known_exploitation_dates_product_references,   get_first_known_exploitation_dates,   "first_known_exploitation_dates"),
        ],
        custom_group_extraction: [],
        custom_product_extraction: [
            get_product_status_product_references,
            get_metrics_product_references,
        ],
    }
}

impl VulnerabilityTrait for Vulnerability20 {
    type RemediationType = Remediation20;
    type ProductStatusType = ProductStatus20;
    // Metrics are not implemented in CSAF 2.0
    type MetricType = Score20;
    type ThreatType = Threat20;
    type FlagType = Flag20;
    type InvolvementType = Involvement20;
    type VulnerabilityIdType = Id20;
    type NoteType = Note20;
    // First known exploitation dates are not implemented in CSAF 2.0
    type FirstKnownExploitationDatesType = NotPresentInCsaf20;

    fn get_remediations(&self) -> &Vec<Self::RemediationType> {
        &self.remediations
    }

    fn get_product_status(&self) -> &Option<Self::ProductStatusType> {
        &self.product_status
    }

    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>> {
        Some(&self.scores)
    }

    fn get_metrics_path(&self) -> String {
        "scores".to_string()
    }

    fn get_threats(&self) -> &Vec<Self::ThreatType> {
        &self.threats
    }

    fn get_disclosure_date(&self) -> Option<CsafDateTime> {
        self.release_date.as_ref().map(CsafDateTime::from)
    }

    fn get_discovery_date(&self) -> Option<CsafDateTime> {
        self.discovery_date.as_ref().map(CsafDateTime::from)
    }

    fn get_flags(&self) -> Option<&Vec<Self::FlagType>> {
        self.flags.as_ref()
    }

    fn get_involvements(&self) -> Option<&Vec<Self::InvolvementType>> {
        self.involvements.as_ref()
    }

    fn get_cve(&self) -> Option<&String> {
        self.cve.as_deref()
    }

    fn get_cwe(&self) -> Option<Vec<Cwe>> {
        self.cwe.as_ref().map(|cwe| vec![Cwe::from(cwe)])
    }

    fn get_ids(&self) -> Option<&Vec<Self::VulnerabilityIdType>> {
        self.ids.as_ref()
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_deref()
    }

    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>> {
        None
    }
}

impl VulnerabilityTrait for Vulnerability21 {
    type RemediationType = Remediation21;
    type ProductStatusType = ProductStatus21;
    type MetricType = Metric21;
    type ThreatType = Threat21;
    type FlagType = Flag21;
    type InvolvementType = Involvement21;
    type VulnerabilityIdType = Id21;
    type NoteType = Note21;
    type FirstKnownExploitationDatesType = FirstKnownExploitationDate21;

    fn get_remediations(&self) -> &Vec<Self::RemediationType> {
        &self.remediations
    }

    fn get_product_status(&self) -> &Option<Self::ProductStatusType> {
        &self.product_status
    }

    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>> {
        self.metrics.as_ref()
    }

    fn get_metrics_path(&self) -> String {
        "metrics".to_string()
    }

    fn get_threats(&self) -> &Vec<Self::ThreatType> {
        &self.threats
    }

    fn get_disclosure_date(&self) -> Option<CsafDateTime> {
        self.disclosure_date.as_ref().map(CsafDateTime::from)
    }

    fn get_discovery_date(&self) -> Option<CsafDateTime> {
        self.discovery_date.as_ref().map(CsafDateTime::from)
    }

    fn get_flags(&self) -> Option<&Vec<Self::FlagType>> {
        self.flags.as_ref()
    }

    fn get_involvements(&self) -> Option<&Vec<Self::InvolvementType>> {
        self.involvements.as_ref()
    }

    fn get_cve(&self) -> Option<&String> {
        self.cve.as_deref()
    }

    fn get_cwe(&self) -> Option<Vec<Cwe>> {
        self.cwes.as_ref().map(|cwes| cwes.iter().map(Cwe::from).collect())
    }

    fn get_ids(&self) -> Option<&Vec<Self::VulnerabilityIdType>> {
        self.ids.as_ref()
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_deref()
    }

    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>> {
        self.first_known_exploitation_dates.as_ref()
    }
}
