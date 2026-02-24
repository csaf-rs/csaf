use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};
use crate::csaf::types::csaf_datetime::ValidCsafDateTime;
use crate::csaf_traits::{CsafTrait, InvolvementTrait, VulnerabilityTrait, WithOptionalDate};
use crate::schema::csaf2_1::schema::PartyCategory;
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_involvement_error(
    date: &Option<ValidCsafDateTime>,
    party: &PartyCategory,
    vul_r: usize,
    inv_r: usize,
) -> ValidationError {
    let date_str = date.as_ref().map_or("none".to_string(), |d| d.to_string());
    ValidationError {
        message: format!("Duplicate usage of tuple of involvement date {date_str} and party {party}"),
        instance_path: format!("/vulnerabilities/{vul_r}/involvements/{inv_r}"),
    }
}

/// Test 6.1.24: Multiple Definition in Involvements
///
/// Vulnerability items must not contain the same tuples of the `/vulnerabilities[]/involvements[]/date`
/// and `/vulnerabilities[]/involvements[]/party` fields.
pub fn test_6_1_24_multiple_definition_in_involvements(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Check if there are any vulnerabilities, if there aren't, this test can be skipped
    if vulnerabilities.is_empty() {
        // This will be WasSkipped later
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // Iterate over all vulnerabilities and their involvements
    for (vuln_i, vulnerability) in vulnerabilities.iter().enumerate() {
        if let Some(involvements) = vulnerability.get_involvements() {
            // Map involvement path indices to (date, party) tuples
            // HashMap key is a tuple of Option<ValidCsafDateTime>, with None being used if the optional date is not present,
            // and the PartyCategory enum, value is a vector of involvement indices
            type DatePartyPathsMap = HashMap<(Option<ValidCsafDateTime>, PartyCategory), Vec<usize>>;
            let mut date_party_paths_map: Option<DatePartyPathsMap> = None;
            for (inv_i, involvement) in involvements.iter().enumerate() {
                // if the involvement does have a date, check if it's valid
                let date = match involvement.get_date() {
                    // If the date is invalid, generate an error and skip this involvement
                    Some(Invalid(err)) => {
                        errors.get_or_insert_default().push(
                            err.into_validation_error(&format!("/vulnerabilities/{vuln_i}/involvements/{inv_i}/date")),
                        );
                        continue;
                    },
                    // If the date is valid, use the parsed date as hash key
                    Some(Valid(date)) => Some(date),
                    // If the date is not present, use None as hash key
                    None => None,
                };
                let party = involvement.get_party();
                let paths = date_party_paths_map
                    .get_or_insert_default()
                    .entry((date, party))
                    .or_default();
                paths.push(inv_i);
            }

            // If there were any involvements with valid dates
            if let Some(date_party_paths_map) = date_party_paths_map {
                // Generate errors for (date, party) tuples with multiple involvement paths indices
                for ((date, party), paths) in &date_party_paths_map {
                    if paths.len() > 1 {
                        errors.get_or_insert_default().extend(
                            paths
                                .iter()
                                .map(|path| generate_duplicate_involvement_error(date, party, vuln_i, *path)),
                        );
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
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
    use crate::csaf::types::csaf_datetime::CsafDateTime::{self, Invalid};
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_24() {
        // Case 01: One vulnerability, two involvements, same date, same party, different status
        // Case 02: One vulnerability, two involvements, same date, same party, same status

        // Case 11: Two vulnerabilities, one involvement each, same date, same party, different status
        // Case 12: Two vulnerabilities, one involvement each, same date, same party, same status

        // Case S01: One vulnerability, two involvements without date, same party, different status
        // Case S02: One vulnerability, one involvement, date is malformed
        // Case S03: One Vulnerability, three involvements, two with same date, party, different status
        // Case S04: One Vulnerability, 4 involvements, 2 pairwise same date, party, different status

        // Case S11: One Vulnerability, Two involvements, same date, different party
        // Case S12: One Vulnerability, Two involvements, different date, same party
        // Case S13: One Vulnerability, No involvements

        // TODO: I left out the coverage of status having no influence here.

        // For CSAF 2.0 and 2.1, different errors are generated, as they have different "default" dates
        // Shared values for the test cases
        let default_date_csaf_20 = Some(ValidCsafDateTime::from_str("2021-04-23T10:00:00.000Z").unwrap());
        let alternate_date_csaf_20 = Some(ValidCsafDateTime::from_str("2021-04-24T10:00:00.000Z").unwrap());
        let default_date_csaf_21 = Some(ValidCsafDateTime::from_str("2023-08-23T10:00:00.000Z").unwrap());
        let alternate_date_csaf_21 = Some(ValidCsafDateTime::from_str("2023-08-24T10:00:00.000Z").unwrap());
        let vendor = PartyCategory::Vendor;
        let coordinator = PartyCategory::Coordinator;

        // Date-independent test cases
        let case_s01 = Err(vec![
            generate_duplicate_involvement_error(&None, &vendor, 0, 0),
            generate_duplicate_involvement_error(&None, &vendor, 0, 1),
        ]);
        let Invalid(case_s02_err) = CsafDateTime::from("not-a-valid-date") else {
            unreachable!()
        };
        let case_s02 = Err(vec![
            case_s02_err.into_validation_error("/vulnerabilities/0/involvements/0/date"),
        ]);

        TESTS_2_0.test_6_1_24.expect(
            // case_01
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 1),
            ]),
            // case_02
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 1),
            ]),
            case_s01.clone(),
            case_s02.clone(),
            // case_s03
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 2),
            ]),
            // case_s04
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_20, &vendor, 0, 2),
                generate_duplicate_involvement_error(&alternate_date_csaf_20, &coordinator, 0, 1),
                generate_duplicate_involvement_error(&alternate_date_csaf_20, &coordinator, 0, 3),
            ]),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_s11
            Ok(()), // case_s12
            Ok(()), // case_s13
        );

        TESTS_2_1.test_6_1_24.expect(
            // case_01
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 1),
            ]),
            // case_02
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 1),
            ]),
            case_s01,
            case_s02,
            // case_s03
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 2),
            ]),
            // case_s04
            Err(vec![
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 0),
                generate_duplicate_involvement_error(&default_date_csaf_21, &vendor, 0, 2),
                generate_duplicate_involvement_error(&alternate_date_csaf_21, &coordinator, 0, 1),
                generate_duplicate_involvement_error(&alternate_date_csaf_21, &coordinator, 0, 3),
            ]),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_s11
            Ok(()), // case_s12
            Ok(()), // case_s13
        );
    }
}
