use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::document_references_with_summary_and_category::check_references_with_summary_prefix_and_category;

/// 6.1.27.19 Reference to superseding document
///
/// This test only applies to documents with `/document/category` with value `csaf_superseded` and only if the document language is English (i.e., `/document/lang` with value `en`) or unspecified.
///
/// It MUST be tested that at least one item in document references exists that has a summary starting with "Superseding Document".
/// The category of this item MUST be external.
pub fn test_6_1_27_19_reference_to_superseding_document(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
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

    check_references_with_summary_prefix_and_category(
        doc.get_document().get_references().map(Vec::as_slice),
        "Superseding Document",
        &CategoryOfReference::External,
        &doc_category,
    )
    .map(|findings| findings.into_iter().map(TestFinding::Error).collect())
    .map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafSuperseded]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_19,
    test_6_1_27_19_reference_to_superseding_document
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_19 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::document_references_with_summary_and_category::{create_incorrect_category_data, create_missing_reference_data};

    #[test]
    fn test_test_6_1_27_19() {
        let undefined_lang_wrong_category = Err(vec![TestFinding::Error(
            create_incorrect_category_data(
                "Superseding Document",
                &CategoryOfReference::Self_,
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
                0,
            ),
        )]);
        let lang_en_missing_category = Err(vec![TestFinding::Error(
            create_missing_reference_data(
                "Superseding Document",
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
            ),
        )]);
        TESTS_2_1.test_6_1_27_19.expect(ExpectedResults {
            case_01: undefined_lang_wrong_category.clone(),
            case_02: Ok(()), // ToDo this test case is currently marked as failing, but the data is valid see https://github.com/oasis-tcs/csaf/issues/1359
            case_s01: undefined_lang_wrong_category,
            case_s02: lang_en_missing_category,
            case_11: Ok(()),  // lang: unspecified, single correct reference
            case_12: Ok(()),  // lang: en-us, multiple correct references
            case_13: Ok(()),  // lang: de-DE is ignored
            case_s11: Ok(()), // lang: en-us, wrong category
        });
    }
}
