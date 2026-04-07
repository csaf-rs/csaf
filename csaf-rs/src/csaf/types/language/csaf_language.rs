use super::invalid_language::CsafLanguageError;
use super::valid_language::ValidCsafLanguage;
use crate::csaf::types::language::language_subtags::{
    is_valid_grandfathered_subtag, is_valid_language_subtag, is_valid_region_subtag, is_valid_script_subtag,
};
use crate::schema::csaf2_0::schema::LangT as LangT20;
use crate::schema::csaf2_1::schema::LangT as LangT21;
use oxilangtag::LanguageTag;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CsafLanguage {
    Valid(ValidCsafLanguage),
    Invalid(String, CsafLanguageError),
}

impl Display for CsafLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CsafLanguage::Valid(lang) => write!(f, "{lang}"),
            CsafLanguage::Invalid(lang, _) => write!(f, "{lang}"),
        }
    }
}

impl PartialEq for CsafLanguage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CsafLanguage::Valid(a), CsafLanguage::Valid(b)) => a == b,
            (CsafLanguage::Invalid(a, _), CsafLanguage::Invalid(b, _)) => a.eq_ignore_ascii_case(b),
            _ => false,
        }
    }
}

impl From<&LangT20> for CsafLanguage {
    fn from(lang_code: &LangT20) -> Self {
        CsafLanguage::from(&lang_code.to_string())
    }
}

impl From<&LangT21> for CsafLanguage {
    fn from(lang_code: &LangT21) -> Self {
        CsafLanguage::from(&lang_code.to_string())
    }
}

