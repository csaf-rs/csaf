use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalProductIds};
use crate::validation::ValidationError;

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
                    .push(test_6_1_29_err_generator(vuln_index, rem_index));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_29_err_generator(vuln_index: usize, rem_index: usize) -> ValidationError {
    ValidationError {
        message: "Remediations need to at least have one of the elements group_ids or product_ids".to_string(),
        instance_path: format!("/vulnerabilities/{}/remediations/{}", vuln_index, rem_index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_29() {
        let errors = HashMap::from([("01", vec![test_6_1_29_err_generator(0, 0)])]);
        run_csaf20_tests("29", test_6_1_29_remediation_without_product_reference, errors.clone());
        run_csaf21_tests("29", test_6_1_29_remediation_without_product_reference, errors);
    }
}
