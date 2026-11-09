//! `format` checks for CSAF 2.1 properties. The generated schema types only assert the pattern
//! and length restrictions; the JSON Schema `format` keywords are checked separately here, so
//! that extracted and configured values cannot end up as format-invalid converted output.

use std::str::FromStr;

use email_address::EmailAddress;

use crate::schema::csaf2_1;

/// The `addr-spec` search pattern. It is deliberately loose about the dot-atom rules: the dot is
/// part of the local-part character class, so that an address like `security..alerts@example.org`
/// is found as a whole and rejected by [`is_valid_email`] instead of being partially extracted.
pub(super) const EMAIL_SEARCH_PATTERN: &str = r"[A-Za-z0-9!#$%&'*+/=?^_`{|}~.-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)+";

/// Checks whether `value` fulfills the requirements of the CSAF 2.1 property `email`:
/// `format: email`, `minLength: 6`. The format is validated with the same `email_address`
/// library the schema validator uses for `format: email`, so the accepted values mirror its
/// semantics; non-ASCII local parts, which the CSAF `email` format does not allow, are
/// rejected as well.
pub fn is_valid_email(value: &str) -> bool {
    EmailAddress::from_str(value).is_ok_and(|parsed| parsed.local_part().is_ascii())
        && csaf2_1::schema::Email::from_str(value).is_ok()
}

/// Checks whether `value` fulfills the requirements of the CSAF 2.1 property
/// `public_openpgp_key_url`: `format: uri`, `pattern: ^https://`, `minLength: 11`
pub fn is_valid_public_openpgp_key_url(value: &str) -> bool {
    csaf2_1::schema::PublicOpenPgpKeyUrl::from_str(value).is_ok() && is_valid_uri(value)
}

/// Checks the `format: uri` requirement: only characters allowed by RFC 3986 and well-formed
/// percent escapes
fn is_valid_uri(value: &str) -> bool {
    fn is_allowed(byte: u8) -> bool {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b':'
                    | b'/'
                    | b'?'
                    | b'#'
                    | b'['
                    | b']'
                    | b'@'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b'%'
            )
    }

    if !value.is_ascii() {
        return false;
    }

    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' => {
                // a percent escape requires two hex digits
                let escaped = bytes
                    .get(index + 1..index + 3)
                    .is_some_and(|escape| escape.iter().all(u8::is_ascii_hexdigit));
                if !escaped {
                    return false;
                }
                index += 3;
            },
            byte if is_allowed(byte) => index += 1,
            _ => return false,
        }
    }
    true
}

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
    fn email_is_validated_against_the_csaf_2_1_requirements(#[case] value: &str, #[case] is_valid: bool) {
        assert_eq!(is_valid, is_valid_email(value), "{value}");
    }

    #[rstest]
    #[case::plain_url("https://example.com/key", true)]
    #[case::query_and_fragment("https://example.com/key?op=get&search=0x00#hash", true)]
    #[case::percent_escaped("https://example.com/ke%20y", true)]
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
