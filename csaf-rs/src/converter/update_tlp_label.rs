use crate::schema::{csaf2_0, csaf2_1, loose};

use super::{Convert, Diagnostics, Settings, Warning};

// csaf-rs/csaf#1067
pub struct UpdateTlpLabel;

impl Convert for UpdateTlpLabel {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let tlp = if let Some(tlp) = doc
            .v20_view_mut()
            .document_mut()
            .and_then(|doc| doc.distribution_mut())
            .and_then(|dist| dist.tlp_mut())
        {
            if let Some(label) = tlp.label() {
                if let Ok(label) = label.get_enum() {
                    if label != csaf2_0::schema::LabelOfTlp::White {
                        return;
                    }
                } else {
                    diagnostics.warning(Warning::InvalidTlpLabel);
                }
            } else {
                diagnostics.warning(Warning::TlpIsMissingLabel);
            }
            tlp.caution_as_v21_mut()
        } else {
            doc.v21_view_mut()
                .caution_get_or_insert_document()
                .caution_get_or_insert_distribution()
                .caution_get_or_insert_tlp()
        };

        tlp.caution_get_or_insert_label()
            .set_enum(csaf2_1::schema::LabelOfTlp::Clear);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::{NoDiagnostics, OnlyWarnings};

    use super::*;

    #[test]
    fn has_white_tlp_label() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "WHITE"
                    }
                }
            }
        });
        successful_conversion(input);
    }

    #[test]
    fn has_other_valid_tlp_label() {
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
        let mut doc = loose::Document::new(input);
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());
    }

    #[test]
    fn has_invalid_tlp_label() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "INVALID!"
                    }
                }
            }
        });
        conversion_with_warnings(input, &[Warning::InvalidTlpLabel]);
    }

    #[test]
    fn no_tlp_label() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {}
                }
            }
        });
        conversion_with_warnings(input, &[Warning::TlpIsMissingLabel]);
    }

    #[test]
    fn no_tlp_node() {
        let input = json!({
            "document": {
                "distribution": {}
            }
        });
        successful_conversion(input);
    }

    #[test]
    fn no_distribution_node() {
        let input = json!({
            "document": {}
        });
        successful_conversion(input);
    }

    #[test]
    fn no_document_node() {
        let input = json!({});
        successful_conversion(input);
    }

    fn successful_conversion(input: serde_json::Value) {
        let mut doc = loose::Document::new(input);
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(expected(), doc.into_raw());
    }

    fn conversion_with_warnings(input: serde_json::Value, warnings: &[Warning]) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();
        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(warnings, diagnostics.warnings());
        assert_eq!(expected(), doc.into_raw());
    }

    fn expected() -> serde_json::Value {
        json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "CLEAR"
                    }
                }
            }
        })
    }

    // TODO(#1067) implement "SHOULD" logic
}
