use serde_json::Value;

use crate::{
    csaf::types::{
        csaf_document_category::CsafDocumentCategory,
        language::CsafLanguage::{self, Valid},
    },
    schema::csaf2_1::schema::CategoryOfReference,
    validation::{TestFinding, TestFindingData},
    validations::utils::{
        document_references_with_summary_and_category::{
            create_incorrect_category_data, create_missing_reference_data,
        },
        language_specific_translations::{
            create_no_translation_known_info, get_translation_for_term_superseding_document,
        },
        raw_json::{JsonValuePresence, is_present_and_set, property_string_value_is},
    },
};

const SUPERSEDING_DOCUMENT_EN: &str = "Superseding Document";

/// 6.2.39.4 Language Specific Superseding Document
///
/// This test only applies to documents with `/document/category` with value `csaf_superseded` and
/// only if the document language is specified but not English.
///
/// It MUST be tested that at least one item in document references exists that has a summary
/// starting with the language specific translation of the term `Superseding Document`. The
/// category of this item MUST be `external`. If no language specific translation has been
/// recorded, the test MUST be skipped and output an information to the user that no such
/// translation is known.
pub fn test_6_2_39_4_language_specific_superseding_document(json: &Value) -> Result<(), Vec<TestFinding>> {
    if !property_string_value_is(
        "/document/category",
        &CsafDocumentCategory::CsafSuperseded.to_string(),
        json,
    ) || is_present_and_set("/document/lang", json) == JsonValuePresence::Missing
    {
        // Ignore documents with wrong category or unset languages where default is assumed
        return Ok(());
    }

    let language = if let Some(Value::String(lang)) = json.pointer("/document/lang")
        && let Valid(lang) = CsafLanguage::from(lang)
        && !lang.is_english()
    {
        lang
    } else {
        return Ok(());
    };

    let term = if let Some(term) = get_translation_for_term_superseding_document(language.primary_language()) {
        term
    } else {
        // If the translation is unknown it will be for all occurrences. Reporting for each
        // individual note does not make sense and is time inefficient
        return Err(vec![create_no_translation_known_info(
            SUPERSEDING_DOCUMENT_EN,
            &language.to_string(),
            "/document/references/*",
        )]);
    };

    let Some(Value::Array(references)) = json.pointer("/document/references") else {
        return Ok(());
    };

    let mut reference_iterator = references.iter().enumerate()
        .filter(|(_, reference)| {
            matches!(reference.pointer("/summary"), Some(Value::String(summary)) if summary.starts_with(term))
        })
        .peekable();
    // TODO: Discuss if we need this here again or if 6.1.27.2 as a check is sufficient
    // If not this can be removed and findings constructed directly from the iterator
    if reference_iterator.peek().is_none() {
        return Err(vec![TestFinding::Warning(create_missing_reference_data(
            term,
            &CategoryOfReference::External,
            &CsafDocumentCategory::CsafSuperseded,
        ))]);
    }
    let findings: Vec<_> = reference_iterator
        .filter(|(_, reference)| {
            !property_string_value_is("/category", &CategoryOfReference::External.to_string(), reference)
        })
        .map(|(reference_idx, reference)| {
            if is_present_and_set("/category", reference) == JsonValuePresence::Missing {
                create_external_category_missing_error(reference_idx)
            } else {
                // TODO: Do we expect there to be other categories as well?
                create_wrong_category_set_for_superseeding_document(term, &CategoryOfReference::Self_, reference_idx)
            }
        })
        .collect();

    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

fn create_external_category_missing_error(idx: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: String::from("Reference entry for a superseding document does not set the category `external`."),
        instance_path: format!("/document/references/{idx}/category"),
    })
}

fn create_wrong_category_set_for_superseeding_document(
    term: &str,
    wrong_category: &CategoryOfReference,
    idx: usize,
) -> TestFinding {
    TestFinding::Warning(create_incorrect_category_data(
        term,
        wrong_category,
        &CategoryOfReference::External,
        &CsafDocumentCategory::CsafSuperseded,
        idx,
    ))
}

crate::test_validation::impl_raw_json_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_4,
    test_6_2_39_4_language_specific_superseding_document
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_4 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::document_references_with_summary_and_category::{
        create_incorrect_category_data, create_missing_reference_data,
    };

    #[test]
    fn test_test_6_2_39_4() {
        let de_summary_prefix = get_translation_for_term_superseding_document("de").unwrap();

        let no_reference_with_prefix = Err(vec![TestFinding::Warning(create_missing_reference_data(
            de_summary_prefix,
            &CategoryOfReference::External,
            &CsafDocumentCategory::CsafSuperseded,
        ))]);

        let incorrect_category = Err(vec![TestFinding::Warning(create_incorrect_category_data(
            de_summary_prefix,
            &CategoryOfReference::Self_,
            &CategoryOfReference::External,
            &CsafDocumentCategory::CsafSuperseded,
            0,
        ))]);

        let reference_entry_to_superseeding_doc_missing_external_category = Err(vec![
            create_external_category_missing_error(0),
            TestFinding::Warning(create_incorrect_category_data(
                de_summary_prefix,
                &CategoryOfReference::Self_,
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
                2,
            )),
        ]);

        let multiple_incorrect_category = Err(vec![
            TestFinding::Warning(create_incorrect_category_data(
                de_summary_prefix,
                &CategoryOfReference::Self_,
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
                0,
            )),
            TestFinding::Warning(create_incorrect_category_data(
                de_summary_prefix,
                &CategoryOfReference::Self_,
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
                1,
            )),
        ]);

        // Case 11: correct category + prefix
        // Case 12: multiple correct category + prefix
        let case_s11_esperanto_no_translation = Err(vec![create_no_translation_known_info(
            "Superseding Document",
            "eo",
            "/document/references/*",
        )]);

        TESTS_2_1.test_6_2_39_4.expect(ExpectedResults {
            case_01: no_reference_with_prefix,
            case_02: reference_entry_to_superseeding_doc_missing_external_category,
            case_s01: incorrect_category,
            case_s02: multiple_incorrect_category,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: case_s11_esperanto_no_translation,
        });
    }
}
