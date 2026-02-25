use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::csaf::types::csaf_product_id_helper_number::{CsafModelNumber, CsafSerialNumber};
use crate::csaf::types::csaf_version_number::CsafVersionNumber;
use crate::csaf_traits::{
    BranchTrait, CategoryOfTheBranch as CategoryOfTheBranchTrait, ContentTrait, CsafTrait, CsafVersion, Cwe,
    DistributionTrait, DocumentReferenceTrait, DocumentTrait, FileHashTrait, FirstKnownExploitationDatesTrait,
    FlagTrait, GeneratorTrait, HashTrait, InvolvementTrait, MetricTrait, NoteTrait, ProductGroupTrait,
    ProductIdentificationHelperTrait, ProductStatusTrait, ProductTrait, ProductTreeTrait, PublisherTrait,
    RelationshipTrait, RemediationTrait, RevisionTrait, SharingGroupTrait, ThreatTrait, TlpTrait, TrackingTrait,
    VulnerabilityIdTrait, VulnerabilityTrait, WithDate, WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds,
};
use crate::csaf2_1::ssvc_dp_selection_list::SelectionList;
use crate::schema::csaf2_0::schema::{
    Branch, CategoryOfPublisher, CategoryOfReference, CategoryOfTheBranch, CategoryOfTheRemediation,
    CategoryOfTheThreat, CommonSecurityAdvisoryFramework, CryptographicHashes, CsafVersion as CsafVersion20,
    DocumentGenerator, DocumentLevelMetaData, DocumentStatus, FileHash, Flag, FullProductNameT,
    HelperToIdentifyTheProduct, Id, Involvement, LabelOfTheFlag, LabelOfTlp, Note, NoteCategory, PartyCategory,
    ProductGroup, ProductStatus, ProductTree, Publisher, Reference, Relationship, Remediation, Revision,
    RulesForSharingDocument, Score, Threat, Tracking, TrafficLightProtocolTlp, Vulnerability,
};
use crate::schema::csaf2_1::schema::{
    CategoryOfPublisher as CategoryOfPublisher21, CategoryOfReference as CategoryOfReference21,
    CategoryOfTheRemediation as Remediation21, CategoryOfTheThreat as CategoryOfTheThreat21,
    DocumentStatus as Status21, Epss, LabelOfTheFlag as LabelOfTheFlag21, LabelOfTlp as Tlp21,
    NoteCategory as NoteCategory21, PartyCategory as PartyCategory21,
};
use crate::validation::ValidationError;
use serde::de::Error;
use serde_json::{Map, Value};
use std::ops::Deref;
use uuid::Uuid;
use crate::csaf::types::csaf_language::CsafLanguage;

impl WithOptionalGroupIds for Remediation {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Remediation {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl RemediationTrait for Remediation {
    /// Normalizes the remediation categories from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the list of remediation categories was expanded, making it a superset of those
    /// in CSAF 2.0. This function ensures that the remediation category from a CSAF 2.0 remediation
    /// object is converted into the corresponding category defined in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `CategoryOfTheRemediation` that corresponds to the remediation category of the
    /// current object.
    fn get_category(&self) -> Remediation21 {
        match self.category {
            CategoryOfTheRemediation::Workaround => Remediation21::Workaround,
            CategoryOfTheRemediation::Mitigation => Remediation21::Mitigation,
            CategoryOfTheRemediation::VendorFix => Remediation21::VendorFix,
            CategoryOfTheRemediation::NoFixPlanned => Remediation21::NoFixPlanned,
            CategoryOfTheRemediation::NoneAvailable => Remediation21::NoneAvailable,
        }
    }
}

impl WithOptionalDate for Remediation {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl ProductStatusTrait for ProductStatus {
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_not_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.last_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.recommended.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.under_investigation
            .as_ref()
            .map(|p| (*p).iter().map(|x| x.deref()))
    }

    /// Not specified for CSAF 2.0, so `None`
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl MetricTrait for Score {
    type ContentType = Score;

    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        self.products.iter().map(|x| x.deref())
    }

    fn get_content(&self) -> &Self::ContentType {
        self
    }

    fn get_source(&self) -> &Option<String> {
        &None
    }
}

impl ContentTrait for Score {
    fn has_ssvc(&self) -> bool {
        false
    }

    fn get_ssvc(&self) -> Result<SelectionList, serde_json::Error> {
        Err(serde_json::Error::custom(
            "SSVC metrics are not implemented in CSAF 2.0",
        ))
    }

    fn get_cvss_v2(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v2.is_empty() {
            None
        } else {
            Some(&self.cvss_v2)
        }
    }

    fn get_cvss_v3(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v3.is_empty() {
            None
        } else {
            Some(&self.cvss_v3)
        }
    }

