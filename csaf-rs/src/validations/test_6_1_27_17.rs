use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::document_notes_with_title_and_category::check_notes_with_title_and_category;

/// 6.1.27.17 Reasoning for withdrawal
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` and only if the document language is English (i.e., `/document/lang` with value `en`) or unspecified.
///
/// If the document language is English or unspecified, it MUST be tested that exactly one item in document notes exists that has the title Reasoning for Withdrawal.
/// The category of this item MUST be description.
pub fn test_6_1_27_17_document_notes_for_withdrawal(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }
    match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_default() || !valid_lang.is_english() => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(_) => {}, // this is english
        None => {},    // no language set
    }

    check_notes_with_title_and_category(
        doc.get_document().get_notes().map(Vec::as_slice),
        "Reasoning for Withdrawal",
        &NoteCategory::Description,
        &doc_category,
    )
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafWithdrawn]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_17,
    test_6_1_27_17_document_notes_for_withdrawal
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_17 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::NoteCategory;
    use crate::validations::utils::document_notes_with_title_and_category::{
        create_duplicated_note_error, create_incorrect_category_error, create_missing_note_error,
    };

    #[test]
    fn test_test_6_1_27_17() {
        const TITLE: &str = "Reasoning for Withdrawal";
        const DOC_CATEGORY: &CsafDocumentCategory = &CsafDocumentCategory::CsafWithdrawn;
        const NOTE_CATEGORY: &NoteCategory = &NoteCategory::Description;
        let create_incorrect_category_error = |wrong_category: &NoteCategory, index| {
            create_incorrect_category_error(TITLE, wrong_category, NOTE_CATEGORY, DOC_CATEGORY, index)
        };
        let create_missing_reasoning_error = create_missing_note_error(TITLE, NOTE_CATEGORY, DOC_CATEGORY);
        let create_duplicated_reasoning_error = |index| create_duplicated_note_error(TITLE, DOC_CATEGORY, index);

        let case_01_wrong_category_summary = Err(vec![create_incorrect_category_error(&NoteCategory::Summary, 0)]);
        // Case 02: duplicate titles, different category
        // Case 04: duplicate titles, same category
        let duplicate_title = Err(vec![
            create_duplicated_reasoning_error(0),
            create_duplicated_reasoning_error(1),
        ]);
        // Case 03: 2 notes, one with correct title, wrong category, one with wrong title, correct category
        // We only report the correct title, wrong category note
        let case_03_wrong_category = Err(vec![create_incorrect_category_error(&NoteCategory::Details, 0)]);
        // Case 05: 2 notes, one with correct title, wrong category, one with wrong title, correct category, also language is en-US
        // We only report the correct title, wrong category note
        let case_05_lang_en_us_wrong_category = Err(vec![create_incorrect_category_error(&NoteCategory::General, 0)]);
        // Case S01: no notes at all
        // Case S02: only one note, wrong category, wrong title, also en-GB
        let missing_reasoning = Err(vec![create_missing_reasoning_error]);

        // Case 11: no lang, correct title / description
        // Case 12: en-GB, correct title / description
        // Case 13: de-DE, test does not apply

        TESTS_2_1.test_6_1_27_17.expect(ExpectedResults {
            case_01: case_01_wrong_category_summary,
            case_02: duplicate_title.clone(),
            case_03: case_03_wrong_category,
            case_04: duplicate_title,
            case_05: case_05_lang_en_us_wrong_category,
            case_s01: missing_reasoning.clone(),
            case_s02: missing_reasoning,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
