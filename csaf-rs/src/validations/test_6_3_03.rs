use crate::{
    validation::{TestFinding, TestFindingData},
    validations::utils::raw_json_value_helper::{JsonValuePresence, get_json_presence},
};
use serde_json::Value;

fn create_missing_cve_info(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Vulnerability is missing 'cve' property".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

fn create_unset_cve_info(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Vulnerability 'cve' property is not set (e.g., `null`)".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/cve"),
    })
}

fn create_empty_cve_info(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Vulnerability 'cve' property is empty (e.g., `\"\"`)".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/cve"),
    })
}

/// 6.3.3 Missing CVE
///
/// It SHALL be tested that the element `cve` is present and set. A CSAF Validator SHALL differentiate
/// in the error message between the key being present but having no or an empty value
/// and not being present at all.
pub fn test_6_3_3_missing_cve(json: &Value) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    // check if there are vulnerabilities at all
    let Some(vulnerabilities) = json.pointer("/vulnerabilities").and_then(Value::as_array) else {
        return Ok(());
    };

    for (v_i, _) in vulnerabilities.iter().enumerate() {
        let cve_path = format!("/vulnerabilities/{v_i}/cve");
        let finding = match get_json_presence(&cve_path, json) {
            JsonValuePresence::Missing => create_missing_cve_info(v_i),
            JsonValuePresence::Unset => create_unset_cve_info(v_i),
            JsonValuePresence::EmptyString => create_empty_cve_info(v_i),
            // empty array and empty object are schema violations, which are reported by the schema test
            JsonValuePresence::EmptyArray | JsonValuePresence::EmptyObject |
            // set is valid
            JsonValuePresence::Set => continue
        };
        errors.get_or_insert_default().push(finding);
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_raw_json_validator!(ValidatorForTest6_3_3, test_6_3_3_missing_cve);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_3 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_3 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_3() {
        let single_vuln_no_cve = Err(vec![create_missing_cve_info(0)]);
        let multi_vuln_alternating_no_cve = Err(vec![create_missing_cve_info(0), create_missing_cve_info(2)]);

        // Case 01: 1 vuln, no CVE
        // Case 02: 3 vuln, alternating cve presence
        // Case 11: 1 vuln, with CVE (fixed case 01)
        // Case 12: 3 vuln, all with CVEs (fixed case 02)
        // Case S11: no vulns at all

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
