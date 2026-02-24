use crate::generated::language_subtags::is_valid_language_subtag;

#[derive(Debug, PartialEq)]
pub enum CsafLanguage {
    Valid(String),
    Invalid(CsafLanguageError),
}

impl From<&String> for CsafLanguage {
    fn from(lang_code: &String) -> Self {
        let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);
        if !is_valid_language_subtag(primary_subtag) {
            CsafLanguage::Invalid(CsafLanguageError::InvalidLangTag(lang_code.to_owned()))
        } else {
            CsafLanguage::Valid(lang_code.to_owned())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CsafLanguageError {
    InvalidLangTag(String),
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
    fn test_invalid_language() {
        let lang = CsafLanguage::from(&"EZ".to_string());
        assert_eq!(
            lang,
            CsafLanguage::Invalid(CsafLanguageError::InvalidLangTag("EZ".to_string()))
        );
    }
}
