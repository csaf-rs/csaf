use crate::csaf_traits::CsafTrait;
use crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::try_contains_unlisted_license_identifier_or_exception;

fn create_unregistered_license_used_info(license_expression: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "The license expression contains a license identifier or exception that is not listed in the SPDX license list or ScanCode LicenseDB: `{license_expression}`."
        ),
        instance_path: "/document/license_expression".to_string(),
    })
}

/// 6.3.17 Use of Unregistered License
///
/// It SHALL be tested that the all license identifiers and exceptions are listed either in the
/// official SPDX license identifier list or AboutCode's "ScanCode LicenseDB".
pub fn test_6_3_17_use_of_unregistered_license(doc: &CommonSecurityAdvisoryFramework) -> Result<(), Vec<TestFinding>> {
    // if there is a license expression
    let Some(license_expression) = doc.get_document().license_expression.as_ref() else {
        return Ok(());
    };

    // check if it contains at least one unlisted license identifier or exception
    match try_contains_unlisted_license_identifier_or_exception(license_expression) {
        // there was
        Ok(true) => Err(vec![create_unregistered_license_used_info(license_expression.as_str())]),
        // there wasn't or a parsing error occurred (before one was found)
        Ok(false) | Err(_) => Ok(()),
    }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_3_17, test_6_3_17_use_of_unregistered_license);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_17 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_17() {
        TESTS_2_1.test_6_3_17.expect(ExpectedResults {
            case_01: Err(vec![create_unregistered_license_used_info(
                "LicenseRef-www.example.com-no-work-pd",
            )]),
            case_11: Ok(()),
        });
    }
}
