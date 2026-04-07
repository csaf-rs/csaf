use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

/// 6.2.15 Use of Default Language
///
/// The language tag in `/document/lang` and `/document/source_lang` must not contain the default language code `i-default`.
pub fn test_6_2_15_use_of_default_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    if document.get_lang().is_none() && document.get_source_lang().is_none() {
        return Ok(()); // This should be a wasSkipped later (see #409)
    }

    let mut errors: Option<Vec<ValidationError>> = None;

    validate_default_language(document.get_lang(), "/document/lang", &mut errors);
    validate_default_language(document.get_source_lang(), "/document/source_lang", &mut errors);

    errors.map_or(Ok(()), Err)
}

/// Helper function to validate a `lang` tag and check if it is the default language.
///
/// If the optional language tag is `Some` and is the default language (`i-default`), an
/// error will be added to `errors` vector.
///
/// # Arguments
/// - `lang`: The (optional) language tag to validate
/// - `json_path`: The JSON path to the language tag
/// - `errors`: A mutable reference to the errors vector
fn validate_default_language(lang: Option<CsafLanguage>, json_path: &str, errors: &mut Option<Vec<ValidationError>>) {
    if let Some(CsafLanguage::Valid(valid_lang)) = lang
        && valid_lang.is_default()
    {
        errors.get_or_insert_default().push(create_default_language_error(
            valid_lang.as_str().to_string(),
            json_path,
        ));
    }
}

fn create_default_language_error(lang_tag: String, instance_path: &str) -> ValidationError {
    ValidationError {
        message: format!("The default language tag '{lang_tag}' may not be used"),
        instance_path: instance_path.to_string(),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_15, test_6_2_15_use_of_default_language);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_15() {
        let case_01_default_lang = Err(vec![create_default_language_error(
            "i-default".to_string(),
            "/document/lang",
        )]);
        let case_02_default_source_lang = Err(vec![create_default_language_error(
            "i-default".to_string(),
            "/document/source_lang",
        )]);

        let case_s01_default_both_langs = Err(vec![
            create_default_language_error("i-default".to_string(), "/document/lang"),
            create_default_language_error("i-default".to_string(), "/document/source_lang"),
        ]);
        let case_s02_default_lang_uppercase = Err(vec![create_default_language_error(
            "I-DEFAULT".to_string(),
            "/document/lang",
        )]);

        // Case 11: /document/lang is not set to the default language
        // Case S11: Both /document/lang and /document/source_lang are missing (should be skipped? #409)
        TESTS_2_0.test_6_2_15.expect(
            case_01_default_lang.clone(),
            case_02_default_source_lang.clone(),
            case_s01_default_both_langs.clone(),
            case_s02_default_lang_uppercase.clone(),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_2_15.expect(
            case_01_default_lang,
            case_02_default_source_lang,
            case_s01_default_both_langs,
            case_s02_default_lang_uppercase,
            Ok(()),
            Ok(()),
        );
    }
}
