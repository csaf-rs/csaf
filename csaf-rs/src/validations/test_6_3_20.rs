use crate::csaf_traits::{CsafTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use crate::validations::utils::rvisc;

fn create_unregistered_id_system_error(system_name: &str, vuln_index: usize, id_index: usize) -> ValidationError {
    ValidationError {
        message: format!("The system_name '{system_name}' is not registered in RVISC."),
        instance_path: format!("/vulnerabilities/{vuln_index}/ids/{id_index}/system_name"),
    }
}

/// 6.3.20 Use of Unregistered ID System
///
/// For each item in `/vulnerabilities[]/ids` it MUST be tested that the value of `system_name`
/// belongs to a registered vulnerability ID system in RVISC.
pub fn test_6_3_20_use_of_unregistered_id_system(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(ids) = vuln.get_ids() {
            for (i_i, id) in ids.iter().enumerate() {
                if !rvisc::is_registered_id_system(id.get_system_name()) {
                    errors.get_or_insert_default().push(create_unregistered_id_system_error(
                        id.get_system_name(),
                        v_i,
                        i_i,
                    ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_20,
    test_6_3_20_use_of_unregistered_id_system
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_20() {
        let case_01 = Err(vec![create_unregistered_id_system_error(
            "OASIS Open CSAF TC GitHub Issues",
            0,
            0,
        )]);

        // Case 11: Valid OASIS CSAF TC Issues system name

        TESTS_2_1.test_6_3_20.expect(case_01, Ok(()));
    }
}
