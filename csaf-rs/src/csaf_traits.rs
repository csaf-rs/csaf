use crate::csaf2_1::schema::{
    CategoryOfPublisher, CategoryOfReference, CategoryOfTheRemediation, DocumentStatus, Epss, LabelOfTheFlag,
    LabelOfTlp, NoteCategory, PartyCategory,
};
use crate::csaf2_1::ssvc_dp_selection_list::SelectionList;
use crate::helpers::resolve_product_groups;
use crate::product_helpers::prepend_path;
use crate::validation::ValidationError;
use semver::Version;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

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

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType>;

    /// Utility function to get all group IDs referenced in vulnerabilities along with their JSON paths
    fn get_vulnerability_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (vuln_index, vulnerability) in self.get_vulnerabilities().iter().enumerate() {
            let prefix_string = "vulnerabilities";
            ids.append(&mut prepend_path(
                prefix_string,
                &vuln_index,
                vulnerability.get_flags_group_references(),
            ));
            ids.append(&mut prepend_path(
                prefix_string,
                &vuln_index,
                vulnerability.get_involvement_group_references(),
            ));
            ids.append(&mut prepend_path(
                prefix_string,
                &vuln_index,
                vulnerability.get_notes_group_references(),
            ));
            ids.append(&mut prepend_path(
                prefix_string,
                &vuln_index,
                vulnerability.get_remediations_group_references(),
            ));
            ids.append(&mut prepend_path(
                prefix_string,
                &vuln_index,
                vulnerability.get_threats_group_references(),
            ));
        }
        ids
    }

    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> &Self::DocumentType;

    /// Utility function to get all group IDs referenced in the document along with their JSON paths
    fn get_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        ids.append(&mut self.get_document().get_notes_group_references());
        ids.append(&mut self.get_vulnerability_group_references());
        ids
    }
}

/// Trait representing document meta-level information
pub trait DocumentTrait {
    /// Type representing document tracking information
    type TrackingType: TrackingTrait;

    /// Type representing document distribution information
    type DistributionType: DistributionTrait;

    /// Type representing document notes
    type NoteType: NoteTrait;

    /// Type representing document publisher information
    type PublisherType: PublisherTrait;

    type DocumentReferenceType: DocumentReferenceTrait;

    /// Returns the tracking information for this document
    fn get_tracking(&self) -> &Self::TrackingType;

    /// Returns the distribution information for this document with CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError>;

    /// Returns the distribution information for this document with CSAF 2.0 semantics
    fn get_distribution_20(&self) -> Option<&Self::DistributionType>;

    /// Returns the notes associated with this document
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;

    /// Utility function to get all group IDs referenced in notes along with their JSON paths
    fn get_notes_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        if let Some(notes) = self.get_notes() {
            for (note_index, note) in notes.iter().enumerate() {
                if let Some(group_ids) = note.get_group_ids() {
                    for (group_index, group_id) in group_ids.enumerate() {
                        ids.push((
                            group_id.to_owned(),
                            format!("/document/notes/{}/group_ids/{}", note_index, group_index),
                        ))
                    }
                }
            }
        }
        ids
    }

    /// Returns the language associated with this document.
    fn get_lang(&self) -> Option<&String>;

    /// Returns the source language associated with this document.
    fn get_source_lang(&self) -> Option<&String>;

    /// Returns the publisher information for this document
    fn get_publisher(&self) -> &Self::PublisherType;

    /// Returns the category of the document as a string
    fn get_category_string(&self) -> &String;

    /// Returns the category of the document as an enum
    fn get_category(&self) -> DocumentCategory {
        DocumentCategory::from_string(self.get_category_string())
    }

    /// Returns the references of this document
    fn get_references(&self) -> Option<&Vec<Self::DocumentReferenceType>>;

    fn get_csaf_version(&self) -> &CsafVersion;
}

/// Enum representing CSAF versions
///
/// Contrary to other enums that are based on enums in the generated schemas, we are re-defining
/// this enum in the trait. Each schema only contains an enum with "their" version, and merging them
/// would be more complex then defining here and mapping in the implementations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsafVersion {
    X20,
    X21,
}

