use crate::schema::csaf2_1::schema::{
    CategoryOfPublisher, CategoryOfReference, CategoryOfTheBranch, CategoryOfTheRemediation, CategoryOfTheThreat,
    DocumentStatus, Epss, LabelOfTheFlag, LabelOfTlp, NoteCategory, PartyCategory,
};

use crate::csaf2_1::ssvc_dp_selection_list::SelectionList;
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use chrono::{DateTime, Utc};
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
            .map(|(group_or_product_id, path)| {
                (group_or_product_id.to_owned(), format!("/{}/{}/{}", prefix, idx, path))
            })
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
        self.get_notes().extract_group_references("/document/notes")
    }

    fn get_notes_product_references(&self) -> Vec<(String, String)> {
        self.get_notes().extract_product_references("/document/notes")
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
        DocumentCategory::from(self.get_category_string())
    }

    /// Returns the references of this document
    fn get_references(&self) -> Option<&Vec<Self::DocumentReferenceType>>;

    fn get_csaf_version(&self) -> &CsafVersion;
}

/// Enum representing CSAF versions
///
/// Contrary to other enums that are based on enums in the generated schemas, we are re-defining
/// this enum in the trait. Each schema only contains an enum with "their" version, and merging them
/// would be more complex than defining them here and mapping to them in each implementation.
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
impl From<&String> for DocumentCategory {
    fn from(category: &String) -> Self {
        match category.as_str() {
            "csaf_informational_advisory" => DocumentCategory::CsafInformationalAdvisory,
            "csaf_security_incident_response" => DocumentCategory::CsafSecurityIncidentResponse,
            "csaf_security_advisory" => DocumentCategory::CsafSecurityAdvisory,
            "csaf_vex" => DocumentCategory::CsafVex,
            "csaf_deprecated_security_advisory" => DocumentCategory::CsafDeprecatedSecurityAdvisory,
            "csaf_withdrawn" => DocumentCategory::CsafWithdrawn,
            "csaf_superseded" => DocumentCategory::CsafSuperseded,
            other => DocumentCategory::Other(other.to_string()),
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

pub trait NoteTrait: WithOptionalGroupIds + WithOptionalProductIds {
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

/// Type alias for a vector of revision history items
pub type RevisionHistory = Vec<RevisionHistoryItem>;

/// Struct representing a revision history item
/// Includes the path index in the original revision history, the date, and the version number
#[derive(Clone)]
pub struct RevisionHistoryItem {
    pub path_index: usize,
    pub date: DateTime<Utc>,
    pub number: VersionNumber,
}

/// Trait providing sorting functionality for revision history
pub trait RevisionHistorySortable {
    /// Sorts the revision history items first by date, second by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long the second order key (revision history numbers) are unique
    fn inplace_sort_by_date_then_number(&mut self);

    /// Sorts the revision history items by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long as the order key (revision history numbers) are unique
    fn inplace_sort_by_number(&mut self);
}

impl RevisionHistorySortable for RevisionHistory {
    fn inplace_sort_by_date_then_number(&mut self) {
        self.sort_unstable_by_key(|item| (item.date, item.number.clone()));
    }

    fn inplace_sort_by_number(&mut self) {
        self.sort_unstable_by(|a, b| a.number.cmp(&b.number));
    }
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

    /// Utility function to get revision history as structs containing revision history path index, date and number
    fn get_revision_history_tuples(&self) -> RevisionHistory {
        let mut revision_history: RevisionHistory = Vec::new();
        for (i_r, revision) in self.get_revision_history().iter().enumerate() {
            let date = DateTime::parse_from_rfc3339(revision.get_date()).map(|dt| dt.with_timezone(&Utc));
            if let Ok(date) = date {
                revision_history.push(RevisionHistoryItem {
                    path_index: i_r,
                    date,
                    number: revision.get_number(),
                });
            } else {
                panic!(
                    "Encountered date that could not be parsed as RFC3339: {}",
                    revision.get_date()
                );
            }
        }
        revision_history
    }

    /// Returns the status of this document
    fn get_status(&self) -> DocumentStatus;

    /// Returns the tracking ID of this document
    fn get_id(&self) -> &String;

    /// Returns the version of this document
    fn get_version_string(&self) -> &String;

    fn get_version(&self) -> VersionNumber {
        VersionNumber::from(self.get_version_string())
    }
}

#[derive(Debug, Clone, Eq)]
pub enum VersionNumber {
    Integer(u64),
    Semver(Version),
}

impl From<&str> for VersionNumber {
    /// Parses a string to either intver or semver
    /// Will panic if not parseable
    fn from(number: &str) -> Self {
        if let Ok(number) = number.parse::<u64>() {
            return VersionNumber::Integer(number);
        } else if let Ok(number) = Version::parse(number) {
            return VersionNumber::Semver(number);
        }
        panic!("Version could not be parsed as intver or semver")
    }
}

impl From<&String> for VersionNumber {
    /// Parses a string to either intver or semver
    /// Will panic if not parseable
    fn from(value: &String) -> Self {
        VersionNumber::from(value.as_str())
    }
}

impl VersionNumber {
    /// Gets the version number for intver / the major version for semver
    pub fn get_major(&self) -> u64 {
        match self {
            VersionNumber::Integer(num) => *num,
            VersionNumber::Semver(semver) => semver.major,
        }
    }

    /// Checks whether the intver version is zero, always `false` for semver
    /// Hard coupled check that version is intver and zero
    pub fn is_intver_is_zero(&self) -> bool {
        if let VersionNumber::Integer(version) = self {
            return *version == 0;
        }
        false
    }

    /// Checks whether the semver major version is zero, always `false` for intver
    /// Hard coupled check that version is semver and major is zero
    pub fn is_semver_is_major_zero(&self) -> bool {
        if let VersionNumber::Semver(version) = self {
            return version.major == 0;
        }
        false
    }

    /// Checks whether the semver has a pre-release part, always `false` for intver
    /// Hard coupled check that version is semver and has pre-release part
    pub fn is_semver_has_prerelease(&self) -> bool {
        if let VersionNumber::Semver(version) = self {
            return !version.pre.is_empty();
        }
        false
    }

    /// Checks whether the semver has build metadata, always `false` for intver
    /// Hard coupled check that version is semver and has build metadata
    pub fn is_semver_has_build_metadata(&self) -> bool {
        if let VersionNumber::Semver(version) = self {
            return !version.build.is_empty();
        }
        false
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
        VersionNumber::from(self.get_number_string())
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
        self.get_remediations().extract_group_references("remediations")
    }

    fn get_remediations_product_references(&self) -> Vec<(String, String)> {
        self.get_remediations().extract_product_references("remediations")
    }

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

    /// Utility function to get all group IDs referenced in metrics along with their JSON paths
    fn get_metrics_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        if let Some(metrics) = self.get_metrics().as_ref() {
            for (metric_i, metric) in metrics.iter().enumerate() {
                for (x_i, x) in metric.get_products().enumerate() {
                    ids.push((x.to_owned(), format!("metrics/{}/products/{}", metric_i, x_i)));
                }
            }
        }

        ids
    }

    /// Retrieves a list of potential threats related to the vulnerability.
    fn get_threats(&self) -> &Vec<Self::ThreatType>;

    /// Utility function to get all group IDs referenced in threats along with their JSON paths
    fn get_threats_group_references(&self) -> Vec<(String, String)> {
        self.get_threats().extract_group_references("threats")
    }

    fn get_threats_product_references(&self) -> Vec<(String, String)> {
        self.get_threats().extract_product_references("threats")
    }

    /// Returns the date when this vulnerability was initially disclosed.
    fn get_disclosure_date(&self) -> &Option<String>;

    /// Returns the date when this vulnerability was initially discovered.
    fn get_discovery_date(&self) -> &Option<String>;

    /// Returns all flags associated with this vulnerability.
    fn get_flags(&self) -> &Option<Vec<Self::FlagType>>;

    /// Utility function to get all group IDs referenced in flags along with their JSON paths
    fn get_flags_group_references(&self) -> Vec<(String, String)> {
        self.get_flags().extract_group_references("flags")
    }

    fn get_flags_product_references(&self) -> Vec<(String, String)> {
        self.get_flags().extract_product_references("flags")
    }

    /// Returns all involvements associated with this vulnerability.
    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>>;

    /// Utility function to get all group IDs referenced in involvements along with their JSON paths
    fn get_involvement_group_references(&self) -> Vec<(String, String)> {
        self.get_involvements().extract_group_references("involvements")
    }

    /// Returns the CVE associated with the vulnerability.
    fn get_cve(&self) -> Option<&String>;

    /// Returns the vulnerability IDs associated with this vulnerability.
    fn get_ids(&self) -> &Option<Vec<Self::VulnerabilityIdType>>;

    /// Returns the notes associated with this vulnerability.
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;

    /// Utility function to get all group IDs referenced in notes along with their JSON paths
    fn get_notes_group_references(&self) -> Vec<(String, String)> {
        self.get_notes().extract_group_references("notes")
    }

    /// Returns the information about the first known exploitation dates of this vulnerability.
    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>>;
}

pub trait VulnerabilityIdTrait {
    fn get_system_name(&self) -> &String;

    fn get_text(&self) -> &String;
}

/// Trait for accessing vulnerability flags information
pub trait FlagTrait: WithOptionalGroupIds + WithOptionalProductIds {
    /// Returns the date associated with this vulnerability flag
    fn get_date(&self) -> &Option<String>;

    /// Returns the label of the vulnerability flag
    fn get_label(&self) -> LabelOfTheFlag;
}

pub trait FirstKnownExploitationDatesTrait {
    fn get_date(&self) -> &String;
}

/// Trait for accessing vulnerability involvement information
pub trait InvolvementTrait: WithOptionalGroupIds {
    /// Returns the date associated with this vulnerability involvement
    fn get_date(&self) -> &Option<String>;

    /// Returns the party associated with this vulnerability involvement
    fn get_party(&self) -> PartyCategory;
}

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait: WithOptionalGroupIds + WithOptionalProductIds {
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

    /// Helper method to add product references with a given label to the result vector.
    fn extract_product_references<'a>(
        &self,
        ids: &mut Vec<(String, String)>,
        products: Option<impl Iterator<Item = &'a String> + 'a>,
        label: &str,
    ) {
        if let Some(iter) = products {
            for (x_i, x) in iter.enumerate() {
                ids.push(((*x).to_owned(), format!("product_status/{}/{}", label, x_i)));
            }
        }
    }

    fn get_all_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();
        self.extract_product_references(&mut ids, self.get_first_affected(), "first_affected");
        self.extract_product_references(&mut ids, self.get_first_fixed(), "first_fixed");
        self.extract_product_references(&mut ids, self.get_fixed(), "fixed");
        self.extract_product_references(&mut ids, self.get_known_affected(), "known_affected");
        self.extract_product_references(&mut ids, self.get_known_not_affected(), "known_not_affected");
        self.extract_product_references(&mut ids, self.get_last_affected(), "last_affected");
        self.extract_product_references(&mut ids, self.get_recommended(), "recommended");
        self.extract_product_references(&mut ids, self.get_under_investigation(), "under_investigation");
        ids
    }

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
pub trait ThreatTrait: WithOptionalGroupIds + WithOptionalProductIds {
    /// Returns the date associated with this threat
    fn get_date(&self) -> &Option<String>;

    /// Returns the category of the threat
    fn get_category(&self) -> CategoryOfTheThreat;
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

    /// Utility function to get all product references in product groups along with their JSON paths
    fn get_product_groups_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (pg_i, pg) in self.get_product_groups().iter().enumerate() {
            for (p_i, p) in pg.get_product_ids().enumerate() {
                ids.push((
                    (*p).to_owned(),
                    format!("/product_tree/product_groups/{}/product_ids/{}", pg_i, p_i),
                ));
            }
        }

        ids
    }

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_relationships(&self) -> &Vec<Self::RelationshipType>;

    /// Utility function to get all product references in relationships along with their JSON paths
    fn get_relationships_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (rel_i, rel) in self.get_relationships().iter().enumerate() {
            ids.push((
                rel.get_product_reference().to_owned(),
                format!("/product_tree/relationships/{}/product_reference", rel_i),
            ));
            ids.push((
                rel.get_relates_to_product_reference().to_owned(),
                format!("/product_tree/relationships/{}/relates_to_product_reference", rel_i),
            ));
        }

        ids
    }

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

