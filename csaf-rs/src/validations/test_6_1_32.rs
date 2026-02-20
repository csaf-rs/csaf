use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalGroupIds, WithOptionalProductIds};
use crate::validation::ValidationError;

fn create_flag_without_product_reference_error(vulnerability_index: usize, flag_index: usize) -> ValidationError {
    ValidationError {
        message: "A flag must have at least of the elements group_ids or product_ids".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/flags/{flag_index}"),
    }
}

/// 6.1.32 Flag without Product Reference
///
/// Each `/vulnerabilities[]/flags[]` must have at least one of the elements group_ids or product_ids.
pub fn test_6_1_32_flag_without_product_reference(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Check if there are any vulnerabilities, if there aren't, this test can be skipped
    if vulnerabilities.is_empty() {
        // This will be WasSkipped later
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // Check each flag in each vulnerability
    for (vuln_i, vulnerability) in vulnerabilities.iter().enumerate() {
        if let Some(flags) = vulnerability.get_flags() {
            for (flag_i, flag) in flags.iter().enumerate() {
                // Check if both product_ids and group_ids are None, if so, generate an error
                // Through the schema, it is ensured that if they are not None, they contain at least one entry
                if flag.get_product_ids().is_none() && flag.get_group_ids().is_none() {
                    errors
                        .get_or_insert_with(Vec::new)
                        .push(create_flag_without_product_reference_error(vuln_i, flag_i));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_32
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_32_flag_without_product_reference(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_32
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_32_flag_without_product_reference(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_32() {
        // Case 01: A flag without product_ids and group_ids
        let case_01 = Err(vec![create_flag_without_product_reference_error(0, 0)]);
        // Case 11: A flag with only product_ids

        // Case S01: Three vulnerabilities, two flags each, each with one flag without product_ids and group_ids
        let case_s01 = Err(vec![
            create_flag_without_product_reference_error(0, 0),
            create_flag_without_product_reference_error(1, 1),
            create_flag_without_product_reference_error(2, 1),
        ]);
        // Case S11: A flag with only group_ids
        // Case S12: A flag with both product_ids and group_ids
        // Case S13: A vulnerability without a flag

        TESTS_2_0
            .test_6_1_32
            .expect(case_01.clone(), case_s01.clone(), Ok(()), Ok(()), Ok(()), Ok(()));
        TESTS_2_1
            .test_6_1_32
            .expect(case_01, case_s01, Ok(()), Ok(()), Ok(()), Ok(()));
    }
}
