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

/// Returns `true` if all license identifiers and exceptions in the expression are listed by SPDX or AboutCode's "ScanCode LicenseDB".
/// Returns `false` as soon as an unlisted license identifier or exception is found.
/// A lexical error returns `true` if it is encountered before any unlisted identifier, until `PreconditionFailed` is supported.
pub(crate) fn has_only_listed_license_identifiers_or_is_invalid(license_expression: &LicenseExpression) -> bool {
    for lexer_result in Lexer::new_mode(license_expression.as_str(), CSAF_PARSE_MODE) {
        let token = match lexer_result {
            Ok(lexer_token) => lexer_token.token,
            // The current `spdx` 0.13.4 lexer does not advance past a lexical error.
            // Continuing after such an error would require custom recovery/lexer logic.
            //
            // TODO #409: Once precondition failures are supported, any lexical error encountered
            // while scanning the entire expression should cause this function to return PreconditionFailed.
            Err(_) => return true,
        };

        // TODO #1170: SPDX 3.0.1 requires license and exception identifiers to be matched case-insensitively,
        // while `spdx` 0.13.4 currently performs case-sensitive identifier lookup. Update `spdx` once case-insensitive lookup is supported and adjust this handling accordingly.
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
