use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Map, Value};

use crate::{
    converter::{Convert, Diagnostics, PublisherContactSettings, Settings, Warning},
    schema::{csaf2_1, loose},
};

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
        let configured = ConfiguredContact::new(contact_settings, diagnostics);

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

        // The `contact` property is not part of CSAF 2.0. If it is present anyway (e.g. in
        // partially converted input or with `allow_invalid`) its values count as document values
        // and additional values are merged into it.
        let existing = ExistingContact::from_map(publisher.get("contact").and_then(Value::as_object));
        let contact_present = publisher.get("contact").is_some_and(Value::is_object);

        // A value already present under `contact` is a document value as well, and takes
        // precedence over the same value extracted from `contact_details`.
        let document = ContactValues {
            details: prefer_existing(
                existing.details,
                extracted.as_ref().and_then(|extracted| extracted.details.as_deref()),
            )
            .map(str::to_string),
            email: prefer_existing(
                existing.email,
                extracted.as_ref().and_then(|extracted| extracted.email.as_deref()),
            )
            .map(str::to_string),
            public_openpgp_key_url: prefer_existing(
                existing.public_openpgp_key_url,
                extracted
                    .as_ref()
                    .and_then(|extracted| extracted.public_openpgp_key_url.as_deref()),
            )
            .map(str::to_string),
        };

        let details = resolve(
            document.details.as_deref(),
            configured.details.as_deref(),
            contact_settings,
        )
        .map(str::to_string);
        let email =
            resolve(document.email.as_deref(), configured.email.as_deref(), contact_settings).map(str::to_string);
        let mut public_openpgp_key_url = resolve(
            document.public_openpgp_key_url.as_deref(),
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

        if contact_present || details.is_some() || email.is_some() || public_openpgp_key_url.is_some() {
            let contact = publisher
                .entry("contact".to_string())
                .or_insert_with(|| Value::Object(Map::new()));
            if !contact.is_object() {
                // A non-object `contact` cannot be merged with, so it is replaced
                *contact = Value::Object(Map::new());
            }
            // `unwrap_or_else` instead of `unwrap`: the value is turned into an object above
            #[allow(clippy::unwrap_used)]
            let contact = contact.as_object_mut().unwrap_or_else(|| unreachable!());

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
        }
    }
}

/// The values of the CSAF 2.1 `contact_t` type, gathered from the different sources
struct ContactValues {
    details: Option<String>,
    email: Option<String>,
    public_openpgp_key_url: Option<String>,
}

/// Values already present in the `contact` property of the document. The property is not part of
/// CSAF 2.0, but documents with partially converted contact data must not lose their values.
struct ExistingContact<'a> {
    details: Option<&'a str>,
    email: Option<&'a str>,
    public_openpgp_key_url: Option<&'a str>,
}

impl<'a> ExistingContact<'a> {
    fn from_map(contact: Option<&'a Map<String, Value>>) -> Self {
        let property = |name: &str| contact.and_then(|contact| contact.get(name)).and_then(Value::as_str);
        Self {
            details: property("details"),
            email: property("email"),
            public_openpgp_key_url: property("public_openpgp_key_url"),
        }
    }
}

/// Values provided via the settings; invalid ones are skipped with a warning
struct ConfiguredContact {
    details: Option<String>,
    email: Option<String>,
    public_openpgp_key_url: Option<String>,
}

impl ConfiguredContact {
    fn new(settings: &PublisherContactSettings, diagnostics: &mut dyn Diagnostics) -> Self {
        Self {
            details: configured(
                "details",
                &settings.details,
                csaf2_1::schema::ContactDetails::from_str,
                diagnostics,
            ),
            email: configured("email", &settings.email, csaf2_1::schema::Email::from_str, diagnostics),
            public_openpgp_key_url: configured(
                "public_openpgp_key_url",
                &settings.public_openpgp_key_url,
                csaf2_1::schema::PublicOpenPgpKeyUrl::from_str,
                diagnostics,
            ),
        }
    }
}

