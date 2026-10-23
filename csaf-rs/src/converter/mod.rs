use core::fmt;
use std::borrow::Cow;

use crate::schema::loose;

mod convert_release_date_to_disclosure_date;
mod helpers;
mod lowercase_hash_value;
mod move_purl_into_purls_array;
mod replace_legacy_branch_category;
mod set_csaf_21_schema;
mod update_csaf_version;
mod update_tlp_label;

pub fn convert(raw: serde_json::Value, settings: &Settings, diagnostics: &mut dyn Diagnostics) -> loose::Document {
    let mut doc = loose::Document::new(raw);

    for conversion in CONVERSIONS {
        conversion.convert(&mut doc, settings, diagnostics)
    }

    doc
}

const CONVERSIONS: &[&dyn Convert] = &[
    &lowercase_hash_value::LowercaseHashValue,
    &replace_legacy_branch_category::ReplaceLegacyBranchCategory,
    &set_csaf_21_schema::SetCsaf21Schema,
    &update_csaf_version::UpdateCsafVersion,
    &move_purl_into_purls_array::MoveSinglePurlIntoPurlsArray,
    &convert_release_date_to_disclosure_date::ConvertReleaseDateToDisclosureDate,
    &update_tlp_label::UpdateTlpLabel,
];

/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    fn convert(&self, scope: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}

pub trait Diagnostics {
    fn error(&mut self, path: Cow<'static, str>, error: Error);
    fn warning(&mut self, path: Cow<'static, str>, warning: Warning);
}

#[derive(Default)]
pub struct Settings {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    ValidationWarning(String),
    CsafVersionIsMissing,
    InvalidCsafVersion,
    InvalidTlpLabel,
    TlpIsMissingLabel,
    LegacyBranchCategoryReplaced,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Warning::ValidationWarning(message) => message,
            Warning::CsafVersionIsMissing => "CSAF version node is missing",
            Warning::InvalidCsafVersion => "CSAF version node contained an invalid value",
            Warning::InvalidTlpLabel => "TLP label contained an invalid value",
            Warning::TlpIsMissingLabel => "TLP node was present but did not contain the `label` property",
            Warning::LegacyBranchCategoryReplaced => {
                "Branch category 'legacy' does not exist in CSAF 2.1 \
                and was replaced with 'product_name'. This replacement \
                may be incorrect or another category may be a better fit. \
                Please review the converted document to ensure the \
                conversion is correct and not misleading."
            },
        };
        f.write_str(s)
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
                "csaf_version": "2.0"
            },
            "product_tree": {
                "full_product_names": [
                    {
                        "name": "Product A",
                        "product_id": "CSAFPID-0001",
                        "product_identification_helper": {
                            "purl": "pkg:maven/com.example/product-A"
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
                            "purls": ["pkg:maven/com.example/product-A"]
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
