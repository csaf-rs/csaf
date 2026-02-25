use crate::generated::language_subtags::is_valid_language_subtag;
use crate::validation::ValidationError;
use crate::schema::csaf2_0::schema::LangT as LangT20;
use crate::schema::csaf2_1::schema::LangT as LangT21;

#[derive(Debug, PartialEq)]
pub enum CsafLanguage {
    Valid(ValidLanguage),
    Invalid(CsafLanguageError),
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
        let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);
        if !is_valid_language_subtag(primary_subtag) {
            CsafLanguage::Invalid(CsafLanguageError::InvalidLangTag(
                lang_code.to_owned(),
                primary_subtag.to_string(),
            ))
        } else {
            CsafLanguage::Valid(ValidLanguage(lang_code.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidLanguage(String);

impl ValidLanguage {
    pub fn is_default_language(&self) -> bool {
        self.0 == "i-default"
    }
    
    pub fn get_lowercase(&self) -> String {
        self.0.to_lowercase()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CsafLanguageError {
    InvalidLangTag(String, String),
}

impl CsafLanguageError {
    pub fn into_validation_error(self, json_path: &str) -> ValidationError {
        match self {
            CsafLanguageError::InvalidLangTag(invalid_lang_tag, primary_lab_subtag) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': primary language subtag '{primary_lab_subtag}' is not a valid language subtag"
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
        assert_eq!(lang, CsafLanguage::Valid(ValidLanguage("en".to_string())));
    }

    #[test]
    fn test_invalid_language() {
        let lang = CsafLanguage::from(&"EZ".to_string());
        assert_eq!(
            lang,
            CsafLanguage::Invalid(CsafLanguageError::InvalidLangTag("EZ".to_string(), "EZ".to_string()))
        );
    }
}
