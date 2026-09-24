use spdx::Expression;

use crate::schema::csaf2_1::schema::LicenseExpression;

/// Parses a license expression according to the restrictions defined by CSAF.
///
/// Unknown and deprecated SPDX identifiers are allowed, while DocumentRef
/// references are not allowed.
pub(crate) fn parse_csaf_license_expression(license: &LicenseExpression) -> Result<Expression, spdx::ParseError> {
    let expression = Expression::parse_mode(
        license.as_str(),
        spdx::ParseMode {
            allow_slash_as_or_operator: false,
            allow_imprecise_license_names: false,
            allow_postfix_plus_on_gpl: true,
            allow_deprecated: true,
            allow_unknown: true,
        },
    )?;
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