    fn get_category(&self) -> &CategoryOfTheBranch;

    fn get_name(&self) -> &str;

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

    /// Returns the textual description of the product
    fn get_name(&self) -> &str;

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

    /// Returns true if only hashes with the specified algorithm are present
    fn contains_only_hash_algorithm(&self, algorithm: HashAlgorithm) -> bool {
        for hash in self.get_file_hashes() {
            if hash.get_algorithm() != algorithm {
                return false;
            }
        }
        true
    }
}

/// Enum representing supported hash algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Blake2b512,
    Blake2s256,
    Md4,
    Md5,
    Md5Sha1,
    Mdc2,
    Ripemd,
    Ripemd160,
    Rmd160,
    Sha1,
    Sha224,
    Sha256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Shake128,
    Shake256,
    Sm3,
    Ssl3Md5,
    Ssl3Sha1,
    Whirlpool,
    Other(String),
}

impl From<&String> for HashAlgorithm {
    fn from(algo: &String) -> Self {
        match algo.as_str() {
            "blake2b512" => HashAlgorithm::Blake2b512,
            "blake2s256" => HashAlgorithm::Blake2s256,
            "md4" => HashAlgorithm::Md4,
            "md5" => HashAlgorithm::Md5,
            "md5-sha1" => HashAlgorithm::Md5Sha1,
            "mdc2" => HashAlgorithm::Mdc2,
            "ripemd" => HashAlgorithm::Ripemd,
            "ripemd160" => HashAlgorithm::Ripemd160,
            "rmd160" => HashAlgorithm::Rmd160,
            "sha1" => HashAlgorithm::Sha1,
            "sha224" => HashAlgorithm::Sha224,
            "sha256" => HashAlgorithm::Sha256,
            "sha3-224" => HashAlgorithm::Sha3_224,
            "sha3-256" => HashAlgorithm::Sha3_256,
            "sha3-384" => HashAlgorithm::Sha3_384,
            "sha3-512" => HashAlgorithm::Sha3_512,
            "sha384" => HashAlgorithm::Sha384,
            "sha512" => HashAlgorithm::Sha512,
            "sha512-224" => HashAlgorithm::Sha512_224,
            "sha512-256" => HashAlgorithm::Sha512_256,
            "shake128" => HashAlgorithm::Shake128,
            "shake256" => HashAlgorithm::Shake256,
            "sm3" => HashAlgorithm::Sm3,
            "ssl3-md5" => HashAlgorithm::Ssl3Md5,
            "ssl3-sha1" => HashAlgorithm::Ssl3Sha1,
            "whirlpool" => HashAlgorithm::Whirlpool,
            other => HashAlgorithm::Other(other.to_string()),
        }
    }
}

