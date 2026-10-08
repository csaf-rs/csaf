use spdx::Expression;
use spdx::lexer::{Lexer, Token};

use crate::helpers::SCANCODE_LICENSEDB;
use crate::schema::csaf2_1::schema::LicenseExpression;

pub(crate) const CSAF_PARSE_MODE: spdx::ParseMode = spdx::ParseMode {
    allow_slash_as_or_operator: false,
    allow_imprecise_license_names: false,
    allow_postfix_plus_on_gpl: true,
    allow_deprecated: true,
    allow_unknown: true,
};

/// Parses the given license expression using the SPDX parser with specific options that align with the requirements of CSAF.
/// For example, unknown SPDX identifiers should not fail test 6.1.54, whereas expressions with DocumentRef are not allowed.
pub(crate) fn parse_csaf_license_expression(license: &LicenseExpression) -> Result<Expression, spdx::ParseError> {
    let expression = Expression::parse_mode(license.as_str(), CSAF_PARSE_MODE)?;
    expression
        .requirements()
        .filter_map(|requirement| {
            if let spdx::LicenseItem::Other(license_ref) = &requirement.req.license
                && license_ref.doc_ref.is_some()
            {
                Some(spdx::ParseError {
                    original: license.to_string(),
                    span: requirement.span.start as usize..requirement.span.end as usize,
                    reason: spdx::error::Reason::Unexpected(&["LicenseRef"]),
                })
            } else if let Some(spdx::AdditionItem::Other(addition_ref)) = &requirement.req.addition
                && addition_ref.doc_ref.is_some()
            {
                Some(spdx::ParseError {
                    original: license.to_string(),
                    span: requirement.span.start as usize..requirement.span.end as usize,
                    reason: spdx::error::Reason::Unexpected(&["AdditionRef"]),
                })
            } else {
                None
            }
        })
        .next()
        .map_or(Ok(()), Err)?;
    Ok(expression)
}

/// Returns `Ok(false)` if all license identifiers and exceptions in the expression are listed by SPDX or AboutCode's "ScanCode LicenseDB".
/// Returns `Ok(true)` as soon as an unlisted license identifier or exception is found.
/// Returns an error if a lexical error is encountered before any unlisted license identifier or exception.
pub(crate) fn try_contains_unlisted_license_identifier_or_exception(
    license_expression: &LicenseExpression,
) -> Result<bool, spdx::ParseError> {
    for lexer_result in Lexer::new_mode(license_expression.as_str(), CSAF_PARSE_MODE) {
        // The current `spdx` 0.13.4 lexer does not advance past a lexical error.
        // Continuing after such an error would require custom recovery/lexer logic.
        let token = lexer_result?.token;

        // TODO #1170: SPDX 3.0.1 requires license and exception identifiers to be matched case-insensitively,
        // while `spdx` 0.13.4 currently performs case-sensitive identifier lookup.
        // Update `spdx` once case-insensitive lookup is supported and adjust this handling accordingly.
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
            return Ok(true);
        }
        // If the token is listed, continue to the next token
    }
    Ok(false)
}
