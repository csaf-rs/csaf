use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_missing_cve_error(vulnerability_index: usize) -> ValidationError {
    ValidationError {
        message: "Vulnerability is missing 'cve' property".to_string(),
        instance_path: format!("/vulnerabilities/{}", vulnerability_index),
    }
}

/// 6.3.3 Missing CVE
///
/// Tests if all vulnerabilities have their `/vulnerabilities[]/cve` field filled.
pub fn test_6_3_3_missing_cve(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if vuln.get_cve().is_none() {
            errors.get_or_insert_with(Vec::new).push(create_missing_cve_error(v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_3_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_3_missing_cve(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_3_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_3_missing_cve(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_3() {
        let case_01 = Err(vec![create_missing_cve_error(0)]);
        let case_02 = Err(vec![create_missing_cve_error(0), create_missing_cve_error(2)]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0
            .test_6_3_3
            .expect(case_01.clone(), case_02.clone(), Ok(()), Ok(()));
        TESTS_2_1.test_6_3_3.expect(case_01, case_02, Ok(()), Ok(()));
    }
}
