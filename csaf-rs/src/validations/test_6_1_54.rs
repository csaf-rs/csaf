use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::validate_license_expression;

fn create_invalid_license_expression_error(license_expression: &str, error: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Invalid license expression '{license_expression}': {error}."),
        instance_path: "/document/license_expression".to_string(),
    })
}

/// 6.1.54 License Expression
///
/// It SHALL be tested that the license expression is valid.
/// To implement this test, it is deemed sufficient to check for the ABNF defined
/// in annex B of [SPDX](https://spdx.github.io/spdx-spec/v3.1-dev/) and the restriction
/// on the `DocumentRef` part and case sensitivity given in 3.2.2.8.
/// Although a match against the SPDX License List (`license-id`)
/// or a specific `LicenseRef` or `AdditionRef` list is not in scope for this test,
/// it checks all other constraints given in the ABNF.
pub fn test_6_1_54_invalid_license_expression(
    doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    document
        .license_expression
        .as_ref()
        // Unknown SPDX identifiers should not fail test 6.1.54
        .map(|license| match validate_license_expression(license) {
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
    use rstest::rstest;

    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_54 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[rstest]
    fn valid_license_expressions(
        #[values(
            // Regular SPDX expressions.
            "MIT",
            "MIT AND Apache-2.0",
            "MIT OR Apache-2.0",
            "MIT WITH LLVM-exception",
            // Special identifiers.
            "NONE",
            "NOASSERTION",
            "NONE AND MIT",
            "MIT AND NONE",
            "NOASSERTION AND Apache-2.0",
            "NONE AND NOASSERTION",
            // Parentheses and operator precedence.
            "NONE AND (MIT OR Apache-2.0)",
            "MIT OR (Apache-2.0 AND BSD-3-Clause)",
            "(NONE AND MIT) AND Apache-2.0",
            "MIT AND (NOASSERTION AND Apache-2.0)",
            "( MIT )",
            // Unknown license identifiers.
            "Unknown-License",
            "Unknown-License+",
            "Unknown-A+ AND Unknown-B+",
            "Unknown-A+ OR MIT",
            "Unknown-License+ WITH Unknown-Exception",
            "Unknown-licence+",
            "mit",
            "apache-2.0",
            "MIT WITH llvm-exception",
            // Valid user-defined references.
            "LicenseRef-Custom",
            "MIT AND LicenseRef-Custom",
            "LicenseRef-..Custom--",
            "MIT WITH AdditionRef-Custom",
            // Special identifiers are only restricted as direct operands.
            "NONE AND MIT OR Apache-2.0",
            "((NONE AND MIT)) OR Apache-2.0",
            "(NONE AND MIT) OR (NOASSERTION AND Apache-2.0)",
            "(MIT OR Apache-2.0) AND NONE",
            "NONE AND MIT AND NOASSERTION",
            "Apache-2.0 OR (MIT AND NOASSERTION)"
        )]
        expression: &str,
    ) {
        assert!(
            validate_license_expression(expression).is_ok(),
            "Expected valid expression: {expression}"
        );
    }

    #[rstest]
    fn invalid_license_expressions(
        #[values(
            // Invalid combinations with special identifiers.
            "NONE OR MIT",
            "MIT OR NONE",
            "NOASSERTION OR Apache-2.0",
            "MIT OR NOASSERTION",
            "NONE WITH LLVM-exception",
            "MIT WITH NOASSERTION",
            // Invalid combinations with parentheses.
            "MIT AND (NONE OR Apache-2.0)",
            "NONE AND (MIT OR NOASSERTION)",
            "((NONE)) OR MIT",
            "MIT OR (NONE)",
            // Invalid '+' usage.
            "NONE+",
            "NOASSERTION+",
            "LicenseRef-Custom+",
            "MIT WITH Unknown-Exception+",
            // Case sensitivity under our strict policy.
            "MIT or Apache-2.0",
            "MIT and Apache-2.0",
            "MIT with LLVM-exception",
            "none",
            "noassertion",
            "licenseref-Custom",
            "additionref-Custom",
            // DocumentRef restrictions.
            "DocumentRef-Example:LicenseRef-Custom",
            // Invalid whitespace.
            "MIT AND  Apache-2.0",
            "MIT  OR Apache-2.0",
            "MIT\tAND Apache-2.0",
            "MIT AND\nApache-2.0",
            "MIT AND(Apache-2.0)",
            "MIT OR(Apache-2.0)",
            // Invalid syntax.
            "MIT AND",
            "MIT OR",
            "MIT AND OR Apache-2.0",
            "(MIT AND Apache-2.0",
            "MIT AND Apache-2.0)",
            // Invalid reference identifiers.
            "LicenseRef-",
            "LicenseRef-.",
            "MIT WITH AdditionRef-",
            "MIT WITH AdditionRef-.",
            // Invalid unknown identifiers.
            "Foo:Bar",
            "---",
            // Invalid surrounding whitespace.
            " MIT",
            "MIT ",
            // Multiple '+' suffixes after a license identifier.
            "Unknown-License++",
            "Unknown-License+++",
            "Unknown-A++ AND MIT",
            "MIT AND Unknown-B++",
            // '+' suffixes after a special identifier.
            "NONE++",
            "NOASSERTION++",
            "NOASSERTION+++",
            "MIT AND NOASSERTION+",
            "NOASSERTION+ AND MIT",
            // Invalid: '+' must immediately follow the license identifier.
            "Unknown-License +",
            "Unknown-License + AND MIT",
            "MIT WITH Unknown-Exception +",
            // Multiple WITH clauses on the same license requirement.
            "MIT WITH Unknown-Exception WITH LLVM-exception",
            "Unknown-License+ WITH Unknown-Exception WITH LLVM-exception",
            "MIT WITH Unknown-Exception WITH Another-Exception"
        )]
        expression: &str,
    ) {
        assert!(
            validate_license_expression(expression).is_err(),
            "Expected invalid expression: {expression}"
        );
    }

    #[test]
    fn parser_error_preserves_original_expression() {
        let expression = "Unknown-License+ AND )";

        let error = validate_license_expression(expression).unwrap_err();

        // The error must refer to the original string, not the
        // normalized version where '+' has been replaced.
        assert_eq!(error.original, expression);
    }

    #[test]
    fn structural_validation_respects_operator_precedence() {
        // OR is entirely within the right operand of AND.
        assert!(validate_license_expression("NONE AND (MIT OR Apache-2.0)").is_ok());

        // AND binds tighter, so NONE is an operand of AND, not of OR.
        assert!(validate_license_expression("MIT OR NONE AND Apache-2.0").is_ok());

        // Parentheses are transparent: NONE is still a direct OR operand.
        assert!(validate_license_expression("(NONE) OR Apache-2.0").is_err());
    }

    #[test]
    fn test_test_6_1_54() {
        TESTS_2_1.test_6_1_54.expect(ExpectedResults {
            case_01: Err(vec![create_invalid_license_expression_error(
                "This is a license text that should not be here.",
                "Error at position 5: expected one of `AND`, `OR`, `WITH`, `)`, `+` here",
            )]),
            case_02: Err(vec![create_invalid_license_expression_error(
                "DocumentRef-some-document-reference:LicenseRef-www.example.org-Example-CSAF-License-2.0",
                "Error at position 0: expected a `LicenseRef` here",
            )]),
            case_03: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0+",
                "Error at position 51: expected one of `AND`, `OR`, `WITH`, `)` here",
            )]),
            case_04: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org/Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_05: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org%20Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_06: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org#Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_07: Err(vec![create_invalid_license_expression_error(
                "licenseref-www.example.org-Example-CSAF-License-3.0",
                "Error at position 0: expected a `correctly cased reference identifier` here",
            )]),
            case_08: Err(vec![create_invalid_license_expression_error(
                "licenseREF-www.example.org-Example-CSAF-License-3.0",
                "Error at position 0: expected a `correctly cased reference identifier` here",
            )]),
            case_09: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH additionref-www.example.org-Example-exception-0.9",
                "Error at position 57: expected a `correctly cased reference identifier` here",
            )]),
            case_21: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH  AdditionRef-www.example.org-Example-exception-0.9",
                "Error at position 57: expected a `single space` here",
            )]),
            case_22: Err(vec![create_invalid_license_expression_error(
                "MIT OR (any-OSI and NONE)",
                "Error at position 16: expected a `uppercase operator` here",
            )]),
            case_23: Err(vec![create_invalid_license_expression_error(
                "MIT or )any-OSI WITH NONE)",
                "Error at position 4: expected a `uppercase operator` here",
            )]),

            // Case 11: "CC-BY-4.0"
            // Case 12: "LicenseRef-www.example.org-Example-CSAF-License-2.0"
            // Case 13: "LicenseRef-www.example.org-Example-CSAF-License-3.0"
            // Case 14: "A.SPDX-License-ID-that.technically.could.exist-but-obviously-does-not"
            // Case 15: "A.SPDX-License-ID-that.technically.could.exist-but-obviously-does-not.including+"
            // Case 16: "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH AdditionRef-www.example.org-Example-exception-0.9"
            // Case 17: "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH Some.SPDX-License-Exception-ID-that.technically.could.exist-but-obviously-does-not"
            // Case 18: "MIT OR any-OSI"
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
            case_18: Ok(()),
        });
    }
}