impl Display for HashAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                HashAlgorithm::Blake2b512 => "blake2b512",
                HashAlgorithm::Blake2s256 => "blake2s256",
                HashAlgorithm::Md4 => "md4",
                HashAlgorithm::Md5 => "md5",
                HashAlgorithm::Md5Sha1 => "md5-sha1",
                HashAlgorithm::Mdc2 => "mdc2",
                HashAlgorithm::Ripemd => "ripemd",
                HashAlgorithm::Ripemd160 => "ripemd160",
                HashAlgorithm::Rmd160 => "rmd160",
                HashAlgorithm::Sha1 => "sha1",
                HashAlgorithm::Sha224 => "sha224",
                HashAlgorithm::Sha256 => "sha256",
                HashAlgorithm::Sha3_224 => "sha3-224",
                HashAlgorithm::Sha3_256 => "sha3-256",
                HashAlgorithm::Sha3_384 => "sha3-384",
                HashAlgorithm::Sha3_512 => "sha3-512",
                HashAlgorithm::Sha384 => "sha384",
                HashAlgorithm::Sha512 => "sha512",
                HashAlgorithm::Sha512_224 => "sha512-224",
                HashAlgorithm::Sha512_256 => "sha512-256",
                HashAlgorithm::Shake128 => "shake128",
                HashAlgorithm::Shake256 => "shake256",
                HashAlgorithm::Sm3 => "sm3",
                HashAlgorithm::Ssl3Md5 => "ssl3-md5",
                HashAlgorithm::Ssl3Sha1 => "ssl3-sha1",
                HashAlgorithm::Whirlpool => "whirlpool",
                HashAlgorithm::Other(other) => other.as_str(),
            }
        )
    }
}

