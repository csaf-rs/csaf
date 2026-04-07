use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::test_validation::impl_validator;
use crate::validation::ValidationError;

fn create_usage_of_deprecated_profile_error(category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!("Document category '{category}' starts with 'csaf_deprecated_'"),
        instance_path: "/document/category".to_string(),
    }
}

/// 6.2.38 Usage of Deprecated Profile
///
/// It MUST be tested that the `/document/category` does not start with `csaf_deprecated_`.
pub fn test_6_2_38_usage_of_deprecated_profile(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let category = doc.get_document().get_category();

    if category.starts_with_csaf_deprecated() {
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

        TESTS_2_1.test_6_2_38.expect(case_01, Ok(()));
    }
}
