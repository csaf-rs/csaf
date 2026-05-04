use crate::csaf_traits::{CsafTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use crate::validations::utils::rvisc;

fn create_matching_text_error(system_name: &str, text: &str, vuln_index: usize, id_index: usize) -> ValidationError {
    ValidationError {
        message: format!("The text '{text}' does not match the pattern for registered ID system '{system_name}'"),
        instance_path: format!("/vulnerabilities/{vuln_index}/ids/{id_index}/text"),
    }
}

/// 6.2.53 Matching Text for Registered ID System
///
/// For each item in `/vulnerabilities[]/ids` that has the value of a registered vulnerability ID system
/// as `system_name`, it MUST be tested that the `text` in the CSAF document matches the `text_pattern`
/// given by the "Registry for Vulnerability ID Systems for CSAF" (RVISC).
pub fn test_6_2_53_matching_text_for_registered_id_system(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(ids) = vuln.get_ids() {
            for (i_i, id) in ids.iter().enumerate() {
                let system_name = id.get_system_name();
                if let Some(regex) = rvisc::lookup_regex(system_name) {
                    let text = id.get_text();
                    if !regex.is_match(text) {
                        errors
                            .get_or_insert_default()
                            .push(create_matching_text_error(system_name, text, v_i, i_i));
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_53,
    test_6_2_53_matching_text_for_registered_id_system
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_53() {
        let case_01 = Err(vec![create_matching_text_error(
            "https://github.com/oasis-tcs/csaf",
            "Issue 1217",
            0,
            0,
        )]);

        // Case 11: Valid OASIS CSAF TC Issues text

        TESTS_2_1.test_6_2_53.expect(case_01, Ok(()));
    }
}
