use crate::schema::{csaf2_0, csaf2_1, loose};

use super::{Convert, Diagnostics, Settings, Warning};

// csaf-rs/csaf#1067
pub struct UpdateTlpLabel;

impl Convert for UpdateTlpLabel {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let distribution = doc.v20_view().document().and_then(|doc| doc.distribution());

        // Convert the original CSAF 2.0 label using the standard TLP v1.0 -> v2.0 mapping.
        let (converted_label, missing_label) = match distribution.and_then(|dist| dist.tlp()) {
            Some(tlp) => match tlp.label() {
                Some(label) => match label.get_enum() {
                    Ok(csaf2_0::schema::LabelOfTlp::White) => (csaf2_1::schema::LabelOfTlp::Clear, false),
                    Ok(csaf2_0::schema::LabelOfTlp::Green) => (csaf2_1::schema::LabelOfTlp::Green, false),
                    Ok(csaf2_0::schema::LabelOfTlp::Amber) => (csaf2_1::schema::LabelOfTlp::Amber, false),
                    Ok(csaf2_0::schema::LabelOfTlp::Red) => (csaf2_1::schema::LabelOfTlp::Red, false),
                    Err(_) => {
                        diagnostics.warning("/document/distribution/tlp/label".into(), Warning::InvalidTlpLabel);
                        (csaf2_1::schema::LabelOfTlp::Clear, false)
                    },
                },
                None => {
                    diagnostics.warning("/document/distribution/tlp".into(), Warning::TlpIsMissingLabel);
                    (csaf2_1::schema::LabelOfTlp::Clear, true)
                },
            },
            None => (csaf2_1::schema::LabelOfTlp::Clear, true),
        };

        // Only inspect distribution.text when explicitly enabled.
        let text_label = if settings.allow_tlp_from_distribution_text {
            distribution
                .and_then(|dist| dist.text())
                .and_then(|text| text.get_str().ok())
                .and_then(first_valid_tlp_v2_label)
        } else {
            None
        };

        // A valid label from distribution.text takes precedence.
        // Otherwise use the standard conversion.
        let final_label = text_label.unwrap_or(converted_label);

        // If the input document has no TLP label, CLEAR is used as
        // the standard fallback.
        // Warn if the label from distribution.text differs from
        // the standard conversion result.
        if text_label.is_some() && final_label != converted_label {
            diagnostics.warning(
                "/document/distribution/text".into(),
                Warning::TlpLabelFromDistributionText {
                    converted: converted_label.to_string(),
                    from_text: final_label.to_string(),
                },
            );
        }

        // Warn only if CLEAR was actually selected as the default.
        if missing_label && text_label.is_none() {
            diagnostics.warning("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet);
        }

        doc.v21_view_mut()
            .caution_get_or_insert_document()
            .caution_get_or_insert_distribution()
            .caution_get_or_insert_tlp()
            .caution_get_or_insert_label()
            .set_enum(final_label);
    }
}

