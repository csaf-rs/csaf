use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::vulnerability_notes_with_title_and_category::check_vulnerability_notes_with_title_and_category;

/// 6.1.27.20 Vulnerability Notes
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report` and only if the document language is English or unspecified.
///
/// It SHALL be tested that at least one item in each vulnerability's notes has either
/// `Vulnerability Summary` as title and `summary` as category,
/// or `CVE Description` as title and `description` as category.
pub fn test_6_1_27_20_vulnerability_notes(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    // Only proceed if the document category is "csaf_vulnerability_report"
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    // Only proceed if the document language is English or unspecified
    match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_default() || !valid_lang.is_english() => return Ok(()), // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
        Some(_) => {}, // this is english
        None => {},    // no language set
    }

    let mut errors = Vec::new();

    // Check each vulnerability for a required note title/category combination and collect all errors
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(note_errors) = check_vulnerability_notes_with_title_and_category(
            vulnerability.get_notes().map(Vec::as_slice),
            VULNERABILITY_SUMMARY_TITLE,
            CVE_DESCRIPTION_TITLE,
            vulnerability_index,
        ) {
            errors.extend(note_errors.into_iter().map(TestFinding::Error));
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);
const VULNERABILITY_SUMMARY_TITLE: &str = "Vulnerability Summary";
const CVE_DESCRIPTION_TITLE: &str = "CVE Description";

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_27_20, test_6_1_27_20_vulnerability_notes);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_20 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::NoteCategory;
    use crate::validations::utils::vulnerability_notes_with_title_and_category::{
        create_incorrect_vulnerability_note_category_data, create_missing_vulnerability_note_data,
    };

    #[test]
    fn test_test_6_1_27_20() {
        let case_01 = Err(vec![TestFinding::Error(
            create_incorrect_vulnerability_note_category_data(
                VULNERABILITY_SUMMARY_TITLE,
                &NoteCategory::Description,
                &NoteCategory::Summary,
                0,
                0,
            ),
        )]);

        let case_02 = Err(vec![
            TestFinding::Error(create_missing_vulnerability_note_data(
                VULNERABILITY_SUMMARY_TITLE,
                CVE_DESCRIPTION_TITLE,
                0,
            )),
            TestFinding::Error(create_incorrect_vulnerability_note_category_data(
                VULNERABILITY_SUMMARY_TITLE,
                &NoteCategory::General,
                &NoteCategory::Summary,
                1,
                0,
            )),
            TestFinding::Error(create_missing_vulnerability_note_data(
                VULNERABILITY_SUMMARY_TITLE,
                CVE_DESCRIPTION_TITLE,
                2,
            )),
        ]);

        TESTS_2_1.test_6_1_27_20.expect(ExpectedResults {
            case_01,
            case_02,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
