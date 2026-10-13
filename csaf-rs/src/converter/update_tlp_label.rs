use crate::schema::{csaf2_0, csaf2_1, csaf2x};

use super::{Convert, Diagnostics, Settings, Warning};

// csaf-rs/csaf#1067
pub struct UpdateTlpLabel;

impl Convert for UpdateTlpLabel {
    fn convert(&self, doc: &mut csaf2x::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let clear_label = csaf2_1::schema::LabelOfTlp::Clear;
        if let Some(label) = doc
            .v20_view()
            .document()
            .and_then(|doc| doc.distribution())
            .and_then(|distribution| distribution.tlp())
            .and_then(|tlp| tlp.label())
        {
            if let Ok(label) = label.get_enum() {
                if label == csaf2_0::schema::LabelOfTlp::White {
                    doc.v21_view_mut()
                        .document_mut()
                        .unwrap()
                        .distribution_mut()
                        .unwrap()
                        .tlp_mut()
                        .unwrap()
                        .label_mut()
                        .unwrap()
                        .set_enum(clear_label);
                }
            } else {
                // XXX error/warning? invalid value as per CSAF 2.0 schema
            }
            return;
        }

        if let Some(tlp) = doc
            .v21_view_mut()
            .document_mut()
            .and_then(|doc| doc.distribution_mut())
            .and_then(|distribution| distribution.tlp_mut())
            && let Ok(tlp) = tlp.caution_as_object_mut()
        {
            tlp.insert("label".into(), serde_json::to_value(clear_label).unwrap());
            diagnostics.warning(Warning::NoTlpLabel);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::{DiagnosticsSpy, NoDiagnostics};

    use super::*;

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
        let mut doc = csaf2x::Document::new(input);
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    #[test]
    fn has_other_tlp_label() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "RED"
                    }
                }
            }
        });
        let output = input.clone();
        let mut doc = csaf2x::Document::new(input);
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    #[test]
    fn no_tlp_label() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {}
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

        let mut doc = csaf2x::Document::new(input);
        let mut diagnostics = DiagnosticsSpy::default();
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert!(diagnostics.errors().is_empty());
        assert_eq!([Warning::NoTlpLabel], diagnostics.warnings());
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    #[test]
    fn no_scope() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "distribution": {}
            }
        });
        let output = input.clone();

        let mut doc = csaf2x::Document::new(input);
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    // TODO implement "SHOULD" logic
}
