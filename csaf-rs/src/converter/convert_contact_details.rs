use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Map, Value};

use crate::{
    converter::{Convert, Diagnostics, PublisherContactSettings, Settings, Warning},
    schema::loose,
};

use super::checks::{email_search_pattern, is_valid_email, is_valid_public_openpgp_key_url};

/// Transforms `$.document.publisher.contact_details` into `$.document.publisher.contact`
/// as mandated by Conformance Clause 18 for CSAF 2.0 to CSAF 2.1 converters.
///
/// The content of `contact_details` becomes `contact.details`. A URL to a public OpenPGP key and
/// an email address are extracted from the content, if applicable. Values that the document does
/// not provide can be configured via the settings; document values take precedence unless
/// [`PublisherContactSettings::overwrite_extracted`] is set.
pub struct ConvertContactDetails;

impl Convert for ConvertContactDetails {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let Some(publisher) = doc
            .v21_view_mut()
            .document_mut()
            .and_then(|document| document.publisher_mut())
        else {
            return;
        };
        let Ok(publisher) = publisher.caution_as_object_mut() else {
            return;
        };

        let contact_settings = &settings.publisher_contact;
        // The configured values are already validated by their CSAF 2.1 types, e.g. at command
        // line parsing time when using them directly as argument values. Their `format`
        // requirements are checked here as well: via the library API, values can be configured
        // that only fulfill the generated type checks (e.g. `Email` accepts `not-an-email`).
        let configured = ContactValues {
            details: contact_settings.details.as_deref().cloned(),
            email: format_checked("email", contact_settings.email.as_deref(), is_valid_email, diagnostics),
            public_openpgp_key_url: format_checked(
                "public_openpgp_key_url",
                contact_settings.public_openpgp_key_url.as_deref(),
                is_valid_public_openpgp_key_url,
                diagnostics,
            ),
        };

        // `contact_details` has no equivalent in CSAF 2.1, so it MUST NOT be kept. Its content
        // becomes `details` and is the source for the extracted email and OpenPGP key URL.
        // Non-string values are left untouched; schema validation already flags them.
        let extracted = match publisher.get("contact_details") {
            Some(Value::String(content)) => Some(extract_contact(content)),
            _ => None,
        };
        if extracted.is_some() {
            publisher.remove("contact_details");
        }

        // `contact` is not defined in CSAF 2.0; valid documents do not provide one, so a 2.0
        // value is never merged with it here. Document values therefore only come from the
        // (optional) extraction above; configured values fill the remaining gaps.
        let details = resolve(
            extracted.as_ref().and_then(|extracted| extracted.details.as_deref()),
            configured.details.as_deref(),
            contact_settings,
        )
        .map(str::to_string);
        let email = resolve(
            extracted.as_ref().and_then(|extracted| extracted.email.as_deref()),
            configured.email.as_deref(),
            contact_settings,
        )
        .map(str::to_string);
        let mut public_openpgp_key_url = resolve(
            extracted
                .as_ref()
                .and_then(|extracted| extracted.public_openpgp_key_url.as_deref()),
            configured.public_openpgp_key_url.as_deref(),
            contact_settings,
        )
        .map(str::to_string);

        // CSAF 2.1 requires the sibling `email` property whenever `public_openpgp_key_url` is
        // set, so a key URL without an associated email address must not be adopted. It remains
        // part of the `details` text, so the information is not lost.
        if let Some(url) = &public_openpgp_key_url
            && email.is_none()
        {
            diagnostics.warning(
                "/document/publisher/contact".into(),
                Warning::PublisherContactOpenPgpKeyUrlRequiresEmail { url: url.clone() },
            );
            public_openpgp_key_url = None;
        }

        if details.is_some() || email.is_some() || public_openpgp_key_url.is_some() {
            let mut contact = Map::new();
            if let Some(details) = details {
                contact.insert("details".to_string(), Value::String(details));
            }
            if let Some(email) = email {
                contact.insert("email".to_string(), Value::String(email));
            }
            if let Some(public_openpgp_key_url) = public_openpgp_key_url {
                contact.insert(
                    "public_openpgp_key_url".to_string(),
                    Value::String(public_openpgp_key_url),
                );
            }
            publisher.insert("contact".to_string(), Value::Object(contact));
        }
    }
}

