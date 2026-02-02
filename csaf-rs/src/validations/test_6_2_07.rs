use crate::csaf_traits::{CsafTrait, VulnerabilityTrait, WithOptionalDate};
use crate::validation::ValidationError;

/// 6.2.7 Missing Date in Involvements
///
/// Each involvement item must have the `date` field set.
pub fn test_6_2_07_missing_date_in_involvements(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each vuln and each of its involvements, check if date is set
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(involvements) = vuln.get_involvements() {
            for (inv_i, involvement) in involvements.iter().enumerate() {
                // if not, generate an error
                if involvement.get_date().is_none() {
                    errors
                        .get_or_insert_with(Vec::new)
                        .push(create_missing_date_in_involvements_error(v_i, inv_i));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_missing_date_in_involvements_error(vulnerability_index: usize, involvement_index: usize) -> ValidationError {
    ValidationError {
        message: "Involvement item is missing required 'date' field".to_string(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/involvements/{involvement_index}"),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_07_missing_date_in_involvements(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_07_missing_date_in_involvements(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_07() {
        let case_01 = Err(vec![create_missing_date_in_involvements_error(0, 0)]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_7.expect(case_01.clone());
        TESTS_2_1.test_6_2_7.expect(case_01);
    }
}
