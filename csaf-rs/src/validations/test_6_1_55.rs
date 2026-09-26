use spdx::lexer::{Lexer, Token};
use std::sync::LazyLock;

use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait};
use crate::helpers::SCANCODE_LICENSEDB;
use crate::schema::csaf2_1::schema::LicenseExpression;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::CSAF_PARSE_MODE;

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

fn has_only_listed_license_identifiers_or_is_invalid(license_expression: &LicenseExpression) -> bool {
    for lexer_result in Lexer::new_mode(license_expression.as_str(), CSAF_PARSE_MODE) {
        let token = match lexer_result {
            Ok(lexer_token) => lexer_token.token,
            // The current `spdx` 0.13.4 lexer does not advance past a lexical error.
            // Continuing after such an error would require custom recovery/lexer logic.
            //
            // Lexical errors are handled by 6.1.54.
            // TODO #409: return a precondition failed here.
            Err(_) => return true,
        };

        // TODO: SPDX 3.0.1 spec requires license and exception identifiers to be matched case-insensitively,
        // while `spdx` 0.13.4 currently performs case-sensitive identifier lookup. Revisit this once case-insensitive parsing is supported.
        let is_listed = match token {
            Token::Unknown(_) => false,
            Token::LicenseRef { lic_ref, .. } => SCANCODE_LICENSEDB
                .get(&lic_ref.to_lowercase())
                .is_some_and(|info| !info.is_exception),
            Token::AdditionRef { add_ref, .. } => SCANCODE_LICENSEDB
                .get(&add_ref.to_lowercase())
                .is_some_and(|info| info.is_exception),

            // Known SPDX licenses/exceptions, operators, parentheses, etc.
            _ => true,
        };
        if !is_listed {
            return false;
        }
        // If the token is listed, continue to the next token
    }
    true
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
            .is_some_and(|license_expression| !has_only_listed_license_identifiers_or_is_invalid(license_expression))
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
        let category_other = Err(vec![create_incorrect_license_text_category_error(
            "/document/notes/0/category",
            &NoteCategory::Other,
        )]);
        let category_general = Err(vec![create_incorrect_license_text_category_error(
            "/document/notes/0/category",
            &NoteCategory::General,
        )]);

        let multiple_license_text_notes_for_unlisted_license_identifier =
            Err(vec![MULTIPLE_LICENSE_TEXT_ERROR.clone()]);
        let unlisted_license_exception_without_license_text = Err(vec![MISSING_LICENSE_TEXT_ERROR.clone()]);

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
