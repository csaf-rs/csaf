use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_cwe_error(vulnerability_index: usize, field_name: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Vulnerability is missing '{field_name}' property"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

/// 6.3.4 Missing CWE
///
/// Tests if all vulnerabilities have a `/vulnerabilities[]/cwe` (CSAF 2.0) /
/// `/vulnerabilities[]/cwes` (CSAF 2.1) field present.
pub fn test_6_3_4_missing_cwe(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    let vulnerabilities = doc.get_vulnerabilities();

    if vulnerabilities.is_empty() {
        // TODO #409 wasSkipped
        return Ok(());
    }

    for (v_i, vuln) in vulnerabilities.iter().enumerate() {
        if vuln.get_cwes().is_none() {
            errors
                .get_or_insert_default()
                .push(create_missing_cwe_error(v_i, vuln.get_cwe_property_name()));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_4, test_6_3_4_missing_cwe);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_4 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_4 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_4() {
        let make_cwe_error_20 = |idx| create_missing_cwe_error(idx, "cwe");
        let make_cwe_error_21 = |idx| create_missing_cwe_error(idx, "cwes");

        let single_vuln_no_cwe_20 = Err(vec![make_cwe_error_20(0)]);
        let multi_vuln_alternating_no_cwe_20 = Err(vec![make_cwe_error_20(0), make_cwe_error_20(2)]);
        let single_vuln_no_cwe_21 = Err(vec![make_cwe_error_21(0)]);
        let multi_vuln_alternating_no_cwe_21 = Err(vec![make_cwe_error_21(0), make_cwe_error_21(2)]);

        // Case 11: 1 vuln, with CWE (fixed case 01)
        // Case 12: 3 vuln, all with CWEs (fixed case 02)
        // Case S11: no vulns TODO #409 wasskipped

        // Both CSAF 2.0 and 2.1 have 4 test cases
        TESTS_2_0.test_6_3_4.expect(ExpectedResults_2_0 {
            case_01: single_vuln_no_cwe_20,
            case_02: multi_vuln_alternating_no_cwe_20,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_3_4.expect(ExpectedResults_2_1 {
            case_01: single_vuln_no_cwe_21,
            case_02: multi_vuln_alternating_no_cwe_21,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
    }
}