    fn get_cvss_v4(&self) -> Option<&Map<String, Value>> {
        None
    }

    fn get_epss(&self) -> &Option<Epss> {
        &None::<Epss>
    }

    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String {
        format!("/vulnerabilities/{vulnerability_idx}/scores/{metric_idx}")
    }
}

impl WithOptionalGroupIds for Threat {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Threat {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl ThreatTrait for Threat {
    fn get_category(&self) -> CategoryOfTheThreat21 {
        match self.category {
            CategoryOfTheThreat::ExploitStatus => CategoryOfTheThreat21::ExploitStatus,
            CategoryOfTheThreat::Impact => CategoryOfTheThreat21::Impact,
            CategoryOfTheThreat::TargetSet => CategoryOfTheThreat21::TargetSet,
        }
    }
}

impl WithOptionalDate for Threat {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl VulnerabilityTrait for Vulnerability {
    type RemediationType = Remediation;
    type ProductStatusType = ProductStatus;
    // Metrics are not implemented in CSAF 2.0
    type MetricType = Score;
    type ThreatType = Threat;
    type FlagType = Flag;
    type InvolvementType = Involvement;
    type VulnerabilityIdType = Id;
    type NoteType = Note;
    // First known exploitation dates are not implemented in CSAF 2.0
    type FirstKnownExploitationDatesType = ();

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

    fn get_flags(&self) -> &Option<Vec<Self::FlagType>> {
        &self.flags
    }

    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>> {
        &self.involvements
    }

    fn get_cve(&self) -> Option<&String> {
        self.cve.as_deref()
    }

    fn get_cwe(&self) -> Option<Vec<Cwe>> {
        self.cwe.as_ref().map(|cwe| vec![Cwe::from(cwe)])
    }

    fn get_ids(&self) -> &Option<Vec<Self::VulnerabilityIdType>> {
        &self.ids
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_deref()
    }

    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>> {
        None
    }
}

impl VulnerabilityIdTrait for Id {
    fn get_system_name(&self) -> &String {
        self.system_name.deref()
    }

    fn get_text(&self) -> &String {
        self.text.deref()
    }
}

impl WithOptionalGroupIds for Flag {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Flag {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl FlagTrait for Flag {
    fn get_label(&self) -> LabelOfTheFlag21 {
        match self.label {
            LabelOfTheFlag::ComponentNotPresent => LabelOfTheFlag21::ComponentNotPresent,
            LabelOfTheFlag::InlineMitigationsAlreadyExist => LabelOfTheFlag21::InlineMitigationsAlreadyExist,
            LabelOfTheFlag::VulnerableCodeCannotBeControlledByAdversary => {
                LabelOfTheFlag21::VulnerableCodeCannotBeControlledByAdversary
            },
            LabelOfTheFlag::VulnerableCodeNotInExecutePath => LabelOfTheFlag21::VulnerableCodeNotInExecutePath,
            LabelOfTheFlag::VulnerableCodeNotPresent => LabelOfTheFlag21::VulnerableCodeNotPresent,
        }
    }
}

impl WithOptionalDate for Flag {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl FirstKnownExploitationDatesTrait for () {}

impl WithOptionalProductIds for () {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithOptionalGroupIds for () {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithDate for () {
    fn get_date(&self) -> CsafDateTime {
        panic!("First known exploitation dates are not implemented in CSAF 2.0");
    }
}

impl InvolvementTrait for Involvement {
    fn get_party(&self) -> PartyCategory21 {
        match self.party {
            PartyCategory::Coordinator => PartyCategory21::Coordinator,
            PartyCategory::Discoverer => PartyCategory21::Discoverer,
            PartyCategory::Other => PartyCategory21::Other,
            PartyCategory::User => PartyCategory21::User,
            PartyCategory::Vendor => PartyCategory21::Vendor,
        }
    }
}

impl WithOptionalDate for Involvement {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalGroupIds for Involvement {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithOptionalProductIds for Involvement {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl CsafTrait for CommonSecurityAdvisoryFramework {
    type VulnerabilityType = Vulnerability;
    type ProductTreeType = ProductTree;
    type DocumentType = DocumentLevelMetaData;

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

impl DocumentTrait for DocumentLevelMetaData {
    type TrackingType = Tracking;
    type DistributionType = RulesForSharingDocument;
    type NoteType = Note;
    type PublisherType = Publisher;
    type DocumentReferenceType = Reference;

    fn get_tracking(&self) -> &Self::TrackingType {
        &self.tracking
    }

    /// Return distribution as ref Option, it is optional anyways
    fn get_distribution_20(&self) -> Option<&Self::DistributionType> {
        self.distribution.as_ref()
    }

