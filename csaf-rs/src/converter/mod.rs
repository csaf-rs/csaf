use core::fmt;
use std::borrow::Cow;

use crate::schema::loose;

mod checks;
mod convert_contact_details;
mod convert_release_date_to_disclosure_date;
mod deduplicate_sbom_urls;
mod deduplicate_skus;
mod deduplicate_x_generic_uris;
mod helpers;
mod lowercase_hash_value;
mod move_purl_into_purls_array;
mod replace_legacy_branch_category;
mod set_csaf_21_schema;
mod set_initial_release_date;
mod settings;
mod sort_json_keys;
mod update_csaf_version;
mod update_publisher_category;
mod update_tlp_label;

pub use checks::{is_valid_email, is_valid_public_openpgp_key_url};
pub use settings::{PublisherCategoryOtherSettings, PublisherContactSettings, Settings};

pub fn convert(raw: serde_json::Value, settings: &Settings, diagnostics: &mut dyn Diagnostics) -> loose::Document {
    let mut doc = loose::Document::new(raw);

    for conversion in CONVERSIONS {
        conversion.convert(&mut doc, settings, diagnostics)
    }

    doc
}

const CONVERSIONS: &[&dyn Convert] = &[
    // conversions that work on leaf nodes
    &lowercase_hash_value::LowercaseHashValue,
    &replace_legacy_branch_category::ReplaceLegacyBranchCategory,
    &set_csaf_21_schema::SetCsaf21Schema,
    &update_csaf_version::UpdateCsafVersion,
    &update_tlp_label::UpdateTlpLabel,
    &update_publisher_category::UpdatePublisherCategory,
    &set_initial_release_date::SetInitialReleaseDate,
    // conversions that move leaf nodes
    &move_purl_into_purls_array::MoveSinglePurlIntoPurlsArray,
    &convert_release_date_to_disclosure_date::ConvertReleaseDateToDisclosureDate,
    &deduplicate_sbom_urls::DeduplicateSbomUrls,
    &deduplicate_skus::DeduplicateSkus,
    &deduplicate_x_generic_uris::DeduplicateXGenericUris,
    &convert_contact_details::ConvertContactDetails,
    // conversions that move subtrees
    // sorting keys should be last
    &sort_json_keys::SortJsonKeys,
];

/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    fn convert(&self, scope: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}

