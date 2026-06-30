use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_revision_history_error() -> ValidationError {
    ValidationError {
        message: "items must be in ascending order when sorted by `date` and `number`".to_string(),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

/// 6.1.14 Sorted Revision History
///
/// The revision history items, when sorted by their `/document/tracking/revision_history[]/date` field,
/// must be in the same order as when sorted by their `/document/tracking/revision_history[]/number` field.
/// If the version numbers are mixed between semantic versioning and non-semantic versioning, the non-semantic versioning numbers are interpreted as semantic versioning numbers.
pub fn test_6_1_14_sorted_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Generate tuples of (revision history path index, date, number)
    let mut rev_history_tuples_sort_by_date = doc.get_document().get_tracking().aggregate_revision_history();
    let mut rev_history_tuples_sort_by_number = rev_history_tuples_sort_by_date.clone();

    // Sort by date and by number
    rev_history_tuples_sort_by_date.inplace_sort_by_date_then_number();
    rev_history_tuples_sort_by_number.inplace_sort_by_number();

    // Generate errors if revision history items are sorted differently between sort by date and sort by number
    let mut errors = Vec::new();
    for i in 0..rev_history_tuples_sort_by_date.len() {
        if let Some(by_date) = rev_history_tuples_sort_by_date.get(i)
            && let Some(by_number) = rev_history_tuples_sort_by_number.get(i)
        {
            if by_date.date != by_number.date {
                errors.push(create_revision_history_error());
                break;
            }
        } else {
            unreachable!("Both arrays should have same length, this looks like a dev error.")
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_14, test_6_1_14_sorted_revision_history);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_14() {
        // Error cases
        let case_error = Err(vec![create_revision_history_error()]);

        // CSAF 2.0 has 17 test cases (01-08, 11-19)
        TESTS_2_0.test_6_1_14.expect(
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
            Ok(()), // supplementary case s11 mixed versioning
        );

        // CSAF 2.1 has 19 test cases (01-09, 11-19, 31)
        TESTS_2_1.test_6_1_14.expect(
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error.clone(),
            case_error,
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
            Ok(()), // supplementary case s11 mixed versioning
        );
    }
}