/// Returns the first valid TLP v2.0 label in the text.
///
/// The prefix is matched literally and case-sensitively.
/// Invalid occurrences are skipped.
fn first_valid_tlp_v2_label(text: &str) -> Option<csaf2_1::schema::LabelOfTlp> {
    const PREFIX: &str = "TLP v2.0: TLP:";

    // Check AMBER+STRICT before AMBER to avoid partial matches.
    const LABELS: [&str; 5] = ["AMBER+STRICT", "CLEAR", "GREEN", "AMBER", "RED"];

    for (position, _) in text.match_indices(PREFIX) {
        let remaining = &text[position + PREFIX.len()..];

        for label in LABELS {
            let Some(after_label) = remaining.strip_prefix(label) else {
                continue;
            };

            // Reject partial matches such as REDXYZ or AMBER+STRICT_EXTRA.
            let valid_boundary = after_label
                .chars()
                .next()
                .is_none_or(|c| !c.is_alphanumeric() && c != '+' && c != '_' && c != '-');

            if !valid_boundary {
                continue;
            }

            // Use the CSAF 2.1 schema enum to validate the label.
            if let Ok(label) =
                serde_json::from_value::<csaf2_1::schema::LabelOfTlp>(serde_json::Value::String(label.to_owned()))
            {
                return Some(label);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json::{Value, json};

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
        for label in ["GREEN", "AMBER", "RED"] {
            let input = json!({
                "document": {
                    "distribution": {
                        "tlp": {
                            "label": label
                        }
                    }
                }
            });

            let output = input.clone();
            let mut doc = loose::Document::new(input);

            UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

            assert_eq!(output, doc.into_raw());
        }
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
        conversion_with_warnings(
            input,
            &[("/document/distribution/tlp/label".into(), Warning::InvalidTlpLabel)],
        );
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
        conversion_with_warnings(
            input,
            &[
                ("/document/distribution/tlp".into(), Warning::TlpIsMissingLabel),
                ("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet),
            ],
        );
    }

    #[test]
    fn no_tlp_node() {
        let input = json!({
            "document": {
                "distribution": {}
            }
        });

        conversion_with_warnings(
            input,
            &[("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet)],
        );
    }

    #[test]
    fn no_distribution_node() {
        let input = json!({
            "document": {}
        });

        conversion_with_warnings(
            input,
            &[("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet)],
        );
    }

    #[test]
    fn no_document_node() {
        let input = json!({});

        conversion_with_warnings(
            input,
            &[("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet)],
        );
    }

    // Tests for the optional TLP label from distribution.text

    #[test]
    fn option_disabled_ignores_distribution_text() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "RED"
                    },
                    "text": "TLP v2.0: TLP:AMBER+STRICT"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input.clone(), false);

        assert_eq!(output, input);
        assert!(diagnostics.warnings().is_empty());
    }

    #[test]
    fn option_enabled_uses_distribution_text() {
        let text = "TLP v2.0: TLP:AMBER+STRICT";

        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "RED",
                        "url": "https://example.com/tlp"
                    },
                    "text": text
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, true);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "AMBER+STRICT");

        // Other properties must remain unchanged.
        assert_eq!(
            output["document"]["distribution"]["tlp"]["url"],
            "https://example.com/tlp"
        );
        assert_eq!(output["document"]["distribution"]["text"], text);

        assert_eq!(
            diagnostics.warnings(),
            &[(
                "/document/distribution/text".into(),
                Warning::TlpLabelFromDistributionText {
                    converted: "RED".into(),
                    from_text: "AMBER+STRICT".into(),
                },
            )]
        );
    }

    #[test]
    fn first_valid_tlp_is_selected() {
        let text = "\
            TLP v2.0: TLP:INVALID\n\
            TLP v2.0: TLP:GREEN\n\
            TLP v2.0: TLP:RED";

        let label = first_valid_tlp_v2_label(text).unwrap();

        assert_eq!(label.to_string(), "GREEN");
    }

    #[test]
    fn all_valid_tlp_v2_labels_are_recognized() {
        for expected in ["CLEAR", "GREEN", "AMBER", "AMBER+STRICT", "RED"] {
            let text = format!("TLP v2.0: TLP:{expected}");

            let label = first_valid_tlp_v2_label(&text).unwrap();

            assert_eq!(label.to_string(), expected);
        }
    }

    #[test]
    fn partial_tlp_labels_are_rejected() {
        let text = "\
            TLP v2.0: TLP:REDXYZ\n\
            TLP v2.0: TLP:AMBER+STRICT_EXTRA\n\
            TLP v2.0: TLP:CLEAR";

        let label = first_valid_tlp_v2_label(text).unwrap();

        assert_eq!(label.to_string(), "CLEAR");
    }

    #[test]
    fn missing_tlp_with_option_uses_text() {
        let input = json!({
            "document": {
                "distribution": {
                    "text": "TLP v2.0: TLP:AMBER"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, true);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "AMBER");

        assert_eq!(
            diagnostics.warnings(),
            &[(
                "/document/distribution/text".into(),
                Warning::TlpLabelFromDistributionText {
                    converted: "CLEAR".into(),
                    from_text: "AMBER".into(),
                },
            )]
        );
    }

    #[test]
    fn missing_tlp_without_option_defaults_to_clear() {
        let input = json!({
            "document": {
                "distribution": {
                    "text": "TLP v2.0: TLP:AMBER"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, false);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "CLEAR");

        assert_eq!(
            diagnostics.warnings(),
            &[("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet,)]
        );
    }

    #[test]
    fn option_enabled_without_valid_text_defaults_to_clear() {
        let input = json!({
            "document": {
                "distribution": {
                    "text": "TLP v2.0: TLP:INVALID"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, true);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "CLEAR");

        assert_eq!(
            diagnostics.warnings(),
            &[("/document/distribution/tlp/label".into(), Warning::DefaultTlpLabelSet,)]
        );
    }

    #[test]
    fn same_label_does_not_emit_override_warning() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "GREEN"
                    },
                    "text": "TLP v2.0: TLP:GREEN"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, true);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "GREEN");

        assert!(diagnostics.warnings().is_empty());
    }

    #[test]
    fn warning_uses_standard_converted_value() {
        let input = json!({
            "document": {
                "distribution": {
                    "tlp": {
                        "label": "WHITE"
                    },
                    "text": "TLP v2.0: TLP:RED"
                }
            }
        });

        let (output, diagnostics) = convert_with_option(input, true);

        assert_eq!(output["document"]["distribution"]["tlp"]["label"], "RED");

        assert_eq!(
            diagnostics.warnings(),
            &[(
                "/document/distribution/text".into(),
                Warning::TlpLabelFromDistributionText {
                    converted: "CLEAR".into(),
                    from_text: "RED".into(),
                },
            )]
        );
    }

    // Test helpers

    fn successful_conversion(input: Value) {
        let mut doc = loose::Document::new(input);

        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(expected(), doc.into_raw());
    }

    fn conversion_with_warnings(input: Value, warnings: &[(Cow<'static, str>, Warning)]) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        UpdateTlpLabel.convert(&mut doc, &Settings::default(), &mut diagnostics);

        assert_eq!(warnings, diagnostics.warnings());
        assert_eq!(expected(), doc.into_raw());
    }

    fn convert_with_option(input: Value, allow_text: bool) -> (Value, OnlyWarnings) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        let settings = Settings {
            allow_tlp_from_distribution_text: allow_text,
        };

        UpdateTlpLabel.convert(&mut doc, &settings, &mut diagnostics);

        (doc.into_raw(), diagnostics)
    }

    fn expected() -> Value {
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
}
