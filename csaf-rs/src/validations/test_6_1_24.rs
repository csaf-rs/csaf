use crate::csaf_traits::{CsafTrait, InvolvementTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::PartyCategory;
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_involvement_error(
    date: &str,
    party: &PartyCategory,
    vul_r: usize,
    inv_r: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Duplicate usage of tuple of involvement date {} and party {}",
            date, party
        ),
        instance_path: format!("/vulnerabilities/{}/involvements/{}", vul_r, inv_r),
    }
}

/// Test 6.1.24: Multiple Definition in Involvements
///
/// Vulnerability items must not contain the same tuples of the `/vulnerabilities[]/involvements[]/date`
/// and `/vulnerabilities[]/involvements[]/party` fields.
pub fn test_6_1_24_multiple_definition_in_involvements(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    let mut errors = Vec::new();
    // Iterate over all vulnerabilities and their involvements
    for (vul_r, vulnerability) in vulnerabilities.iter().enumerate() {
        let involvements = vulnerability.get_involvements();
        if let Some(involvements) = involvements {
            // Map involvement path indices to (date,party) tuples
            let mut date_party_paths_map: HashMap<(String, PartyCategory), Vec<usize>> = HashMap::new();
            for (inv_r, involvement) in involvements.iter().enumerate() {
                if let Some(date) = involvement.get_date() {
                    let party = involvement.get_party();
                    let paths = date_party_paths_map.entry((date.clone(), party)).or_default();
                    paths.push(inv_r);
                }
            }
            // Generate errors for (date, party) tuples with multiple involvement paths indices
            for ((date, party), paths) in &date_party_paths_map {
                if paths.len() > 1 {
                    errors.extend(
                        paths
                            .iter()
                            .map(|path| generate_duplicate_involvement_error(date, party, vul_r, *path)),
                    );
                }
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_24
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_24_multiple_definition_in_involvements(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_24
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_24_multiple_definition_in_involvements(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_24() {
        // Error cases for CSAF 2.0 (different dates than CSAF 2.1)
        let case_01_v2_0 = Err(vec![
            generate_duplicate_involvement_error(
                "2021-04-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                0,
            ),
            generate_duplicate_involvement_error(
                "2021-04-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                1,
            ),
        ]);
        let case_02_v2_0 = Err(vec![
            generate_duplicate_involvement_error(
                "2021-04-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                0,
            ),
            generate_duplicate_involvement_error(
                "2021-04-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                1,
            ),
        ]);

        // Error cases for CSAF 2.1 (different dates than CSAF 2.0)
        let case_01_v2_1 = Err(vec![
            generate_duplicate_involvement_error(
                "2023-08-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                0,
            ),
            generate_duplicate_involvement_error(
                "2023-08-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                1,
            ),
        ]);
        let case_02_v2_1 = Err(vec![
            generate_duplicate_involvement_error(
                "2023-08-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                0,
            ),
            generate_duplicate_involvement_error(
                "2023-08-23T10:00:00.000Z",
                &PartyCategory::Vendor,
                0,
                1,
            ),
        ]);

        // CSAF 2.0 has 4 test cases (01-02, 11-12)
        TESTS_2_0.test_6_1_24.expect(
            case_01_v2_0,
            case_02_v2_0,
            Ok(()), // case_11
            Ok(()), // case_12
        );

        // CSAF 2.1 has 4 test cases (01-02, 11-12)
        TESTS_2_1.test_6_1_24.expect(
            case_01_v2_1,
            case_02_v2_1,
            Ok(()), // case_11
            Ok(()), // case_12
        );
    }
}