/// The values of the CSAF 2.1 `contact_t` type, either extracted from `contact_details` or
/// provided via the settings
struct ContactValues {
    details: Option<String>,
    email: Option<String>,
    public_openpgp_key_url: Option<String>,
}

/// Decides between the document value and the configured value: document values win unless the
/// settings allow overwriting them. A configured value always fills a gap in the document.
fn resolve<'a>(
    document: Option<&'a str>,
    configured: Option<&'a str>,
    settings: &PublisherContactSettings,
) -> Option<&'a str> {
    if settings.overwrite_extracted {
        configured.or(document)
    } else {
        document.or(configured)
    }
}

/// Validates a configured value against the `format` requirements of its CSAF 2.1 property. An
/// invalid value is announced via a warning and skipped instead of ending up in the converted
/// document.
fn format_checked(
    property: &'static str,
    value: Option<&String>,
    is_valid: impl Fn(&str) -> bool,
    diagnostics: &mut dyn Diagnostics,
) -> Option<String> {
    let value: &str = value?;
    if is_valid(value) {
        Some(value.to_string())
    } else {
        diagnostics.warning(
            format!("/document/publisher/contact/{property}").into(),
            Warning::PublisherContactInvalidFormat {
                property,
                value: value.to_string(),
            },
        );
        None
    }
}

/// Extracts the values that the CSAF 2.1 `contact_t` type provides dedicated properties for from
/// the content of the CSAF 2.0 `contact_details` property
fn extract_contact(content: &str) -> ContactValues {
    let found_key_url = find_openpgp_key_url(content);

    ContactValues {
        details: Some(content.to_string()),
        email: find_email(content, found_key_url.as_ref()),
        public_openpgp_key_url: found_key_url.map(|found| found.value),
    }
}

struct FoundUrl {
    value: String,
    range: std::ops::Range<usize>,
}

/// Finds the first URL in `content` that can serve as the value of `public_openpgp_key_url`, i.e.
/// - fulfills the requirements of the CSAF 2.1 property `public_openpgp_key_url`, and
/// - is recognizable as pointing to an OpenPGP key, either by the URL itself or by its context.
///
/// URLs that look like key URLs but do not fulfill the requirements (e.g. because they are not
/// HTTPS URLs or not valid URIs) are intentionally not adopted; they stay part of the `details`
/// text.
fn find_openpgp_key_url(content: &str) -> Option<FoundUrl> {
    static URL_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r#"https://[^\s<>"']+"#).unwrap()
    });

    for url in URL_RE.find_iter(content) {
        let value = trim_url_tail(url.as_str());
        if is_valid_public_openpgp_key_url(value) && mentions_openpgp(&content[..url.start()], value) {
            return Some(FoundUrl {
                value: value.to_string(),
                range: url.start()..url.start() + value.len(),
            });
        }
    }
    None
}

/// Checks whether the URL itself or the text directly preceding it indicates an OpenPGP key. The
/// limited lookbehind window keeps the heuristic local: an OpenPGP mention farther away is not
/// enough of an association for the URL.
fn mentions_openpgp(preceding_text: &str, url: &str) -> bool {
    const PRECEDING_CHARS: usize = 100;

    static OPENPGP_TEXT_HINT_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r"(?i)openpgp|pgp|gpg").unwrap()
    });
    static OPENPGP_URL_HINT_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r"(?i)openpgp|pks/lookup|pubkey|publickey|gpg").unwrap()
    });

    let window_start = preceding_text
        .char_indices()
        .nth_back(PRECEDING_CHARS - 1)
        .map_or(0, |(index, _)| index);
    if OPENPGP_TEXT_HINT_RE.is_match(&preceding_text[window_start..]) {
        return true;
    }

    let path = url.split(['?', '#']).next().unwrap_or(url);
    OPENPGP_URL_HINT_RE.is_match(url) || path.to_lowercase().ends_with(".asc")
}

/// Trims trailing punctuation, which typically ends a sentence rather than being part of the URL
fn trim_url_tail(url: &str) -> &str {
    let mut trimmed = url.trim_end_matches(['.', ',', ';', ':', '!', '?', '"', '\'']);

    // Closing brackets only belong to the URL if the URL contains the opening ones
    while let Some(last) = trimmed.chars().last() {
        let (opening, closing) = match last {
            ')' => ('(', ')'),
            ']' => ('[', ']'),
            _ => break,
        };
        if trimmed.matches(opening).count() >= trimmed.matches(closing).count() {
            break;
        }
        trimmed = &trimmed[..trimmed.len() - last.len_utf8()];
    }
    trimmed
}

