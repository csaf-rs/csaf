use core::fmt;
use std::borrow::Cow;

use crate::schema::loose;

mod convert_release_date_to_disclosure_date;
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
    &set_csaf_21_schema::SetCsaf21Schema,
    &update_csaf_version::UpdateCsafVersion,
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
    CsafVersionIsMissing,
    InvalidCsafVersion,
    InvalidTlpLabel,
    TlpIsMissingLabel,
    ValidationWarning(String),
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Warning::CsafVersionIsMissing => "CSAF version node is missing",
            Warning::InvalidCsafVersion => "CSAF version node contained an invalid value",
            Warning::InvalidTlpLabel => "TLP label contained an invalid value",
            Warning::TlpIsMissingLabel => "TLP node was present but did not contain the `label` property",
        };
        f.write_str(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Todo,
    ValidationError(String),
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
