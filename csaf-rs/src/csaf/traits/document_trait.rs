use crate::csaf::enums::csaf_version::CsafVersion;
use crate::csaf::traits::util::extract_references::{
    ExtractGroupReferences, ExtractProductReferences, define_reference_accessors,
};
use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{
    AcknowledgmentTrait, AggregateSeverityTrait, DistributionTrait, ExtensionsTrait, NoteTrait, PublisherTrait,
    ReferenceTrait, TrackingTrait,
};
use crate::schema::csaf2_0::schema::{
    Acknowledgment as Acknowledgment20, AggregateSeverity as AggregateSeverity20, CsafVersion as CsafVersion20,
    DocumentLevelMetaData as DocumentLevelMetaData20, Note as Note20, Publisher as Publisher20,
    Reference as Reference20, RulesForSharingDocument as RulesForSharingDocument20, Tracking as Tracking20,
};
use crate::schema::csaf2_1::schema::{
    Acknowledgment as Acknowledgment21, AggregateSeverity as AggregateSeverity21,
    CategoryOfReference as CategoryOfReference21, CsafVersion as CsafVersion21,
    DocumentLevelMetaData as DocumentLevelMetaData21, ExtensionsT as Extensions21, Note as Note21,
    Publisher as Publisher21, Reference as Reference21, RulesForDocumentSharing as RulesForDocumentSharing21,
    Tracking as Tracking21,
};
use crate::validation::{TestFinding, TestFindingData};

/// Returns an iterator over the reference URLs that satisfy the canonical URL requirements:
/// `category = "self"`, starts with `https://`, has a non-empty hostname,
/// and has the tracking-ID-derived filename preceded by `/`.
fn canonical_url_candidates<'a, R: ReferenceTrait>(
    references: Option<&'a Vec<R>>,
    expected_filename: &str,
) -> impl Iterator<Item = &'a str> {
    references
        .into_iter()
        .flatten()
        // using CategoryOfReference21 here is fine, CategoryOfReference20 is 1:1 mapped to this
        .filter(|r| r.get_category() == CategoryOfReference21::Self_)
        .map(|r| r.get_url())
        .filter(move |url| {
            // Check that the URL starts with "https://"
            if let Some(after_scheme) = url.strip_prefix("https://")
                // Check that there is a '/' after the authority
                && let Some((authority, path)) = after_scheme.split_once('/')
                // Check that the authority is non-empty
                && !authority.is_empty()
                // Check that the authority has a non-empty hostname
                && has_non_empty_hostname(authority)
                // Check that the last segment of the path matches the expected filename
                && path.rsplit('/').next() == Some(expected_filename)
            {
                true
            } else {
                false
            }
        })
}

// URI syntax is validated separately by the JSON Schema ('format: "uri"').
// This helper only checks the additional 6.2.11 CSAF 2.1 requirement that the hostname is non-empty.
fn has_non_empty_hostname(authority: &str) -> bool {
    // Strip optional userinfo
    let hostname_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, hostname_port)| hostname_port);

    // Strip optional port (or the part after the last ':' for IPv6)
    let hostname = hostname_port
        .rsplit_once(':')
        .map_or(hostname_port, |(hostname, _)| hostname);

    // The hostname is non-empty if any character remains after handling userinfo and port
    !hostname.is_empty()
}

/// Trait representing document meta-level information
pub trait DocumentTrait {
    type AcknowledgmentType: AcknowledgmentTrait;

    type AggregateSeverityType: AggregateSeverityTrait;
    /// Type representing document tracking information
    type TrackingType: TrackingTrait;

    /// Type representing document distribution information
    type DistributionType: DistributionTrait;

    /// Type representing document notes
    type NoteType: NoteTrait;

    /// Type representing document publisher information
    type PublisherType: PublisherTrait;

    type ReferenceType: ReferenceTrait;

    type ExtensionsType: ExtensionsTrait;

    fn get_acknowledgments(&self) -> Option<&Vec<Self::AcknowledgmentType>>;

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
    fn get_references(&self) -> Option<&Vec<Self::ReferenceType>>;

    /// Returns the canonical URLs from this document's references.
    fn get_canonical_urls(&self) -> Vec<&str> {
        let expected_filename = self.get_tracking().get_canonical_filename();
        canonical_url_candidates(self.get_references(), expected_filename.as_str()).collect()
    }

