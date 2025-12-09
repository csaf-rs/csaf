use crate::csaf_traits::{CsafTrait, InvolvementTrait, VulnerabilityTrait};
use crate::csaf2_1::schema::PartyCategory;
use crate::validation::ValidationError;
use std::collections::HashMap;

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
                    for path in paths.iter() {
                        errors.push(ValidationError {
                            message: format!(
                                "Duplicate usage of tuple of involvement date {} and party {}",
                                date, party
                            ),
                            instance_path: format!("/vulnerabilities/{}/involvements/{}", vul_r, path),
                        });
                    }
                }
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
    use crate::validations::test_6_1_24::test_6_1_24_multiple_definition_in_involvements;

    #[test]
    fn test_test_6_1_24() {
        let errors_20 = std::collections::HashMap::from([
            (
                "01",
                vec![
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2021-04-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/0".to_string(),
                    },
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2021-04-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/1".to_string(),
                    },
                ],
            ),
            (
                "02",
                vec![
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2021-04-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/0".to_string(),
                    },
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2021-04-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/1".to_string(),
                    },
                ],
            ),
        ]);
        let errors_21 = std::collections::HashMap::from([
            (
                "01",
                vec![
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2023-08-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/0".to_string(),
                    },
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2023-08-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/1".to_string(),
                    },
                ],
            ),
            (
                "02",
                vec![
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2023-08-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/0".to_string(),
                    },
                    crate::validation::ValidationError {
                        message:
                            "Duplicate usage of tuple of involvement date 2023-08-23T10:00:00.000Z and party vendor"
                                .to_string(),
                        instance_path: "/vulnerabilities/0/involvements/1".to_string(),
                    },
                ],
            ),
        ]);
        run_csaf20_tests("24", test_6_1_24_multiple_definition_in_involvements, errors_20);
        run_csaf21_tests("24", test_6_1_24_multiple_definition_in_involvements, errors_21);
    }
}