/// Validates a configured value against the requirements of its CSAF 2.1 schema type. Invalid
/// values are announced via a warning and skipped instead of ending up in the converted document.
fn configured<T, E>(
    property: &'static str,
    value: &Option<String>,
    parse: impl Fn(&str) -> Result<T, E>,
    diagnostics: &mut dyn Diagnostics,
) -> Option<String>
where
    E: std::fmt::Display,
{
    let Some(value) = value else {
        return None;
    };
    match parse(value) {
        Ok(_) => Some(value.clone()),
        Err(reason) => {
            diagnostics.warning(
                format!("/document/publisher/contact/{property}").into(),
                Warning::PublisherContactInvalidSetting {
                    property,
                    value: value.clone(),
                    reason: reason.to_string(),
                },
            );
            None
        },
    }
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

/// Keeps the value from the `contact` property if present, otherwise uses the extracted one
fn prefer_existing<'a>(existing: Option<&'a str>, extracted: Option<&'a str>) -> Option<&'a str> {
    existing.or(extracted)
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
/// HTTPS URLs) are intentionally not adopted; they stay part of the `details` text.
fn find_openpgp_key_url(content: &str) -> Option<FoundUrl> {
    static URL_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r#"https://[^\s<>"']+"#).unwrap()
    });

    for url in URL_RE.find_iter(content) {
        let value = trim_url_tail(url.as_str());
        if csaf2_1::schema::PublicOpenPgpKeyUrl::try_from(value).is_ok()
            && mentions_openpgp(&content[..url.start()], value)
        {
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
/// If a public OpenPGP key URL was found, the email address closest to it is preferred, so that
/// the key and the email address it is valid for stay associated ("the matching email address").
/// Otherwise, the first email address in the content is used.
fn find_email(content: &str, openpgp_key_url: Option<&FoundUrl>) -> Option<String> {
    static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r"[A-Za-z0-9._%+'-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)+").unwrap()
    });

    let url_range = openpgp_key_url.map(|found| &found.range);
    let within_key_url = |start: usize| url_range.is_some_and(|range| range.contains(&start));

    let mut candidates = EMAIL_RE
        .find_iter(content)
        .filter(|email| !within_key_url(email.start()))
        .map(|email| (email.start(), email.as_str()));

    match url_range {
        Some(range) => candidates
            .min_by_key(|(start, _)| start.abs_diff(range.start))
            .map(|(_, email)| email.to_string()),
        None => candidates.next().map(|(_, email)| email.to_string()),
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
                details: Some("security@example.com".to_string()),
                email: Some("security@example.com".to_string()),
                public_openpgp_key_url: Some("https://example.com/key".to_string()),
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
                details: Some("Configured details".to_string()),
                email: Some("security@configured.example".to_string()),
                public_openpgp_key_url: Some("https://configured.example/key".to_string()),
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
    fn merges_into_existing_contact() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "contact": {
                        "details": "Already present details"
                    },
                    "contact_details": "Contact security@document.example.",
                    "name": "Example Publisher",
                    "namespace": "https://example.com"
                }
            }
        });

        let output = convert(input);

        // `details` is already present in the `contact` property, so only `email` is filled in
        assert_eq!(
            json!({
                "details": "Already present details",
                "email": "security@document.example"
            }),
            output["document"]["publisher"]["contact"]
        );
        // The `contact_details` property has no equivalent in CSAF 2.1 and must be gone,
        // even though only the extracted email address ended up in the document
        assert!(output["document"]["publisher"].get("contact_details").is_none());
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
                email: Some("security@example.com".to_string()),
                public_openpgp_key_url: Some("https://example.com/key".to_string()),
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
    fn skips_invalid_configured_values_with_warning() {
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
                email: Some("a@b.c".to_string()),
                public_openpgp_key_url: Some("http://example.com/key".to_string()),
                overwrite_extracted: false,
            },
        );

        assert!(output["document"]["publisher"].get("contact").is_none());
        assert_eq!(
            vec![
                (
                    "/document/publisher/contact/email".into(),
                    Warning::PublisherContactInvalidSetting {
                        property: "email",
                        value: "a@b.c".to_string(),
                        reason: "shorter than 6 characters".to_string(),
                    },
                ),
                (
                    "/document/publisher/contact/public_openpgp_key_url".into(),
                    Warning::PublisherContactInvalidSetting {
                        property: "public_openpgp_key_url",
                        value: "http://example.com/key".to_string(),
                        reason: r#"doesn't match pattern "^https:\/\/""#.to_string(),
                    },
                ),
            ],
            diagnostics.warnings()
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
                details: Some("Configured details".to_string()),
                email: Some("security@example.com".to_string()),
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
