use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithGroupIds, WithProductIds};
use crate::validation::ValidationError;

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
                errors.push(ValidationError {
                    message: "Each flag must reference at least one group_id or product_id".to_string(),
                    instance_path: format!("/vulnerabilities/{}/flags/{}", v_r, f_r),
                });
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_32::test_6_1_32_flag_without_product_reference;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_32() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "Each flag must reference at least one group_id or product_id".to_string(),
                instance_path: "/vulnerabilities/0/flags/0".to_string(),
            }],
        )]);
        run_csaf20_tests("32", test_6_1_32_flag_without_product_reference, errors.clone());
        run_csaf21_tests("32", test_6_1_32_flag_without_product_reference, errors);
    }
}
