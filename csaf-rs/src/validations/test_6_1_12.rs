use crate::csaf::types::csaf_language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

pub fn test_6_1_12_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    if document.get_lang().is_none() && document.get_source_lang().is_none() {
        return Ok(()); // This should be a wasSkipped later (see #409)
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    validate_language(document.get_lang(), "/document/lang", &mut errors);
    validate_language(document.get_source_lang(), "/document/source_lang", &mut errors);

    errors.map_or(Ok(()), Err)
}


/// Validate a language code and append the validation error to the errors vector.
///
/// If the given language is [`CsafLanguage::Invalid`], a [`ValidationError`] is created
/// with the specified JSON path and added to the errors collection.
/// 
/// # Arguments
/// - `lang`: An optional language code to validate. 
/// - `json_path`: The JSON path to the language code being validated
/// - `errors`: A mutable reference to an optional vector of validation errors.
fn validate_language(
    lang: Option<CsafLanguage>,
    json_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    if let Some(CsafLanguage::Invalid(err)) = lang {
        errors
            .get_or_insert_default()
            .push(err.into_validation_error(json_path));
    }
}


impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_12
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_12_language(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_12
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_12_language(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_language::CsafLanguage::Invalid;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_12() {
        // Case 01: Invalid language code in /document/lang
        // Case S01: Invalid language code in /document/source_lang
        // Case S02: Invalid language code in both /document/lang and /document/source_lang
        // Case S11: Valid language code in both /document/lang and /document/source_lang
        // Case S12: Both /document/lang and /document/source_lang are missing (should be skipped? #409)

        let Invalid(ez_error) = CsafLanguage::from(&"EZ".to_string()) else {
            unreachable!()
        };
        let Invalid(zzz_error) = CsafLanguage::from(&"ZZZ".to_string()) else {
            unreachable!()
        };
        let case_01 = Err(vec![ez_error.clone().into_validation_error("/document/lang")]);
        let case_s01 = Err(vec![ez_error.clone().into_validation_error("/document/source_lang")]);
        let case_s02 = Err(vec![
            ez_error.clone().into_validation_error("/document/lang"),
            zzz_error.into_validation_error("/document/source_lang"),
        ]);

        TESTS_2_0
            .test_6_1_12
            .expect(case_01.clone(), case_s01.clone(), case_s02.clone(), Ok(()), Ok(()));
        TESTS_2_1
            .test_6_1_12
            .expect(case_01, case_s01, case_s02, Ok(()), Ok(()));
    }
}
