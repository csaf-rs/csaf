//! Checks for CSAF 2.1 properties. The generated schema types only assert a subset of the
//! property constraints (pattern, length); the remaining constraints are validated here. The
//! validators are derived at runtime from the same CSAF 2.1 schema asset that the document
//! validation is built from — so the property checks cannot diverge from the output validation.

use std::sync::LazyLock;

use jsonschema::Validator;
use serde_json::{Value, json};

/// The CSAF 2.1 JSON Schema — the same asset the document validators are built from
const CSAF_21_SCHEMA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/csaf_2.1_json_schema.json"));

/// The parsed CSAF 2.1 JSON Schema, shared by the property validators below
static PARSED_CSAF_21_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    serde_json::from_str(CSAF_21_SCHEMA).unwrap()
});

/// Extracts a subschema of the `contact_t` type from the CSAF 2.1 schema asset. The property
/// subschemas carry no references, so they compile standalone.
// the pointer addresses are statically known parts of the embedded CSAF 2.1 schema asset
#[allow(clippy::expect_used)]
fn contact_property_subschema(pointer: &str) -> Value {
    PARSED_CSAF_21_SCHEMA
        .pointer(pointer)
        .expect("CSAF 2.1 schema asset must contain the contact property constraints")
        .clone()
}

fn build_validator(schema: &Value) -> Validator {
    // the extracted subschema carries no `$schema` key, so the draft is fixed explicitly
    #[allow(clippy::unwrap_used)]
    jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(schema)
        .unwrap()
}

/// Validates the exact constraints of the CSAF 2.1 property `email`
static EMAIL_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| build_validator(&contact_property_subschema("/$defs/contact_t/properties/email")));

/// Validates the exact constraints of the CSAF 2.1 property `public_openpgp_key_url`
static PUBLIC_OPENPGP_KEY_URL_VALIDATOR: LazyLock<Validator> = LazyLock::new(|| {
    build_validator(&contact_property_subschema(
        "/$defs/contact_t/properties/public_openpgp_key_url",
    ))
});

/// Checks whether `value` fulfills the exact constraints of the CSAF 2.1 property `email`,
/// including its `format: email` and `minLength: 6` requirements
pub fn is_valid_email(value: &str) -> bool {
    EMAIL_VALIDATOR.is_valid(&json!(value))
}

/// Checks whether `value` fulfills the exact constraints of the CSAF 2.1 property
/// `public_openpgp_key_url`, including its `format: uri`, `pattern: ^https://` and
/// `minLength: 11` requirements
pub fn is_valid_public_openpgp_key_url(value: &str) -> bool {
    PUBLIC_OPENPGP_KEY_URL_VALIDATOR.is_valid(&json!(value))
}

/// The `addr-spec` search pattern. It is deliberately loose about the dot-atom rules: the dot is
/// part of the local-part character class, so that an address like `security..alerts@example.org`
/// is found as a whole and rejected by [`is_valid_email`] instead of being partially extracted.
pub(super) const EMAIL_SEARCH_PATTERN: &str = r"[A-Za-z0-9!#$%&'*+/=?^_`{|}~.-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)+";

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::plain_email("security@example.org", true)]
    #[case::atext_local_part("security=alerts@example.org", true)]
    #[case::dotted_local_part("security.alerts@example.org", true)]
    #[case::too_short("a@b.c", false)]
    #[case::not_an_email("not-an-email", false)]
    #[case::missing_domain_part("security@", false)]
    #[case::missing_local_part("@example.org", false)]
    #[case::double_dot_in_local_part("security..alerts@example.org", false)]
    #[case::leading_dot_in_local_part(".security@example.org", false)]
    #[case::trailing_dot_in_local_part("security.@example.org", false)]
    #[case::double_dot_in_domain("security@example..org", false)]
    #[case::leading_hyphen_in_domain_label("security@-example.org", false)]
    #[case::trailing_hyphen_in_domain_label("security@example-.org", false)]
    fn email_is_validated_against_the_csaf_2_1_requirements(#[case] value: &str, #[case] is_valid: bool) {
        assert_eq!(is_valid, is_valid_email(value), "{value}");
    }

    #[rstest]
    #[case::plain_url("https://example.com/key", true)]
    #[case::query_and_fragment("https://example.com/key?op=get&search=0x00#hash", true)]
    #[case::percent_escaped("https://example.com/ke%20y", true)]
    #[case::ipv6_host_literal("https://[2001:db8::1]/key", true)]
    #[case::bracket_in_path("https://example.com/[key", false)]
    #[case::incomplete_percent_escape("https://example.org/key%ZZ.asc", false)]
    #[case::trailing_percent_escape("https://example.org/key%", false)]
    #[case::space("https://example.com/a b", false)]
    #[case::non_ascii("https://example.de/schl\u{00fc}ssel.asc", false)]
    #[case::control_character("https://example.com/ke\u{7f}y", false)]
    #[case::angle_brackets("https://example.com/<key>", false)]
    #[case::backslash("https://example.com\\key", false)]
    fn public_openpgp_key_url_is_validated_against_the_csaf_2_1_requirements(
        #[case] value: &str,
        #[case] is_valid: bool,
    ) {
        assert_eq!(is_valid, is_valid_public_openpgp_key_url(value), "{value}");
    }
}
