# Data structures and interfaces for validation, conversion and quick fixes

## TL;DR how to use the sketch

Run validator and converter unit tests
``` rust
$ # all commands must be executed from the `/sketch` directory
$ cargo test
```

Validate an input CSAF document
``` rust
$ cargo run -- validate input.json
```

Convert an input CSAF document
``` rust
$ cargo run -- convert input.json
```

## Background

Tools like the validator and converter need to operate on likely invalid CSAF documents.
The words "valid" and "invalid" are not fine grained enough to express correctness so we further define the following levels of correctness.

a. Syntactically invalid
b. Syntactically valid but does not adhere to the CSAF schema
c. Adheres to the CSAF schema but does not pass CSAF validation tests
d. Passes CSAF validation tests

This document proposes operating on an intermediate representation that corresponds to, at least, correctness level (b).
For documents that have correctness level (a), it should be possible to fix the syntactic errors or omit incorrect sections and raise the correctness level to (b).
But as these operations are done at the syntactic level and are CSAF agnostic they will not be covered in this proposal but could be performed as a preprocessing step in, for example, the converter.

## `csaf20` and `csaf21` types

These types represent schema valid CSAF document that can sit at correctness levels (c) or (d).
These types already exist in the repository so not much more will be said above them.
It should be possible to deserialize a string, that is a JSON document, into these types.
These types are useful for end users and should be exposed in a *library*.
Unless there is user demand for it, traits that abstract over both `csaf20` and `csaf21` types should *not* be added to a first iteration of the library.
Such traits won't be used by the tools and the concrete types are already useful as DTO (Data Transfer Object) without any methods as they allow for type-safe modifications.

``` rust
pub mod csaf20 {
    #[derive(Deserialize, Serialize)]
    pub struct Csaf {
        pub document: DocumentLevelMetadata,
        // ..
    }
    
    #[derive(Deserialize, Serialize)]
    pub enum CsafVersion {
        #[serde(rename = "2.0")]
        X20,
    }
}

pub mod csaf21 {
    #[derive(Deserialize, Serialize)]
    pub struct Csaf {
        pub document: DocumentLevelMetadata,
        // ..
    }

    #[derive(Deserialize, Serialize)]
    pub enum CsafVersion {
        #[serde(rename = "2.1")]
        X21,
    }
}
```

## `csaf2x` types

The tools in this repository need to deal with likely invalid CSAF 2.0 and likely invalid CSAF 2.1 documents.
Furthermore quick fixes and conversions need to mutate the document to make it valid, or at least more valid than it originally was.
To represent all the intermediate states of incremental conversion, this document proposes a collection of `csaf2x` types that are new types over `serde_json::Value`, which is a "raw" JSON value.

``` rust
/// path: `/`
pub struct Csaf {
    raw: serde_json::Value,
}
```

These types should stay as an implementation detail of the CSAF tools and should *not* be part of the public API of user-facing libraries.

### Traversing 

These types provide methods to access different parts of the CSAF document.
As the document is at correctness (b) all access methods are fallible and return an `Option` to indicate the absence of a node.

``` rust
/// path: `/document`
pub struct Document(serde_json::Value);

impl Csaf {
    pub fn v2x_document(&self) -> Option<&Document> { .. }
    pub fn v2x_document_mut(&mut self) -> Option<&mut Document> { .. }
}
```

The `csaf2x` types must represent either a CSAF 2.0 or a CSAF 2.1 document so they must provide access methods for all nodes that are present in *either* version.
For methods that work on both 2.0 and 2.1 documents, the method name shall start with the `v2x_` prefix.
For methods that would only work on 2.0 documents, the prefix shall be `v20_`.
And for methods that would only work on 2.1 documents, `v21_`.

``` rust
/// path: `/document/distribution`
pub struct RulesForSharingDocument(serde_json::Value);

impl RulesForSharingDocument {
    // NOTE: only present in CSAF 2.1 documents
    pub fn v21_sharing_group(&self) -> Option<&SharingGroup> { .. }
    // ..
}
```