/// Trait representing document references
pub trait DocumentReferenceTrait {
    // Returns the category of the document reference as enum
    fn get_category(&self) -> &CategoryOfReference;
    // Returns the summary of the document reference
    fn get_summary(&self) -> &String;
    // Returns the URL of the document reference
    fn get_url(&self) -> &String;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocumentCategory {
    CsafInformationalAdvisory,
    CsafSecurityIncidentResponse,
    CsafSecurityAdvisory,
    CsafVex,
    Other(String),
    // These categories are only mentioned in CSAF 2.1, but as this is just a string wrapper used
    // for syntactic sugar, we don't need to make this distinction here
    CsafWithdrawn,
    CsafSuperseded,
    CsafDeprecatedSecurityAdvisory,
}

impl DocumentCategory {
    pub fn from_string(category: &str) -> Self {
        match category {
            "csaf_informational_advisory" => DocumentCategory::CsafInformationalAdvisory,
            "csaf_security_incident_response" => DocumentCategory::CsafSecurityIncidentResponse,
            "csaf_security_advisory" => DocumentCategory::CsafSecurityAdvisory,
            "csaf_vex" => DocumentCategory::CsafVex,
            "csaf_deprecated_security_advisory" => DocumentCategory::CsafDeprecatedSecurityAdvisory,
            "csaf_withdrawn" => DocumentCategory::CsafWithdrawn,
            "csaf_superseded" => DocumentCategory::CsafSuperseded,
            _ => DocumentCategory::Other("_".to_string()),
        }
    }
}

impl Display for DocumentCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DocumentCategory::CsafInformationalAdvisory => write!(f, "csaf_informational_advisory"),
            DocumentCategory::CsafSecurityIncidentResponse => write!(f, "csaf_security_incident_response"),
            DocumentCategory::CsafSecurityAdvisory => write!(f, "csaf_security_advisory"),
            DocumentCategory::CsafVex => write!(f, "csaf_vex"),
            DocumentCategory::CsafDeprecatedSecurityAdvisory => write!(f, "csaf_deprecated_security_advisory"),
            DocumentCategory::CsafWithdrawn => write!(f, "csaf_withdrawn"),
            DocumentCategory::CsafSuperseded => write!(f, "csaf_superseded"),
            DocumentCategory::Other(other) => write!(f, "{}", other),
        }
    }
}

pub trait PublisherTrait {
    fn get_category(&self) -> CategoryOfPublisher;
}

/// Trait representing distribution information for a document
pub trait DistributionTrait {
    /// Type representing sharing group information
    type SharingGroupType: SharingGroupTrait;

    /// Type representing TLP (Traffic Light Protocol) information
    type TlpType: TlpTrait;

    /// Returns the sharing group for this distribution
    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType>;

    /// Returns the TLP information for this distribution with CSAF 2.0 semantics
    fn get_tlp_20(&self) -> Option<&Self::TlpType>;

    /// Returns the TLP information for this distribution with CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError>;
}

pub trait NoteTrait: WithGroupIds + WithProductIds {
    fn get_category(&self) -> NoteCategory;
}

/// Trait representing sharing group information
pub trait SharingGroupTrait {
    /// Returns the ID of the sharing group
    fn get_id(&self) -> &Uuid;

    /// Returns the optional name of the sharing group
    fn get_name(&self) -> Option<&String>;
}

/// Trait representing TLP (Traffic Light Protocol) information
pub trait TlpTrait {
    /// Returns the TLP label
    fn get_label(&self) -> LabelOfTlp;
}

pub trait TrackingTrait {
    /// Type representing document generator information
    type GeneratorType: GeneratorTrait;

    /// Type representing revision history entries
    type RevisionType: RevisionTrait;

    /// The release date of this document's latest version
    fn get_current_release_date(&self) -> &String;

    /// The initial release date of this document
    fn get_initial_release_date(&self) -> &String;

    /// Returns the generator information for this document
    fn get_generator(&self) -> &Option<Self::GeneratorType>;

    /// Returns the revision history for this document
    fn get_revision_history(&self) -> &Vec<Self::RevisionType>;

    /// Returns the status of this document
    fn get_status(&self) -> DocumentStatus;

    /// Returns the tracking ID of this document
    fn get_id(&self) -> &String;

    /// Returns the version of this document
    fn get_version_string(&self) -> &String;

    fn get_version(&self) -> VersionNumber {
        VersionNumber::from_number(self.get_version_string())
    }
}

#[derive(Debug, Clone, Eq)]
pub enum VersionNumber {
    Integer(u64),
    Semver(Version),
}

