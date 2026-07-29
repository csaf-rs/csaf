use crate::csaf::enums::csaf_version::CsafVersion;
use crate::schema::csaf2_0::schema::{
    CategoryOfPublisher as CategoryOfPublisher20, CategoryOfReference as CategoryOfReference20,
    DocumentLevelMetaData as DocumentLevelMetaData20, Publisher as Publisher20, Reference as Reference20,
};
use crate::schema::csaf2_1::schema::{
    CategoryOfPublisher, CategoryOfReference, DocumentLevelMetaData as DocumentLevelMetaData21,
    Publisher as Publisher21, Reference as Reference21,
};

/// Experimental, zero-copy view of document metadata that may be incomplete.
///
/// This mirrors only a small part of `DocumentTrait`. Return-position `impl
/// Trait` lets typify-backed and JSON-backed implementations return their own
/// borrowed view types without allocating wrapper collections.
pub trait NonStrictDocumentTrait {
    fn get_title(&self) -> Option<&str>;

    fn get_publisher(&self) -> Option<impl NonStrictPublisherTrait + '_>;

    fn get_references(&self) -> Option<impl Iterator<Item = impl NonStrictDocumentReferenceTrait + '_> + '_>;
}

/// Experimental, optional view of publisher data.
pub trait NonStrictPublisherTrait {
    fn get_category(&self) -> Option<CategoryOfPublisher>;
}

/// Experimental, optional view of document reference data.
pub trait NonStrictDocumentReferenceTrait {
    fn get_category(&self) -> Option<CategoryOfReference>;

    fn get_summary(&self) -> Option<&str>;

    fn get_url(&self) -> Option<&str>;
}

struct TypedPublisher20Ref<'a>(&'a Publisher20);
struct TypedPublisher21Ref<'a>(&'a Publisher21);
struct TypedReference20Ref<'a>(&'a Reference20);
struct TypedReference21Ref<'a>(&'a Reference21);

impl NonStrictPublisherTrait for TypedPublisher20Ref<'_> {
    fn get_category(&self) -> Option<CategoryOfPublisher> {
        Some(match self.0.category {
            CategoryOfPublisher20::Coordinator => CategoryOfPublisher::Coordinator,
            CategoryOfPublisher20::Discoverer => CategoryOfPublisher::Discoverer,
            CategoryOfPublisher20::Other => CategoryOfPublisher::Other,
            CategoryOfPublisher20::Translator => CategoryOfPublisher::Translator,
            CategoryOfPublisher20::User => CategoryOfPublisher::User,
            CategoryOfPublisher20::Vendor => CategoryOfPublisher::Vendor,
        })
    }
}

impl NonStrictPublisherTrait for TypedPublisher21Ref<'_> {
    fn get_category(&self) -> Option<CategoryOfPublisher> {
        Some(self.0.category)
    }
}

impl NonStrictDocumentReferenceTrait for TypedReference20Ref<'_> {
    fn get_category(&self) -> Option<CategoryOfReference> {
        Some(match self.0.category {
            CategoryOfReference20::External => CategoryOfReference::External,
            CategoryOfReference20::Self_ => CategoryOfReference::Self_,
        })
    }

    fn get_summary(&self) -> Option<&str> {
        Some(&self.0.summary)
    }

    fn get_url(&self) -> Option<&str> {
        Some(&self.0.url)
    }
}

impl NonStrictDocumentReferenceTrait for TypedReference21Ref<'_> {
    fn get_category(&self) -> Option<CategoryOfReference> {
        Some(self.0.category)
    }

    fn get_summary(&self) -> Option<&str> {
        Some(&self.0.summary)
    }

    fn get_url(&self) -> Option<&str> {
        Some(&self.0.url)
    }
}

impl NonStrictDocumentTrait for DocumentLevelMetaData20 {
    fn get_title(&self) -> Option<&str> {
        Some(&self.title)
    }

    fn get_publisher(&self) -> Option<impl NonStrictPublisherTrait + '_> {
        Some(TypedPublisher20Ref(&self.publisher))
    }

    fn get_references(&self) -> Option<impl Iterator<Item = impl NonStrictDocumentReferenceTrait + '_> + '_> {
        self.references
            .as_deref()
            .map(|references| references.iter().map(TypedReference20Ref))
    }
}

impl NonStrictDocumentTrait for DocumentLevelMetaData21 {
    fn get_title(&self) -> Option<&str> {
        Some(&self.title)
    }

    fn get_publisher(&self) -> Option<impl NonStrictPublisherTrait + '_> {
        Some(TypedPublisher21Ref(&self.publisher))
    }

    fn get_references(&self) -> Option<impl Iterator<Item = impl NonStrictDocumentReferenceTrait + '_> + '_> {
        self.references
            .as_deref()
            .map(|references| references.iter().map(TypedReference21Ref))
    }
}

/// Borrowed non-strict document metadata view over arbitrary JSON.
#[derive(Clone, Copy)]
pub struct JsonDocumentRef<'a> {
    pub value: &'a serde_json::Value,
    pub version: CsafVersion,
}

impl<'a> JsonDocumentRef<'a> {
    pub fn new(value: &'a serde_json::Value, version: CsafVersion) -> Self {
        Self { value, version }
    }
}

#[derive(Clone, Copy)]
pub struct JsonPublisherRef<'a> {
    pub value: &'a serde_json::Value,
    pub version: CsafVersion,
}

#[derive(Clone, Copy)]
pub struct JsonDocumentReferenceRef<'a> {
    pub value: &'a serde_json::Value,
    pub version: CsafVersion,
}