/// Extracts an email address from `content`.
///
/// Only addresses that fulfill the CSAF 2.1 requirements for `email` are considered, so an
/// invalid address neither ends up in the document nor blocks a valid one or a configured
/// fallback.
///
/// If a public OpenPGP key URL was found, the email address closest to it is preferred, so that
/// the key and the email address it is valid for stay associated ("the matching email address").
/// Otherwise, the first email address in the content is used.
fn find_email(content: &str, openpgp_key_url: Option<&FoundUrl>) -> Option<String> {
    static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(&email_search_pattern()).unwrap()
    });

    let url_range = openpgp_key_url.map(|found| &found.range);
    let within_key_url = |start: usize| url_range.is_some_and(|range| range.contains(&start));

    let mut candidates = EMAIL_RE
        .find_iter(content)
        .filter(|email| !within_key_url(email.start()))
        .filter(|email| is_valid_email(email.as_str()))
        .map(|email| (email.start(), email.end(), email.as_str()));

    match url_range {
        Some(range) => candidates
            .min_by_key(|(start, end, _)| gap_to_key_url(*start, *end, range))
            .map(|(_, _, email)| email.to_string()),
        None => candidates.next().map(|(_, _, email)| email.to_string()),
    }
}

/// The distance between the email address and the whole URL range: addresses before the URL are
/// measured from their end, addresses after it from the URL's end
fn gap_to_key_url(email_start: usize, email_end: usize, url_range: &std::ops::Range<usize>) -> usize {
    if email_end <= url_range.start {
        url_range.start - email_end
    } else {
        email_start.saturating_sub(url_range.end)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use crate::converter::tests::OnlyWarnings;

    use super::*;

    fn convert_with_settings(
        input: serde_json::Value,
        publisher_contact: PublisherContactSettings,
    ) -> (serde_json::Value, OnlyWarnings) {
        let settings = Settings {
            publisher_contact,
            ..Default::default()
        };
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        ConvertContactDetails.convert(&mut doc, &settings, &mut diagnostics);

        (doc.into_raw(), diagnostics)
    }

    fn convert(input: serde_json::Value) -> serde_json::Value {
        convert_with_settings(input, PublisherContactSettings::default()).0
    }

    #[test]
    fn moves_contact_details_into_contact() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "See contact points at https://github.com/ISDuBA",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert_eq!(
            json!({
                "document": {
                    "publisher": {
                        "category": "vendor",
                        "contact": {
                            "details": "See contact points at https://github.com/ISDuBA"
                        },
                        "name": "Example Publisher",
                        "namespace": "https://example.com"
                    }
                }
            }),
            output
        );
        // the `contact_details` property has no equivalent in CSAF 2.1 and must be gone
        assert!(output["document"]["publisher"].get("contact_details").is_none());
    }

    #[test]
    fn extracts_public_openpgp_key_url_and_matching_email() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Other contact: support@example.org. Security contact: security@debian.org. \
                        Public OpenPGP key: https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x0d59d2b15144766a14d241c66baf400b05c3e651",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        let contact = &output["document"]["publisher"]["contact"];
        assert_eq!(
            "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x0d59d2b15144766a14d241c66baf400b05c3e651",
            contact["public_openpgp_key_url"]
        );
        assert_eq!("security@debian.org", contact["email"]);
        assert!(contact["details"].as_str().unwrap().contains("support@example.org"));
    }

    #[test]
    fn extracts_first_email_when_no_openpgp_key_url_is_present() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Contact first@example.org or second@example.org.",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert_eq!("first@example.org", output["document"]["publisher"]["contact"]["email"]);
    }

    #[test]
    fn preserves_legal_characters_in_the_email_local_part() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Security contact: security=alerts@example.org",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert_eq!(
            "security=alerts@example.org",
            output["document"]["publisher"]["contact"]["email"]
        );
    }

    #[test]
    fn skips_email_addresses_below_the_csaf_minimum_length() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Contact a@b.c or security@example.org.",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        // the too-short address is skipped, the later valid one is used
        assert_eq!(
            "security@example.org",
            output["document"]["publisher"]["contact"]["email"]
        );
    }

    #[test]
    fn skips_email_addresses_violating_the_dot_atom_rules() {
        for contact_details in [
            "Contact security..alerts@example.org",
            "Contact .security@example.org",
            "Contact security.@example.org",
        ] {
            let input = json!({
                "document": {
                    "publisher": {
                        "category": "vendor",
                        "contact_details": contact_details,
                        "name": "Example Publisher",
                        "namespace": "https://example.com"
                    }
                }
            });

            let output = convert(input);

            // the invalid address is rejected as a whole instead of partially extracted
            let contact = &output["document"]["publisher"]["contact"];
            assert!(contact.get("email").is_none(), "{contact_details}");
            assert_eq!(contact_details, contact["details"]);
        }
    }

    #[test]
    fn warns_about_and_skips_configured_email_with_invalid_format() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: None,
                // the generated type accepts this value, its `format: email` does not
                email: Some("not-an-email".parse().unwrap()),
                public_openpgp_key_url: None,
                overwrite_extracted: false,
            },
        );

        assert!(output["document"]["publisher"].get("contact").is_none());
        assert_eq!(
            vec![(
                "/document/publisher/contact/email".into(),
                Warning::PublisherContactInvalidFormat {
                    property: "email",
                    value: "not-an-email".to_string(),
                },
            )],
            diagnostics.warnings()
        );
    }

    #[test]
    fn does_not_adopt_configured_openpgp_key_url_when_configured_email_has_invalid_format() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: None,
                email: Some("not-an-email".parse().unwrap()),
                public_openpgp_key_url: Some("https://example.com/key".parse().unwrap()),
                overwrite_extracted: false,
            },
        );

        // without a valid email, the key URL must not be adopted either
        assert!(output["document"]["publisher"].get("contact").is_none());
        assert_eq!(
            vec![
                (
                    "/document/publisher/contact/email".into(),
                    Warning::PublisherContactInvalidFormat {
                        property: "email",
                        value: "not-an-email".to_string(),
                    },
                ),
                (
                    "/document/publisher/contact".into(),
                    Warning::PublisherContactOpenPgpKeyUrlRequiresEmail {
                        url: "https://example.com/key".to_string(),
                    },
                ),
            ],
            diagnostics.warnings()
        );
    }

    #[test]
    fn uses_configured_email_when_only_invalid_addresses_were_extracted() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Contact a@b.c",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: None,
                email: Some("security@example.org".parse().unwrap()),
                public_openpgp_key_url: None,
                overwrite_extracted: false,
            },
        );

        assert!(diagnostics.warnings().is_empty());
        assert_eq!(
            "security@example.org",
            output["document"]["publisher"]["contact"]["email"]
        );
    }

    #[test]
    fn measures_email_proximity_against_the_whole_openpgp_key_url() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Contact support@example.org or see our OpenPGP key: \
                        https://keyserver.example.org/pks/lookup?op=get&search=0x0123456789abcdef0123456789abcdef01234567 \
                        (security@example.org)",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        // the address right after the URL is closer to it than the one at the beginning
        assert_eq!(
            "security@example.org",
            output["document"]["publisher"]["contact"]["email"]
        );
    }

    #[test]
    fn does_not_adopt_openpgp_key_url_with_invalid_percent_escape() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Public OpenPGP key: https://example.org/key%ZZ.asc",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        let contact = &output["document"]["publisher"]["contact"];
        assert!(contact.get("public_openpgp_key_url").is_none());
        assert!(
            contact["details"]
                .as_str()
                .unwrap()
                .contains("https://example.org/key%ZZ.asc")
        );
    }

    #[test]
    fn does_not_adopt_non_ascii_openpgp_key_url() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Public OpenPGP key: https://example.de/schl\u{fc}ssel.asc",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        let contact = &output["document"]["publisher"]["contact"];
        assert!(contact.get("public_openpgp_key_url").is_none());
        assert!(
            contact["details"]
                .as_str()
                .unwrap()
                .contains("https://example.de/schl\u{fc}ssel.asc")
        );
    }

    #[test]
    fn does_not_adopt_non_https_openpgp_key_url() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Security contact: security@debian.org. \
                        Public OpenPGP key: http://keyserver.ubuntu.com/pks/lookup?op=get&search=0x0d59d2b15144766a14d241c66baf400b05c3e651",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert_eq!(
            json!({
                "details": "Security contact: security@debian.org. \
                    Public OpenPGP key: http://keyserver.ubuntu.com/pks/lookup?op=get&search=0x0d59d2b15144766a14d241c66baf400b05c3e651",
                "email": "security@debian.org"
            }),
            output["document"]["publisher"]["contact"]
        );
    }

    #[test]
    fn does_not_adopt_openpgp_key_url_without_associated_email() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Public OpenPGP key: https://example.com/key.asc",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(input, PublisherContactSettings::default());

        assert!(
            output["document"]["publisher"]["contact"]
                .get("public_openpgp_key_url")
                .is_none()
        );
        assert!(output["document"]["publisher"]["contact"].get("email").is_none());
        assert_eq!(
            vec![(
                "/document/publisher/contact".into(),
                Warning::PublisherContactOpenPgpKeyUrlRequiresEmail {
                    url: "https://example.com/key.asc".to_string()
                },
            )],
            diagnostics.warnings()
        );
    }

    #[test]
    fn creates_contact_from_configured_values() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: Some("security@example.com".parse().unwrap()),
                email: Some("security@example.com".parse().unwrap()),
                public_openpgp_key_url: Some("https://example.com/key".parse().unwrap()),
                overwrite_extracted: false,
            },
        );

        assert!(diagnostics.warnings().is_empty());
        assert_eq!(
            json!({
                "details": "security@example.com",
                "email": "security@example.com",
                "public_openpgp_key_url": "https://example.com/key"
            }),
            output["document"]["publisher"]["contact"]
        );
    }

    #[rstest]
    #[case::document_value_wins_by_default(false)]
    #[case::configured_value_wins_with_overwrite(true)]
    fn configured_values_fill_gaps_and_overwrite_flag_changes_precedence(#[case] overwrite_extracted: bool) {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "Contact security@document.example.",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: Some("Configured details".parse().unwrap()),
                email: Some("security@configured.example".parse().unwrap()),
                public_openpgp_key_url: Some("https://configured.example/key".parse().unwrap()),
                overwrite_extracted,
            },
        );

        assert!(diagnostics.warnings().is_empty());
        let contact = output["document"]["publisher"]["contact"].clone();
        if overwrite_extracted {
            assert_eq!(
                json!({
                    "details": "Configured details",
                    "email": "security@configured.example",
                    "public_openpgp_key_url": "https://configured.example/key"
                }),
                contact
            );
        } else {
            // The configured key URL fills the gap, the other values come from the document
            assert_eq!(
                json!({
                    "details": "Contact security@document.example.",
                    "email": "security@document.example",
                    "public_openpgp_key_url": "https://configured.example/key"
                }),
                contact
            );
        }
    }

    #[test]
    fn fills_missing_values_from_configured_ones() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "See our website at https://www.example.com/contact.",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input,
            PublisherContactSettings {
                details: None,
                email: Some("security@example.com".parse().unwrap()),
                public_openpgp_key_url: Some("https://example.com/key".parse().unwrap()),
                overwrite_extracted: false,
            },
        );

        assert!(diagnostics.warnings().is_empty());
        assert_eq!(
            json!({
                "details": "See our website at https://www.example.com/contact.",
                "email": "security@example.com",
                "public_openpgp_key_url": "https://example.com/key"
            }),
            output["document"]["publisher"]["contact"]
        );
    }

    #[test]
    fn ignores_publisher_with_invalid_contact_details_type() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": 42,
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input.clone());

        assert_eq!(input, output);
    }

    #[test]
    fn ignores_missing_publisher() {
        let input = json!({
            "document": {
                "title": "Example"
            }
        });

        let (output, diagnostics) = convert_with_settings(
            input.clone(),
            PublisherContactSettings {
                details: Some("Configured details".parse().unwrap()),
                email: Some("security@example.com".parse().unwrap()),
                public_openpgp_key_url: None,
                overwrite_extracted: false,
            },
        );

        assert_eq!(input, output);
        assert!(diagnostics.warnings().is_empty());
    }

    #[test]
    fn does_not_create_contact_without_values() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert!(output["document"]["publisher"].get("contact").is_none());
    }

    #[test]
    fn empty_contact_details_value_is_moved_as_is() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact_details": "",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        assert_eq!(json!({ "details": "" }), output["document"]["publisher"]["contact"]);
    }
}