pub trait Diagnostics {
    fn error(&mut self, path: Cow<'static, str>, error: Error);
    fn has_errors(&self) -> bool;
    fn warning(&mut self, path: Cow<'static, str>, warning: Warning);
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    ValidationWarning(String),
    CsafVersionIsMissing,
    InvalidCsafVersion,
    LegacyBranchCategoryReplaced,
    InvalidProductTree,
    InvalidBranches,
    PublisherCategoryOther,
    InconsistentInitialReleaseDate { old: String, new: String },
    DefaultTlpLabelSet,
    TlpLabelFromDistributionText { converted: String, from_text: String },
    PublisherContactOpenPgpKeyUrlRequiresEmail { url: String },
    PublisherContactInvalidFormat { property: &'static str, value: String },
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Warning::ValidationWarning(message) => f.write_str(message),
            Warning::InconsistentInitialReleaseDate { old, new } => write!(
                f,
                "An inconsistent initial release date `{old}` was detected and replaced with \
                the date `{new}` from the revision history item with a number of 1 or 1.0.0"
            ),
            Warning::CsafVersionIsMissing => f.write_str("CSAF version node is missing"),
            Warning::InvalidCsafVersion => f.write_str("CSAF version node contained an invalid value"),
            Warning::DefaultTlpLabelSet => f.write_str("default TLP has been set to CLEAR"),
            Warning::TlpLabelFromDistributionText { converted, from_text } => {
                write!(
                    f,
                    "TLP label was taken from distribution text \
                     (converted value: {converted}, text value: {from_text})"
                )
            },
            Warning::LegacyBranchCategoryReplaced => f.write_str(
                "Branch category 'legacy' does not exist in CSAF 2.1 \
                and was replaced with 'product_name'. This replacement \
                may be incorrect or another category may be a better fit. \
                Please review the converted document to ensure the \
                conversion is correct and not misleading.",
            ),
            Warning::InvalidProductTree => f.write_str("Skipped branch traversal: product_tree is not a JSON object"),
            Warning::InvalidBranches => f.write_str("Skipped branch traversal: branches is not a JSON array"),
            Warning::PublisherCategoryOther => f.write_str(
                "Some publishers previously categorized as 'other' \
                have been regrouped into the new 'multiplier' category in CSAF 2.1. \
                Please manually check whether the publisher category should be changed to 'multiplier'. \
                The converter provides options to perform this change automatically or suppress this warning.",
            ),
            Warning::PublisherContactOpenPgpKeyUrlRequiresEmail { url } => write!(
                f,
                "The URL `{url}` in the publisher contact details was recognized as a public OpenPGP key URL, \
                but it was not adopted as `public_openpgp_key_url` because no email address could be \
                associated with it. CSAF 2.1 requires the `email` property when `public_openpgp_key_url` \
                is set. The URL remains part of the `details` property. \
                The converter provides options to set `email` (and `public_openpgp_key_url`) explicitly.",
            ),
            Warning::PublisherContactInvalidFormat { property, value } => write!(
                f,
                "The configured value `{value}` for the publisher contact property `{property}` does not \
                fulfill the `format` requirements of the CSAF 2.1 schema and was ignored.",
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ValidationError(String),
    Todo,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::ValidationError(message) => message,
            Error::Todo => "TODO",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    pub struct NoDiagnostics;

    impl Diagnostics for NoDiagnostics {
        fn error(&mut self, _path: Cow<'static, str>, error: Error) {
            panic!("expected no diagnostics; got {error:?}")
        }

        fn has_errors(&self) -> bool {
            panic!("no errors")
        }

        fn warning(&mut self, _path: Cow<'static, str>, warning: Warning) {
            panic!("expected no diagnostics; got {warning:?}")
        }
    }

    #[derive(Default)]
    pub struct OnlyWarnings {
        warnings: Vec<(Cow<'static, str>, Warning)>,
    }

    impl Diagnostics for OnlyWarnings {
        fn error(&mut self, _path: Cow<'static, str>, error: Error) {
            panic!("expected no errors; got {error:?}")
        }

        fn has_errors(&self) -> bool {
            false
        }

        fn warning(&mut self, path: Cow<'static, str>, warning: Warning) {
            self.warnings.push((path, warning))
        }
    }

    impl OnlyWarnings {
        pub fn warnings(&self) -> &[(Cow<'static, str>, Warning)] {
            &self.warnings
        }
    }

    // Sanity check that all the `CONVERSIONS` are executed, without
    // attempting to test interaction between conversions
    #[test]
    fn end_to_end() {
        let input = json!({
            "document": {
                "csaf_version": "2.0",
                "distribution": {
                    "tlp": {
                        "label": "WHITE"
                    }
                }
            },
            "product_tree": {
                "full_product_names": [
                    {
                        "name": "Product A",
                        "product_id": "CSAFPID-0001",
                        "product_identification_helper": {
                            "purl": "pkg:maven/com.example/product-A",
                            "sbom_urls": [
                                "https://example.com/sbom-a.json",
                                "https://example.com/sbom-a.json"
                            ],
                            "skus": ["AAA", "AAA"],
                            "x_generic_uris": [
                                {
                                    "namespace": "https://example.com",
                                    "uri": "https://example.com/uri-a"
                                },
                                {
                                    "namespace": "https://example.com",
                                    "uri": "https://example.com/uri-a"
                                }
                            ]
                        }
                    }
                ],
            },
            "vulnerabilities": [
                {
                    "release_date": "2025-03-11T15:12:00.000Z"
                }
            ],
        });
        let output = json!({
            "$schema": "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
            "document": {
                "csaf_version": "2.1",
                "distribution": {
                    "tlp": {
                        "label": "CLEAR"
                    }
                }
            },
             "product_tree": {
                "full_product_names": [
                    {
                        "name": "Product A",
                        "product_id": "CSAFPID-0001",
                        "product_identification_helper": {
                            "purls": ["pkg:maven/com.example/product-A"],
                            "sbom_urls": [
                                "https://example.com/sbom-a.json",
                            ],
                            "skus": ["AAA"],
                            "x_generic_uris": [
                                {
                                    "namespace": "https://example.com",
                                    "uri": "https://example.com/uri-a"
                                }
                            ]
                        }
                    }
                ],
            },
            "vulnerabilities": [
                {
                    "disclosure_date": "2025-03-11T15:12:00.000Z"
                }
            ],
        });
        let doc = convert(input, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());
    }
}
