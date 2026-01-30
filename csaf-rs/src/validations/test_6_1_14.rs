use std::ops::Deref;
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history::{TypedValidCsafRevisionHistory, ValidRevisionHistory, ValidatedRevisionHistory, VersionNumberKind};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_revision_history_error(revision_number: impl std::fmt::Display, path_index: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Revision history is not sorted by date, revision with number {revision_number} is out of place"
        ),
        instance_path: format!("/document/tracking/revision_history/{path_index}"),
    }
}

/// 6.1.14 Sorted Revision History
///
/// The revision history items, when sorted by their `/document/tracking/revision_history[]/date` field,
/// must be in the same order as when sorted by their `/document/tracking/revision_history[]/number` field.
pub fn test_6_1_14_sorted_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Get the revision history
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    let validated = ValidatedRevisionHistory::from(&revision_history);
    // Check if revision history is valid, if not return the errors and skip this test
    let valid = match validated {
        ValidatedRevisionHistory::Valid(valid) => {valid}
        ValidatedRevisionHistory::Invalid(errors) => {return Err(errors.into())}
    };

    match valid {
        ValidRevisionHistory::IntVer(intver) => {
            check_for_sorting_errors(intver)
        }
        ValidRevisionHistory::SemVer(semver) => {
            check_for_sorting_errors(semver)
        }
    }
}

// not getting the inner type is what we want here, as we want to compare the ptrs
#[allow(clippy::suspicious_double_ref_op)]
fn check_for_sorting_errors<V: VersionNumberKind>(history: TypedValidCsafRevisionHistory<V>) -> Result<(), Vec<ValidationError>> {
    let sorted_by_date_by_number = history.get_sorted_by_date_by_number();
    let sorted_by_number = history.get_sorted_by_number();
    let mut errors :Option<Vec<ValidationError>> = None;

    // Generate errors if revision history items are sorted differently between sort by date and sort by number
    for (idx, by_date_by_number ) in sorted_by_date_by_number.iter().enumerate() {
        let by_date_by_number = by_date_by_number.deref();
        let by_number = &sorted_by_number[idx];
        // Compare the two references, if they point to different items, it means that the order is different
        if !std::ptr::eq(by_date_by_number, by_number) {
            errors.get_or_insert_default().push(create_revision_history_error(
                by_date_by_number.number,
                by_date_by_number.path_index,
            ));
        }
    }
    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_14
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_14_sorted_revision_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_14
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_14_sorted_revision_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_14() {
        // Error cases
        let case_01 = Err(vec![
            create_revision_history_error("2", 0),
            create_revision_history_error("1", 1),
        ]);
        let case_02 = Err(vec![
            create_revision_history_error("2", 0),
            create_revision_history_error("1", 1),
        ]);
        let case_03 = Err(vec![
            create_revision_history_error("2", 1),
            create_revision_history_error("1", 0),
        ]);
        let case_04 = Err(vec![
            create_revision_history_error("2.0.0", 0),
            create_revision_history_error("1.0.0", 1),
        ]);
        let case_05 = Err(vec![
            create_revision_history_error("2.0.0", 0),
            create_revision_history_error("1.0.0", 1),
        ]);
        let case_06 = Err(vec![
            create_revision_history_error("10", 9),
            create_revision_history_error("9", 8),
        ]);
        let case_07 = Err(vec![
            create_revision_history_error("1.10.0", 10),
            create_revision_history_error("1.9.0", 9),
        ]);
        let case_08 = Err(vec![
            create_revision_history_error("2", 1),
            create_revision_history_error("1", 0),
        ]);

        // CSAF 2.0 has 17 test cases (01-08, 11-19)
        TESTS_2_0.test_6_1_14.expect(
            case_01.clone(),
            case_02.clone(),
            case_03.clone(),
            case_04.clone(),
            case_05.clone(),
            case_06.clone(),
            case_07.clone(),
            case_08.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
        );

        // CSAF 2.1 has 19 test cases (01-09, 11-19, 31)
        TESTS_2_1.test_6_1_14.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            case_07,
            case_08,
            Err(vec![
                create_revision_history_error("2", 0),
                create_revision_history_error("1", 1),
            ]),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
            Ok(()), // case_31
        );
    }
}
