use crate::csaf_traits::{CsafTrait, CsafDocumentCategory, DocumentTrait, NoteTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

fn create_missing_note_error(doc_category: CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{doc_category}' must have at least one document note with category 'description', 'details', 'general' or 'summary'"
        ),
        instance_path: "/document/notes".to_string(),
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().shared(&[
    CsafDocumentCategory::CsafInformationalAdvisory,
    CsafDocumentCategory::CsafSecurityIncidentResponse,
]);

/// 6.1.27.1 Document Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory`
/// or `csaf_security_incident_response`.
///
/// Documents with these categories must have at least one entry in `/document/notes` with `category` values
/// of `description`, `details`, `general` or `summary`.
pub fn test_6_1_27_01_document_notes(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    // check if there is a document note with the required category
    let mut found_valid_note = false;
    if let Some(notes) = doc.get_document().get_notes() {
        for note in notes.iter() {
            let category = note.get_category();
            if category == NoteCategory::Description
                || category == NoteCategory::Details
                || category == NoteCategory::General
                || category == NoteCategory::Summary
            {
                found_valid_note = true;
                break;
            }
        }
    }

    // if there isn't a note with the required category, return an error
    if !found_valid_note {
        return Err(vec![create_missing_note_error(doc_category)]);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_01_document_notes(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_01_document_notes(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf_traits::CsafDocumentCategory;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_01() {
        let case_01 = Err(vec![create_missing_note_error(
            CsafDocumentCategory::CsafSecurityIncidentResponse,
        )]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_27_1.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_1.expect(case_01);
    }
}
