use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::csaf_language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

fn create_missing_reasoning_error(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "The document does not contain a note with title `Reasoning for Withdrawal` and category `description`  which is required for documents with category {document_category}"
        ),
        instance_path: "/document/notes".to_string(),
    }
}

fn create_duplicated_reasoning_error(document_category: &CsafDocumentCategory, note_index: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Duplicate note with title `Reasoning for Withdrawal` found while only one is allowed for documents with category {document_category}"
        ),
        instance_path: format!("/document/notes[{note_index}]").to_string(),
    }
}

fn create_incorrect_category_error(note_index: usize) -> ValidationError {
    ValidationError {
        message: "The note has the correct title. However it uses the wrong category.".to_string(),
        instance_path: format!("/document/notes[{note_index}]").to_string(),
    }
}

/// 6.1.27.17 Reasoning for withdrawal
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` and only if the document language is English (i.e., `/document/lang` with value `en`) or unspecified.
///
/// If the document language is English or unspecified, it MUST be tested that exactly one item in document notes exists that has the title Reasoning for Withdrawal.
/// The category of this item MUST be description.
pub fn test_6_1_27_14_document_notes_with_description(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
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

    let mut errors = Vec::new();
    let mut withdrawals = Vec::new();

    if let Some(notes) = doc.get_document().get_notes() {
        for (i_n, note) in notes.iter().enumerate() {
            if let Some(title) = note.get_title()
                && title == "Reasoning for Withdrawal"
            {
                if note.get_category() != NoteCategory::Description {
                    errors.push(create_incorrect_category_error(i_n));
                }
                withdrawals.push(i_n);
            }
        }
    }

    if withdrawals.is_empty() {
        return Err(vec![create_missing_reasoning_error(&doc_category)]);
    } else if withdrawals.len() > 1 {
        return Err(withdrawals
            .iter()
            .map(|f| create_duplicated_reasoning_error(&doc_category, *f))
            .collect::<Vec<_>>());
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafWithdrawn]);

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_17
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_14_document_notes_with_description(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_17() {
        let undefined_lang_wrong_category = Err(vec![create_incorrect_category_error(0)]);
        let undefined_lang_duplicate_title = Err(vec![
            create_duplicated_reasoning_error(&CsafDocumentCategory::CsafWithdrawn, 0),
            create_duplicated_reasoning_error(&CsafDocumentCategory::CsafWithdrawn, 1),
        ]);
        let lang_en_us_wrong_category = Err(vec![create_incorrect_category_error(0)]);
        let undefined_lang_missing_reasoning = Err(vec![create_missing_reasoning_error(
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        TESTS_2_1.test_6_1_27_17.expect(
            undefined_lang_wrong_category.clone(),
            undefined_lang_duplicate_title.clone(),
            undefined_lang_wrong_category,
            undefined_lang_duplicate_title,
            lang_en_us_wrong_category,
            undefined_lang_missing_reasoning.clone(),
            undefined_lang_missing_reasoning,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
