use csaf_macros::profile_test_applies_to_category;
use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, NoteTrait};
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

fn create_missing_note_error(doc_category: &DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have at least one document note with category 'description', 'details', 'general' or 'summary'",
            doc_category
        ),
        instance_path: "/document/notes".to_string(),
    }
}

/// 6.1.27.1 Document Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory`
/// or `csaf_security_incident_response`.
///
/// Documents with these categories must have at least one entry in `/document/notes` with `category` values
/// of `description`, `details`, `general` or `summary`.
#[profile_test_applies_to_category(
    all = [CsafInformationalAdvisory, CsafSecurityIncidentResponse],
)]
pub fn test_6_1_27_01_document_notes(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
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
        return Err(vec![create_missing_note_error(&doc.get_document().get_category())]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_01() {
        let errors = HashMap::from([(
            "01",
            vec![create_missing_note_error(
                DocumentCategory::CsafSecurityIncidentResponse,
            )],
        )]);
        run_csaf20_tests("27-01", test_6_1_27_01_document_notes, errors.clone());
        run_csaf21_tests("27-01", test_6_1_27_01_document_notes, errors);
    }
}
