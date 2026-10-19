use crate::csaf::enums::csaf_version::CsafVersion;
use crate::csaf::traits::util::canonical_url::filter_canonical_url_candidates;
use crate::csaf::traits::util::extract_references::define_reference_accessors;
use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{
    AcknowledgmentTrait, AggregateSeverityTrait, Csaf21InvolvementTrait, DistributionTrait, ExtensionsTrait, NoteTrait,
    PublisherTrait, ReferenceTrait, TrackingTrait,
};
use crate::schema::csaf2_0::schema::{
    Acknowledgment as Acknowledgment20, AggregateSeverity as AggregateSeverity20, CsafVersion as CsafVersion20,
    DocumentLevelMetaData as DocumentLevelMetaData20, Note as Note20, Publisher as Publisher20,
    Reference as Reference20, RulesForSharingDocument as RulesForSharingDocument20, Tracking as Tracking20,
};
use crate::schema::csaf2_1::schema::{
    Acknowledgment as Acknowledgment21, AggregateSeverity as AggregateSeverity21, CsafVersion as CsafVersion21,
    DocumentLevelMetaData as DocumentLevelMetaData21, ExtensionsT as Extensions21, Involvement as Involvement21,
    Note as Note21, Publisher as Publisher21, Reference as Reference21,
    RulesForDocumentSharing as RulesForDocumentSharing21, Tracking as Tracking21,
};
use crate::validation::{TestFinding, TestFindingData};

/// Trait representing document meta-level information
pub trait DocumentTrait {
    /// Type representing document acknowledgment information
    type AcknowledgmentType: AcknowledgmentTrait;

    /// Type representing document aggregate severity information
    type AggregateSeverityType: AggregateSeverityTrait;
    /// Type representing document tracking information
    type TrackingType: TrackingTrait;

    /// Type representing document distribution information
    type DistributionType: DistributionTrait;

    /// Type representing document involvement information
    ///
    /// Note: In CSAF 2.0, involvements worked differently and were present under `vulnerabilities`.
    ///
    /// For CSAF 2.0 documents, this type is `NotPresentInCsaf20`.
    /// For CSAF 2.1 documents, this type has the information described above.
    type InvolvementType: Csaf21InvolvementTrait;

    /// Type representing document notes
    type NoteType: NoteTrait;

    /// Type representing document publisher information
    type PublisherType: PublisherTrait;

    /// Type representing document references
    type ReferenceType: ReferenceTrait;

    /// Type representing document extensions
    type ExtensionsType: ExtensionsTrait;

    /// Returns the acknowledgments information for this document
    fn get_acknowledgments(&self) -> Option<&Vec<Self::AcknowledgmentType>>;

    /// Returns the aggregate severity information for this document
    fn get_aggregate_severity(&self) -> Option<&Self::AggregateSeverityType>;

    /// Returns the tracking information for this document
    fn get_tracking(&self) -> &Self::TrackingType;

    /// Returns the distribution information for this document with CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, TestFinding>;

    /// Returns the distribution information for this document with CSAF 2.0 semantics
    fn get_distribution_20(&self) -> Option<&Self::DistributionType>;

    /// Returns the notes associated with this document
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;

    define_reference_accessors! {
        optional_vec: [
            (get_notes_group_references, get_notes_product_references, get_notes, "/document/notes"),
        ],
    }

    /// Returns the language associated with this document.
    fn get_lang(&self) -> Option<CsafLanguage>;

    /// Returns the source language associated with this document.
    fn get_source_lang(&self) -> Option<CsafLanguage>;

    /// Returns the publisher information for this document
    fn get_publisher(&self) -> &Self::PublisherType;

    /// Returns the category of the document as an enum
    fn get_category(&self) -> CsafDocumentCategory;

    /// Returns the references of this document
    fn get_references(&self) -> Option<&Vec<Self::ReferenceType>>;

    /// Returns the involvement information for this document
    ///
    /// Note: In CSAF 2.0, involvements worked differently and were present under `vulnerabilities`.
    ///
    /// If called for CSAF 2.0 documents, this method returns `None`.
    /// If called for CSAF 2.1 documents, this method returns the involvement information if present, or `None` if not present.
    fn get_involvement(&self) -> Option<&Self::InvolvementType>;

    /// Returns the canonical URLs from this document's references.
    fn get_canonical_urls(&self) -> Vec<&str> {
        let expected_filename = self.get_tracking().get_canonical_filename();
        filter_canonical_url_candidates(self.get_references(), expected_filename.as_str()).collect()
    }

