use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::document_notes_with_title_and_category::check_document_notes_with_title_and_category;

/// 6.1.27.18 Reasoning for supersession
///
/// This test only applies to documents with `/document/category` with value `csaf_superseded` and only if the document language is English (i.e., `/document/lang` with value `en`) or unspecified.
///
/// If the document language is English or unspecified, it MUST be tested that exactly one item in document notes exists that has the title Reasoning for Supersession.
/// The category of this item MUST be description.
pub fn test_6_1_27_18_document_notes_for_supersession(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
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

    check_document_notes_with_title_and_category(
        doc.get_document().get_notes().map(Vec::as_slice),
        "Reasoning for Supersession",
        &doc_category,
    )
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafSuperseded]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_18,
    test_6_1_27_18_document_notes_for_supersession
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_18 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::document_notes_with_title_and_category::{
        create_duplicated_note_error, create_incorrect_category_error, create_missing_note_error,
    };

    #[test]
    fn test_test_6_1_27_18() {
        const TITLE: &str = "Reasoning for Supersession";
        let create_missing_reasoning_error =
            |doc_category: &CsafDocumentCategory| create_missing_note_error(TITLE, doc_category);
        let create_duplicated_reasoning_error =
            |doc_category: &CsafDocumentCategory, index| create_duplicated_note_error(TITLE, doc_category, index);

        let undefined_lang_wrong_category = Err(vec![create_incorrect_category_error(0)]);
        let undefined_lang_duplicate_title = Err(vec![
            create_duplicated_reasoning_error(&CsafDocumentCategory::CsafSuperseded, 0),
            create_duplicated_reasoning_error(&CsafDocumentCategory::CsafSuperseded, 1),
        ]);
        let lang_en_us_wrong_category = Err(vec![create_incorrect_category_error(0)]);
        let lang_en_gb_missing_reasoning = Err(vec![create_missing_reasoning_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        TESTS_2_1.test_6_1_27_18.expect(ExpectedResults {
            case_01: undefined_lang_wrong_category.clone(),
            case_02: undefined_lang_duplicate_title.clone(),
            case_03: undefined_lang_wrong_category,
            case_04: undefined_lang_duplicate_title,
            case_05: lang_en_us_wrong_category,
            case_s01: lang_en_gb_missing_reasoning.clone(),
            case_s02: lang_en_gb_missing_reasoning,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
