use crate::csaf::types::csaf_language::CsafLanguage;
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
    if let Some(CsafLanguage::DefaultLanguage(lang_tag)) = lang {
        errors
            .get_or_insert_default()
            .push(create_default_language_error(lang_tag, json_path));
    }
}

fn create_default_language_error(lang_tag: String, instance_path: &str) -> ValidationError {
    ValidationError {
        message: format!("The default language tag '{lang_tag}' may not be used"),
        instance_path: instance_path.to_string(),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_15
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_15_use_of_default_language(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_15
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_15_use_of_default_language(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_15() {
        // Case 01: /document/lang is set to the default language
        let case_01 = Err(vec![create_default_language_error(
            "i-default".to_string(),
            "/document/lang",
        )]);
        // Case 02: /document/source_lang is set to the default language
        let case_02 = Err(vec![create_default_language_error(
            "i-default".to_string(),
            "/document/source_lang",
        )]);
        // Case 11: /document/lang is not set to the default language

        // Case S01: Both /document/lang and /document/source_lang are set to the default language
        let case_s01 = Err(vec![
            create_default_language_error("i-default".to_string(), "/document/lang"),
            create_default_language_error("i-default".to_string(), "/document/source_lang"),
        ]);
        // Case S02: default lang in /document/lang tag is uppercase
        let case_s02 = Err(vec![create_default_language_error(
            "I-DEFAULT".to_string(),
            "/document/lang",
        )]);
        // Case S11: Both /document/lang and /document/source_lang are missing (should be skipped? #409)
        TESTS_2_0.test_6_2_15.expect(
            case_01.clone(),
            case_02.clone(),
            case_s01.clone(),
            case_s02.clone(),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1
            .test_6_2_15
            .expect(case_01, case_02, case_s01, case_s02, Ok(()), Ok(()));
    }
}
