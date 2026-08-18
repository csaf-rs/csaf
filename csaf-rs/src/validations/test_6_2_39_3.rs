use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::document_notes_with_title_and_category::check_document_notes_with_title_and_category;
use crate::validations::utils::language_specific_translations::{
    create_no_translation_known_info, get_translation_for_term_reasoning_for_supersession,
};

/// 6.2.39.3 Language Specific Reasoning for Supersession
///
/// This test only applies to documents with `/document/category` with value `csaf_superseded` and
/// only if the document language is specified but not English.
///
/// It MUST be tested that exactly one item in document notes exists that has the language specific
/// translation of the term `Reasoning for Supersession` as title. The category of this item MUST
/// be `description`. If no language specific translation has been recorded, the test MUST be
/// skipped and output an information to the user that no such translation is known.
pub fn test_6_2_39_3_language_specific_reasoning_for_supersession(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let primary_lang = match doc.get_document().get_lang() {
        None => return Ok(()), // language unspecified, test 6.1.27.18 covers this
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // wasSkipped in #407
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_english() => return Ok(()), // language is default or english, english is covered by 6.1.27.18
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.primary_language().to_string(),
    };

    // get language-specific translation
    let Some(translated_title) = get_translation_for_term_reasoning_for_supersession(&primary_lang) else {
        return Err(vec![create_no_translation_known_info(
            "Reasoning for Supersession",
            &primary_lang,
        )]);
    };

    check_document_notes_with_title_and_category(
        doc.get_document().get_notes().map(Vec::as_slice),
        translated_title,
        &doc_category,
    )
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafSuperseded]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_3,
    test_6_2_39_3_language_specific_reasoning_for_supersession
);

#[cfg(test)]
mod tests {
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_3 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::document_notes_with_title_and_category::create_incorrect_category_error;
    use crate::validations::utils::language_specific_translations::create_no_translation_known_info;

    #[test]
    fn test_test_6_2_39_3() {
        let case_01_category_summary = Err(vec![create_incorrect_category_error(0)]);
        // Case 11: correct category description
        let case_s11_esperanto_no_translation = Err(vec![create_no_translation_known_info(
            "Reasoning for Supersession",
            "eo",
        )]);

        TESTS_2_1.test_6_2_39_3.expect(ExpectedResults {
            case_01: case_01_category_summary,
            case_11: Ok(()),
            case_s11: case_s11_esperanto_no_translation,
        });
    }
}