### Reading 

Except for the top level `Csaf` type, all `csaf2x` types appear behind references.
For leaf node types, the value behind the reference can be read into a specific `v20` or `v21` type.
This read operation is effectively lazy, or deferred, deserialization.

``` rust
/// path: `/document/csaf_version`
pub struct CsafVersion(serde_json::Value);

impl CsafVersion {
    // NOTE: deserializes the node into a CSAF 2.0 type
    pub fn v20_get(&self) -> Result<csaf20::CsafVersion> { .. }
    // NOTE: deserializes the node into a CSAF 2.1 type
    pub fn v21_get(&self) -> Result<csaf21::CsafVersion> { .. }
}
```

In the case a `csaf2x` node is only present on one CSAF version and not the other then only a getter for that particular version will be provided.
Note that the name of the getters follow the naming convention that the traversal API uses.

### Modifying 

The converter and the quick fixes will need to modify the document in place.
For this functionality, setters are provided on the leaf node types.
These allow writing either a `csaf20` or `csaf21` value into the document node.

``` rust
/// path: `/document/csaf_version`
pub struct CsafVersion(serde_json::Value);

impl CsafVersion {
    pub fn v20_set(&mut self, value: csaf20::CsafVersion) { .. }
    pub fn v21_set(&mut self, value: csaf21::CsafVersion) { .. }
}
```

The naming convention for these methods follow the traversal and read API.

### Casting

Nodes that represent "objects" in the JSON document should provide a method to cast the node into a raw `&mut serde_json::Map`.
This is so the converter and the quick fixes can add or remove child nodes from the node.

``` rust
/// path: `/document/distribution/tlp`
pub struct TrafficLightProtocolTlp(serde_json::Value);

impl TrafficLightProtocolTlp {
    /// NOTE: casts the node into a JSON object
    pub fn as_object_mut(&mut self) -> Option<&mut Object> { .. }
}

pub type Object = serde_json::Map<String, serde_json::Value>;
```

## Code generation

All the `csaf2x`, `csaf20` and `csaf21` types and API must be generated from the official CSAF schemas.

## Converter

### Updated trait

This document proposes using the interfaces and test setup proposed in PR1141 but adjusted to operate on `csaf2x` types.

``` rust
/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    type Scope;

    /// Reduces the scope of the operation to avoid unintendedly mutating unrelated parts
    fn scope<'doc>(&self, doc: &'doc mut csaf2x::Csaf) -> Option<&'doc mut Self::Scope>;

    /// Performs the actual conversion on the reduced `scope`
    fn convert(&self, scope: &mut Self::Scope, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}
```

### Example conversion

An example conversion is shown below:

``` rust
// csaf-rs/csaf#1067
pub struct UpdateTlpLabel;

impl Convert for UpdateTlpLabel {
    type Scope = csaf2x::TrafficLightProtocolTlp;

    fn scope<'doc>(&self, doc: &'doc mut csaf2x::Csaf) -> Option<&'doc mut Self::Scope> {
        doc.v2x_document_mut()?.v2x_distribution_mut()?.v2x_tlp_mut()
    }

    fn convert(&self, tlp: &mut Self::Scope, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let clear = csaf21::LabelOfTlp::Clear;
        if let Some(label) = tlp.v2x_label_mut() {
            if let Ok(v20_label) = label.v20_get() {
                if v20_label == csaf20::LabelOfTlp::White {
                    label.v21_set(clear);
                }
            } else {
                // XXX error/warning? invalid value as per CSAF 2.0 schema
            }
            return;
        }

        if let Some(tlp) = tlp.as_object_mut() {
            tlp.insert("label".to_string(), clear.into());
            diagnostics.warning(Warning::NoTlpLabel);
        } else {
            // XXX error/warning? invalid type as per CSAF 2.0 schema
        }
    }
}
```