impl VersionNumber {
    /// Parses a string to either intver or semver
    /// Will panic if not parseable
    pub fn from_number(number: &str) -> Self {
        if let Ok(number) = number.parse::<u64>() {
            return VersionNumber::Integer(number);
        } else if let Ok(number) = Version::parse(number) {
            return VersionNumber::Semver(number);
        }
        panic!("Version could not be parsed as intver or semver")
    }

    /// Gets the version number for intver / the major version for semver
    pub fn get_major(&self) -> u64 {
        match self {
            VersionNumber::Integer(num) => *num,
            VersionNumber::Semver(semver) => semver.major,
        }
    }
}

impl PartialEq for VersionNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VersionNumber::Integer(a), VersionNumber::Integer(b)) => a == b,
            (VersionNumber::Semver(a), VersionNumber::Semver(b)) => a == b,
            // Integer and Semver are always unequal
            (VersionNumber::Integer(_), VersionNumber::Semver(_)) => false,
            (VersionNumber::Semver(_), VersionNumber::Integer(_)) => false,
        }
    }
}

impl Display for VersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            VersionNumber::Integer(num) => write!(f, "{}", num),
            VersionNumber::Semver(version) => write!(f, "{}", version),
        }
    }
}

impl PartialOrd for VersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (VersionNumber::Integer(a), VersionNumber::Integer(b)) => a.cmp(b),
            (VersionNumber::Semver(a), VersionNumber::Semver(b)) => a.cmp(b),
            // Panic if intver and semver are compared against each other
            (VersionNumber::Integer(a), VersionNumber::Semver(b)) => {
                panic!(
                    "While comparing versions, you tried to compare integer versioning {} and semantic versioning {}",
                    a, b
                )
            },
            (VersionNumber::Semver(a), VersionNumber::Integer(b)) => {
                panic!(
                    "While comparing versions, you tried to compare integer versioning {} and semantic versioning {}",
                    b, a
                )
            },
        }
    }
}

/// Trait for accessing document generator information
pub trait GeneratorTrait {
    /// Returns the date when this document was generated
    fn get_date(&self) -> &Option<String>;
}

/// Trait for accessing revision history entry information
pub trait RevisionTrait {
    /// Returns the date associated with this revision entry
    fn get_date(&self) -> &String;

    /// Returns the number/identifier of this revision
    fn get_number_string(&self) -> &String;

    fn get_number(&self) -> VersionNumber {
        VersionNumber::from_number(self.get_number_string())
    }

    /// Returns the summary of changes in this revision
    fn get_summary(&self) -> &String;
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

    /// Utility function to get all group IDs referenced in remediations along with their JSON paths
    fn get_remediations_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        for (remediation_index, remediation) in self.get_remediations().iter().enumerate() {
            if let Some(group_ids) = remediation.get_group_ids() {
                for (group_index, group_id) in group_ids.enumerate() {
                    ids.push((
                        group_id.to_owned(),
                        format!("remediations/{}/group_ids/{}", remediation_index, group_index),
                    ))
                }
            }
        }
        ids
    }

    /// Retrieves the status of products affected by the vulnerability, if available.
    fn get_product_status(&self) -> &Option<Self::ProductStatusType>;

    /// Returns an optional vector of metrics related to the vulnerability.
    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>>;

    /// Retrieves a list of potential threats related to the vulnerability.
    fn get_threats(&self) -> &Vec<Self::ThreatType>;

    /// Utility function to get all group IDs referenced in threats along with their JSON paths
    fn get_threats_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        for (threat_index, threat) in self.get_threats().iter().enumerate() {
            if let Some(group_ids) = threat.get_group_ids() {
                for (group_index, group_id) in group_ids.enumerate() {
                    ids.push((
                        group_id.to_owned(),
                        format!("threats/{}/group_ids/{}", threat_index, group_index),
                    ))
                }
            }
        }
        ids
    }

    /// Returns the date when this vulnerability was initially disclosed.
    fn get_disclosure_date(&self) -> &Option<String>;

    /// Returns the date when this vulnerability was initially discovered.
    fn get_discovery_date(&self) -> &Option<String>;

    /// Returns all flags associated with this vulnerability.
    fn get_flags(&self) -> &Option<Vec<Self::FlagType>>;

    /// Utility function to get all group IDs referenced in flags along with their JSON paths
    fn get_flags_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        if let Some(flags) = self.get_flags() {
            for (flag_index, flag) in flags.iter().enumerate() {
                if let Some(group_ids) = flag.get_group_ids() {
                    for (group_index, group_id) in group_ids.enumerate() {
                        ids.push((
                            group_id.to_owned(),
                            format!("flags/{}/group_ids/{}", flag_index, group_index),
                        ))
                    }
                }
            }
        }
        ids
    }

    /// Returns all involvements associated with this vulnerability.
    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>>;

    /// Utility function to get all group IDs referenced in involvements along with their JSON paths
    fn get_involvement_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        if let Some(involvements) = self.get_involvements() {
            for (involvement_index, involvement) in involvements.iter().enumerate() {
                if let Some(group_ids) = involvement.get_group_ids() {
                    for (group_index, group_id) in group_ids.enumerate() {
                        ids.push((
                            group_id.to_owned(),
                            format!("involvements/{}/group_ids/{}", involvement_index, group_index),
                        ))
                    }
                }
            }
        }
        ids
    }

    /// Returns the CVE associated with the vulnerability.
    fn get_cve(&self) -> Option<&String>;

    /// Returns the vulnerability IDs associated with this vulnerability.
    fn get_ids(&self) -> &Option<Vec<Self::VulnerabilityIdType>>;

    /// Returns the notes associated with this vulnerability.
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;

    /// Utility function to get all group IDs referenced in notes along with their JSON paths
    fn get_notes_group_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        if let Some(notes) = self.get_notes() {
            for (note_index, note) in notes.iter().enumerate() {
                if let Some(group_ids) = note.get_group_ids() {
                    for (group_index, group_id) in group_ids.enumerate() {
                        ids.push((
                            group_id.to_owned(),
                            format!("notes/{}/group_ids/{}", note_index, group_index),
                        ))
                    }
                }
            }
        }
        ids
    }

    /// Returns the information about the first known exploitation dates of this vulnerability.
    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>>;
}

