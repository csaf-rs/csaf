use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::test_validation::impl_validator;
use crate::validation::ValidationError;

fn create_usage_of_deprecated_profile_error(category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!("Document category '{category}' starts with 'csaf_deprecated_' (or similar)"),
        instance_path: "/document/category".to_string(),
    }
}

/// 6.2.38 Usage of Deprecated Profile
///
/// It MUST be tested that the `/document/category` does not start with `csaf_deprecated_`.
/// To implement this test it is deemed sufficient to do a "starts with" check.
pub fn test_6_2_38_usage_of_deprecated_profile(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let category = doc.get_document().get_category();

    if category.to_string().starts_with("csaf_deprecated_") {
        Err(vec![create_usage_of_deprecated_profile_error(&category)])
    } else {
        Ok(())
    }
}

impl_validator!(csaf2_1, ValidatorForTest6_2_38, test_6_2_38_usage_of_deprecated_profile);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_38() {
        let case_01 = Err(vec![create_usage_of_deprecated_profile_error(
            &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
        )]);
        let case_02 = Err(vec![create_usage_of_deprecated_profile_error(
            &CsafDocumentCategory::CsafBaseOther("csaf_deprecated_unknown_type".to_string()),
        )]);
        // Case 11: different profile ("csaf_security_advisory")
        // Case 12: with prefix ("Example Company csaf_deprecated_security_advisory")´
        // Case 13: casing ("CSAF_deprecated_security_advisory")
        // Case S11: leading whitespace (" csaf_deprecated_some_other_type")
        TESTS_2_1
            .test_6_2_38
            .expect(case_01, case_02, Ok(()), Ok(()), Ok(()), Ok(()));
    }
}
