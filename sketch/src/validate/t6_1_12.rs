use crate::csaf2x;

use super::{Diagnostics, Error, StatedVersion, Validate, VersionSelector};

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

fn check_language_code(lang: &str, diagnostics: &mut dyn Diagnostics) {
    // FIXME simplified logic
    if lang != "EN" {
        diagnostics.error(Error::InvalidLanguageCode);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::validate::tests::{DiagnosticsSpy, NoDiagnostics};
    use crate::validate::{self, StatedVersion};

    use super::*;

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

    #[test]
    fn not_applicable() {
        let input = json!( {
            "document": {}
        });

        let doc = csaf2x::Csaf::new(input);
        for version in [StatedVersion::V20, StatedVersion::V21] {
            validate::execute(&T6_1_12, version, &doc, &mut NoDiagnostics);
        }
    }
}