pub trait VulnerabilityIdTrait {
    fn get_system_name(&self) -> &String;

    fn get_text(&self) -> &String;
}

/// Trait for accessing vulnerability flags information
pub trait FlagTrait: WithGroupIds + WithProductIds {
    /// Returns the date associated with this vulnerability flag
    fn get_date(&self) -> &Option<String>;

    /// Returns the label of the vulnerability flag
    fn get_label(&self) -> LabelOfTheFlag;
}

pub trait FirstKnownExploitationDatesTrait {
    fn get_date(&self) -> &String;
}

/// Trait for accessing vulnerability involvement information
pub trait InvolvementTrait: WithGroupIds {
    /// Returns the date associated with this vulnerability involvement
    fn get_date(&self) -> &Option<String>;

    /// Returns the party associated with this vulnerability involvement
    fn get_party(&self) -> PartyCategory;
}

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait: WithGroupIds + WithProductIds {
    /// Returns the category of the remediation.
    ///
    /// Categories are defined by the CSAF schema.
    fn get_category(&self) -> CategoryOfTheRemediation;

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
                Some(product_ids) => product_ids.map(|id| (*id).to_owned()).collect(),
                None => BTreeSet::new(),
            };
            if let Some(product_groups) = self.get_group_ids() {
                if let Some(product_ids) = resolve_product_groups(doc, product_groups) {
                    product_set.extend(product_ids.iter().map(|id| id.to_owned()));
                }
            }
            Some(product_set)
        }
    }

    /// Returns the date associated with this remediation
    fn get_date(&self) -> &Option<String>;
}

/// Enum representing product status groups
#[derive(PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum ProductStatusGroup {
    // first_affected, known_affected, last_affected
    Affected,
    // known_not_affected
    NotAffected,
    // first_fixed, fixed
    Fixed,
    // under_investigation
    UnderInvestigation,
    // unknown
    Unknown,
    // recommended
    Recommended,
}

impl Display for ProductStatusGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatusGroup::Affected => write!(f, "affected"),
            ProductStatusGroup::NotAffected => write!(f, "not affected"),
            ProductStatusGroup::Fixed => write!(f, "fixed"),
            ProductStatusGroup::UnderInvestigation => write!(f, "under investigation"),
            ProductStatusGroup::Unknown => write!(f, "unknown"),
            ProductStatusGroup::Recommended => write!(f, "recommended"),
        }
    }
}

