use crate::schema::{csaf2_1, csaf2x};

use super::{Convert, Diagnostics, Settings};

// csaf-rs/csaf#1066
pub struct UpdateCsafVersion;

impl Convert for UpdateCsafVersion {
    fn convert(&self, doc: &mut csaf2x::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        if let Some(csaf_version) = doc.v20_view().document().and_then(|doc| doc.csaf_version()) {
            if csaf_version.get_enum().is_err() {
                // XXX error/warning? invalid value as per CSAF 2.0 schema
            }

            // the previous lookup has already checked that this node exists so we unwrap
            doc.v21_view_mut()
                .document_mut()
                .unwrap()
                .csaf_version_mut()
                .unwrap()
                .set_enum(csaf2_1::schema::CsafVersion::X21);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn it_works() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "csaf_version": "2.0",
            }
        });
        let output = json!({
            "document": {
                "csaf_version": "2.1",
            }
        });
        let mut doc = csaf2x::Document::new(input);
        UpdateCsafVersion.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    #[test]
    fn does_not_apply() -> serde_json::Result<()> {
        let input = json!({
            "document": {}
        });
        let output = input.clone();
        let mut doc = csaf2x::Document::new(input);
        UpdateCsafVersion.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }
}
