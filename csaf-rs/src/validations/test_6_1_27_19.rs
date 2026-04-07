use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::csaf_language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentReferenceTrait, DocumentTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::ValidationError;

fn create_missing_reference_error(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category `{document_category}' must have at least one reference whose summary starts with `Superseding Document` and has the category `external`"
        ),
        instance_path: "/document/references".to_string(),
    }
}

fn create_incorrect_category_error(reference_index: usize) -> ValidationError {
    ValidationError {
        message: "The reference summary starts with the correct string \"Superseding Document\". However it uses the wrong category.".to_string(),
        instance_path: format!("/document/references[]/{reference_index}").to_string(),
    }
}

/// 6.1.27.19 Reference to superseding document
///
/// This test only applies to documents with `/document/category` with value `csaf_superseded` and only if the document language is English (i.e., `/document/lang` with value `en`) or unspecified.
///
/// It MUST be tested that at least one item in document references exists that has a summary starting with "Superseding Document".
/// The category of this item MUST be external.
pub fn test_6_1_27_19_reference_to_superseding_document(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }
    match doc.get_document().get_lang() {
        Some(CsafLanguage::DefaultLanguage(_)) | Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(CsafLanguage::Valid(valid_lang)) if !valid_lang.is_english() => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(_) => {},                                                                      // this is english
        None => {},                                                                         // no language set
    }

    let mut has_external_reference_with_correct_summary = false;
    let mut errors: Option<Vec<ValidationError>> = None;
    // Check for summary starting with "Superseding Document" and category external
    if let Some(references) = doc.get_document().get_references() {
        for (r_i, reference) in references.iter().enumerate() {
            if reference.get_summary().starts_with("Superseding Document") {
                if *reference.get_category() != CategoryOfReference::External {
                    errors
                        .get_or_insert_default()
                        .push(create_incorrect_category_error(r_i));
                } else {
                    has_external_reference_with_correct_summary = true;
                }
            }
        }
    }

    // We first check for an incorrect category, because if there is a reference with the correct summary but wrong category,
    // the document is not valid, even if there is also a reference with correct summary and correct category.
    // So the incorrect category has precedence over the missing reference. This way the error message is more specific and hints more
    // directly to the wrong instance.
    if errors.clone().is_some_and(|e| !e.is_empty()) {
        return Err(errors.unwrap());
    }

    // completely missing reference with correct summary and category has second precedence
    if !has_external_reference_with_correct_summary {
        return Err(vec![create_missing_reference_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
    }
    Ok(())
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
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_19() {
        let undefined_lang_wrong_category = Err(vec![create_incorrect_category_error(0)]);
        let lang_en_missing_category = Err(vec![create_missing_reference_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        TESTS_2_1.test_6_1_27_19.expect(
            undefined_lang_wrong_category.clone(),
            Ok(()), // ToDo this test case is currently marked as failing, but the data is valid see https://github.com/oasis-tcs/csaf/issues/1359
            undefined_lang_wrong_category,
            lang_en_missing_category,
            Ok(()), // lang: unspecified, single correct reference
            Ok(()), // lang: en-us, multiple correct references
            Ok(()), // lang: de-DE is ignored
            Ok(()), // lang: en-us, wrong category
        );
    }
}
