use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;
use std::sync::LazyLock;

/// 6.2.12 Missing Document Language
///
/// `/document/lang` must be set.
pub fn test_6_2_12_missing_document_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    if doc.get_document().get_lang().is_none() {
        return Err(vec![MISSING_DOCUMENT_LANGUAGE.clone()]);
    }
    Ok(())
}

static MISSING_DOCUMENT_LANGUAGE: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The document language is not defined".to_string(),
    instance_path: "/document/lang".to_string(),
});

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_12
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_12_missing_document_language(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_12
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_12_missing_document_language(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_12() {
        let err = Err(vec![MISSING_DOCUMENT_LANGUAGE.clone()]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_12.expect(err.clone());
        TESTS_2_1.test_6_2_12.expect(err);
    }
}
