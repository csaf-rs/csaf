use core::fmt;

use crate::schema::loose;

mod update_csaf_schema;
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
    &update_csaf_schema::UpdateCsafSchema,
    &update_csaf_version::UpdateCsafVersion,
    &update_tlp_label::UpdateTlpLabel,
];

/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    fn convert(&self, scope: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}

pub trait Diagnostics {
    fn error(&mut self, error: Error);
    fn warning(&mut self, warning: Warning);
}

#[derive(Default)]
pub struct Settings {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    CsafVersionIsMissing,
    InvalidCsafVersion,
    InvalidTlpLabel,
    TlpIsMissingLabel,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Warning::CsafVersionIsMissing => "/document/csaf_version node is missing",
            Warning::InvalidCsafVersion => "/document/csaf_version contained an invalid value",
            Warning::InvalidTlpLabel => "/document/distribution/tlp/label contained an invalid value",
            Warning::TlpIsMissingLabel => {
                "/document/distribution/tlp was present but it did not contain the `label` property"
            },
        };
        f.write_str(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Todo,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
        fn error(&mut self, error: Error) {
            panic!("expected no diagnostics; got {error:?}")
        }

        fn warning(&mut self, warning: Warning) {
            panic!("expected no diagnostics; got {warning:?}")
        }
    }

    #[derive(Default)]
    pub struct OnlyWarnings {
        warnings: Vec<Warning>,
    }

    impl Diagnostics for OnlyWarnings {
        fn error(&mut self, error: Error) {
            panic!("expected no errors; got {error:?}")
        }

        fn warning(&mut self, warning: Warning) {
            self.warnings.push(warning)
        }
    }

    impl OnlyWarnings {
        pub fn warnings(&self) -> &[Warning] {
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
            }
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
        });
        let doc = convert(input, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());
    }
}
