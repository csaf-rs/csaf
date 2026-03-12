use crate::generated::language_subtags::is_valid_language_subtag;
use crate::schema::csaf2_0::schema::LangT as LangT20;
use crate::schema::csaf2_1::schema::LangT as LangT21;
use crate::validation::ValidationError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CsafLanguage {
    Valid(String),
    DefaultLanguage(String),
    Invalid(String, CsafLanguageError),
}

impl Display for CsafLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CsafLanguage::Valid(lang) => write!(f, "{lang}"),
            CsafLanguage::DefaultLanguage(lang) => write!(f, "{lang}"),
            CsafLanguage::Invalid(lang, _) => write!(f, "{lang}"),
        }
    }
}

impl PartialEq for CsafLanguage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CsafLanguage::Valid(a), CsafLanguage::Valid(b)) => a.eq_ignore_ascii_case(b),
            (CsafLanguage::DefaultLanguage(a), CsafLanguage::DefaultLanguage(b)) => a.eq_ignore_ascii_case(b),
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
    fn from(lang_code: &String) -> Self {
        if lang_code.eq_ignore_ascii_case("i-default") {
            CsafLanguage::DefaultLanguage(lang_code.to_string())
        } else {
            let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);
            if !is_valid_language_subtag(primary_subtag) {
                CsafLanguage::Invalid(
                    lang_code.to_string(),
                    CsafLanguageError::InvalidPrimaryLangTag(lang_code.to_string(), primary_subtag.to_string()),
                )
            } else {
                CsafLanguage::Valid(lang_code.to_string())
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CsafLanguageError {
    InvalidPrimaryLangTag(String, String),
}

impl CsafLanguageError {
    pub fn into_validation_error(self, json_path: &str) -> ValidationError {
        match self {
            CsafLanguageError::InvalidPrimaryLangTag(invalid_lang_tag, primary_lang_subtag) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': primary language subtag '{primary_lang_subtag}' is not a valid language subtag"
                ),
                instance_path: json_path.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_language() {
        let lang = CsafLanguage::from(&"en".to_string());
        assert_eq!(lang, CsafLanguage::Valid("en".to_string()));
    }

    #[test]
    fn test_default_language() {
        let lang1 = CsafLanguage::from(&"i-default".to_string());
        assert_eq!(lang1, CsafLanguage::DefaultLanguage("i-default".to_string()));
    }

    #[test]
    fn test_valid_language_compare_case_insensitive() {
        let lang1 = CsafLanguage::from(&"en-US".to_string());
        let lang2 = CsafLanguage::from(&"en-us".to_string());
        assert_eq!(lang1, lang2);
    }

    #[test]
    fn test_invalid_language_compare_case_insensitive() {
        // this is a region tag
        let lang1 = CsafLanguage::from(&"EZ".to_string());
        // this does not exist
        let lang2 = CsafLanguage::from(&"ez".to_string());
        assert_eq!(lang1, lang2);
    }

    #[test]
    fn test_default_lang_compare_case_insensitive() {
        let lang1 = CsafLanguage::from(&"i-default".to_string());
        let lang2 = CsafLanguage::from(&"I-DEFAULT".to_string());
        assert_eq!(lang1, lang2);
    }

    #[test]
    fn test_invalid_language() {
        let lang = CsafLanguage::from(&"EZ".to_string());
        assert_eq!(
            lang,
            CsafLanguage::Invalid(
                "EZ".to_string(),
                CsafLanguageError::InvalidPrimaryLangTag("EZ".to_string(), "EZ".to_string())
            )
        );
    }
}
