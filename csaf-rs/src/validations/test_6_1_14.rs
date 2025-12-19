use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::validation::ValidationError;

fn create_revision_history_error(revision_number: impl std::fmt::Display, path_index: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Revision history is not sorted by date, revision with number {} is out of place",
            revision_number
        ),
        instance_path: format!("/document/tracking/revision_history/{}", path_index),
    }
}

/// 6.1.14 Sorted Revision History
///
/// The revision history items, when sorted by their `/document/tracking/revision_history[]/date` field,
/// must be in the same order as when sorted by their `/document/tracking/revision_history[]/number` field.
pub fn test_6_1_14_sorted_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Generate tuples of (revision history path index, date, number)
    let mut rev_history_tuples_sort_by_date = doc.get_document().get_tracking().get_revision_history_tuples();
    let mut rev_history_tuples_sort_by_number = rev_history_tuples_sort_by_date.clone();

    // Sort by date and by number
    rev_history_tuples_sort_by_date.inplace_sort_by_date_then_number();
    rev_history_tuples_sort_by_number.inplace_sort_by_number();

    // Generate errors if revision history items are sorted differently between sort by date and sort by number
    let mut errors = Vec::new();
    for i in 0..rev_history_tuples_sort_by_date.len() {
        if rev_history_tuples_sort_by_date[i].date != rev_history_tuples_sort_by_number[i].date {
            errors.push(create_revision_history_error(
                &rev_history_tuples_sort_by_date[i].number,
                rev_history_tuples_sort_by_date[i].path_index,
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
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
        let case_09 = Err(vec![
            create_revision_history_error("2", 0),
            create_revision_history_error("1", 1),
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
            case_09,
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
