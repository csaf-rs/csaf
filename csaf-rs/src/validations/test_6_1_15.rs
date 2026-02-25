use crate::csaf::types::csaf_language::{CsafLanguage, CsafLanguageError};
use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::schema::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;
use std::sync::LazyLock;

static MISSING_SOURCE_LANG_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "source_lang is required when the publisher category is 'translator'".to_string(),
    instance_path: "/document/source_lang".to_string(),
});

fn invalid_language_warning(invalid_lang_tag: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "source_lang is required when the publisher category is 'translator', but the provided value is invalid: '{invalid_lang_tag}'"
        ),
        instance_path: "/document/source_lang".to_string(),
    }
}

/// 6.1.15 Translator
///
/// If the `/document/publisher/category` is "translator", then the `/document/source_lang` must be present and set.
pub fn test_6_1_15_translator(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    // This test only applies if the publisher category is "translator"
    if CategoryOfPublisher::Translator != document.get_publisher().get_category() {
        // This should be a wasSkipped later (see #409)
        return Ok(());
    }

    // Check if source_lang is present
    let source_lang = match document.get_source_lang() {
        Some(lang) => lang,
        None => return Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]),
    };

    // Check if source_lang is set
    match CsafLanguage::from(source_lang) {
        CsafLanguage::Invalid(err) => match err {
            // This should be a warning, but we don't have those yet. So for now, return an error (see #409)
            CsafLanguageError::InvalidLangTag(invalid_lang_tag, _) => {
                Err(vec![invalid_language_warning(&invalid_lang_tag)])
            },
        },
        CsafLanguage::Valid(_) => Ok(()),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_15
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_15_translator(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_15
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_15_translator(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_15() {
        // Error cases
        let missing_source_lang_error = Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);
        let invalid_source_lang_error = Err(vec![invalid_language_warning("EZ")]);

        // case 01: translator category without source_lang
        // case 02: translator category without source_lang, but lang field is present
        // case 11: translator category with source_lang
        // case 12: translator category with source_lang and lang field is present
        // case S01: source_lang is present but invalid (should give warning later)
        // case S11: source_lang is missing, but category is not translator (should be skipped)

        TESTS_2_0.test_6_1_15.expect(
            missing_source_lang_error.clone(),
            missing_source_lang_error.clone(),
            invalid_source_lang_error.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_15.expect(
            missing_source_lang_error.clone(),
            missing_source_lang_error,
            invalid_source_lang_error,
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()),
        );
    }
}
