use spdx::Expression;

use crate::csaf_traits::CsafTrait;
use crate::schema::csaf2_1::schema::LicenseExpression;
use crate::validation::ValidationError;

fn create_invalid_license_expression_error(license_expression: &str, error: &str) -> ValidationError {
    ValidationError {
        message: format!("Invalid license expression '{license_expression}': {error}."),
        instance_path: "/document/license_expression".to_string(),
    }
}

/// Parses the given license expression using the SPDX parser with specific options that align with the requirements of CSAF.
/// For example, unknown SPDX identifiers should not fail test 6.1.54, whereas expressions with DocumentRef are not allowed.
fn parse_license_as_allowed_in_csaf(license: &LicenseExpression) -> Result<Expression, spdx::ParseError> {
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
            } else if let Some(spdx::AdditionItem::Other(addition)) = &requirement.req.addition
                && addition.doc_ref.is_some()
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

/// 6.1.54 License Expression
///
/// It MUST be tested that the license expression is valid.
/// To implement this test, it it deemed sufficient to check for the ABNF defined in annex B of [SPDX301] and the restriction on the DocumentRef part given in 3.2.2.7.
pub fn test_6_1_54_invalid_license_expression(
    doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    document
        .license_expression
        .as_ref()
        .map(|license| match parse_license_as_allowed_in_csaf(license) {
            Ok(_) => Ok(()),
            Err(error) => Err(vec![create_invalid_license_expression_error(
                license.as_str(),
                format!("Error at position {}: {}", error.span.start, error.reason).as_str(),
            )]),
        })
        .unwrap_or(Ok(())) // TODO: this may become passed - not data #409
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_54, test_6_1_54_invalid_license_expression);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_54() {
        // Only CSAF 2.1 has this test with 6 test cases (3 error cases, 3 success case)
        TESTS_2_1.test_6_1_54.expect(
            Err(vec![create_invalid_license_expression_error(
                "This is a license text that should not be here.",
                r#"Error at position 5: expected one of `AND`, `OR`, `WITH`, `)`, `+` here"#,
            )]),
            // Ok(()),
            Err(vec![create_invalid_license_expression_error(
                "DocumentRef-some-document-reference:LicenseRef-www.example.org-Example-CSAF-License-2.0",
                r#"Error at position 0: expected a `LicenseRef` here"#,
            )]),
            Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0+",
                r#"Error at position 51: expected one of `AND`, `OR`, `WITH`, `)` here"#,
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