/// Trait representing a file_hash, identified by the used hash algorithm and the hash
pub trait FileHashTrait {
    /// Returns the hashing algorithm of this hash
    fn get_algorithm_string(&self) -> &String;

    /// Returns the hash
    fn get_hash(&self) -> &String;

    /// Returns the hashing algorithm as HashAlgorithm enum
    fn get_algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::from(self.get_algorithm_string())
    }
}

pub trait WithOptionalGroupIds {
    /// Returns the product group IDs associated with this entity
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

pub trait WithOptionalProductIds {
    /// Returns the product IDs associated with this entity
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

/// Central helper function for extracting group references.
///
/// This function implements the core logic for extracting group IDs and their JSON paths
/// from an iterator of items that implement `WithGroupIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement WithGroupIds
/// * `path_prefix` - A string representing the prefix for the JSON path (e.g., "flags", "notes")
///
/// # Returns
///
/// A vector of tuples containing (group_id, json_path) for each group reference found.
fn extract_group_id_impl<'a, T: WithOptionalGroupIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path_prefix: &str,
) -> Vec<(String, String)> {
    let mut ids: Vec<(String, String)> = Vec::new();
    for (index, item) in items.enumerate() {
        if let Some(group_ids) = item.get_group_ids() {
            for (group_index, group_id) in group_ids.enumerate() {
                ids.push((
                    group_id.to_owned(),
                    format!("{}/{}/group_ids/{}", path_prefix, index, group_index),
                ))
            }
        }
    }
    ids
}

/// Central helper function for extracting group references.
///
/// This function implements the core logic for extracting group IDs and their JSON paths
/// from an iterator of items that implement `WithProductIds`. This avoids code duplication
/// across multiple trait implementations.
///
/// # Arguments
///
/// * `items` - An iterator over items that implement WithProductIds
/// * `path_prefix` - A string representing the prefix for the JSON path (e.g., "flags", "notes")
///
/// # Returns
///
/// A vector of tuples containing (product_id, json_path) for each product reference found.
fn extract_product_id_impl<'a, T: WithOptionalProductIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path_prefix: &str,
) -> Vec<(String, String)> {
    let mut ids: Vec<(String, String)> = Vec::new();
    for (index, item) in items.enumerate() {
        if let Some(product_ids) = item.get_product_ids() {
            for (product_index, product_id) in product_ids.enumerate() {
                ids.push((
                    product_id.to_owned(),
                    format!("{}/{}/product_ids/{}", path_prefix, index, product_index),
                ))
            }
        }
    }
    ids
}

