use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::schema::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;
use std::sync::LazyLock;

static MISSING_SOURCE_LANG_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "source_lang is required when the publisher category is 'translator'".to_string(),
    instance_path: "/document/source_lang".to_string(),
});

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
    match document.get_source_lang() {
        None => Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]),
        _ => Ok(()), // We do not care if the language tag is valid or invalid
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
