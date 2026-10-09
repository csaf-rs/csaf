use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json::{JsonValuePresence, is_present_and_set},
};
use serde_json::Value;
use std::sync::LazyLock;

/// 6.2.12 Missing Document Language
///
/// It SHALL be tested that the document language member is present and set.
/// A CSAF Validator SHALL differentiate in the error message between the key being present
/// but having no or an empty value and not being present at all.
pub fn test_6_2_12_missing_document_language(json: &Value) -> Result<(), Vec<TestFinding>> {
    match is_present_and_set("/document/lang", json) {
        JsonValuePresence::Missing => Err(vec![MISSING_DOCUMENT_LANGUAGE_WARNING.clone()]),
        JsonValuePresence::Unset | JsonValuePresence::Empty => Err(vec![UNSET_DOCUMENT_LANGUAGE_WARNING.clone()]),
        JsonValuePresence::Set => Ok(()),
    }
}

static MISSING_DOCUMENT_LANGUAGE_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language is not defined".to_string(),
        instance_path: "/document".to_string(),
    })
});

static UNSET_DOCUMENT_LANGUAGE_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language is empty or not set (e.g., `null`)".to_string(),
        instance_path: "/document/lang".to_string(),
    })
});

crate::test_validation::impl_raw_json_validator!(ValidatorForTest6_2_12, test_6_2_12_missing_document_language);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_12 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_12 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_12() {
        let missing_document_language_property = Err(vec![MISSING_DOCUMENT_LANGUAGE_WARNING.clone()]);
        let unset_document_language = Err(vec![UNSET_DOCUMENT_LANGUAGE_WARNING.clone()]);
        let empty_document_language = unset_document_language.clone();

        // Case 11: document lang "en"
        // Case 12: document lang is bool true
        // Case 13: document lang is number 42

        TESTS_2_0.test_6_2_12.expect(ExpectedResults_2_0 {
            case_01: missing_document_language_property.clone(),
            case_s01: unset_document_language.clone(),
            case_s02: empty_document_language.clone(),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
        TESTS_2_1.test_6_2_12.expect(ExpectedResults_2_1 {
            case_01: missing_document_language_property,
            case_s01: unset_document_language,
            case_s02: empty_document_language,
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
    }
}