/// Extension trait for extracting group references from collections where T implements WithOptionalGroupIds.
///
/// This trait provides a generic method to extract group IDs from collections of objects
/// that implement the `WithOptionalGroupIds` trait, returning them as tuples of (group_id, json_path).
///
/// Implemented for:
/// - `Option<&Vec<T>>`
/// - `&Option<Vec<T>>`
/// - `Vec<T>`
///
/// TODO: As already discussed, we should simplify / align our return params here.
/// It does not make sense to have the same functionality return either `Option<&Vec<T>>` or
/// `&Option<Vec<T>>` in some cases. When this is done, we can remove the unused case.
pub trait ExtractGroupReferences<T: WithOptionalGroupIds> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for Option<&Vec<T>> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter().flat_map(|x| x.iter()), path_prefix)
    }
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for &Option<Vec<T>> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter().flatten(), path_prefix)
    }
}

impl<T: WithOptionalGroupIds> ExtractGroupReferences<T> for Vec<T> {
    fn extract_group_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_group_id_impl(self.iter(), path_prefix)
    }
}

/// Extension trait for extracting group references from collections where T implements WithOptionalProductIds.
///
/// This trait provides a generic method to extract group IDs from collections of objects
/// that implement the `WithOptionalProductIds` trait, returning them as tuples of (product_id, json_path).
///
/// Implemented for:
/// - `Option<&Vec<T>>`
/// - `&Option<Vec<T>>`
/// - `Vec<T>`
///
/// TODO: As already discussed, we should simplify / align our return params here.
/// It does not make sense to have the same functionality return either `Option<&Vec<T>>` or
/// `&Option<Vec<T>>` in some cases. When this is done, we can remove the unused case.
pub trait ExtractProductReferences<T: WithOptionalProductIds> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)>;
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for Option<&Vec<T>> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter().flat_map(|x| x.iter()), path_prefix)
    }
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for &Option<Vec<T>> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter().flatten(), path_prefix)
    }
}

impl<T: WithOptionalProductIds> ExtractProductReferences<T> for Vec<T> {
    fn extract_product_references(&self, path_prefix: &str) -> Vec<(String, String)> {
        extract_product_id_impl(self.iter(), path_prefix)
    }
}
