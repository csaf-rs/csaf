use crate::csaf::types::language::language_subtags::{
    is_language_private_use, is_region_private_use, is_script_private_use,
};
use oxilangtag::LanguageTag;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// Newtype wrapper around [`oxilangtag::LanguageTag`] representing a valid CSAF language tag.
///
/// This includes regular language tags (e.g. `en-US`), the default language tag (`i-default`),
/// and private-use language tags (e.g. `x-private`, `de-x-foo-bar`).
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
        Self(
            LanguageTag::parse(input.to_string())
                .expect("ValidCsafLanguage::new_for_tests was called with invalid input."),
        )
    }

    /// Checks if this is the grandfathered default language tag (`i-default`).
    pub fn is_default(&self) -> bool {
        self.0.as_str().eq_ignore_ascii_case("i-default")
    }

    /// Checks if this language tag contains a private-use subtag (e.g. `x-private` or `de-x-foo`)
    /// or if the primary language, script or region subtag itself is registered as private-use (e.g. `qtx` from `qaa..qtz`).
    pub fn is_private_use(&self) -> bool {
        self.0.private_use().is_some()
            || is_language_private_use(self.0.primary_language())
            || self.0.script().is_some_and(is_script_private_use)
            || self.0.region().is_some_and(is_region_private_use)
    }

    /// Gets the "reasons" a language tag is private-use.
    ///
    /// If there are reasons this tag is private, they are inherently ordered to match the order in the tag, i.e.
    /// 1. [PrivateUseReason::PrivateUsePrimaryLangSubtag] (e.g. `qaa`)
    /// 2. [PrivateUseReason::PrivateUseScriptSubtag] (e.g. `Qaaa`)
    /// 3. [PrivateUseReason::PrivateUseRegionSubtag] (e.g. `QM`)
    /// 4. [PrivateUseReason::PrivateUseSubtag] (e.g. `x-private-use`)
    ///
    /// If the language tag is a "standalone" private-use tag (e.g. `x-private-use`), only a [PrivateUseReason::PrivateUseSubtag] will be
    /// returned.
    ///
    /// Returns:
    /// * `Some(Vec<PrivateUseReason>)` if the language tag is private-use, with the vector containing
    ///   the specific [PrivateUseReason] values listed above
    /// * `None` if the language tag is not private-use
    pub fn get_private_use(&self) -> Option<Vec<PrivateUseReason>> {
        let mut result: Option<Vec<PrivateUseReason>> = None;
        if is_language_private_use(self.0.primary_language()) {
            result
                .get_or_insert_default()
                .push(PrivateUseReason::PrivateUsePrimaryLangSubtag(
                    self.0.primary_language().to_string(),
                ));
        }
        if let Some(script) = self.0.script()
            && is_script_private_use(script)
        {
            result
                .get_or_insert_default()
                .push(PrivateUseReason::PrivateUseScriptSubtag(script.to_string()));
        }
        if let Some(region) = self.0.region()
            && is_region_private_use(region)
        {
            result
                .get_or_insert_default()
                .push(PrivateUseReason::PrivateUseRegionSubtag(region.to_string()));
        }
        if let Some(private_use_subtag) = self.0.private_use() {
            result
                .get_or_insert_default()
                .push(PrivateUseReason::PrivateUseSubtag(private_use_subtag.to_string()));
        }
        result
    }

    /// Checks if the primary language subtag is case-insensitive `"en"` (English).
    pub fn is_english(&self) -> bool {
        self.0.primary_language().eq_ignore_ascii_case("en")
    }
}

/// Utility enum used in the return value in [ValidCsafLanguage::get_private_use]
#[derive(Debug, PartialEq)]
pub enum PrivateUseReason {
    PrivateUseSubtag(String),
    PrivateUsePrimaryLangSubtag(String),
    PrivateUseScriptSubtag(String),
    PrivateUseRegionSubtag(String),
}

/// So far, only used during 6.2.14, if we get more use cases for it, we should consider moving
/// this into the specific test.
impl Display for PrivateUseReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateUseReason::PrivateUseSubtag(subtag) => write!(f, "Private-use subtag '{subtag}'"),
            PrivateUseReason::PrivateUsePrimaryLangSubtag(primary_lang) => {
                write!(f, "Private-use primary language subtag '{primary_lang}'")
            },
            PrivateUseReason::PrivateUseScriptSubtag(script) => write!(f, "Private-use script subtag '{script}'"),
            PrivateUseReason::PrivateUseRegionSubtag(region) => write!(f, "Private-use region subtag '{region}'"),
        }
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
    // with private-use
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
    // private-use subtag
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
    // not private-use
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

    #[rstest]
    // not private-use
    #[case("en-US", None)]
    #[case("i-default", None)]
    // private-use primary language subtag
    #[case("qaa", Some(vec![PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string())]))]
    // private-use script subtag
    #[case("en-Qaaa", Some(vec![PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string())]))]
    // private-use region subtag
    #[case("en-QM", Some(vec![PrivateUseReason::PrivateUseRegionSubtag("QM".to_string())]))]
    // private-use subtag
    #[case("x-private-use", Some(vec![PrivateUseReason::PrivateUseSubtag("x-private-use".to_string())]))]
    // two reasons combined
    #[case("qaa-Qaaa", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
    ]))]
    #[case("qaa-QM", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseRegionSubtag("QM".to_string()),
    ]))]
    #[case("qaa-x-foo", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-foo".to_string()),
    ]))]
    #[case("en-Qaaa-x-foo", Some(vec![
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-foo".to_string()),
    ]))]
    #[case("en-XZ-x-bar", Some(vec![
        PrivateUseReason::PrivateUseRegionSubtag("XZ".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-bar".to_string()),
    ]))]
    // three reasons combined
    #[case("qaa-Qaaa-QM", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
        PrivateUseReason::PrivateUseRegionSubtag("QM".to_string()),
    ]))]
    #[case("qaa-Qaaa-x-baz", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-baz".to_string()),
    ]))]
    #[case("qaa-QM-x-quux", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseRegionSubtag("QM".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-quux".to_string()),
    ]))]
    #[case("en-Qaaa-XA-x-test", Some(vec![
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
        PrivateUseReason::PrivateUseRegionSubtag("XA".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-test".to_string()),
    ]))]
    // all four reasons combined
    #[case("qaa-Qaaa-QM-x-private-use", Some(vec![
        PrivateUseReason::PrivateUsePrimaryLangSubtag("qaa".to_string()),
        PrivateUseReason::PrivateUseScriptSubtag("Qaaa".to_string()),
        PrivateUseReason::PrivateUseRegionSubtag("QM".to_string()),
        PrivateUseReason::PrivateUseSubtag("x-private-use".to_string()),
    ]))]
    fn test_get_private_use(#[case] input: &str, #[case] expected: Option<Vec<PrivateUseReason>>) {
        let lang = ValidCsafLanguage::new_for_tests(input);
        assert_eq!(lang.get_private_use(), expected, "Mismatch for '{input}'");
    }
}
