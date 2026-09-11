use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_cve_error(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Vulnerability is missing 'cve' property".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

/// 6.3.3 Missing CVE
///
/// Tests if all vulnerabilities have their `/vulnerabilities[]/cve` field filled.
pub fn test_6_3_3_missing_cve(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    let vulnerabilities = doc.get_vulnerabilities();

    if vulnerabilities.is_empty() {
        // TODO #409 wasSkipped
        return Ok(());
    }

    for (v_i, vuln) in vulnerabilities.iter().enumerate() {
        if vuln.get_cve().is_none() {
            errors.get_or_insert_default().push(create_missing_cve_error(v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_3, test_6_3_3_missing_cve);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_3 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_3 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_3() {
        let single_vuln_no_cve = Err(vec![create_missing_cve_error(0)]);
        let multi_vuln_alternating_no_cve = Err(vec![create_missing_cve_error(0), create_missing_cve_error(2)]);

        // Case 11: 1 vuln, with CVE (fixed case 01)
        // Case 12: 3 vuln, with CWE (fixed case 02)
        // Case S11: no vulns, this might be wasSkipped later #409

        // TODO: Clarify upstream if this test should actually be "present and set" instead.
        // TODO If so, add present and set test coverage
        TESTS_2_0.test_6_3_3.expect(ExpectedResults_2_0 {
            case_01: single_vuln_no_cve.clone(),
            case_02: multi_vuln_alternating_no_cve.clone(),
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_3_3.expect(ExpectedResults_2_1 {
            case_01: single_vuln_no_cve,
            case_02: multi_vuln_alternating_no_cve,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
    }
}
