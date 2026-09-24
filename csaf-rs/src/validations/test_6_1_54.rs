use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::parse_csaf_license_expression;

fn create_invalid_license_expression_error(license_expression: &str, error: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Invalid license expression '{license_expression}': {error}."),
        instance_path: "/document/license_expression".to_string(),
    })
}

/// 6.1.54 License Expression
///
/// It MUST be tested that the license expression is valid.
/// To implement this test, it is deemed sufficient to check for the ABNF defined
/// in annex B of [SPDX](https://spdx.github.io/spdx-spec/) and the restriction
/// on the DocumentRef part given in 3.2.2.8.
pub fn test_6_1_54_invalid_license_expression(
    doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    document
        .license_expression
        .as_ref()
        .map(|license| match parse_csaf_license_expression(license) {
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
    use crate::csaf2_1::testcases::ExpectedResults_6_1_54 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_54() {
        TESTS_2_1.test_6_1_54.expect(ExpectedResults {
            case_01: Err(vec![create_invalid_license_expression_error(
                "This is a license text that should not be here.",
                r#"Error at position 5: expected one of `AND`, `OR`, `WITH`, `)`, `+` here"#,
            )]),
            case_02: Err(vec![create_invalid_license_expression_error(
                "DocumentRef-some-document-reference:LicenseRef-www.example.org-Example-CSAF-License-2.0",
                r#"Error at position 0: expected a `LicenseRef` here"#,
            )]),
            case_03: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0+",
                r#"Error at position 51: expected one of `AND`, `OR`, `WITH`, `)` here"#,
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
            case_07: Ok(()), // TODO: clarify if licenseref-... should be allowed, #672
            case_08: Ok(()), // TODO: clarify if licenseREF-... should be allowed, #672
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
        });
    }
}
