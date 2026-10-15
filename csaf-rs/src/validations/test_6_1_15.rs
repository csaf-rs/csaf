use serde_json::Value;

use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json_value_helper::{JsonValuePresence, get_json_presence, property_string_value_is},
};
use std::sync::LazyLock;

static MISSING_SOURCE_LANG_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message: "`source_lang` is required when the publisher category is 'translator'".to_string(),
        instance_path: "/document".to_string(),
    })
});

static UNSET_SOURCE_LANG_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message:
            "`source_lang` property not set (e.g., `null`) even though the publisher category is 'translator'"
                .to_string(),
        instance_path: "/document/source_lang".to_string(),
    })
});

static EMPTY_SOURCE_LANG_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message:
        "`source_lang` property is empty (e.g., `\"\"`) even though the publisher category is 'translator'"
            .to_string(),
        instance_path: "/document/source_lang".to_string(),
    })
});


/// 6.1.15 Translator
///
/// It SHALL be tested that `$.document.source_lang is present and set if the value `translator` is
/// used for `$.document.publisher.category. A CSAF Validator SHALL differentiate in the error message
/// between the key being present but having no or an empty value and not being present at all.
pub fn test_6_1_15_translator(json: &Value) -> Result<(), Vec<TestFinding>> {
    // check if the category is translator
    if !property_string_value_is("/document/publisher/category", "translator", json) {
        return Ok(());
    }

    match get_json_presence("/document/source_lang", json) {
        JsonValuePresence::Missing => Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]),
        JsonValuePresence::Unset => Err(vec![UNSET_SOURCE_LANG_ERROR.clone()]),
        JsonValuePresence::EmptyString => Err(vec![EMPTY_SOURCE_LANG_ERROR.clone()]),
        // empty array and empty object are schema violations, which are reported by the schema test
        JsonValuePresence::EmptyArray | JsonValuePresence::EmptyObject |
        // set is valid
        JsonValuePresence::Set => Ok(())
    }
}

crate::test_validation::impl_raw_json_validator!(ValidatorForTest6_1_15, test_6_1_15_translator);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_15 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_15 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_15() {
        let missing_source_lang_error = Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);
        let unset_source_language = Err(vec![UNSET_SOURCE_LANG_ERROR.clone()]);
        let empty_source_language = Err(vec![EMPTY_SOURCE_LANG_ERROR.clone()]);

        // case 01: translator category without source_lang
        // case 02: translator category without source_lang, but lang field is present
        // case s01: translator category with source_lang set to null
        // case s02: translator category with source_lang set to empty string
        // case 11: translator category with source_lang
        // case 12: translator category with source_lang and lang field is present
        // case S11: source_lang is missing, but category is not translator (should be skipped)
        // case s12: translator category with source_lang set to an empty array
        // case s13: translator category with source_lang set to an empty object


        TESTS_2_0.test_6_1_15.expect(ExpectedResults_2_0 {
            case_01: missing_source_lang_error.clone(),
            case_02: missing_source_lang_error.clone(),
            case_s01: unset_source_language.clone(),
            case_s02: empty_source_language.clone(),
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),

        });

        TESTS_2_1.test_6_1_15.expect(ExpectedResults_2_1 {
            case_01: missing_source_lang_error.clone(),
            case_02: missing_source_lang_error,
            case_s01: unset_source_language,
            case_s02: empty_source_language,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
    }
}