/// Helper macro to add product status groups to a HashMap
macro_rules! add_product_status {
    ($result:ident, $status_group:expr, $getter:expr) => {
        if let Some(products) = $getter {
            $result
                .entry($status_group)
                .or_insert_with(HashSet::new)
                .extend(products);
        }
    };
}

/// Trait representing an abstract product status in a CSAF document.
pub trait ProductStatusTrait {
    /// Returns a reference to the list of first affected product IDs.
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of first fixed product IDs.
    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of fixed product IDs.
    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known affected product IDs.
    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known not-affected product IDs.
    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of last affected product IDs.
    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of recommended product IDs.
    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of product IDs currently under investigation.
    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Return a reference to the list of product IDs with unknown status.
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a `HashMap` containing all product IDs grouped by their statuses.
    fn get_all_by_product_status(&self) -> HashMap<ProductStatusGroup, HashSet<&String>> {
        let mut result: HashMap<ProductStatusGroup, HashSet<&String>> = HashMap::new();

        // affected
        add_product_status!(result, ProductStatusGroup::Affected, self.get_first_affected());
        add_product_status!(result, ProductStatusGroup::Affected, self.get_last_affected());
        add_product_status!(result, ProductStatusGroup::Affected, self.get_known_affected());

        // not affected
        add_product_status!(result, ProductStatusGroup::NotAffected, self.get_known_not_affected());

        // fixed
        add_product_status!(result, ProductStatusGroup::Fixed, self.get_fixed());
        add_product_status!(result, ProductStatusGroup::Fixed, self.get_first_fixed());

        // under investigation
        add_product_status!(
            result,
            ProductStatusGroup::UnderInvestigation,
            self.get_under_investigation()
        );

        // unknown
        add_product_status!(result, ProductStatusGroup::Unknown, self.get_unknown());

        // recommended
        add_product_status!(result, ProductStatusGroup::Recommended, self.get_recommended());

        result
    }
}

/// Trait representing an abstract metric in a CSAF document.
pub trait MetricTrait {
    type ContentType: ContentTrait;

    /// Retrieves an iterator over product IDs associated with this metric.
    fn get_products(&self) -> impl Iterator<Item = &String> + '_;

    /// Retrieves the "content" (i.e., actual metrics) of this metric.
    fn get_content(&self) -> &Self::ContentType;

    /// Retrieves the "source" (i.e., description of the metrics' origin) of this metric.
    fn get_source(&self) -> &Option<String>;
}

/// Trait representing a "content holder" for actual metrics inside a "metric" object.
pub trait ContentTrait {
    /// Returns whether this content contains a non-empty SSVC metric.
    fn has_ssvc(&self) -> bool;

    /// Returns a parsed instance of the contained SSVC metric, or a `serde_json::Error`,
    /// encapsulated as a `Result`.
    fn get_ssvc(&self) -> Result<SelectionList, serde_json::Error>;

    /// Returns a JSON representation of the contained CVSS 2.0 metric, if any.
    fn get_cvss_v2(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    /// Returns a JSON representation of the contained CVSS 3.0/3.1 metric, if any.
    fn get_cvss_v3(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    /// Returns a JSON representation of the contained CVSS 4.0 metric, if any.
    fn get_cvss_v4(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    /// Returns a reference to the contained EPSS metric if it exists.
    fn get_epss(&self) -> &Option<Epss>;

    /// This function constructs a JSON path string that can be used to locate the specific
    /// content object within a CSAF document's JSON structure. The path format varies between
    /// CSAF versions due to structural differences in how metrics and content are organized.
    ///
    /// # Parameters
    ///
    /// * `vulnerability_idx` - The zero-based index of the vulnerability in the document's
    ///   vulnerability array
    /// * `metric_idx` - The zero-based index of the metric within the vulnerability's metrics array
    ///
    /// # Returns
    ///
    /// A `String` containing the JSON path to the content object, formatted according to the
    /// appropriate CSAF version specification. The path can be used for validation error reporting,
    /// debugging, or programmatic access to the content location within the document.
    ///
    /// # Examples
    ///
    /// For CSAF 2.0, the path might look like:
    /// `/vulnerabilities/0/scores/0`
    ///
    /// For CSAF 2.1, the path might look like:
    /// `/vulnerabilities/0/metrics/0/content`
    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String;
}

/// Types of vulnerability metrics known until CSAF 2.1
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum VulnerabilityMetric {
    SsvcV1,
    CvssV2,
    CvssV3(String),
    CvssV4,
    Epss,
}

/// Display implementation for VulnerabilityMetrics.
impl Display for VulnerabilityMetric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VulnerabilityMetric::SsvcV1 => write!(f, "SSVC-v1"),
            VulnerabilityMetric::CvssV2 => write!(f, "CVSS-v2"),
            VulnerabilityMetric::CvssV3(version) => write!(f, "CVSS-v{}", *version),
            VulnerabilityMetric::CvssV4 => write!(f, "CVSS-v4"),
            VulnerabilityMetric::Epss => write!(f, "EPSS"),
        }
    }
}