    /// Return distribution or a Validation error to satisfy CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError> {
        match self.distribution.as_ref() {
            None => Err(ValidationError {
                message: "CSAF 2.1 requires the distribution property, but it is not set.".to_string(),
                instance_path: "/document/distribution".to_string(),
            }),
            Some(distribution) => Ok(distribution),
        }
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_deref()
    }

    fn get_lang(&self) -> Option<CsafLanguage> {
        self.lang.as_deref().map(CsafLanguage::from)
    }

    fn get_source_lang(&self) -> Option<CsafLanguage> {
        self.lang.as_deref().map(CsafLanguage::from)
    }

    fn get_publisher(&self) -> &Publisher {
        &self.publisher
    }

    fn get_category(&self) -> CsafDocumentCategory {
        CsafDocumentCategory::from(&self.category)
    }

    fn get_references(&self) -> Option<&Vec<Self::DocumentReferenceType>> {
        self.references.as_deref()
    }

    fn get_csaf_version(&self) -> &CsafVersion {
        match self.csaf_version {
            CsafVersion20::X20 => &CsafVersion::X20,
        }
    }
}

impl DocumentReferenceTrait for Reference {
    fn get_category(&self) -> &CategoryOfReference21 {
        match &self.category {
            CategoryOfReference::External => &CategoryOfReference21::External,
            CategoryOfReference::Self_ => &CategoryOfReference21::Self_,
        }
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }

    fn get_url(&self) -> &String {
        &self.url
    }
}

impl PublisherTrait for Publisher {
    fn get_category(&self) -> CategoryOfPublisher21 {
        match self.category {
            CategoryOfPublisher::Coordinator => CategoryOfPublisher21::Coordinator,
            CategoryOfPublisher::Discoverer => CategoryOfPublisher21::Discoverer,
            CategoryOfPublisher::Other => CategoryOfPublisher21::Other,
            CategoryOfPublisher::Translator => CategoryOfPublisher21::Translator,
            CategoryOfPublisher::Vendor => CategoryOfPublisher21::Vendor,
            CategoryOfPublisher::User => CategoryOfPublisher21::User,
        }
    }
}

impl DistributionTrait for RulesForSharingDocument {
    type SharingGroupType = ();
    type TlpType = TrafficLightProtocolTlp;

    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType> {
        &None
    }

    /// Return TLP as ref Option, it is an option anyway
    fn get_tlp_20(&self) -> Option<&Self::TlpType> {
        self.tlp.as_ref()
    }

    /// Return TLP or a ValidationError to satisfy CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError> {
        match self.tlp.as_ref() {
            None => Err(ValidationError {
                message: "CSAF 2.1 requires the TLP property, but it is not set.".to_string(),
                instance_path: "/document/distribution/sharing_group/tlp".to_string(),
            }),
            Some(tlp) => Ok(tlp),
        }
    }
}

impl WithOptionalGroupIds for Note {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithOptionalProductIds for Note {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl NoteTrait for Note {
    fn get_category(&self) -> NoteCategory21 {
        match self.category {
            NoteCategory::Summary => NoteCategory21::Summary,
            NoteCategory::Details => NoteCategory21::Details,
            NoteCategory::Other => NoteCategory21::Other,
            NoteCategory::Description => NoteCategory21::Description,
            NoteCategory::Faq => NoteCategory21::Faq,
            NoteCategory::General => NoteCategory21::General,
            NoteCategory::LegalDisclaimer => NoteCategory21::LegalDisclaimer,
        }
    }
}

impl SharingGroupTrait for () {
    fn get_id(&self) -> &Uuid {
        panic!("Sharing groups are not implemented in CSAF 2.0");
    }

    fn get_name(&self) -> Option<&String> {
        panic!("Sharing groups are not implemented in CSAF 2.0");
    }
}

impl TlpTrait for TrafficLightProtocolTlp {
    /// Normalizes the TLP (Traffic Light Protocol) labels from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the TLP labeling scheme was updated to align with the official TLP 2.0 standard,
    /// which renamed "WHITE" to "CLEAR". This function ensures that TLP labels from CSAF 2.0
    /// are converted to their corresponding labels in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `Tlp21` value that corresponds to the TLP label of the current object.
    fn get_label(&self) -> Tlp21 {
        match self.label {
            LabelOfTlp::Amber => Tlp21::Amber,
            LabelOfTlp::Green => Tlp21::Green,
            LabelOfTlp::Red => Tlp21::Red,
            LabelOfTlp::White => Tlp21::Clear,
        }
    }
}

impl TrackingTrait for Tracking {
    type GeneratorType = DocumentGenerator;
    type RevisionType = Revision;

