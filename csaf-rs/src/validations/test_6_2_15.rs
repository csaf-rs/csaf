use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

/// 6.2.15 Use of Default Language
///
/// The language tag in `/document/lang` and `/document/source_lang` must not contain the default language code `i-default`.
pub fn test_6_2_15_use_of_default_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    let default_lang_tag = "i-default";
    let document = doc.get_document();
    if let Some(lang) = document.get_lang()
        && lang == default_lang_tag
    {
        errors
            .get_or_insert_with(Vec::new)
            .push(create_default_language_error("/document/lang"));
    }
    if let Some(source_lang) = document.get_source_lang()
        && source_lang == default_lang_tag
    {
        errors
            .get_or_insert_with(Vec::new)
            .push(create_default_language_error("/document/source_lang"));
    }

    errors.map_or(Ok(()), Err)
}

fn create_default_language_error(instance_path: &str) -> ValidationError {
    ValidationError {
        message: "The default language tag 'i-default' may not be used".to_string(),
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
    fn test_test_6_2_10() {
        let case_01 = Err(vec![create_default_language_error("/document/lang")]);
        let case_02 = Err(vec![create_default_language_error("/document/source_lang")]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_15.expect(case_01.clone(), case_02.clone(), Ok(()));
        TESTS_2_1.test_6_2_15.expect(case_01, case_02, Ok(()));
    }
}