/// Returns the name of the metric property for the given metric type.
pub fn get_metric_prop_name(metric: VulnerabilityMetric) -> &'static str {
    match metric {
        VulnerabilityMetric::SsvcV1 => "ssvc_v1",
        VulnerabilityMetric::CvssV2 => "cvss_v2",
        VulnerabilityMetric::CvssV3(_) => "cvss_v3",
        VulnerabilityMetric::CvssV4 => "cvss_v4",
        VulnerabilityMetric::Epss => "epss",
    }
}

/// Trait representing an abstract threat in a CSAF document.
pub trait ThreatTrait: WithGroupIds + WithProductIds {
    /// Returns the date associated with this threat
    fn get_date(&self) -> &Option<String>;
}

/// Trait representing an abstract product tree in a CSAF document.
///
/// The `ProductTreeTrait` defines the structure of a product tree and allows
/// access to its product groups.
pub trait ProductTreeTrait {
    /// The associated type representing the type of branch in the product tree.
    type BranchType: BranchTrait<Self::FullProductNameType>;

    /// The associated type representing the type of product groups in the product tree.
    type ProductGroupType: ProductGroupTrait;

    /// The associated type representing the type of relationships in the product tree.
    type RelationshipType: RelationshipTrait<Self::FullProductNameType>;

    /// The associated type representing the type of the full product name.
    type FullProductNameType: ProductTrait;

    /// Returns an optional reference to the list of branches in the product tree.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves a reference to the list of product groups in the product tree.
    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType>;

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_relationships(&self) -> &Vec<Self::RelationshipType>;

    /// Retrieves a reference to the list of full product names in the product tree.
    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType>;

    /// Visits all product references in the product tree by invoking the provided callback for each
    /// product. Returns with collected error Results provided by `callback`, if occurring.
    ///
    /// This method traverses all locations in the product tree where products can be referenced:
    /// - Products within branches (recursively)
    /// - Full product names at the top level
    /// - Full product names within relationships
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string
    ///   and returns a `Result<(), Vec<ValidationError>>`. The path string represents the JSON
    ///   pointer to the product's location in the document.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(Vec<ValidationError>)` if any callback(s) returned errors for any products
    fn visit_all_products_generic(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str)) {
        // Visit products in branches
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(
                    &format!("/product_tree/branches/{}", i),
                    &mut |branch: &Self::BranchType, path| {
                        if let Some(product_ref) = branch.get_product() {
                            callback(product_ref, &format!("{}/product", path));
                        }
                    },
                );
            }
        }

        // Visit full_product_names
        for (i, fpn) in self.get_full_product_names().iter().enumerate() {
            callback(fpn, &format!("/product_tree/full_product_names/{}", i));
        }

        // Visit relationships
        for (i, rel) in self.get_relationships().iter().enumerate() {
            callback(
                rel.get_full_product_name(),
                &format!("/product_tree/relationships/{}/full_product_name", i),
            );
        }
    }

    /// A trait wrapper for `visit_all_products_generic()` that allows implementations to provide
    /// type-specific callbacks for product traversal.
    ///
    /// This method is intended to be implemented by trait objects to handle their specific
    /// product name types while reusing the generic traversal logic defined in
    /// `visit_all_products_generic()`.
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string,
    ///   returning a `Result<(), ValidationError>`. The callback will be invoked with the concrete
    ///   type specified by the implementing trait.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(ValidationError)` if the callback returned an error for any product
    ///
    /// # Implementation Notes
    /// Trait implementers should typically implement this by delegating to
    /// `visit_all_products_generic()` with the same callback.
    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str));
}

/// Trait representing an abstract branch in a product tree.
pub trait BranchTrait<FPN: ProductTrait>: Sized {
    /// Returns an optional reference to the child branches of this branch.
    fn get_branches(&self) -> Option<&Vec<Self>>;