impl NonStrictDocumentTrait for JsonDocumentRef<'_> {
    fn get_title(&self) -> Option<&str> {
        self.value.get("title")?.as_str()
    }

    fn get_publisher(&self) -> Option<impl NonStrictPublisherTrait + '_> {
        let publisher = self.value.get("publisher")?;
        publisher.as_object()?;
        Some(JsonPublisherRef {
            value: publisher,
            version: self.version,
        })
    }

    fn get_references(&self) -> Option<impl Iterator<Item = impl NonStrictDocumentReferenceTrait + '_> + '_> {
        Some(
            self.value
                .get("references")?
                .as_array()?
                .iter()
                .map(|value| JsonDocumentReferenceRef {
                    value,
                    version: self.version,
                }),
        )
    }
}

impl NonStrictPublisherTrait for JsonPublisherRef<'_> {
    fn get_category(&self) -> Option<CategoryOfPublisher> {
        CategoryOfPublisher::try_from(self.value.get("category")?.as_str()?).ok()
    }
}

impl NonStrictDocumentReferenceTrait for JsonDocumentReferenceRef<'_> {
    fn get_category(&self) -> Option<CategoryOfReference> {
        CategoryOfReference::try_from(self.value.get("category")?.as_str()?).ok()
    }

    fn get_summary(&self) -> Option<&str> {
        self.value.get("summary")?.as_str()
    }

    fn get_url(&self) -> Option<&str> {
        self.value.get("url")?.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn assert_document_view(document: &impl NonStrictDocumentTrait, title: &str) {
        assert_eq!(document.get_title(), Some(title));
        assert_eq!(
            document.get_publisher().and_then(|publisher| publisher.get_category()),
            Some(CategoryOfPublisher::Vendor)
        );

        let mut references = document.get_references().expect("references are present");
        let reference = references.next().expect("reference is present");
        assert_eq!(reference.get_category(), Some(CategoryOfReference::External));
        assert_eq!(reference.get_summary(), Some("Reference"));
        assert_eq!(reference.get_url(), Some("https://example.test/reference"));
        assert!(references.next().is_none());
    }

    #[test]
    fn typify_documents_use_borrowed_views() {
        let document_20: DocumentLevelMetaData20 = serde_json::from_value(json!({
            "category": "csaf_security_advisory",
            "csaf_version": "2.0",
            "publisher": {
                "category": "vendor",
                "name": "Example",
                "namespace": "https://example.test"
            },
            "title": "Typed 2.0",
            "tracking": {
                "current_release_date": "2026-01-01T00:00:00Z",
                "id": "CSAF-20",
                "initial_release_date": "2026-01-01T00:00:00Z",
                "revision_history": [{
                    "date": "2026-01-01T00:00:00Z",
                    "number": "1",
                    "summary": "Initial release"
                }],
                "status": "final",
                "version": "1"
            },
            "references": [{
                "category": "external",
                "summary": "Reference",
                "url": "https://example.test/reference"
            }]
        }))
        .expect("CSAF 2.0 fixture deserializes");
        let document_21: DocumentLevelMetaData21 = serde_json::from_value(json!({
            "category": "csaf_security_advisory",
            "csaf_version": "2.1",
            "distribution": { "tlp": { "label": "CLEAR" } },
            "publisher": {
                "category": "vendor",
                "name": "Example",
                "namespace": "https://example.test"
            },
            "title": "Typed 2.1",
            "tracking": {
                "current_release_date": "2026-01-01T00:00:00Z",
                "id": "CSAF-21",
                "initial_release_date": "2026-01-01T00:00:00Z",
                "revision_history": [{
                    "date": "2026-01-01T00:00:00Z",
                    "number": "1",
                    "summary": "Initial release"
                }],
                "status": "final",
                "version": "1"
            },
            "references": [{
                "category": "external",
                "summary": "Reference",
                "url": "https://example.test/reference"
            }]
        }))
        .expect("CSAF 2.1 fixture deserializes");

        assert_document_view(&document_20, "Typed 2.0");
        assert_document_view(&document_21, "Typed 2.1");
    }

    #[test]
    fn json_document_uses_borrowed_lazy_views() {
        let json = json!({
            "title": "JSON",
            "publisher": { "category": "vendor" },
            "references": [{
                "category": "external",
                "summary": "Reference",
                "url": "https://example.test/reference"
            }]
        });
        let document = JsonDocumentRef::new(&json, CsafVersion::X21);

        assert_eq!(document.version, CsafVersion::X21);
        assert_document_view(&document, "JSON");
    }

    #[test]
    fn json_document_treats_missing_or_malformed_values_as_absent() {
        let json = json!({
            "title": 42,
            "publisher": { "category": "not-a-category" },
            "references": [{ "category": true, "summary": 42 }, "not-an-object"]
        });
        let document = JsonDocumentRef::new(&json, CsafVersion::X20);

        assert_eq!(document.version, CsafVersion::X20);
        assert_eq!(document.get_title(), None);
        assert_eq!(
            document.get_publisher().and_then(|publisher| publisher.get_category()),
            None
        );

        let mut references = document.get_references().expect("array is present");
        let first = references.next().expect("first value is wrapped");
        assert_eq!(first.get_category(), None);
        assert_eq!(first.get_summary(), None);
        assert_eq!(first.get_url(), None);
        let second = references.next().expect("second value is wrapped");
        assert_eq!(second.get_category(), None);
        assert_eq!(second.get_summary(), None);
        assert_eq!(second.get_url(), None);
        assert!(references.next().is_none());
    }
}
