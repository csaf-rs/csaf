use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use crate::version_number::{CsafVersionNumber, VersionNumber};
use std::collections::HashMap;

fn generate_duplicate_revision_error(number: &VersionNumber, path: &usize) -> ValidationError {
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
    let revision_history = doc.get_document().get_tracking().get_revision_history();

    let mut errors: Option<Vec<ValidationError>> = None;
    // Map occurrence paths indexes to revision numbers
    let mut number_revision_index_map: HashMap<VersionNumber, Vec<usize>> = HashMap::new();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let number = match revision.get_number() {
            CsafVersionNumber::Valid(number) => number,
            CsafVersionNumber::Invalid(err) => {
                errors.get_or_insert_default().push(
                    err.get_validation_error(format!("/document/tracking/revision_history/{i_r}/number").as_str()),
                );
                continue;
            },
        };
        let path = number_revision_index_map.entry(number.clone()).or_default();
        path.push(i_r);
    }

    // Generate errors for revision numbers with multiple occurrence paths indexes
    for (number, paths) in &number_revision_index_map {
        if paths.len() > 1 {
            errors.get_or_insert_default().extend(
                paths
                    .iter()
                    .map(|revision_index| generate_duplicate_revision_error(number, revision_index)),
            );
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
            generate_duplicate_revision_error(&VersionNumber::from_str("1").unwrap(), &0),
            generate_duplicate_revision_error(&VersionNumber::from_str("1").unwrap(), &1),
        ]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_22.expect(case_01.clone());
        TESTS_2_1.test_6_1_22.expect(case_01);
    }
}
