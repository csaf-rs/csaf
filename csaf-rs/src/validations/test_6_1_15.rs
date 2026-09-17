use serde_json::Value;

use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json::{JsonValuePresence, is_present_and_set, property_string_value_is},
};
use std::sync::LazyLock;

static MISSING_SOURCE_LANG_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message: "source_lang is required when the publisher category is 'translator'".to_string(),
        instance_path: "/document/source_lang".to_string(),
    })
});

static UNSET_SOURCE_LANG_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message:
            "source_lang property is empty or not set (e.g., `null`) even though the publisher category is 'translator'"
                .to_string(),
        instance_path: "/document/source_lang".to_string(),
    })
});

/// 6.1.15 Translator
///
/// If the `/document/publisher/category` is "translator", then the `/document/source_lang` must be present and set.
pub fn test_6_1_15_translator(json: &Value) -> Result<(), Vec<TestFinding>> {
    if !property_string_value_is("/document/publisher/category", "translator", json) {
        return Ok(());
    }
    match is_present_and_set("/document/source_lang", json) {
        JsonValuePresence::Missing => Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]),
        JsonValuePresence::Unset | JsonValuePresence::Empty => Err(vec![UNSET_SOURCE_LANG_ERROR.clone()]),
        JsonValuePresence::Set => Ok(()),
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
        // Error cases
        let missing_source_lang_error = Err(vec![MISSING_SOURCE_LANG_ERROR.clone()]);
        let unset_source_language = Err(vec![UNSET_SOURCE_LANG_ERROR.clone()]);
        let empty_source_language = unset_source_language.clone();

        // case 01: translator category without source_lang
        // case 02: translator category without source_lang, but lang field is present
        // case 11: translator category with source_lang
        // case 12: translator category with source_lang and lang field is present
        // case S11: source_lang is missing, but category is not translator (should be skipped)

        TESTS_2_0.test_6_1_15.expect(ExpectedResults_2_0 {
            case_01: missing_source_lang_error.clone(),
            case_02: missing_source_lang_error.clone(),
            case_11: Ok(()),
            case_12: Ok(()),
            case_s01: unset_source_language.clone(),
            case_s02: empty_source_language.clone(),
            case_s11: Ok(()),
        });

        TESTS_2_1.test_6_1_15.expect(ExpectedResults_2_1 {
            case_01: missing_source_lang_error.clone(),
            case_02: missing_source_lang_error,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s01: unset_source_language,
            case_s02: empty_source_language,
            case_s11: Ok(()),
        });
    }
}
