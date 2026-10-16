use spdx::lexer::{Lexer, Token};

use crate::csaf_traits::CsafTrait;
use crate::helpers::SCANCODE_LICENSEDB;
use crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::CSAF_PARSE_MODE;

/// 6.2.45 Non-Existing License Identifier
///
/// It SHALL be tested that all license identifiers and exceptions used exist.
/// This SHALL be tested for the SPDX license list and AboutCode's "ScanCode LicenseDB".
/// The test MAY be skipped for other license inventoring entities.
pub fn test_6_2_45_non_existing_license_identifier(
    doc: &CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let Some(license_expression) = doc.get_document().license_expression.as_ref() else {
        // Missing license expressions are handled by 6.2.43.
        // TODO #409: Return `WasSkipped` once supported.
        return Ok(());
    };

    let mut warnings = Vec::new();

    for lexer_result in Lexer::new_mode(license_expression.as_str(), CSAF_PARSE_MODE) {
        let lexer_token = match lexer_result {
            Ok(lexer_token) => lexer_token,
            // The current `spdx` 0.13.4 lexer does not advance past a lexical error.
            // Continuing after such an error would require custom recovery/lexer logic.
            //
            // TODO #409: Represent the lexical error as `PreconditionFailed` once supported.
            Err(_) => {
                return if warnings.is_empty() { Ok(()) } else { Err(warnings) };
            },
        };

        // TODO #1170: SPDX 3.0.1 requires license and exception identifiers to be matched
        //  case-insensitively, while `spdx` 0.13.4 currently performs case-sensitive identifier lookup.
        //  Update `spdx` once case-insensitive lookup is supported and adjust this handling accordingly.
        let is_existing_or_skipped = match lexer_token.token {
            // An unknown bare identifier is not present in the SPDX license list
            Token::Unknown(_) => false,

            Token::LicenseRef { lic_ref, .. } => {
                let lic_ref = lic_ref.to_lowercase();

                match SCANCODE_LICENSEDB.get(&lic_ref) {
                    // The identifier exists in ScanCode and represents a license
                    Some(info) => !info.is_exception,

                    // If the identifier explicitly belongs to ScanCode via the "scancode-" prefix
                    // but is not found, it does not exist. Other inventoring entities may be skipped.
                    None => !lic_ref.starts_with("scancode-"),
                }
            },

            Token::AdditionRef { add_ref, .. } => {
                let add_ref = add_ref.to_lowercase();

                match SCANCODE_LICENSEDB.get(&add_ref) {
                    // The identifier exists in ScanCode and represents an exception
                    Some(info) => info.is_exception,

                    // If the identifier explicitly belongs to ScanCode via the "scancode-" prefix
                    // but is not found, it does not exist. Other inventoring entities may be skipped.
                    None => !add_ref.starts_with("scancode-"),
                }
            },

            // Known SPDX licenses/exceptions, operators, parentheses, etc.
            _ => true,
        };

        if !is_existing_or_skipped {
            // Preserve the identifier exactly as written in the license expression for the warning text
            let identifier = &license_expression.as_str()[lexer_token.span];
            warnings.push(create_non_existing_license_identifier_or_exception_warning(identifier));
        }
    }

    if warnings.is_empty() { Ok(()) } else { Err(warnings) }
}

fn create_non_existing_license_identifier_or_exception_warning(identifier: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("The license identifier or exception '{identifier}' does not exist."),
        instance_path: "/document/license_expression".to_string(),
    })
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_45,
    test_6_2_45_non_existing_license_identifier
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_45 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_45() {
        let non_existing_spdx_license_identifier =
            Err(vec![create_non_existing_license_identifier_or_exception_warning(
                "A-License-Identifier-That-Does-Not-Exist",
            )]);

        let non_existing_scancode_license_identifier =
            Err(vec![create_non_existing_license_identifier_or_exception_warning(
                "LicenseRef-scancode-A-License-Identifier-That-Does-Not-Exist",
            )]);

        // Case 11: Existing SPDX license identifiers.
        // Case 12: Existing ScanCode license identifier.
        TESTS_2_1.test_6_2_45.expect(ExpectedResults {
            case_01: non_existing_spdx_license_identifier,
            case_02: non_existing_scancode_license_identifier,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
