use serde_json::Value;

use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf::types::language::CsafLanguage::Valid;
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::TestFinding;
use crate::validations::utils::document_references_with_summary_and_category::check_references_with_summary_prefix_and_category;
use crate::validations::utils::raw_json_value_helper::property_string_value_is;

/// 6.1.27.19 Reference to superseding document
///
/// If the document language is English or unspecified, it SHALL be tested that at least one item in document references exists
/// that has a summary starting with `Superseding Document`.
/// For each of these items, the `category` SHALL be present and have the value `external`.
pub fn test_6_1_27_19_reference_to_superseding_document(json: &Value) -> Result<(), Vec<TestFinding>> {
    // check document category
    if !property_string_value_is(
        "/document/category",
        &CsafDocumentCategory::CsafSuperseded.to_string(),
        json,
    ) {
        return Ok(());
    }

    // check if language is english or unspecified
    let language_is_english_or_unspecified = match json.pointer("/document/lang") {
        None => true,
        Some(Value::String(language)) => {
            matches!(CsafLanguage::from(language), Valid(language) if language.is_english())
        },
        _ => false,
    };

    // if it isn't english or unspecified, skip
    if !language_is_english_or_unspecified {
        return Ok(());
    }

    check_references_with_summary_prefix_and_category(
        json.pointer("/document/references")
            .and_then(Value::as_array)
            .map(Vec::as_slice),
        "Superseding Document",
        &CategoryOfReference::External,
        &CsafDocumentCategory::CsafSuperseded,
    )
    .map(|findings| findings.into_iter().map(TestFinding::Error).collect())
    .map_or(Ok(()), Err)
}

crate::test_validation::impl_raw_json_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_19,
    test_6_1_27_19_reference_to_superseding_document
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_19 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::document_references_with_summary_and_category::{
        create_invalid_category_data, create_missing_reference_data,
    };
    use serde_json::json;

    #[test]
    fn test_test_6_1_27_19() {
        let self_category = json!("self");
        let missing_reference = Err(vec![TestFinding::Error(create_missing_reference_data(
            "Superseding Document",
            &CategoryOfReference::External,
            &CsafDocumentCategory::CsafSuperseded,
        ))]);
        let missing_category = Err(vec![TestFinding::Error(create_invalid_category_data(
            "Superseding Document",
            None,
            &CategoryOfReference::External,
            0,
        ))]);
        let multiple_invalid_categories = Err(vec![
            TestFinding::Error(create_invalid_category_data(
                "Superseding Document",
                None,
                &CategoryOfReference::External,
                0,
            )),
            TestFinding::Error(create_invalid_category_data(
                "Superseding Document",
                Some(&self_category),
                &CategoryOfReference::External,
                3,
            )),
        ]);
        let undefined_lang_wrong_category = Err(vec![TestFinding::Error(create_invalid_category_data(
            "Superseding Document",
            Some(&self_category),
            &CategoryOfReference::External,
            0,
        ))]);

        TESTS_2_1.test_6_1_27_19.expect(ExpectedResults {
            case_01: missing_reference.clone(),
            case_02: missing_category,
            case_03: multiple_invalid_categories,
            case_s01: undefined_lang_wrong_category,
            case_s02: missing_reference,
            case_11: Ok(()),  // lang: unspecified, single correct reference
            case_12: Ok(()),  // lang: en-us, multiple correct references
            case_13: Ok(()),  // lang: de-DE is ignored
            case_s11: Ok(()), // lang: en-us, category not csaf_superseded
        });
    }
}
