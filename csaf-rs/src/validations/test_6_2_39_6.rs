use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::language_specific_translations::{
    create_no_translation_known_info, get_translation_for_term_cve_description,
    get_translation_for_term_vulnerability_summary,
};

/// 6.2.39.6 Language Specific Vulnerability Notes
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report` and only if the document language is specified but not English.
///
/// It SHALL be tested that at least one item in each vulnerability's notes has either
/// the language-specific translation of `Vulnerability Summary` as title and `summary` as category,
/// or the language-specific translation of `CVE Description` as title and `description` as category.
pub fn test_6_2_39_6_language_specific_vulnerability_notes(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    // Only proceed if the document category is "csaf_vulnerability_report"
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    // Only proceed if the document language is specified and not English
    let primary_lang = match doc.get_document().get_lang() {
        None => return Ok(()),
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()),
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_english() => return Ok(()),
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.primary_language().to_string(),
    };

    let translated_vulnerability_summary = get_translation_for_term_vulnerability_summary(&primary_lang);
    let translated_cve_description = get_translation_for_term_cve_description(&primary_lang);

    // If neither translation is known, return an information finding
    if translated_vulnerability_summary.is_none() && translated_cve_description.is_none() {
        return Err(vec![create_no_translation_known_info(
            "either Vulnerability Summary or CVE Description",
            &primary_lang,
            "/vulnerabilities",
        )]);
    }

    let mut warnings = Vec::new();

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(note_warnings) = check_vulnerability_notes(
            vulnerability.get_notes().map(Vec::as_slice),
            translated_vulnerability_summary,
            translated_cve_description,
            &doc_category,
            vulnerability_index,
        ) {
            warnings.extend(note_warnings.into_iter().map(TestFinding::Warning));
        }
    }

    if warnings.is_empty() { Ok(()) } else { Err(warnings) }
}

fn check_vulnerability_notes<Note: NoteTrait>(
    notes: Option<&[Note]>,
    translated_vulnerability_summary: Option<&str>,
    translated_cve_description: Option<&str>,
    document_category: &CsafDocumentCategory,
    vulnerability_index: usize,
) -> Option<Vec<TestFindingData>> {
    let mut incorrect_category_warnings = Vec::new();

    if let Some(notes) = notes {
        for (note_index, note) in notes.iter().enumerate() {
            // Only proceed if the note has a title
            let Some(title) = note.get_title() else {
                continue;
            };

            // Only proceed if the note's title matches either the translated Vulnerability Summary or CVE Description
            let required_category = if translated_vulnerability_summary == Some(title) {
                NoteCategory::Summary
            } else if translated_cve_description == Some(title) {
                NoteCategory::Description
            } else {
                continue;
            };

            let category = note.get_category();

            // Only proceed if the note's category does not match the required category.
            // If it matches, return None, indicating that the valid note was found and no warnings are needed.
            if category == required_category {
                return None;
            }

            // If the note's category does not match the required category, create a warning
            incorrect_category_warnings.push(create_incorrect_vulnerability_note_category_data(
                title,
                &category,
                &required_category,
                document_category,
                vulnerability_index,
                note_index,
            ));
        }
    }

    // If there are any incorrect category warnings, return them
    if !incorrect_category_warnings.is_empty() {
        return Some(incorrect_category_warnings);
    }

    // If no notes with the required title were found, return a warning indicating that the vulnerability is missing the required note
    Some(vec![create_missing_vulnerability_note_data(vulnerability_index)])
}

fn create_missing_vulnerability_note_data(vulnerability_index: usize) -> TestFindingData {
    TestFindingData {
        message: "The vulnerability does not contain a language-specific `Vulnerability Summary` \
                  note with category `summary` or `CVE Description` note with category `description`, \
                  as required for documents with category `csaf_vulnerability_report` \
                  whose language is specified and not English."
            .to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/notes"),
    }
}

fn create_incorrect_vulnerability_note_category_data(
    required_title: &str,
    wrong_category: &NoteCategory,
    required_category: &NoteCategory,
    document_category: &CsafDocumentCategory,
    vulnerability_index: usize,
    note_index: usize,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The vulnerability contains a note with title `{required_title}`, but it uses the \
             wrong note category `{wrong_category}` for documents with category \
             `{document_category}` (should be `{required_category}`)."
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/notes/{note_index}"),
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_6,
    test_6_2_39_6_language_specific_vulnerability_notes
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_6 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_6() {
        let de_title = get_translation_for_term_cve_description("de").unwrap();

        let case_01 = Err(vec![TestFinding::Warning(
            create_incorrect_vulnerability_note_category_data(
                de_title,
                &NoteCategory::Summary,
                &NoteCategory::Description,
                &CsafDocumentCategory::CsafVulnerabilityReport,
                0,
                0,
            ),
        )]);

        TESTS_2_1.test_6_2_39_6.expect(ExpectedResults {
            case_01,
            case_11: Ok(()),
        });
    }
}
