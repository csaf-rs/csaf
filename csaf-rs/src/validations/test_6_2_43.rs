use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

const LICENSE_EXPRESSION_INSTANCE_PATH: &str = "/document/license_expression";

fn create_missing_license_expression_error() -> ValidationError {
    ValidationError {
        message: "The license expression is not defined, but is required.".to_string(),
        instance_path: LICENSE_EXPRESSION_INSTANCE_PATH.to_string(),
    }
}

fn create_empty_license_expression_error() -> ValidationError {
    ValidationError {
        message: "The license expression is present with an empty value, but is required".to_string(),
        instance_path: LICENSE_EXPRESSION_INSTANCE_PATH.to_string(),
    }
}

/// Test 6.2.43: Missing License Expression
///
/// It MUST be tested that the license expression is present and set.
///
/// A CSAF Validator SHALL differentiate in the error message between the key
/// being present but having no or an empty value and not being present at all.
pub fn test_6_2_43_missing_license_expression(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    match document.get_license_expression() {
        Some(license) if license.is_empty() => Err(vec![create_empty_license_expression_error()]),
        Some(_) => Ok(()),
        None => Err(vec![create_missing_license_expression_error()]),
    }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_43, test_6_2_43_missing_license_expression);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_43() {
        // Case 01: license_expression is not defined at all
        let case_01_missing = Err(vec![create_missing_license_expression_error()]);

        // Case 11: license_expression is present and set to a valid value
        TESTS_2_1.test_6_2_43.expect(case_01_missing, Ok(()));
    }
}