impl From<&String> for CsafLanguage {
    fn from(input_lang_tag: &String) -> Self {
        // Try to parse the input with oxilangtag
        let parsed_lang_tag: LanguageTag<String> = match LanguageTag::parse(input_lang_tag.clone()) {
            Ok(lang_tag) => lang_tag,
            Err(err) => {
                return CsafLanguage::Invalid(
                    input_lang_tag.clone(),
                    CsafLanguageError::ParserError(input_lang_tag.clone(), err.to_string()),
                );
            },
        };

        // Grandfathered tags are valid and skip further validation
        // This includes the default language (i-default) tags
        if is_valid_grandfathered_subtag(parsed_lang_tag.as_str()) {
            return CsafLanguage::Valid(ValidCsafLanguage::new(parsed_lang_tag));
        }

        // A bit of a wonky workaround: If the language code is just a private use tag ("x-private"),
        // the entire tag is put into all fields (primary language tag, region tag, ...)
        // So "is private use" + primary lang tag == full tag -> full tag is only private use,
        // so we can skip the validation
        if parsed_lang_tag.private_use().is_some() && input_lang_tag == parsed_lang_tag.primary_language() {
            return CsafLanguage::Valid(ValidCsafLanguage::new(parsed_lang_tag));
        }

        // TODO: Also discuss if we want this precedence of primary -> script -> region,
        // or if we want to expose everything "wrong" with the tag to the consumers.
        // Validate primary language subtag
        if !is_valid_language_subtag(parsed_lang_tag.primary_language()) {
            return CsafLanguage::Invalid(
                input_lang_tag.to_string(),
                CsafLanguageError::InvalidPrimaryLanguageSubtag(
                    input_lang_tag.to_string(),
                    parsed_lang_tag.primary_language().to_string(),
                ),
            );
        }

        // Validate script subtag
        if let Some(script) = parsed_lang_tag.script()
            && !is_valid_script_subtag(script)
        {
            return CsafLanguage::Invalid(
                input_lang_tag.to_string(),
                CsafLanguageError::InvalidScriptSubtag(
                    input_lang_tag.to_string(),
                    script.to_string()
                ),
            );
        }

        // Validate region subtag
        if let Some(region) = parsed_lang_tag.region()
            && !is_valid_region_subtag(region)
        {
            return CsafLanguage::Invalid(
                input_lang_tag.to_string(),
                CsafLanguageError::InvalidRegionSubtag(
                    input_lang_tag.to_string(),
                    region.to_string(),
                ),
            );
        }

        CsafLanguage::Valid(ValidCsafLanguage::new(parsed_lang_tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // plain language codes
    #[case("fr")]
    // with region subtag
    #[case("en-US")]
    #[case("en-us")]
    #[case("en-001")]
    // with script subtag
    #[case("zh-Hans")]
    // with region + script
    #[case("zh-Hans-CN")]
    // private use / default
    #[case("i-default")]
    #[case("x-private")]
    #[case("qtx")]
    #[case("en-x-foo-bar")]
    // grandfathered
    #[case("en-GB-oed")]
    // extlangs are not validated
    #[case("zh-cmn-Hans-CN")]
    // variants are not validated
    #[case("en-GB-oxendict")]
    // extensions are not validated
    #[case("en-US-u-ca-buddhist")]
    fn test_valid_language_tag_parses(#[case] input: &str) {
        let lang = CsafLanguage::from(&input.to_string());
        assert!(
            matches!(lang, CsafLanguage::Valid(_)),
            "Expected Valid for '{input}'"
        );
    }

    #[rstest]
    // invalid primary only
    #[case("EZ", CsafLanguageError::InvalidPrimaryLanguageSubtag)]
    // invalid script only
    #[case("en-Wxyz", CsafLanguageError::InvalidScriptSubtag)]
    // invalid region only
    #[case("en-QK", CsafLanguageError::InvalidRegionSubtag)]
    // invalid primary + valid region
    #[case("EZ-US", CsafLanguageError::InvalidPrimaryLanguageSubtag)]
    // invalid primary + invalid script → primary takes precedence
    #[case("EZ-Wxyz", CsafLanguageError::InvalidPrimaryLanguageSubtag)]
    // invalid primary + invalid region → primary takes precedence
    #[case("EZ-QK", CsafLanguageError::InvalidPrimaryLanguageSubtag)]
    // valid primary + invalid script + invalid region → script takes precedence
    #[case("en-Wxyz-QK", CsafLanguageError::InvalidScriptSubtag)]
    // all three invalid → primary takes precedence
    #[case("EZ-Wxyz-QK", CsafLanguageError::InvalidPrimaryLanguageSubtag)]
    fn test_invalid_language_tag_throws_error(
        #[case] input: &str,
        #[case] expected_error: fn(String, String) -> CsafLanguageError,
    ) {
        let lang = CsafLanguage::from(&input.to_string());
        assert!(
            matches!(&lang, CsafLanguage::Invalid(_, e)
                if std::mem::discriminant(e) == std::mem::discriminant(&expected_error(String::new(), String::new()))),
            "Expected error variant {:?} for '{input}', got: {lang:?}",
            expected_error(String::new(), String::new())
        );
    }

    #[rstest]
    // Valid vs Valid
    #[case("en-US", "en-US")]
    // Case insensitive
    #[case("en-US", "en-us")]
    #[case("i-default", "I-DEFAULT")]
    #[case("x-private", "x-Private")]
    // Invalid vs Invalid
    #[case("EZ", "EZ")]
    #[case("EZ", "ez")]
    fn test_valid_case_insensitive_equality(#[case] a: &str, #[case] b: &str) {
        assert_eq!(CsafLanguage::from(&a.to_string()), CsafLanguage::from(&b.to_string()));
    }

    #[rstest]
    #[case("en-US", "en-QK")] // Valid vs Invalid
    #[case("en-QK", "i-default")] // Invalid vs Valid (default)
    #[case("en-QK", "x-private")] // Invalid vs Valid (private use)
    #[case("en-US", "en-GB")] // differnt region
    #[case("en-Latn-GB", "de-Latg-GB")]
    // different script
    // TODO: Revisit this once equality has been clarified upstream
    // To we want to consider variants for equality (in this case, does the currency
    // being set to CAD matter to the document?)
    #[case("en-US", "en-US-u-cu-cad")]
    fn test_languages_not_equal(#[case] a: &str, #[case] b: &str) {
        assert_ne!(CsafLanguage::from(&a.to_string()), CsafLanguage::from(&b.to_string()));
    }
}
