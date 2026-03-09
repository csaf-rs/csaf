use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

fn create_missing_description_note(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "The document does not contain a note with category `description` which is required for documents with category {document_category}"
        ),
        instance_path: "/document/notes".to_string(),
    }
}

/// 6.1.27.14 Document Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` or `csaf_superseeded`.
///
/// Each item in /document/notes[] must have at least one item with category `description`.
pub fn test_6_1_27_14_document_notes_with_description(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    match doc.get_document().get_notes() {
        None => Err(vec![create_missing_description_note(&doc_category)]),
        Some(notes) => {
            if notes
                .iter()
                .all(|note| note.get_category() != NoteCategory::Description)
            {
                Err(vec![create_missing_description_note(&doc_category)])
            } else {
                Ok(())
            }
        },
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().csaf21(&[
    CsafDocumentCategory::CsafWithdrawn,
    CsafDocumentCategory::CsafSuperseded,
]);

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_14
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
    fn test_test_6_1_27_14() {
        let fail_withdrawn = Err(vec![create_missing_description_note(
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        let fail_superseded = Err(vec![create_missing_description_note(
            &CsafDocumentCategory::CsafSuperseded,
        )]);

        TESTS_2_1
            .test_6_1_27_14
            .expect(fail_withdrawn, fail_superseded, Ok(()), Ok(()));
    }
}
