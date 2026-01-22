use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalGroupIds, WithOptionalProductIds};
use crate::validation::ValidationError;

fn create_flag_without_product_reference_error(vulnerability_index: usize, flag_index: usize) -> ValidationError {
    ValidationError {
        message: "Each flag must reference at least one group_id or product_id".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/flags/{flag_index}"),
    }
}

/// 6.1.32 Flag without Product Reference
///
/// Each `/vulnerabilities[]/flags[]` item needs to contain at least one element
/// in it's `group_ids` or `product_ids` arrays.
pub fn test_6_1_32_flag_without_product_reference(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Check each flag in each vulnerability
    for (v_r, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(flags) = vulnerability.get_flags() {
            for (f_r, flag) in flags.iter().enumerate() {
                // Check if both group_ids and product_ids present and not empty
                if let Some(mut group_ids) = flag.get_group_ids()
                    && group_ids.any(|_| true)
                {
                    continue;
                }
                if let Some(mut product_ids) = flag.get_product_ids()
                    && product_ids.any(|_| true)
                {
                    continue;
                }
                errors.push(create_flag_without_product_reference_error(v_r, f_r));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
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
        let case_01 = Err(vec![create_flag_without_product_reference_error(0, 0)]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_32.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_1_32.expect(case_01, Ok(()));
    }
}
