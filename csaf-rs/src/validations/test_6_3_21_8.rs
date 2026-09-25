use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_vulnerability_extension_info(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The vulnerability contains a CSAF Extension.".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/x_extensions"),
    })
}

/// 6.3.21.8 Usage of Extension at Vulnerabilities Level
///
/// It SHALL be tested that the element `$.vulnerabilities[*].x_extensions` does not exist.
pub fn test_6_3_21_8_usage_of_extension_at_vulnerabilities_level(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_extensions().is_some() {
            findings
                .get_or_insert_default()
                .push(create_vulnerability_extension_info(vulnerability_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_8,
    test_6_3_21_8_usage_of_extension_at_vulnerabilities_level
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_8 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_8() {
        // Case 11: extension at root, but not at vulnerabilities
        TESTS_2_1.test_6_3_21_8.expect(ExpectedResults {
            case_01: Err(vec![create_vulnerability_extension_info(0)]),
            case_11: Ok(()),
        });
    }
}
