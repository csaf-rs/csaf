use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_missing_cwe_error(vulnerability_index: usize, field_name: &str) -> ValidationError {
    ValidationError {
        message: format!("Vulnerability is missing '{field_name}' property"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    }
}

/// 6.3.4 Missing CWE
///
/// Tests if all vulnerabilities have a `/vulnerabilities[]/cwe` (CSAF 2.0) /
/// `/vulnerabilities[]/cwes` (CSAF 2.1) field present.
pub fn test_6_3_4_missing_cwe(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
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
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_4() {
        let case_01_20 = Err(vec![create_missing_cwe_error(0, "cwe")]);
        let case_02_20 = Err(vec![create_missing_cwe_error(0, "cwe"), create_missing_cwe_error(2, "cwe")]);
        let case_01_21 = Err(vec![create_missing_cwe_error(0, "cwes")]);
        let case_02_21 = Err(vec![create_missing_cwe_error(0, "cwes"), create_missing_cwe_error(2, "cwes")]);

        // Both CSAF 2.0 and 2.1 have 4 test cases
        TESTS_2_0
            .test_6_3_4
            .expect(case_01_20, case_02_20, Ok(()), Ok(()));
        TESTS_2_1.test_6_3_4.expect(case_01_21, case_02_21, Ok(()), Ok(()));
    }
}
