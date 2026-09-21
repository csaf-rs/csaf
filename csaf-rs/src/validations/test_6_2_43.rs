use serde_json::Value;

use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json::{JsonValuePresence, is_present_and_set},
};
use std::sync::LazyLock;

static MISSING_LICENSE_EXPRESSION_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document's license expression is missing".to_string(),
        instance_path: "/document/license_expression".to_string(),
    })
});

static UNSET_LICENSE_EXPRESSION_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document's license expression is empty or not set (e.g., `null`)".to_string(),
        instance_path: "/document/license_expression".to_string(),
    })
});

/// 6.2.43 Missing License Expression
///
/// It SHALL be tested that the license expression is present and set.
/// A CSAF Validator SHALL differentiate in the error message between the key being present but
/// having no or an empty value and not being present at all.
pub fn test_6_2_43_missing_license_expression(json: &Value) -> Result<(), Vec<TestFinding>> {
    match is_present_and_set("/document/license_expression", json) {
        JsonValuePresence::Missing => Err(vec![MISSING_LICENSE_EXPRESSION_ERROR.clone()]),
        JsonValuePresence::Unset | JsonValuePresence::Empty => Err(vec![UNSET_LICENSE_EXPRESSION_ERROR.clone()]),
        JsonValuePresence::Set => Ok(()),
    }
}

crate::test_validation::impl_raw_json_validator!(
    csaf2_1,
    ValidatorForTest6_2_43,
    test_6_2_43_missing_license_expression
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_43 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_43() {
        let missing_license_expression_error = Err(vec![MISSING_LICENSE_EXPRESSION_ERROR.clone()]);
        let unset_license_expression = Err(vec![UNSET_LICENSE_EXPRESSION_ERROR.clone()]);
        let empty_license_expression = unset_license_expression.clone();

        TESTS_2_1.test_6_2_43.expect(ExpectedResults_2_1 {
            case_01: missing_license_expression_error,
            case_s01: unset_license_expression,
            case_s02: empty_license_expression,
            case_11: Ok(()),
        });
    }
}
