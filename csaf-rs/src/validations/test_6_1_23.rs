use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_cve_error(cve: &str, path: usize) -> ValidationError {
    ValidationError {
        message: format!("Duplicate usage of same CVE identifier '{cve}'"),
        instance_path: format!("/vulnerabilities/{path}/cve"),
    }
}

/// Test 6.1.23: Multiple Use of Same CVE
///
/// Vulnerability items must not contain the same string in the `/vulnerabilities[]/cve` field.
pub fn test_6_1_23_multiple_use_of_same_cve(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Check if there are any vulnerabilities, if there aren't, this test can be skipped
    if vulnerabilities.is_empty() {
        // This will be WasSkipped later
        return Ok(());
    }

    // Map occurrence paths indexes to CVE identifiers
    let mut cve_paths: HashMap<String, Vec<usize>> = HashMap::new();
    for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
        let cve = vulnerability.get_cve();
        if let Some(cve) = cve {
            let path = cve_paths.entry(cve.clone()).or_default();
            path.push(i_r);
        }
    }

    // Check if there are any CVE identifiers, if there aren't, this test can be skipped
    if cve_paths.is_empty() {
        // This will be WasSkipped later
        return Ok(());
    }

    // Generate errors for CVE identifiers with multiple occurrence paths indexes
    let mut errors: Option<Vec<ValidationError>> = None;
    for (cve, paths) in &cve_paths {
        if paths.len() > 1 {
            errors
                .get_or_insert_default()
                .extend(paths.iter().map(|path| generate_duplicate_cve_error(cve, *path)));
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_23
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_23_multiple_use_of_same_cve(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_23
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_23_multiple_use_of_same_cve(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_23() {
        // Case 01: Two vulnerabilities, same CVE identifier
        let case_01 = Err(vec![
            generate_duplicate_cve_error("CVE-2017-0145", 0),
            generate_duplicate_cve_error("CVE-2017-0145", 1),
        ]);

        // Case S01: Three vulnerabilities, same CVE identifier
        let case_s01 = Err(vec![
            generate_duplicate_cve_error("CVE-2017-0145", 0),
            generate_duplicate_cve_error("CVE-2017-0145", 1),
            generate_duplicate_cve_error("CVE-2017-0145", 2),
        ]);
        // Case S02: Four vulnerabilities, 2 pairs with same CVE identifier
        let case_s02 = Err(vec![
            generate_duplicate_cve_error("CVE-2017-0145", 0),
            generate_duplicate_cve_error("CVE-2017-0145", 2),
            generate_duplicate_cve_error("CVE-2017-0146", 1),
            generate_duplicate_cve_error("CVE-2017-0146", 3),
        ]);
        // Case S03: Three vulnerabilities, two with same CVE identifier
        let case_s03 = Err(vec![
            generate_duplicate_cve_error("CVE-2017-0145", 0),
            generate_duplicate_cve_error("CVE-2017-0145", 2),
        ]);
        // Case S11: Two vulnerabilities, different CVE identifiers (valid)

        // Both CSAF 2.0 and 2.1 have 6 test cases
        TESTS_2_0.test_6_1_23.expect(
            case_01.clone(),
            case_s01.clone(),
            case_s02.clone(),
            case_s03.clone(),
            Ok(()),
        );
        TESTS_2_1
            .test_6_1_23
            .expect(case_01, case_s01, case_s02, case_s03, Ok(()));
    }
}