    fn get_current_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.current_release_date)
    }

    fn get_initial_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.initial_release_date)
    }

    fn get_generator(&self) -> &Option<Self::GeneratorType> {
        &self.generator
    }

    fn get_revision_history(&self) -> &Vec<Self::RevisionType> {
        &self.revision_history
    }

    fn get_status(&self) -> Status21 {
        match self.status {
            DocumentStatus::Draft => Status21::Draft,
            DocumentStatus::Final => Status21::Final,
            DocumentStatus::Interim => Status21::Interim,
        }
    }

    fn get_id(&self) -> &String {
        self.id.deref()
    }

    fn get_version(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.version)
    }
}

impl GeneratorTrait for DocumentGenerator {}

impl WithOptionalDate for DocumentGenerator {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl RevisionTrait for Revision {
    fn get_number(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.number)
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }
}

impl WithDate for Revision {
    fn get_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.date)
    }
}

impl ProductTreeTrait for ProductTree {
    type BranchType = Branch;
    type ProductGroupType = ProductGroup;
    type RelationshipType = Relationship;
    type FullProductNameType = FullProductNameT;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_deref()
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

    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str)) {
        self.visit_all_products_generic(callback)
    }
}

impl BranchTrait<FullProductNameT> for Branch {
    fn get_branches(&self) -> Option<&Vec<Self>> {
        self.branches.as_deref()
    }

    fn get_category(&self) -> &CategoryOfTheBranchTrait {
        match self.category {
            CategoryOfTheBranch::Architecture => &CategoryOfTheBranchTrait::Architecture,
            CategoryOfTheBranch::HostName => &CategoryOfTheBranchTrait::HostName,
            CategoryOfTheBranch::Language => &CategoryOfTheBranchTrait::Language,
            CategoryOfTheBranch::Legacy => &CategoryOfTheBranchTrait::Legacy,
            CategoryOfTheBranch::PatchLevel => &CategoryOfTheBranchTrait::PatchLevel,
            CategoryOfTheBranch::ProductFamily => &CategoryOfTheBranchTrait::ProductFamily,
            CategoryOfTheBranch::ProductName => &CategoryOfTheBranchTrait::ProductName,
            CategoryOfTheBranch::ProductVersion => &CategoryOfTheBranchTrait::ProductVersion,
            CategoryOfTheBranch::ProductVersionRange => &CategoryOfTheBranchTrait::ProductVersionRange,
            CategoryOfTheBranch::ServicePack => &CategoryOfTheBranchTrait::ServicePack,
            CategoryOfTheBranch::Specification => &CategoryOfTheBranchTrait::Specification,
            CategoryOfTheBranch::Vendor => &CategoryOfTheBranchTrait::Vendor,
        }
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product(&self) -> &Option<FullProductNameT> {
        &self.product
    }
}

impl ProductGroupTrait for ProductGroup {
    fn get_group_id(&self) -> &String {
        self.group_id.deref()
    }
    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_ {
        self.product_ids.iter().map(|x| x.deref())
    }
}

impl RelationshipTrait<FullProductNameT> for Relationship {
    fn get_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_relates_to_product_reference(&self) -> &String {
        self.relates_to_product_reference.deref()
    }

    fn get_full_product_name(&self) -> &FullProductNameT {
        &self.full_product_name
    }
}

impl ProductTrait for FullProductNameT {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct;

    fn get_product_id(&self) -> &String {
        self.product_id.deref()
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType> {
        &self.product_identification_helper
    }
}

impl ProductIdentificationHelperTrait for HelperToIdentifyTheProduct {
    type HashType = CryptographicHashes;

    fn get_purls(&self) -> Option<&[String]> {
        self.purl.as_ref().map(std::slice::from_ref)
    }

    fn get_model_numbers(&self) -> Option<Vec<CsafModelNumber>> {
        self.model_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafModelNumber::from).collect())
    }

    fn get_serial_numbers(&self) -> Option<Vec<CsafSerialNumber>> {
        self.serial_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafSerialNumber::from).collect())
    }

    fn get_hashes(&self) -> &Vec<Self::HashType> {
        self.hashes.as_ref()
    }
}

impl HashTrait for CryptographicHashes {
    type FileHashType = FileHash;

    fn get_filename(&self) -> &String {
        self.filename.deref()
    }

    fn get_file_hashes(&self) -> &Vec<Self::FileHashType> {
        self.file_hashes.as_ref()
    }
}

impl FileHashTrait for FileHash {
    fn get_algorithm(&self) -> CsafHashAlgorithm {
        CsafHashAlgorithm::from(&self.algorithm)
    }

    fn get_hash(&self) -> &String {
        self.value.deref()
    }
}
