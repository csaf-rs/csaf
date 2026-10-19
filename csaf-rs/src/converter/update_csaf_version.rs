use crate::schema::{csaf2_1, loose};

use super::{Convert, Diagnostics, Settings, Warning};

// csaf-rs/csaf#1066
pub struct UpdateCsafVersion;

impl Convert for UpdateCsafVersion {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let csaf_version =
            if let Some(csaf_version) = doc.v20_view_mut().document_mut().and_then(|doc| doc.csaf_version_mut()) {
                if csaf_version.get_enum().is_err() {
                    diagnostics.warning(Warning::InvalidCsafVersion);
                }
                csaf_version.caution_as_v21_mut()
            } else {
                diagnostics.warning(Warning::CsafVersionIsMissing);

                doc.v21_view_mut()
                    .caution_get_or_insert_document()
                    .caution_get_or_insert_csaf_version()
            };

        csaf_version.set_enum(csaf2_1::schema::CsafVersion::X21);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::{NoDiagnostics, OnlyWarnings};

    use super::*;

    #[test]
    fn converter_sets_correct_csaf_version() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "csaf_version": "2.0",
            }
        });
        let mut doc = loose::Document::new(input);
        UpdateCsafVersion.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(expected(), doc.into_raw());

        Ok(())
    }

    #[test]
    fn invalid_csaf_version() {
        let input = json!({
            "document": {
                "csaf_version": "1.0",
            }
        });
        conversion_with_warnings(input, &[Warning::InvalidCsafVersion]);
    }

    #[test]
    fn document_without_version() {
        let input = json!({
            "document": {}
        });
        conversion_with_warnings(input, &[Warning::CsafVersionIsMissing]);
    }

    #[test]
    fn no_document_node() {
        let input = json!({});
        conversion_with_warnings(input, &[Warning::CsafVersionIsMissing]);
    }

    fn conversion_with_warnings(input: serde_json::Value, warnings: &[Warning]) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();
        UpdateCsafVersion.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(warnings, diagnostics.warnings());
        assert_eq!(expected(), doc.into_raw());
    }

    fn expected() -> serde_json::Value {
        json!({
            "document": {
                "csaf_version": "2.1",
            }
        })
    }
}
