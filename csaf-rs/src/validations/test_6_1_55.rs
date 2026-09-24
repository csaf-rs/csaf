use std::sync::LazyLock;

use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait};
use crate::helpers::{SCANCODE_LICENSEDB_EXCEPTIONS, SCANCODE_LICENSEDB_LICENSES};
use crate::schema::csaf2_1::schema::LicenseExpression;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::parse_csaf_license_expression;

static MISSING_LICENSE_TEXT_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message:
            "Missing license text (document note with title 'License') for unlisted license identifier or exception."
                .to_string(),
        instance_path: "/document/notes".to_string(),
    })
});

static MULTIPLE_LICENSE_TEXT_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message:
            "Multiple license texts (document notes with title 'License') for unlisted license identifier or exception."
                .to_string(),
        instance_path: "/document/notes".to_string(),
    })
});

fn create_incorrect_license_text_category_error(instance_path: &str, category: &NoteCategory) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Invalid category for license text: '{category}' instead of 'legal_disclaimer'."),
        instance_path: instance_path.to_string(),
    })
}

fn license_listed_in_spdx_licensedb_or_invalid_license_expression(license: &LicenseExpression) -> bool {
    match parse_csaf_license_expression(license) {
        Ok(parsed) => parsed.requirements().all(|requirement| {
            let license_is_listed = match &requirement.req.license {
                spdx::LicenseItem::Other(license_ref) => {
                    let license_ref: &str = &license_ref.lic_ref;
                    SCANCODE_LICENSEDB_LICENSES.contains(&license_ref.to_lowercase()) // The variable parts of SPDX LicenseRef and AdditionRef identifiers are case-insensitive
                },
                spdx::LicenseItem::Spdx { .. } => true,
            };

            let addition_is_listed = match &requirement.req.addition {
                None => true,
                Some(spdx::AdditionItem::Other(addition_ref)) => {
                    let addition_ref: &str = &addition_ref.add_ref;
                    SCANCODE_LICENSEDB_EXCEPTIONS.contains(&addition_ref.to_lowercase()) // The variable parts of SPDX LicenseRef and AdditionRef identifiers are case-insensitive
                },
                Some(spdx::AdditionItem::Spdx(_)) => true,
            };

            license_is_listed && addition_is_listed
        }),
        Err(_) => true, // TODO #409 return a precondition failed here
    }
}

fn is_english_or_unspecified(doc: &impl CsafTrait) -> bool {
    match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => false,
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.is_english(),
        None => true, // no language set
    }
}

fn expect_exactly_one_license_text(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if let Some(notes) = doc.get_document().get_notes() {
        let mut errors: Option<Vec<TestFinding>> = None;
        let license_notes = notes
            .iter()
            .enumerate()
            .filter(|(note_index, note)| {
                if note.get_title().is_some_and(|title| title == "License") {
                    if note.get_category() != NoteCategory::LegalDisclaimer {
                        errors
                            .get_or_insert_default()
                            .push(create_incorrect_license_text_category_error(
                                &format!("/document/notes/{note_index}/category"),
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
            errors.get_or_insert_default().push(MULTIPLE_LICENSE_TEXT_ERROR.clone());
        } else if license_notes < 1 {
            errors.get_or_insert_default().push(MISSING_LICENSE_TEXT_ERROR.clone());
        }

        errors.map_or(Ok(()), Err)
    } else {
        Err(vec![MISSING_LICENSE_TEXT_ERROR.clone()])
    }
}

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
            .is_some_and(|license| !license_listed_in_spdx_licensedb_or_invalid_license_expression(license))
    {
        expect_exactly_one_license_text(doc)
    } else {
        Ok(())
    }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_55, test_6_1_55_license_text);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_55 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_55() {
        let case_01_category_other = Err(vec![create_incorrect_license_text_category_error(
            "/document/notes/0/category",
            &NoteCategory::Other,
        )]);
        let case_02_category_general = Err(vec![create_incorrect_license_text_category_error(
            "/document/notes/0/category",
            &NoteCategory::General,
        )]);

        let multiple_license_text_notes_for_unlisted_license_identifier =
            Err(vec![MULTIPLE_LICENSE_TEXT_ERROR.clone()]);
        let unlisted_license_exception_without_license_text = Err(vec![MISSING_LICENSE_TEXT_ERROR.clone()]);

        TESTS_2_1.test_6_1_55.expect(ExpectedResults {
            case_01: case_01_category_other,
            case_02: case_02_category_general,
            case_s01: multiple_license_text_notes_for_unlisted_license_identifier,
            case_s02: unlisted_license_exception_without_license_text,
            // unlisted license identifier with required license text
            case_11: Ok(()),
            // unlisted license identifier with required license text and English set as document language
            case_12: Ok(()),
            // listed license identifier with listed license exception
            case_s11: Ok(()),
            // listed ScanCode license without license text
            case_s12: Ok(()),
        });
    }
}
