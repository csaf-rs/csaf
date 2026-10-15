use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json_value_helper::{JsonValuePresence, get_json_presence},
};
use serde_json::Value;
use std::sync::LazyLock;

static MISSING_DOCUMENT_LANGUAGE_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language `lang` is not defined".to_string(),
        instance_path: "/document".to_string(),
    })
});

static UNSET_DOCUMENT_LANGUAGE_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language `lang` is not set (e.g., `null`)".to_string(),
        instance_path: "/document/lang".to_string(),
    })
});

static EMPTY_DOCUMENT_LANGUAGE_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language `lang` is empty (e.g., `\"\"`)".to_string(),
        instance_path: "/document/lang".to_string(),
    })
});

/// 6.2.12 Missing Document Language
///
/// It SHALL be tested that the document language member is present and set.
/// A CSAF Validator SHALL differentiate in the error message between the key being present
/// but having no or an empty value and not being present at all.
pub fn test_6_2_12_missing_document_language(json: &Value) -> Result<(), Vec<TestFinding>> {
    match get_json_presence("/document/lang", json) {
        JsonValuePresence::Missing => Err(vec![MISSING_DOCUMENT_LANGUAGE_WARNING.clone()]),
        JsonValuePresence::Unset => Err(vec![UNSET_DOCUMENT_LANGUAGE_WARNING.clone()]),
        JsonValuePresence::EmptyString => Err(vec![EMPTY_DOCUMENT_LANGUAGE_WARNING.clone()]),
        // empty array and empty object are schema violations, which are reported by the schema test
        JsonValuePresence::EmptyArray | JsonValuePresence::EmptyObject |
        // set is valid
        JsonValuePresence::Set => Ok(()),
    }
}



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
        let missing_document_language = Err(vec![MISSING_DOCUMENT_LANGUAGE_WARNING.clone()]);
        let unset_document_language = Err(vec![UNSET_DOCUMENT_LANGUAGE_WARNING.clone()]);
        let empty_document_language = Err(vec![EMPTY_DOCUMENT_LANGUAGE_WARNING.clone()]);

        // Case 01: document lang is missing
        // Case S01: document lang set to null (schema invalid)
        // Case S02: document lang is empty string (schema invalid)
        // Case S11: document lang "en"
        // Case S12: document lang is empty array (schema invalid)
        // Case S13: document lang is empty object (schema invalid)
        // Case S14: document lang is bool true (schema invalid)
        // Case S15: document lang is number 42 (schema invalid)

        // Note: Cases S01 - 04 and S12 - S13 are all schema-invalid.

        TESTS_2_0.test_6_2_12.expect(ExpectedResults_2_0 {
            case_01: missing_document_language.clone(),
            case_s01: unset_document_language.clone(),
            case_s02: empty_document_language.clone(),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
            case_s15: Ok(()),
        });
        TESTS_2_1.test_6_2_12.expect(ExpectedResults_2_1 {
            case_01: missing_document_language,
            case_s01: unset_document_language,
            case_s02: empty_document_language,
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
            case_s15: Ok(()),
        });
    }
}
