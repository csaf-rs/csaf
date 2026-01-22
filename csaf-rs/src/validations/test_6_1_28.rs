use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

fn create_same_language_error(lang: &str) -> ValidationError {
    ValidationError {
        message: format!("document language and source language have the same value {lang}"),
        instance_path: "/document/source_lang".to_string(),
    }
}

/// 6.1.28 Translation
///
/// `/document/lang` and `/document/source_lang` must have different values
pub fn test_6_1_28_translation(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    if let Some(lang) = document.get_lang() {
        if let Some(source_lang) = document.get_source_lang() {
            if lang.to_lowercase() == source_lang.to_lowercase() {
                return Err(vec![create_same_language_error(lang)]);
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_28
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_28_translation(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_28
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_28_translation(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_28() {
        let case_01 = Err(vec![create_same_language_error("en-US")]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_28.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_1_28.expect(case_01, Ok(()));
    }
}
