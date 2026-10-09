use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework;
use crate::validation::TestFinding;
use crate::validations::utils::language_specific_translations::{
    create_no_translation_known_info, get_translation_for_term_license,
};
use crate::validations::utils::license_expressions::try_contains_unlisted_license_identifier_or_exception;
use crate::validations::utils::license_text::check_for_exactly_one_license_text_note;

/// 6.2.46 Language Specific License Text
///
/// If the document language is specified but not English, and the `license_expression`
/// contains license identifiers or exceptions that are not listed in the SPDX license
/// list or AboutCode's "ScanCode LicenseDB", it SHALL be tested that exactly one item
/// in document notes exists that has the language specific translation of the term
/// `License` as title. The category of this item SHALL be `legal_disclaimer`.
///
/// If no language specific translation has been recorded, the test SHALL be skipped
/// and output an information to the user that no such translation is known.
pub fn test_6_2_46_language_specific_license_text(
    doc: &CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    // Only proceed if the document language is specified and is not English
    let primary_lang = match document.get_lang() {
        None => return Ok(()), // language unspecified, test 6.1.55 covers this
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // wasSkipped in #407
        Some(CsafLanguage::Valid(valid_lang)) if valid_lang.is_english() => return Ok(()), // english is covered by 6.1.55
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.primary_language().to_string(),
    };

    if document.license_expression.as_ref().is_some_and(|license_expression| {
        match try_contains_unlisted_license_identifier_or_exception(license_expression) {
            Ok(contains_unlisted) => contains_unlisted,
            Err(_) => false, // TODO #409: Return `PreconditionFailed` once supported
        }
    }) {
        let Some(translated_title) = get_translation_for_term_license(&primary_lang) else {
            return Err(vec![create_no_translation_known_info(
                "License",
                &primary_lang,
                "/document/notes",
            )]);
        };

        check_for_exactly_one_license_text_note(document.get_notes().map(Vec::as_slice), translated_title)
            .map(|findings| findings.into_iter().map(TestFinding::Warning).collect())
            .map_or(Ok(()), Err)
    } else {
        Ok(())
    }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_46,
    test_6_2_46_language_specific_license_text
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_46 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::NoteCategory;
    use crate::validations::utils::license_text::create_incorrect_license_text_category_finding_data;

    #[test]
    fn test_test_6_2_46() {
        let de_title = get_translation_for_term_license("de").unwrap();

        let incorrect_category = Err(vec![TestFinding::Warning(
            create_incorrect_license_text_category_finding_data(de_title, 0, &NoteCategory::Summary),
        )]);

        // Case 11: German translation of "License" with category legal_disclaimer.
        TESTS_2_1.test_6_2_46.expect(ExpectedResults {
            case_01: incorrect_category,
            case_11: Ok(()),
        });
    }
}
