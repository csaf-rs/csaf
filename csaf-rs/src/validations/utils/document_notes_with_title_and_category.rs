use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::NoteTrait;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::{TestFinding, TestFindingData};

pub(crate) fn create_missing_note_error(
    required_title: &str,
    required_category: &NoteCategory,
    document_category: &CsafDocumentCategory,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The document does not contain a note with title `{required_title}` and category `{required_category}` which is required for documents with category `{document_category}`"
        ),
        instance_path: "/document/notes".to_string(),
    })
}

pub(crate) fn create_duplicated_note_error(
    required_title: &str,
    document_category: &CsafDocumentCategory,
    note_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Duplicate note with title `{required_title}` found while only one is allowed for documents with category `{document_category}`"
        ),
        instance_path: format!("/document/notes/{note_index}"),
    })
}

pub(crate) fn create_incorrect_category_error(
    required_title: &str,
    wrong_category: &NoteCategory,
    required_category: &NoteCategory,
    doc_category: &CsafDocumentCategory,
    note_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The document contains a note with title `{required_title}`, but it uses the wrong note category `{wrong_category}` for documents with category `{doc_category}` (should be `{required_category}`)."
        ),
        instance_path: format!("/document/notes/{note_index}"),
    })
}

/// Checks that exactly one document note exists with the given `required_title` and
/// `required_category` among the provided `notes`, in the context of a document with
/// `doc_category`. `doc_category` is forwarded to error generation functions and not
/// used in the error detection logic.
///
/// The following findings are reported in that order / prioritization:
/// - If no note with `required_title` is found: a single missing-note error.
/// - If more than one note with `required_title` is found: one duplicate error per matching note.
/// - If exactly one note with `required_title` is found but its category differs from
///   `required_category`: a wrong-category error.
///
/// Returns `Ok(())` if the check passes, or `Err` with a list of [`TestFinding`]s otherwise.
pub(crate) fn check_notes_with_title_and_category<Note: NoteTrait>(
    notes: Option<&[Note]>,
    required_title: &str,
    required_category: &NoteCategory,
    doc_category: &CsafDocumentCategory,
) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;
    let mut matching_indices = Vec::new();

    // filter notes for required title and category
    // collect correct title, wrong category errors
    if let Some(notes) = notes {
        for (i_n, note) in notes.iter().enumerate() {
            if let Some(title) = note.get_title()
                && title == required_title
            {
                if note.get_category() != *required_category {
                    errors.get_or_insert_default().push(create_incorrect_category_error(
                        required_title,
                        &note.get_category(),
                        required_category,
                        doc_category,
                        i_n,
                    ));
                }
                matching_indices.push(i_n);
            }
        }
    }

    // The fact that there is none or more than one note with the required title is the primary
    // error and we ignore the category check, which is only relevant if there is exactly one
    // occurrence.
    if matching_indices.is_empty() {
        return Err(vec![create_missing_note_error(
            required_title,
            required_category,
            doc_category,
        )]);
    } else if matching_indices.len() > 1 {
        return Err(matching_indices
            .iter()
            .map(|f| create_duplicated_note_error(required_title, doc_category, *f))
            .collect::<Vec<_>>());
    }
    errors.map_or(Ok(()), Err)
}
