use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalDate};
use crate::validation::{TestFinding, TestFindingData};

/// 6.2.7 Missing Date in Involvements
///
/// Each involvement item must have the `date` field set.
pub fn test_6_2_07_missing_date_in_involvements(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    // for each vuln and each of its involvements, check if date is set
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(involvements) = vuln.get_involvements() {
            for (inv_i, involvement) in involvements.iter().enumerate() {
                // if not, generate an error
                if involvement.get_date().is_none() {
                    errors
                        .get_or_insert_default()
                        .push(create_missing_date_in_involvements_error(v_i, inv_i));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_missing_date_in_involvements_error(vulnerability_index: usize, involvement_index: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: "Involvement item is missing required 'date' field".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/involvements/{involvement_index}"),
    })
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_7, test_6_2_07_missing_date_in_involvements);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_7 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_7 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_07() {
        let case_01 = Err(vec![create_missing_date_in_involvements_error(0, 0)]);
        let case_s01 = Err(vec![
            create_missing_date_in_involvements_error(0, 1),
            create_missing_date_in_involvements_error(0, 3),
            create_missing_date_in_involvements_error(1, 1),
            create_missing_date_in_involvements_error(1, 3),
        ]);

        TESTS_2_0.test_6_2_7.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
        });

        // Failing test cases:
        // 01  - vulnerability with one involvement missing a date
        // s01 - alternating presence of dates across multiple vulnerabilities and involvements

        // Valid test cases:
        // s11 - no vulnerabilities
        // s12 - vulnerability without involvements
        // s13 - vulnerability with one involvement containing a date

        TESTS_2_1.test_6_2_7.expect(ExpectedResults_2_1 {
            case_01,
            case_s01,
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
    }
}