    /// Retrieves the full product name associated with this branch, if available.
    fn get_product(&self) -> &Option<FPN>;

    /// Recursively visits all branches in the tree structure,
    /// applying the provided callback function to each branch.
    ///
    /// This method traverses the entire branch hierarchy, starting from the current branch and
    /// proceeding depth-first through all child branches. For each branch, it calls the
    /// provided callback function with the branch object and its path representation.
    ///
    /// # Parameters
    /// * `path` - A string representing the current path in the branch hierarchy
    /// * `callback` - A mutable function that takes a reference to Self and the
    ///   current path string and returns a Result
    ///
    /// # Returns
    /// * `Ok(())` if the traversal completes successfully
    /// * `Err(Vec<ValidationError>)` if the callback returns an error for any branch
    fn visit_branches_rec(&self, path: &str, callback: &mut impl FnMut(&Self, &str)) {
        callback(self, path);
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(&format!("{}/branches/{}", path, i), callback);
            }
        }
    }

    /// Searches for branches that exceed the maximum allowed depth in the branch hierarchy.
    ///
    /// This method recursively checks if the branch structure exceeds the specified depth limit.
    /// It traverses the branch hierarchy depth-first, decrementing the remaining depth parameter
    /// at each level. If branches are found beyond the allowed depth, it returns the path to the
    /// first excessive branch.
    ///
    /// # Parameters
    /// * `remaining_depth` - The maximum number of branch levels still allowed
    ///
    /// # Returns
    /// * `Some(String)` containing the path to the first branch that exceeds the allowed depth
    /// * `None` if no branches exceed the allowed depth
    fn find_excessive_branch_depth(&self, remaining_depth: u32) -> Option<String> {
        if let Some(branches) = self.get_branches() {
            // If we've reached the depth limit and there are branches, we've found a violation
            if remaining_depth == 1 {
                return Some("/branches/0".to_string());
            }
            for (i, branch) in branches.iter().enumerate() {
                if let Some(sub_path) = branch.find_excessive_branch_depth(remaining_depth - 1) {
                    return Some(format!("/branches/{}{}", i, sub_path));
                }
            }
        }
        None
    }
}

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Returns the unique identifier of the product group.
    fn get_group_id(&self) -> &String;

    /// Retrieves a vector of product IDs contained within the product group.
    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_;
}

/// Trait representing an abstract relationship in a product tree.
pub trait RelationshipTrait<FPN: ProductTrait> {
    /// Retrieves the product reference identifier.
    fn get_product_reference(&self) -> &String;

    /// Retrieves the identifier of the related product.
    fn get_relates_to_product_reference(&self) -> &String;

    /// Retrieves the full product name associated with the relationship.
    fn get_full_product_name(&self) -> &FPN;
}

/// Trait representing an abstract full product name in a CSAF document.
pub trait ProductTrait {
    /// The associated type representing a product identification helper.
    type ProductIdentificationHelperType: ProductIdentificationHelperTrait;

    /// Returns the product ID from the full product name.
    fn get_product_id(&self) -> &String;

    /// Returns the product identification helper associated with the full product name.
    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType>;
}

/// Trait representing an abstract product identification helper of a full product name.
pub trait ProductIdentificationHelperTrait {
    type HashType: HashTrait;

    /// Returns the PURLs identifying the associated product.
    fn get_purls(&self) -> Option<&[String]>;

    fn get_model_numbers(&self) -> Option<impl Iterator<Item = &String> + '_>;

    fn get_serial_numbers(&self) -> Option<impl Iterator<Item = &String> + '_>;

    fn get_hashes(&self) -> &Vec<Self::HashType>;
}

/// Trait representing a collection of file_hashes for a file as part of a product identification helper
pub trait HashTrait {
    type FileHashType: FileHashTrait;

    /// Returns the filename
    fn get_filename(&self) -> &String;

    /// returns
    fn get_file_hashes(&self) -> &Vec<Self::FileHashType>;
}

/// Trait representing a file_hash, identified by the used hash algorithm and the hash
pub trait FileHashTrait {
    /// Returns the hashing algorithm of this hash
    fn get_algorithm(&self) -> &String;

    /// Returns the hash
    fn get_hash(&self) -> &String;
}

pub trait WithGroupIds {
    /// Returns the product group IDs associated with this entity
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

pub trait WithProductIds {
    /// Returns the product IDs associated with this entity
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}
