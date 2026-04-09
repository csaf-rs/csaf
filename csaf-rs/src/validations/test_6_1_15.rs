use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::schema::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;
use std::sync::LazyLock;

const SOURCE_LANG_INSTANCE_PATH: &str = "/document/source_lang";

static MISSING_SOURCE_LANG_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "source_lang is not defined but required when the publisher category is 'translator'.".to_string(),
    instance_path: SOURCE_LANG_INSTANCE_PATH.to_string(),
});

static EMPTY_SOURCE_LANG_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "source_lang is present with empty value, but is required when the publisher category is 'translator'.".to_string(),
    instance_path: SOURCE_LANG_INSTANCE_PATH.to_string(),
});

/// 6.1.15 Translator
///
/// It MUST be tested that /document/source_lang is present and set if the value translator is
/// used for /document/publisher/category.
///
/// A CSAF Validator SHALL differentiate in the error message between the key being present but
/// having no or an empty value and not being present at all.
pub fn test_6_1_15_translator(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    // This test only applies if the publisher category is "translator"
    if CategoryOfPublisher::Translator != document.get_publisher().get_category() {
        // This should be a wasSkipped later (see #409)
        return Ok(());
    }

    // Check if source_lang is present and set
    match document.get_source_lang() {
        Some(lang) if lang.to_string().is_empty() => Err(vec![EMPTY_SOURCE_LANG_ERROR.clone()]),
        Some(_) => Ok(()), // We do not care if the language tag is valid or invalid
        None => Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_15, test_6_1_15_translator);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_15() {
        // Error cases
        let missing_source_lang_error = Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);

        // case 01: translator category without source_lang
        // case 02: translator category without source_lang, but lang field is present
        // case 11: translator category with source_lang
        // case 12: translator category with source_lang and lang field is present
        // case S11: source_lang is missing, but category is not translator (should be skipped)

        TESTS_2_0.test_6_1_15.expect(
            missing_source_lang_error.clone(),
            missing_source_lang_error.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_15.expect(
            missing_source_lang_error.clone(),
            missing_source_lang_error,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
