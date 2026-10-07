use crate::csaf_traits::NoteTrait;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::TestFindingData;

/// Checks that exactly one document note exists with `required_title` and that
/// its category is `legal_disclaimer`.
pub(crate) fn check_for_exactly_one_license_text_note<Note: NoteTrait>(
    notes: Option<&[Note]>,
    required_title: &str,
) -> Option<Vec<TestFindingData>> {
    let Some(notes) = notes else {
        return Some(vec![create_missing_license_text_finding_data(required_title)]);
    };

    let mut findings = Vec::new();

    let license_notes = notes
        .iter()
        .enumerate()
        .filter(|(note_index, note)| {
            if note.get_title().is_some_and(|title| title == required_title) {
                if note.get_category() != NoteCategory::LegalDisclaimer {
                    findings.push(create_incorrect_license_text_category_finding_data(
                        required_title,
                        *note_index,
                        &note.get_category(),
                    ));
                }
                true
            } else {
                false
            }
        })
        .count();

    if license_notes > 1 {
        findings.push(create_multiple_license_text_finding_data(required_title));
    } else if license_notes < 1 {
        findings.push(create_missing_license_text_finding_data(required_title));
    }

    if findings.is_empty() { None } else { Some(findings) }
}

pub(crate) fn create_missing_license_text_finding_data(required_title: &str) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The document does not contain a license text note with the title `{required_title}` \
             required for an unlisted license identifier or exception."
        ),
        instance_path: "/document/notes".to_string(),
    }
}

pub(crate) fn create_incorrect_license_text_category_finding_data(
    required_title: &str,
    note_index: usize,
    category: &NoteCategory,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The document contains a license text note with the title `{required_title}`, \
             but it uses the wrong note category `{category}` (should be `legal_disclaimer`)."
        ),
        instance_path: format!("/document/notes/{note_index}/category"),
    }
}

pub(crate) fn create_multiple_license_text_finding_data(required_title: &str) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The document contains multiple license text notes with the title `{required_title}` \
             for an unlisted license identifier or exception while only one is allowed."
        ),
        instance_path: "/document/notes".to_string(),
    }
}
