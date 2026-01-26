use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::schema::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;

static MISSING_SOURCE_LANG_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "source_lang is required when the publisher category is 'translator'".to_string(),
    instance_path: "/document/source_lang".to_string(),
});

/// 6.1.15 Translator
///
/// If the `/document/publisher/category` is "translator", then the `/document/source_lang` must be present.
pub fn test_6_1_15_translator(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    // This test only applies if the publisher category is "translator"
    if CategoryOfPublisher::Translator != document.get_publisher().get_category() {
        return Ok(());
    }

    // Check if source_lang is present
    if document.get_source_lang().is_none() {
        return Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);
    }

    Ok(())
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
        let err = Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);

        // Both CSAF 2.0 and 2.1 have 4 test cases (01, 02, 11, 12)
        // 01 has no lang and no source_lang
        // 02 has lang but no source_lang
        // 11 has no lang and sourcelang
        // 12 has lang and source_lang
        TESTS_2_0.test_6_1_15.expect(
            err.clone(),
            err.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
        );

        TESTS_2_1.test_6_1_15.expect(
            err.clone(),
            err.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
        );
    }
}
mod csaf_20_custom_tests {
    use super::*;
    use crate::schema::csaf2_0::schema::{CommonSecurityAdvisoryFramework, DocumentLevelMetaData, Publisher};

    #[test]
    fn test_publisher_category_behavior() {
        fn create_mock(category: CategoryOfPublisher) -> CommonSecurityAdvisoryFramework {
            CommonSecurityAdvisoryFramework::builder()
                .document(
                    DocumentLevelMetaData::builder()
                        .lang("de-DE")
                        .publisher(Publisher::builder().category(category))
                        .try_into()
                        .unwrap(),
                )
                .try_into()
                .unwrap()
        }

        // Test with Translator category (should error)
        assert!(test_6_1_15_translator(&create_mock(CategoryOfPublisher::Translator)).is_err());

        // Test with Discoverer category (should pass)
        assert!(test_6_1_15_translator(&create_mock(CategoryOfPublisher::Discoverer)).is_ok());
    }
}

mod csaf_21_custom_tests {
    use super::*;
    use crate::schema::csaf2_1::schema::{CommonSecurityAdvisoryFramework, DocumentLevelMetaData, Publisher};

    #[test]
    fn test_publisher_category_behavior() {
        fn create_mock(category: CategoryOfPublisher) -> CommonSecurityAdvisoryFramework {
            CommonSecurityAdvisoryFramework::builder()
                .document(
                    DocumentLevelMetaData::builder()
                        .lang("de-DE")
                        .publisher(Publisher::builder().category(category))
                        .try_into()
                        .unwrap(),
                )
                .try_into()
                .unwrap()
        }

        // Test with Translator category (should error)
        assert!(test_6_1_15_translator(&create_mock(CategoryOfPublisher::Translator)).is_err());

        // Test with Discoverer category (should pass)
        assert!(test_6_1_15_translator(&create_mock(CategoryOfPublisher::Discoverer)).is_ok());
    }
}
