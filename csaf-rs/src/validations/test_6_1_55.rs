use spdx::Expression;

use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{CsafTrait, DocumentTrait, NoteTrait};
use crate::helpers::SCANCODE_LICENSEDB_LICENSES;
use crate::schema::csaf2_1::schema::LicenseExpression;
use crate::schema::csaf2_1::schema::NoteCategory;
use crate::validation::ValidationError;

fn create_missing_license_text_error() -> ValidationError {
    ValidationError {
        message: "Missing license text (document note with title 'License') for non-standard license.".to_string(),
        instance_path: "/document/license_expression".to_string(),
    }
}

fn create_multiple_license_text_error() -> ValidationError {
    ValidationError {
        message: "Multiple license texts (document notes with title 'License') for non-standard license.".to_string(),
        instance_path: "/document/license_expression".to_string(),
    }
}

fn create_incorrect_license_text_category_error(
    license_expression_path: &str,
    category: &NoteCategory,
) -> ValidationError {
    ValidationError {
        message: format!("Invalid category for license text: '{category}' instead of 'legal_disclaimer'."),
        instance_path: license_expression_path.to_string(),
    }
}

fn license_listed_in_spdx_licensedb(license: &LicenseExpression) -> bool {
    match Expression::parse(license.as_str()) {
        Ok(parsed) => parsed.requirements().all(|requirement| match &requirement.req.license {
            spdx::LicenseItem::Other(license_ref) => {
                let license_ref: &str = &license_ref.lic_ref;
                SCANCODE_LICENSEDB_LICENSES.get(license_ref).is_some()
            },
            spdx::LicenseItem::Spdx { id: _, or_later: _ } => true,
        }),
        Err(_) => false,
    }
}

fn is_english_or_default(doc: &impl CsafTrait) -> bool {
    match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => false,
        Some(CsafLanguage::Valid(valid_lang)) => valid_lang.is_default() || valid_lang.is_english(),
        None => true,
    }
}

fn expect_exactly_one_license_text(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    if let Some(notes) = doc.get_document().get_notes() {
        let license_notes = notes
            .iter()
            .filter(|note| note.get_title().is_some_and(|title| title == "License"))
            .count();
        if license_notes > 1 {
            return Err(vec![create_multiple_license_text_error()]);
        } else if license_notes < 1 {
            return Err(vec![create_missing_license_text_error()]);
        }

        let errors: Vec<ValidationError> = notes
            .iter()
            .enumerate()
            .filter_map(|(note_index, note)| {
                if note.get_title().is_some_and(|title| title == "License")
                    && note.get_category() != NoteCategory::LegalDisclaimer
                {
                    Some(create_incorrect_license_text_category_error(
                        &format!("/document/notes/{note_index}/category"),
                        &note.get_category(),
                    ))
                } else {
                    None
                }
            })
            .collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    } else {
        Err(vec![create_missing_license_text_error()])
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
) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    if document
        .license_expression
        .as_ref()
        .is_some_and(|license| !license_listed_in_spdx_licensedb(license))
        && is_english_or_default(doc)
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
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_55() {
        // Only CSAF 2.1 has this test with 6 test cases (3 error cases, 3 success case)
        TESTS_2_1.test_6_1_55.expect(
            Err(vec![create_incorrect_license_text_category_error(
                "/document/notes/0/category",
                &NoteCategory::Other,
            )]),
            Err(vec![create_incorrect_license_text_category_error(
                "/document/notes/0/category",
                &NoteCategory::General,
            )]),
            Err(vec![create_multiple_license_text_error()]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
