use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::TestFinding;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use crate::validations::utils::language_specific_translations::{
    create_no_translation_known_info, get_translation_for_term_cve_description,
    get_translation_for_term_vulnerability_summary,
};
use crate::validations::utils::vulnerability_notes_with_title_and_category::check_vulnerability_notes_with_title_and_category;

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

    // Only proceed if the document language is specified and is not English
    let primary_lang = match doc.get_document().get_lang() {
        None => return Ok(()),
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()),
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_english() => return Ok(()),
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.primary_language().to_string(),
    };

    let translated_vulnerability_summary = get_translation_for_term_vulnerability_summary(&primary_lang);
    let translated_cve_description = get_translation_for_term_cve_description(&primary_lang);

    let (translated_vulnerability_summary, translated_cve_description) =
        match (translated_vulnerability_summary, translated_cve_description) {
            (Some(vulnerability_summary), Some(cve_description)) => (vulnerability_summary, cve_description),
            (None, None) => {
                return Err(vec![create_no_translation_known_info(
                    "either Vulnerability Summary or CVE Description",
                    &primary_lang,
                    "/vulnerabilities",
                )]);
            },
            // As discussed, if translations are available for a language,
            // all required translated definitions are expected to be present.
            (Some(_), None) => {
                return Err(vec![create_no_translation_known_info(
                    "CVE Description",
                    &primary_lang,
                    "/vulnerabilities",
                )]);
            },
            (None, Some(_)) => {
                return Err(vec![create_no_translation_known_info(
                    "Vulnerability Summary",
                    &primary_lang,
                    "/vulnerabilities",
                )]);
            },
        };

    let mut warnings = Vec::new();

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(note_warnings) = check_vulnerability_notes_with_title_and_category(
            vulnerability.get_notes().map(Vec::as_slice),
            translated_vulnerability_summary,
            translated_cve_description,
            vulnerability_index,
        ) {
            warnings.extend(note_warnings.into_iter().map(TestFinding::Warning));
        }
    }

    if warnings.is_empty() { Ok(()) } else { Err(warnings) }
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
    use crate::schema::csaf2_1::schema::NoteCategory;
    use crate::validations::utils::vulnerability_notes_with_title_and_category::create_incorrect_vulnerability_note_category_data;

    #[test]
    fn test_test_6_2_39_6() {
        let de_title = get_translation_for_term_cve_description("de").unwrap();

        let case_01 = Err(vec![TestFinding::Warning(
            create_incorrect_vulnerability_note_category_data(
                de_title,
                &NoteCategory::Summary,
                &NoteCategory::Description,
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
