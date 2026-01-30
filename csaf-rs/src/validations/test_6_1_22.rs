use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_revision_error(number: &ValidVersionNumber, path: &usize) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition of revision history number {number}"),
        instance_path: format!("/document/tracking/revision_history/{path}/number"),
    }
}

/// Test 6.1.22: Multiple Definition in Revision History
///
/// Items of the revision history must not contain the same string in the
/// `/document/tracking/revision_history[]/number` field.
pub fn test_6_1_22_multiple_definition_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    // Map of VersionNumbers to the revision history indices that have that version number
    let mut number_revision_index_map: Option<HashMap<ValidVersionNumber, Vec<usize>>> = None;
    for item in doc.get_document().get_tracking().get_revision_history() {
        match item.number {
            CsafVersionNumber::Invalid(err) => {
                // if number is invalid, add an error
                errors.get_or_insert_default().push(err.get_validation_error(format!("/document/tracking/revision_history/{}/number", item.path_index).as_str()));
            }
            CsafVersionNumber::Valid(number) => {
                // if number is valid, add the revision history index to the map for that version number
                number_revision_index_map.get_or_insert_default().entry(
                    number
                ).or_default().push(item.path_index);
            }
        }
    }

    // If there have been any valid numbers and the map is therefore not empty
    if let Some(number_revision_index_map) = number_revision_index_map {
        // Check for each version number if there are multiple revision history items with that version number
        for (number, revision_indices) in &number_revision_index_map {
            if revision_indices.len() > 1 {
                // if there are multiple revision history items with the same version number, add an error for each of them
                errors.get_or_insert_default().extend(
                    revision_indices
                        .iter()
                        .map(|revision_index| generate_duplicate_revision_error(number, revision_index)),
                );
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_22
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_22_multiple_definition_in_revision_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_22
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_22_multiple_definition_in_revision_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_22() {
        // TODO: Add unit test for semver, more than two duplicates, invalid number
        let case_01 = Err(vec![
            generate_duplicate_revision_error(&ValidVersionNumber::from_str("1").unwrap(), &0),
            generate_duplicate_revision_error(&ValidVersionNumber::from_str("1").unwrap(), &1),
        ]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_22.expect(case_01.clone());
        TESTS_2_1.test_6_1_22.expect(case_01);
    }
}