Operating on `csaf2x` types is less error prone than operating directly on `serde_json::Value` as it was originally proposed in PR1141.
It also makes easier to visualize and handle all error paths due to schema and deserialization errors.

### Example unit test

Operating on `csaf2x` types instead of an schema valid CSAF 2.0 document, also makes it easier to write unit tests as the test inputs can be minimal JSON objects.

``` rust
#[test]
fn has_white_tlp_label() -> serde_json::Result<()> {
    let input = json!({
        "document": {
            "distribution": {
                "tlp": {
                    "label": "WHITE"
                }
            }
        }
    });
    let output = json!({
        "document": {
            "distribution": {
                "tlp": {
                    "label": "CLEAR"
                }
            }
        }
    });
    let mut doc = csaf2x::Csaf::new(input);
    convert::execute(&UpdateTlpLabel, &mut doc, &Settings::default(), &mut NoDiagnostics);
    assert_eq!(output, doc.into_raw());

    Ok(())
}
```

### Other aspects

The other aspects around module organization and conformance testing remain the same as in PR1141.

## Validation

The validator should also operate on `csaf2x` types.
As no assumption is made whether the document actually fulfills the schema, even when `document.csaf_version` specifies a certain version, then
the use of generics and traits can be replaced with an enum based approach.
An option for an updated `Validate` trait is sketched below:

``` rust
/// A single CSAF test
pub trait Validate {
    /// Which CSAF version, or versions, does this validation apply to?
    fn applies_to(&self) -> VersionSelector;

    /// Validation logic
    ///
    /// Only runs if the stated version matches the selector returned by `applies_to`
    fn validate(&self, doc: &csaf2x::Csaf, version: StatedVersion, diagnostics: &mut dyn Diagnostics);
}

/// Stated version
///
/// Either specified via the command-line interface or extracted from the CSAF document
pub enum StatedVersion {
    V20,
    V21,
}

pub enum VersionSelector {
    /// 2.0 Only
    V20,
    /// 2.1 Only
    V21,
    /// Both 2.0 and 2.1
    V2x,
}
```

### Example validation

``` rust
/// 6.1.12 Language
pub struct T6_1_12;

impl Validate for T6_1_12 {
    fn applies_to(&self) -> VersionSelector {
        VersionSelector::V2x
    }

    fn validate(&self, csaf: &csaf2x::Csaf, _version: StatedVersion, diagnostics: &mut dyn Diagnostics) {
        let Some(doc) = csaf.v2x_document() else {
            // nothing to do if node is missing
            return;
        };

        let locations = [doc.v2x_lang(), doc.v2x_source_lang()];
        for lang in locations {
            if let Some(lang) = lang {
                if let Ok(lang) = lang.v2x_get() {
                    check_language_code(&lang, diagnostics)
                } else {
                    // XXX error/warning? invalid value as per CSAF schema
                }
            }
        }
    }
}
```

Unit tests:

``` rust
#[test]
fn pass() {
    let input = json!({
        "document": {
            "lang": "EN",
            "source_lang": "EN",
        }
    });

    let doc = csaf2x::Csaf::new(input);
    for version in [StatedVersion::V20, StatedVersion::V21] {
        validate::execute(&T6_1_12, version, &doc, &mut NoDiagnostics);
    }
}

#[test]
fn fail() {
    let input = json!({
        "document": {
            "lang": "EZ",
        }
    });

    let doc = csaf2x::Csaf::new(input);
    for version in [StatedVersion::V20, StatedVersion::V21] {
        let mut diagnostics = DiagnosticsSpy::default();
        validate::execute(&T6_1_12, version, &doc, &mut diagnostics);
        assert!(diagnostics.warnings().is_empty());
        assert_eq!([Error::InvalidLanguageCode], diagnostics.errors());
    }
}
```

## Quick fixes

This is pending a more concrete design but the quick fix trait, if the `Convert` trait cannot be used as is, should be similar to the `Convert` trait and also operate on `v2x` types.