    /// Returns `true` if the document has at least one canonical URL.
    fn has_canonical_url(&self) -> bool {
        let expected_filename = self.get_tracking().get_canonical_filename();
        canonical_url_candidates(self.get_references(), expected_filename.as_str())
            .next()
            .is_some()
    }

    fn get_csaf_version(&self) -> CsafVersion;

    /// Returns the title of this document
    fn get_title(&self) -> &str;

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
}

impl DocumentTrait for DocumentLevelMetaData21 {
    type AcknowledgmentType = Acknowledgment21;
    type AggregateSeverityType = AggregateSeverity21;
    type TrackingType = Tracking21;
    type DistributionType = RulesForDocumentSharing21;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::csaf2_1::schema::Reference as Reference21;
    use rstest::rstest;
    use serde_json::json;

    /// Reference21 mock, Reference20 is exactly the same though
    fn make_ref21(category: &str, url: &str) -> Reference21 {
        serde_json::from_value(json!({
            "category": category,
            "summary": "x",
            "url": url
        }))
        .unwrap()
    }

    const FILENAME: &str = "example-company-2019-yh3234.json";
    const HTTPS_MATCH: &str = "https://example.com/example-company-2019-yh3234.json";
    const HTTPS_MATCH_WELL_KNOWN: &str = "https://example.com/.well-known/csaf/clear/example-company-2019-yh3234.json";
    const HTTP_MATCH: &str = "http://example.com/example-company-2019-yh3234.json";

    #[rstest]
    // empty
    #[case::no_references(None, 0)]
    // canonical URLs
    #[case::match_simple_path(
        Some(vec![make_ref21("self", HTTPS_MATCH)]),
        1
    )]
    #[case::match_well_known(
        Some(vec![make_ref21("self", HTTPS_MATCH_WELL_KNOWN)]),
        1
    )]
    #[case::userinfo_and_port_with_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:pass@example.com:443/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipv4_hostname(
        Some(vec![make_ref21(
        "self",
        "https://192.168.1.1/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipv6_hostname(
        Some(vec![make_ref21(
        "self",
        "https://[2001:db8::1]/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipvfuture_hostname(
        Some(vec![make_ref21(
        "self",
        "https://[v1.foo]/example-company-2019-yh3234.json"
    )]),
        1
    )]
    // non-canonical URLs
    #[case::reference_category_external(
        Some(vec![make_ref21("external", HTTPS_MATCH)]),
        0
    )]
    #[case::url_scheme_http(
        Some(vec![make_ref21("self", HTTP_MATCH)]),
        0
    )]
    #[case::url_scheme_ftp(
        Some(vec![make_ref21(
        "self",
        "ftp://example.com/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::wrong_filename(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3235.json"
    )]),
        0
    )]
    #[case::only_domain(
        Some(vec![make_ref21("self", "https://example.com")]),
        0
    )]
    #[case::only_domain_with_delim(
        Some(vec![make_ref21("self", "https://example.com/")]),
        0
    )]
    #[case::empty_authority(
        Some(vec![make_ref21(
        "self",
        "https:///example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::empty_authority_additional_path(
        Some(vec![make_ref21(
        "self",
        "https:////example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::filename_as_authority(
        Some(vec![make_ref21(
        "self",
        "https://example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::with_fragment(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3234.json#fragment"
    )]),
        0
    )]
    #[case::with_param(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3234.json?foo=1"
    )]),
        0
    )]
    #[case::userinfo_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:password@/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::port_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://:443/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::userinfo_and_port_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:password@:443/example-company-2019-yh3234.json"
    )]),
        0
    )]
    fn test_candidate_filtering(#[case] refs: Option<Vec<Reference21>>, #[case] expected_count: usize) {
        let refs_ref = refs.as_ref();
        assert_eq!(canonical_url_candidates(refs_ref, FILENAME).count(), expected_count);
    }

    #[test]
    fn only_matching_refs_are_returned_from_mixed_list() {
        let refs = vec![
            make_ref21("self", HTTPS_MATCH),
            make_ref21("self", HTTP_MATCH),
            make_ref21("external", HTTPS_MATCH),
            make_ref21("self", HTTPS_MATCH_WELL_KNOWN),
        ];

        let result: Vec<_> = canonical_url_candidates(Some(&refs), FILENAME).collect();

        assert_eq!(result, vec![HTTPS_MATCH, HTTPS_MATCH_WELL_KNOWN]);
    }
}
