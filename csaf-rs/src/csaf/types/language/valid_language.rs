use crate::csaf::types::language::language_subtags::{
    is_language_private_use, is_region_private_use, is_script_private_use,
};
use oxilangtag::LanguageTag;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// Newtype wrapper around [`oxilangtag::LanguageTag`] representing a valid CSAF language tag.
///
/// This includes regular language tags (e.g. `en-US`), the default language tag (`i-default`),
/// and private use language tags (e.g. `x-private`, `de-x-foo-bar`).
///
/// Exposes the API of [`oxilangtag::LanguageTag`] and some additional utility methods for validation tests.
#[derive(Debug, Clone)]
pub struct ValidCsafLanguage(LanguageTag<String>);

impl ValidCsafLanguage {
    /// Creates a new `ValidCsafLanguage` from an owned [`LanguageTag`].
    /// Available only to language module.
    pub(in crate::csaf::types::language) fn new(tag: LanguageTag<String>) -> Self {
        Self(tag)
    }

    /// Creates a new `ValidCsafLanguage` from an owned [`LanguageTag`].
    /// Available only to tests, will panic on invalid input.
    #[cfg(test)]
    pub fn new_for_tests(input: &str) -> Self {
        Self(LanguageTag::parse(input.to_string()).expect("ValidCsafLanguage::new_for_tests was called with invalid input."))
    }

    /// Checks if this is the grandfathered default language tag (`i-default`).
    pub fn is_default(&self) -> bool {
        self.0.as_str().eq_ignore_ascii_case("i-default")
    }

    /// Checks if this language tag contains a private use component (e.g. `x-private` or `de-x-foo`)
    /// or if the primary language, script or region subtag itself is registered as private use (e.g. `qtx` from `qaa..qtz`).
    pub fn is_private_use(&self) -> bool {
        self.0.private_use().is_some()
            || is_language_private_use(self.0.primary_language())
            || self.0.script().is_some_and(is_script_private_use)
            || self.0.region().is_some_and(is_region_private_use)
    }

    /// Checks if the primary language subtag is case-insensitive `"en"` (English).
    pub fn is_english(&self) -> bool {
        self.0.primary_language().eq_ignore_ascii_case("en")
    }
}

impl Deref for ValidCsafLanguage {
    type Target = LanguageTag<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ValidCsafLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for ValidCsafLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str().eq_ignore_ascii_case(other.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // casing
    #[case("i-default", true)]
    #[case("I-DEFAULT", true)]
    #[case("i-Default", true)]
    // not default
    #[case("en", false)]
    #[case("qaa", false)]
    #[case("x-private", false)]
    fn test_is_default(#[case] input: &str, #[case] expected: bool) {
        let lang = ValidCsafLanguage::new_for_tests(input);
        assert_eq!(
            lang.is_default(),
            expected,
            "Unexpected is_default() result for '{input}'"
        );
    }

    #[rstest]
    // casing
    #[case("en", true)]
    #[case("EN", true)]
    #[case("En", true)]
    // with region
    #[case("en-US", true)]
    // with private use
    #[case("en-x-private", true)]
    // not en
    #[case("es", false)]
    #[case("es-ES", false)]
    #[case("es-x-foo", false)]
    #[case("x-private", false)]
    #[case("i-default", false)]
    fn test_is_english(#[case] input: &str, #[case] expected: bool) {
        let lang = ValidCsafLanguage::new_for_tests(input);
        assert_eq!(
            lang.is_english(),
            expected,
            "Unexpected is_english() result for '{input}'"
        );
    }

    #[rstest]
    // private-use extension
    #[case("x-private", true)]
    #[case("en-x-foo", true)]
    // private-use primary language subtag (qaa..qtz)
    #[case("qaa", true)]
    #[case("qtz", true)]
    // private-use script subtag (Qaaa..Qabx)
    #[case("en-Qaaa", true)]
    #[case("de-Qabx", true)]
    // private-use region subtag (XA..XZ, ZZ)
    #[case("en-XA", true)]
    #[case("en-ZZ", true)]
    // not private use
    #[case("en", false)]
    #[case("en-US", false)]
    #[case("fr-Latn", false)]
    #[case("i-default", false)]
    fn test_is_private_use(#[case] input: &str, #[case] expected: bool) {
        let lang = ValidCsafLanguage::new_for_tests(input);
        assert_eq!(
            lang.is_private_use(),
            expected,
            "Unexpected is_private_use() result for '{input}'"
        );
    }
}
