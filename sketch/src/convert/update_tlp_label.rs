use crate::{csaf20, csaf21, csaf2x};

use super::{Convert, Diagnostics, Settings, Warning};

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::convert::{
        self,
        tests::{DiagnosticsSpy, NoDiagnostics},
    };

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
        let mut doc = csaf2x::Csaf::new(input);
        convert::execute(&UpdateTlpLabel, &mut doc, &Settings::default(), &mut NoDiagnostics);
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
        let mut doc = csaf2x::Csaf::new(input);
        convert::execute(&UpdateTlpLabel, &mut doc, &Settings::default(), &mut NoDiagnostics);
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

        let mut doc = csaf2x::Csaf::new(input.clone());
        let mut diagnostics = DiagnosticsSpy::default();
        convert::execute(&UpdateTlpLabel, &mut doc, &Settings::default(), &mut diagnostics);
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

        let mut doc = csaf2x::Csaf::new(input.clone());
        convert::execute(&UpdateTlpLabel, &mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    // TODO implement "SHOULD" logic
}
