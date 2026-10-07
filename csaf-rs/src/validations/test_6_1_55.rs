use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::TestFinding;
use crate::validations::utils::license_expressions::has_only_listed_license_identifiers_or_is_invalid;
use crate::validations::utils::license_text::check_for_exactly_one_license_text_note;

/// 6.1.55 License Text
///
/// If the document language is English or unspecified, and the license_expression contains license identifiers
/// or exceptions that are not listed in the SPDX license list or AboutCode's "ScanCode LicenseDB", it MUST be
/// tested that exactly one item in document notes exists that has the title License. The category of this item
/// MUST be legal_disclaimer.
pub fn test_6_1_55_license_text(
    doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    if is_english_or_unspecified(doc)
        && document
            .license_expression
            .as_ref()
            .is_some_and(|license_expression| !has_only_listed_license_identifiers_or_is_invalid(license_expression))
    {
        check_for_exactly_one_license_text_note(document.get_notes().map(Vec::as_slice), LICENSE_TITLE)
            .map(|findings| findings.into_iter().map(TestFinding::Error).collect())
            .map_or(Ok(()), Err)
    } else {
        Ok(())
    }
}

fn is_english_or_unspecified(doc: &impl CsafTrait) -> bool {
    match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => false,
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.is_english(),
        None => true, // no language set
    }
}
const LICENSE_TITLE: &str = "License";

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_55, test_6_1_55_license_text);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_55 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::NoteCategory;
    use crate::validations::utils::license_text::{
        create_incorrect_license_text_category_finding_data, create_missing_license_text_finding_data,
        create_multiple_license_text_finding_data,
    };

    #[test]
    fn test_test_6_1_55() {
        let category_other = Err(vec![TestFinding::Error(
            create_incorrect_license_text_category_finding_data(LICENSE_TITLE, 0, &NoteCategory::Other),
        )]);

        let category_general = Err(vec![TestFinding::Error(
            create_incorrect_license_text_category_finding_data(LICENSE_TITLE, 0, &NoteCategory::General),
        )]);

        let multiple_license_text_notes_for_unlisted_license_identifier = Err(vec![TestFinding::Error(
            create_multiple_license_text_finding_data(LICENSE_TITLE),
        )]);

        let unlisted_license_exception_without_license_text = Err(vec![TestFinding::Error(
            create_missing_license_text_finding_data(LICENSE_TITLE),
        )]);

        // Case 11: unlisted license identifier with required license text
        // Case 12: unlisted license identifier with required license text and English set as document language
        // Case S11: listed license identifier with listed license exception
        // Case S12: listed ScanCode license without license text
        // Case S13: listed ScanCode custom addition without license text
        TESTS_2_1.test_6_1_55.expect(ExpectedResults {
            case_01: category_other,
            case_02: category_general,
            case_s01: multiple_license_text_notes_for_unlisted_license_identifier,
            case_s02: unlisted_license_exception_without_license_text,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
    }
}
