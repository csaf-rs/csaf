use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, NoteTrait};
use crate::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

/// 6.1.27.1 Document Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory`
/// or `csaf_security_incident_response`.
///
/// Documents with these categories must have at least one entry in `/document/notes` with `category` values
/// of `description`, `details`, `general` or `summary`.
pub fn test_6_1_27_1_document_notes(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    // check document category
    if doc_category != DocumentCategory::CsafInformationalAdvisory
        && doc_category != DocumentCategory::CsafSecurityIncidentResponse
    {
        return Ok(());
    }

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

    if !found_valid_note {
        return Err(vec![ValidationError {
            message: format!(
                "Document with category '{}' must have at least one document note with category 'description', 'details', 'general' or 'summary'",
                doc_category
            ),
            instance_path: "/document/notes".to_string(),
        }]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_27_1::test_6_1_27_1_document_notes;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_1() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "Document with category 'csaf_security_incident_response' must have at least one document note with category 'description', 'details', 'general' or 'summary'".to_string(),
                instance_path: "/document/notes".to_string(),
            }],
        )]);
        run_csaf20_tests("27-01", test_6_1_27_1_document_notes, errors.clone());
        run_csaf21_tests("27-01", test_6_1_27_1_document_notes, errors);
    }
}
