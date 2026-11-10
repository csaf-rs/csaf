use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::schema::{csaf2_0, loose};

use super::{Convert, Diagnostics, Settings, Warning};

// csaf-rs/csaf#1369
pub struct SetInitialReleaseDate;

impl Convert for SetInitialReleaseDate {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        // Test 6.1.66 only applies to documents with status `final` or `interim`
        let is_non_draft = matches!(
            doc.v20_view()
                .document()
                .and_then(|document| document.tracking())
                .and_then(|tracking| tracking.status())
                .and_then(|status| status.get_enum().ok()),
            Some(csaf2_0::schema::DocumentStatus::Final | csaf2_0::schema::DocumentStatus::Interim)
        );
        if !is_non_draft {
            return;
        }

        let Some(tracking) = doc
            .v20_view_mut()
            .document_mut()
            .and_then(|document| document.tracking_mut())
        else {
            return;
        };

        // The `date` of the revision history item with a `number` of `1` or `1.0.0`
        // is the only meaningful replacement value for the initial release date
        let Some(initial_revision) = tracking
            .revision_history()
            .and_then(|revision_history| revision_history.as_slice().ok())
            .and_then(|items| {
                items.iter().find(|item| {
                    matches!(
                        item.number().and_then(|number| number.get_str().ok()),
                        Some("1" | "1.0.0")
                    )
                })
            })
        else {
            return;
        };
        let Some(initial_date) = initial_revision.date().and_then(|date| date.get_str().ok()) else {
            return;
        };
        let CsafDateTime::Valid(revision_date) = CsafDateTime::from(initial_date) else {
            return;
        };

        let Some(initial_release_date) = tracking.initial_release_date_mut() else {
            return;
        };
        let Ok(old) = initial_release_date.get_str().map(str::to_owned) else {
            return;
        };
        let CsafDateTime::Valid(old_date) = CsafDateTime::from(old.as_str()) else {
            return;
        };

        // CsafDateTime equality takes the timezones of both values into account
        if old_date == revision_date {
            return;
        }

        let new = revision_date.get_raw_string().to_string();
        initial_release_date.set_str(new.clone());
        diagnostics.warning(
            "/document/tracking/initial_release_date".into(),
            Warning::InconsistentInitialReleaseDate { old, new },
        );
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use crate::converter::tests::OnlyWarnings;

    use super::*;

    #[test]
    fn replaces_inconsistent_initial_release_date_with_integer_versioned_revision() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-02-01T10:00:00.000Z",
                                "number": "2",
                                "summary": "Second version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-02-01T10:00:00.000Z",
                                "number": "2",
                                "summary": "Second version."
                            }
                        ]
                    }
                }
            }),
            Some((
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-22T10:00:00.000Z", "2024-01-21T10:00:00.000Z"),
            )),
        );
    }

    #[test]
    fn replaces_inconsistent_initial_release_date_with_semantic_versioned_revision() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-22T10:00:00.000Z",
                                "number": "1.0.0",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-22T10:00:00.000Z",
                                "number": "1.0.0",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            Some((
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-21T10:00:00.000Z", "2024-01-22T10:00:00.000Z"),
            )),
        );
    }

    #[test]
    fn replaces_inconsistent_initial_release_date_on_interim_status() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "interim",
                        "initial_release_date": "2024-01-20T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "interim",
                        "initial_release_date": "2024-01-21T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            Some((
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-20T10:00:00.000Z", "2024-01-21T10:00:00.000Z"),
            )),
        );
    }

    #[test]
    fn equal_dates_are_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn equal_instants_in_different_timezones_are_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T12:00:00+02:00",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T12:00:00+02:00",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn replacement_preserves_the_revision_history_date_format() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T12:00:00+02:00",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-21T12:00:00+02:00",
                        "revision_history": [
                            {
                                "date": "2024-01-21T12:00:00+02:00",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            Some((
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-22T10:00:00.000Z", "2024-01-21T12:00:00+02:00"),
            )),
        );
    }

    #[test]
    fn draft_status_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "draft",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "draft",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn missing_revision_history_item_numbered_one_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1.0.1",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-02-01T10:00:00.000Z",
                                "number": "2",
                                "summary": "Second version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1.0.1",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-02-01T10:00:00.000Z",
                                "number": "2",
                                "summary": "Second version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn first_revision_history_item_numbered_one_wins() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-23T10:00:00.000Z",
                                "number": "1.0.0",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-23T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-23T10:00:00.000Z",
                                "number": "1.0.0",
                                "summary": "Initial version."
                            },
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            Some((
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-22T10:00:00.000Z", "2024-01-23T10:00:00.000Z"),
            )),
        );
    }

    #[test]
    fn invalid_revision_history_date_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "not-a-date",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "not-a-date",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn invalid_initial_release_date_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "not-a-date",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "not-a-date",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn missing_initial_release_date_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn missing_status_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": "1",
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    #[test]
    fn non_string_revision_history_number_is_untouched() {
        conversion(
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": 1,
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            json!({
                "document": {
                    "tracking": {
                        "status": "final",
                        "initial_release_date": "2024-01-22T10:00:00.000Z",
                        "revision_history": [
                            {
                                "date": "2024-01-21T10:00:00.000Z",
                                "number": 1,
                                "summary": "Initial version."
                            }
                        ]
                    }
                }
            }),
            None,
        );
    }

    fn inconsistent_warning(old: &str, new: &str) -> Warning {
        Warning::InconsistentInitialReleaseDate {
            old: old.to_string(),
            new: new.to_string(),
        }
    }

    fn conversion(
        input: serde_json::Value,
        expected: serde_json::Value,
        expected_warning: Option<(std::borrow::Cow<'static, str>, Warning)>,
    ) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        SetInitialReleaseDate.convert(&mut doc, &Settings::default(), &mut diagnostics);

        assert_eq!(expected, doc.into_raw());

        match expected_warning {
            Some(warning) => assert_eq!(&[warning], diagnostics.warnings()),
            None => assert!(diagnostics.warnings().is_empty()),
        }
    }

    // Sanity check that a document which fails test 6.1.66 passes it after conversion
    #[test]
    fn converted_document_passes_test_6_1_66() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "category": "csaf_security_advisory",
                "csaf_version": "2.0",
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                },
                "title": "Example Advisory",
                "tracking": {
                    "current_release_date": "2024-02-01T10:00:00.000Z",
                    "id": "Example Advisory 2024-01",
                    "initial_release_date": "2024-01-22T10:00:00.000Z",
                    "revision_history": [
                        {
                            "date": "2024-01-21T10:00:00.000Z",
                            "number": "1",
                            "summary": "Initial version."
                        },
                        {
                            "date": "2024-02-01T10:00:00.000Z",
                            "number": "2",
                            "summary": "Second version."
                        }
                    ],
                    "status": "final",
                    "version": "2"
                }
            }
        });

        let mut diagnostics = OnlyWarnings::default();
        let doc = crate::converter::convert(input, &Settings::default(), &mut diagnostics);
        assert_eq!(
            &[(
                "/document/tracking/initial_release_date".into(),
                inconsistent_warning("2024-01-22T10:00:00.000Z", "2024-01-21T10:00:00.000Z")
            )],
            diagnostics.warnings()
        );

        let converted = crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework::deserialize(doc.into_raw())?;
        let result = crate::validations::test_6_1_66::test_6_1_66_non_draft_initial_release_date(&converted);

        assert_eq!(Ok(()), result);

        Ok(())
    }
}
