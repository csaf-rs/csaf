use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::document_references_with_summary_and_category::check_references_with_summary_prefix_and_category;
use crate::validations::utils::language_specific_translations::{
    create_no_translation_known_info, get_translation_for_term_superseding_document,
};

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
pub fn test_6_2_39_4_language_specific_superseding_document(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let primary_lang = match doc.get_document().get_lang() {
        None => return Ok(()), // language unspecified, test 6.1.27.19 covers this
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // wasSkipped in #407
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_english() => return Ok(()), // english is covered by 6.1.27.19
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.primary_language().to_string(),
    };

    // get language-specific translation
    let Some(translated_summary_prefix) = get_translation_for_term_superseding_document(&primary_lang) else {
        return Err(vec![create_no_translation_known_info(
            "Superseding Document",
            &primary_lang,
            "/document/references",
        )]);
    };

    check_references_with_summary_prefix_and_category(
        doc.get_document().get_references().map(Vec::as_slice),
        translated_summary_prefix,
        &CategoryOfReference::External,
        &doc_category,
    )
    .map(|v| v.into_iter().map(TestFinding::Warning).collect())
    .map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafSuperseded]);

crate::test_validation::impl_validator!(
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
        let case_s11_esperanto_no_translation =
            Err(vec![create_no_translation_known_info("Superseding Document", "eo", "/document/references")]);

        TESTS_2_1.test_6_2_39_4.expect(ExpectedResults {
            case_01: no_reference_with_prefix,
            case_s01: incorrect_category,
            case_s02: multiple_incorrect_category,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: case_s11_esperanto_no_translation,
        });
    }
}
