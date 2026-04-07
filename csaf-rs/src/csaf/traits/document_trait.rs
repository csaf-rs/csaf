use crate::csaf::enums::csaf_version::CsafVersion;
use crate::csaf::traits::util::extract_references::{
    ExtractGroupReferences, ExtractProductReferences, define_reference_accessors,
};
use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{DistributionTrait, DocumentReferenceTrait, NoteTrait, PublisherTrait, TrackingTrait};
use crate::schema::csaf2_0::schema::{
    CsafVersion as CsafVersion20, DocumentLevelMetaData as DocumentLevelMetaData20, Note as Note20,
    Publisher as Publisher20, Reference as Reference20, RulesForSharingDocument as RulesForSharingDocument20,
    Tracking as Tracking20,
};
use crate::schema::csaf2_1::schema::{
    CsafVersion as CsafVersion21, DocumentLevelMetaData as DocumentLevelMetaData21, Note as Note21,
    Publisher as Publisher21, Reference as Reference21, RulesForDocumentSharing as RulesForDocumentSharing21,
    Tracking as Tracking21,
};
use crate::validation::ValidationError;

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

    define_reference_accessors! {
        both: [
            (get_notes_group_references, get_notes_product_references, get_notes, "/document/notes"),
        ],
        custom_group_extraction: [],
        custom_product_extraction: [],
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
    fn get_references(&self) -> Option<&Vec<Self::DocumentReferenceType>>;

    fn get_csaf_version(&self) -> &CsafVersion;
}

impl DocumentTrait for DocumentLevelMetaData20 {
    type TrackingType = Tracking20;
    type DistributionType = RulesForSharingDocument20;
    type NoteType = Note20;
    type PublisherType = Publisher20;
    type DocumentReferenceType = Reference20;

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
        self.source_lang.as_deref().map(CsafLanguage::from)
    }

    fn get_publisher(&self) -> &Publisher20 {
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

impl DocumentTrait for DocumentLevelMetaData21 {
    type TrackingType = Tracking21;
    type DistributionType = RulesForDocumentSharing21;
    type NoteType = Note21;
    type PublisherType = Publisher21;
    type DocumentReferenceType = Reference21;

    fn get_tracking(&self) -> &Self::TrackingType {
        &self.tracking
    }

    /// We normalize to Option here because property was optional in CSAF 2.0
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError> {
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

    fn get_references(&self) -> Option<&Vec<Reference21>> {
        self.references.as_deref()
    }

    fn get_csaf_version(&self) -> &CsafVersion {
        match self.csaf_version {
            CsafVersion21::X21 => &CsafVersion::X21,
        }
    }
}
