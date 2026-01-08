use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::generated::language_subtags::is_valid_language_subtag;
use crate::validation::ValidationError;

fn generate_invalid_language_error(language: &str, subtag: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Invalid language code '{}': primary language subtag '{}' is not a valid language subtag",
            language, subtag
        ),
        instance_path: path.to_string(),
    }
}

pub fn test_6_1_12_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    let mut errors: Option<Vec<ValidationError>> = None;
    // Check /document/lang if it exists
    if let Some(lang) = document.get_lang()
        && let Err(e) = validate_language_code(lang, "/document/lang")
    {
        errors.get_or_insert_default().extend(e);
    }

    // Check /document/source_lang if it exists
    if let Some(source_lang) = document.get_source_lang()
        && let Err(e) = validate_language_code(source_lang, "/document/source_lang")
    {
        errors.get_or_insert_default().extend(e);
    }

    errors.map_or(Ok(()), Err)
}

fn validate_language_code(lang_code: &str, json_path: &str) -> Result<(), Vec<ValidationError>> {
    // Extract the primary language subtag (everything before the first hyphen)
    let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);

    if !is_valid_language_subtag(primary_subtag) {
        return Err(vec![generate_invalid_language_error(
            lang_code,
            primary_subtag,
            json_path,
        )]);
    }

    Ok(())
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
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_12() {
        let case_01 = Err(vec![generate_invalid_language_error("EZ", "EZ", "/document/lang")]);

        TESTS_2_0.test_6_1_12.expect(case_01.clone());
        TESTS_2_1.test_6_1_12.expect(case_01);
    }
}
