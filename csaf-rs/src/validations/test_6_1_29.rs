use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalProductIds};
use crate::validation::ValidationError;

fn create_missing_product_reference_error(vuln_index: usize, rem_index: usize) -> ValidationError {
    ValidationError {
        message: "Remediations need to at least have one of the elements group_ids or product_ids".to_string(),
        instance_path: format!("/vulnerabilities/{}/remediations/{}", vuln_index, rem_index),
    }
}

/// 6.1.29 Remediation without Product Reference
///
/// Each item in `/vulnerabilities[]/remediations` must have at least one of the elements group_ids or product_ids.
pub fn test_6_1_29_remediation_without_product_reference(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    let mut errors: Option<Vec<ValidationError>> = None;
    for (vuln_index, vulnerability) in vulnerabilities.iter().enumerate() {
        for (rem_index, remediation) in vulnerability.get_remediations().iter().enumerate() {
            if remediation.get_product_ids().is_none() && remediation.get_product_ids().is_none() {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_missing_product_reference_error(vuln_index, rem_index));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_29
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_29_remediation_without_product_reference(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_29
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_29_remediation_without_product_reference(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_29() {
        let case_01 = Err(vec![create_missing_product_reference_error(0, 0)]);

        // Both CSAF 2.0 and 2.1 have 3 test cases
        TESTS_2_0.test_6_1_29.expect(case_01.clone(), Ok(()), Ok(()));
        TESTS_2_1.test_6_1_29.expect(case_01, Ok(()), Ok(()));
    }
}
