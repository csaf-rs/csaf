use crate::extractor::extract::ExtractPrimitive;
use crate::extractor::navigate::AtPath;
use crate::two_step_validation::{TwoStepValidator, impl_two_step_validator, make_validator};
use crate::validation::ValidationError;

fn create_usage_of_deprecated_profile_error(category: &str) -> ValidationError {
    ValidationError {
        message: format!("Document category '{category}' starts with 'csaf_deprecated_'"),
        instance_path: "/document/category".to_string(),
    }
}

/// 6.2.38 Usage of Deprecated Profile
///
/// It MUST be tested that the `/document/category` does not start with `csaf_deprecated_`.
/// To implement this test it is deemed sufficient to do a "starts with" check.
fn create_validator_6_2_38() -> impl TwoStepValidator {
    make_validator(
        AtPath::new("document", ExtractPrimitive::new_string("category")),
        |category: Option<String>| {
            if let Some(category) = category
                && category.starts_with("csaf_deprecated_")
            {
                Err(vec![create_usage_of_deprecated_profile_error(category.as_str())])
            } else {
                Ok(())
            }
        },
    )
}

impl_two_step_validator!(csaf2_1, ValidatorForTest6_2_38, create_validator_6_2_38);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{csaf::types::csaf_document_category::CsafDocumentCategory, csaf2_1::testcases::TESTS_2_1};

    #[test]
    fn test_test_6_2_38() {
        let case_01 = Err(vec![create_usage_of_deprecated_profile_error(
            &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory.to_string(),
        )]);
        let case_02 = Err(vec![create_usage_of_deprecated_profile_error(
            "csaf_deprecated_unknown_type",
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