    /// Returns `true` if the document has at least one canonical URL.
    fn has_canonical_url(&self) -> bool {
        let expected_filename = self.get_tracking().get_canonical_filename();
        filter_canonical_url_candidates(self.get_references(), expected_filename.as_str())
            .next()
            .is_some()
    }

    /// Returns the CSAF version of this document
    fn get_csaf_version(&self) -> CsafVersion;

    /// Returns the title of this document
    fn get_title(&self) -> &str;

    /// Returns the extensions of this document
    fn get_extensions(&self) -> Option<&Self::ExtensionsType>;
}

impl DocumentTrait for DocumentLevelMetaData20 {
    type AcknowledgmentType = Acknowledgment20;
    type AggregateSeverityType = AggregateSeverity20;
    type TrackingType = Tracking20;
    type DistributionType = RulesForSharingDocument20;
    type NoteType = Note20;
    type PublisherType = Publisher20;
    type ReferenceType = Reference20;
    type ExtensionsType = NotPresentInCsaf20;

    fn get_acknowledgments(&self) -> Option<&Vec<Self::AcknowledgmentType>> {
        self.acknowledgments.as_deref()
    }

    fn get_aggregate_severity(&self) -> Option<&Self::AggregateSeverityType> {
        self.aggregate_severity.as_ref()
    }

    fn get_tracking(&self) -> &Self::TrackingType {
        &self.tracking
    }

    /// Return distribution as ref Option, it is optional anyways
    fn get_distribution_20(&self) -> Option<&Self::DistributionType> {
        self.distribution.as_ref()
    }

    /// Return distribution or a Validation error to satisfy CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, TestFinding> {
        match self.distribution.as_ref() {
            None => Err(TestFinding::Error(TestFindingData {
                message: "CSAF 2.1 requires the distribution property, but it is not set.".to_string(),
                instance_path: "/document/distribution".to_string(),
            })),
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
        self.source_lang.as_deref().map(CsafLanguage::from)
    }

    fn get_publisher(&self) -> &Self::PublisherType {
        &self.publisher
    }

    fn get_category(&self) -> CsafDocumentCategory {
        CsafDocumentCategory::from(&self.category)
    }

    fn get_references(&self) -> Option<&Vec<Self::ReferenceType>> {
        self.references.as_deref()
    }

    fn get_csaf_version(&self) -> CsafVersion {
        match &self.csaf_version {
            CsafVersion20::X20 => CsafVersion::X20,
        }
    }

    impl_str_field_getter!(get_title, title);

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        None
    }

    type InvolvementType = NotPresentInCsaf20;

    fn get_involvement(&self) -> Option<&Self::InvolvementType> {
        None
    }
}

impl DocumentTrait for DocumentLevelMetaData21 {
    type AcknowledgmentType = Acknowledgment21;
    type AggregateSeverityType = AggregateSeverity21;
    type TrackingType = Tracking21;
    type DistributionType = RulesForDocumentSharing21;
    type InvolvementType = Involvement21;
    type NoteType = Note21;
    type PublisherType = Publisher21;
    type ReferenceType = Reference21;
    type ExtensionsType = Extensions21;

    fn get_acknowledgments(&self) -> Option<&Vec<Self::AcknowledgmentType>> {
        self.acknowledgments.as_deref()
    }

    fn get_aggregate_severity(&self) -> Option<&Self::AggregateSeverityType> {
        self.aggregate_severity.as_ref()
    }

    fn get_tracking(&self) -> &Self::TrackingType {
        &self.tracking
    }

    /// We normalize to Option here because property was optional in CSAF 2.0
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, TestFinding> {
        Ok(&self.distribution)
    }

    /// Always return the value because it is mandatory
    fn get_distribution_20(&self) -> Option<&Self::DistributionType> {
        Some(&self.distribution)
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_deref()
    }

    fn get_lang(&self) -> Option<CsafLanguage> {
        self.lang.as_deref().map(CsafLanguage::from)
    }

    fn get_source_lang(&self) -> Option<CsafLanguage> {
        self.source_lang.as_deref().map(CsafLanguage::from)
    }

    fn get_publisher(&self) -> &Self::PublisherType {
        &self.publisher
    }

    fn get_category(&self) -> CsafDocumentCategory {
        CsafDocumentCategory::from(&self.category)
    }

    fn get_references(&self) -> Option<&Vec<Self::ReferenceType>> {
        self.references.as_deref()
    }

    fn get_involvement(&self) -> Option<&Self::InvolvementType> {
        self.involvement.as_ref()
    }

    fn get_csaf_version(&self) -> CsafVersion {
        match &self.csaf_version {
            CsafVersion21::X21 => CsafVersion::X21,
        }
    }

    impl_str_field_getter!(get_title, title);

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        self.x_extensions.as_ref()
    }
}
