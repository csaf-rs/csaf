use crate::csaf::types::csaf_language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

fn create_same_language_error(lang: &CsafLanguage) -> ValidationError {
    ValidationError {
        message: format!("document language and source language have the same value '{lang}'"),
        instance_path: "/document/source_lang".to_string(),
    }
}

/// 6.1.28 Translation
///
/// `/document/lang` and `/document/source_lang` must have different values
pub fn test_6_1_28_translation(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    // Check if both lang and source_lang are present
    let (lang, source_lang) = match (document.get_lang(), document.get_source_lang()) {
        (Some(lang), Some(source_lang)) => (lang, source_lang),
        (_, _) => return Ok(()), // This should be a wasSkipped later (see #409)
    };

    if lang == source_lang {
        return Err(vec![create_same_language_error(&lang)]);
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
        let case_01_same_value = Err(vec![create_same_language_error(&CsafLanguage::from(
            &"en-US".to_string(),
        ))]);
        let case_s01_default_casing = Err(vec![create_same_language_error(&CsafLanguage::from(
            &"i-default".to_string(),
        ))]);

        // Case 11: /document/lang and /document/source_lang are set to different values

        TESTS_2_0
            .test_6_1_28
            .expect(case_01_same_value.clone(), case_s01_default_casing.clone(), Ok(()));
        TESTS_2_1
            .test_6_1_28
            .expect(case_01_same_value, case_s01_default_casing, Ok(()));
    }
}
