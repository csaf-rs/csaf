use crate::validation::ValidationError;

/// Represents an error that occurred while parsing or validating a language tag in a CSAF document.
#[derive(Debug, PartialEq, Clone)]
pub enum CsafLanguageError {
    /// Oxilangtag parsing failed
    ParserError(String, String),
    /// Primary language subtag does not exist in the IANA registry
    InvalidPrimaryLanguageSubtag(String, String),
    /// Script subtag does not exist in the IANA registry
    InvalidScriptSubtag(String, String),
    /// Region subtag does not exist in the IANA registry
    InvalidRegionSubtag(String, String),
}

impl CsafLanguageError {
    pub fn into_validation_error(self, instance_path: &str) -> ValidationError {
        match self {
            CsafLanguageError::ParserError(invalid_lang_tag, parser_error) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': parser failed with error: {parser_error}"
                ),
                instance_path: instance_path.to_string(),
            },
            CsafLanguageError::InvalidPrimaryLanguageSubtag(invalid_lang_tag, primary_lang_subtag) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': primary language subtag '{primary_lang_subtag}' is not a valid primary language subtag"
                ),
                instance_path: instance_path.to_string(),
            },

            CsafLanguageError::InvalidScriptSubtag(invalid_lang_tag, script_subtag) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': script subtag '{script_subtag}' is not a valid script subtag"
                ),
                instance_path: instance_path.to_string(),
            },
            CsafLanguageError::InvalidRegionSubtag(invalid_lang_tag, region_sub_tag) => ValidationError {
                message: format!(
                    "Invalid language code '{invalid_lang_tag}': region subtag '{region_sub_tag}' is not a valid region subtag"
                ),
                instance_path: instance_path.to_string(),
            },
        }
    }
}
