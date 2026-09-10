use crate::validation::{TestFinding, TestFindingData};
use serde_json::Value;
use std::sync::LazyLock;

/// 6.2.12 Missing Document Language
///
/// `/document/lang` must be set.
pub fn test_6_2_12_missing_document_language(json: &Value) -> Result<(), Vec<TestFinding>> {
    match json.pointer("/document/lang") {
        Some(Value::Null) => Err(vec![UNSET_DOCUMENT_LANGUAGE.clone()]),
        Some(Value::String(l)) if l.is_empty() => Err(vec![UNSET_DOCUMENT_LANGUAGE.clone()]),
        Some(_) => Ok(()),
        None => Err(vec![MISSING_DOCUMENT_LANGUAGE.clone()]),
    }
}

static MISSING_DOCUMENT_LANGUAGE: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document language is not defined".to_string(),
        instance_path: "/document/lang".to_string(),
    })
});

static UNSET_DOCUMENT_LANGUAGE: LazyLock<TestFinding> = LazyLock::new(|| {
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
        let missing_document_language_property = Err(vec![MISSING_DOCUMENT_LANGUAGE.clone()]);
        let unset_document_language = Err(vec![UNSET_DOCUMENT_LANGUAGE.clone()]);
        let empty_document_language = unset_document_language.clone();

        TESTS_2_0.test_6_2_12.expect(ExpectedResults_2_0 {
            case_01: missing_document_language_property.clone(),
            case_s01: unset_document_language.clone(),
            case_s02: empty_document_language.clone(),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_2_12.expect(ExpectedResults_2_1 {
            case_01: missing_document_language_property,
            case_s01: unset_document_language,
            case_s02: empty_document_language,
            case_s11: Ok(()),
        });
    }
}
