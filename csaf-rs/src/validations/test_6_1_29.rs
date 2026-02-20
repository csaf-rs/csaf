use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalGroupIds, WithOptionalProductIds};
use crate::validation::ValidationError;

fn create_missing_product_reference_error(vulnerability_index: usize, remediation_index: usize) -> ValidationError {
    ValidationError {
        message: "A remediation needs to at least have one of the elements group_ids or product_ids".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/remediations/{remediation_index}"),
    }
}

/// 6.1.29 Remediation without Product Reference
///
/// Each item in `/vulnerabilities[]/remediations[]` must have at least one of the elements group_ids or product_ids.
pub fn test_6_1_29_remediation_without_product_reference(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Check if there are vulnerability, if not, this test can be skipped
    if vulnerabilities.is_empty() {
        // This will be a WasSkipped later
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;

    // Check vulnerability and each remediation in them
    for (vuln_i, vulnerability) in vulnerabilities.iter().enumerate() {
        for (rem_i, remediation) in vulnerability.get_remediations().iter().enumerate() {
            // Check if both product_ids and group_ids are None, if so, generate an error
            // Through the schema, it is ensured that if they are not None, they contain at least one entry
            if remediation.get_product_ids().is_none() && remediation.get_group_ids().is_none() {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_missing_product_reference_error(vuln_i, rem_i));
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
        // Case 01: A remediation without product_ids and group ids
        let case_01 = Err(vec![create_missing_product_reference_error(0, 0)]);
        // Case 11: A remediation with product_ids but without group ids
        // Case 12: A vulnerability without a remediation

        // Case S01: Two vulnerabilities, with two remediations each, two of which are missing product references
        let case_s01 = Err(vec![
            create_missing_product_reference_error(0, 0),
            create_missing_product_reference_error(1, 1),
            create_missing_product_reference_error(2, 1)
        ]);
        // Case S11: A remediation with group_ids but without product_ids
        // Case S12: A remediation with both group_ids and product_ids

        // Both CSAF 2.0 and 2.1 have 6 test cases
        TESTS_2_0
            .test_6_1_29
            .expect(case_01.clone(), case_s01.clone(), Ok(()), Ok(()), Ok(()), Ok(()));
        TESTS_2_1
            .test_6_1_29
            .expect(case_01, case_s01, Ok(()), Ok(()), Ok(()), Ok(()));
    }
}
