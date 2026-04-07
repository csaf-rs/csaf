use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_revision_error(number: &CsafVersionNumber, path: &usize) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition of revision history number {number}"),
        instance_path: format!("/document/tracking/revision_history/{path}/number"),
    }
}

/// Test 6.1.22: Multiple Definition in Revision History
///
/// Items of the revision history must not contain the same value in the
/// `/document/tracking/revision_history[]/number` field.
pub fn test_6_1_22_multiple_definition_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let revision_history = doc.get_document().get_tracking().get_revision_history();

    let mut errors: Option<Vec<ValidationError>> = None;
    // Map occurrence paths indexes to revision numbers
    let mut number_revision_index_map: HashMap<CsafVersionNumber, Vec<usize>> = HashMap::new();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let number = revision.get_number();
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

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_22,
    test_6_1_22_multiple_definition_in_revision_history
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_22() {
        // TODO: Add unit test for more than two duplicates
        let case_intver = Err(vec![
            generate_duplicate_revision_error(&CsafVersionNumber::from("1"), &0),
            generate_duplicate_revision_error(&CsafVersionNumber::from("1"), &1),
        ]);
        let case_intver_double_duplicates = Err(vec![
            generate_duplicate_revision_error(&CsafVersionNumber::from("1"), &0),
            generate_duplicate_revision_error(&CsafVersionNumber::from("2"), &1),
            generate_duplicate_revision_error(&CsafVersionNumber::from("1"), &2),
            generate_duplicate_revision_error(&CsafVersionNumber::from("2"), &3),
        ]);
        let case_semver = Err(vec![
            generate_duplicate_revision_error(&CsafVersionNumber::from("1.0.0"), &0),
            generate_duplicate_revision_error(&CsafVersionNumber::from("1.0.0"), &1),
        ]);
        let case_semver_double_duplicates = Err(vec![
            generate_duplicate_revision_error(&CsafVersionNumber::from("1.0.0"), &0),
            generate_duplicate_revision_error(&CsafVersionNumber::from("2.0.0"), &1),
            generate_duplicate_revision_error(&CsafVersionNumber::from("1.0.0"), &2),
            generate_duplicate_revision_error(&CsafVersionNumber::from("2.0.0"), &3),
        ]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_22.expect(
            case_intver.clone(),
            case_semver.clone(),
            case_intver_double_duplicates.clone(),
            case_semver_double_duplicates.clone(),
        );
        TESTS_2_1.test_6_1_22.expect(
            case_intver,
            case_semver,
            case_intver_double_duplicates,
            case_semver_double_duplicates,
        );
    }
}
