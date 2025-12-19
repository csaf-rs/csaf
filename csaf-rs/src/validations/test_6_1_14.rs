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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_14() {
        let errors = HashMap::from([
            (
                "01",
                vec![
                    create_revision_history_error("2", 0),
                    create_revision_history_error("1", 1),
                ],
            ),
            (
                "02",
                vec![
                    create_revision_history_error("2", 0),
                    create_revision_history_error("1", 1),
                ],
            ),
            (
                "03",
                vec![
                    create_revision_history_error("2", 1),
                    create_revision_history_error("1", 0),
                ],
            ),
            (
                "04",
                vec![
                    create_revision_history_error("2.0.0", 0),
                    create_revision_history_error("1.0.0", 1),
                ],
            ),
            (
                "05",
                vec![
                    create_revision_history_error("2.0.0", 0),
                    create_revision_history_error("1.0.0", 1),
                ],
            ),
            (
                "06",
                vec![
                    create_revision_history_error("10", 9),
                    create_revision_history_error("9", 8),
                ],
            ),
            (
                "07",
                vec![
                    create_revision_history_error("1.10.0", 10),
                    create_revision_history_error("1.9.0", 9),
                ],
            ),
            (
                "08",
                vec![
                    create_revision_history_error("2", 1),
                    create_revision_history_error("1", 0),
                ],
            ),
            (
                "09",
                vec![
                    create_revision_history_error("2", 0),
                    create_revision_history_error("1", 1),
                ],
            ),
        ]);
        run_csaf20_tests("14", test_6_1_14_sorted_revision_history, errors.clone());
        run_csaf21_tests("14", test_6_1_14_sorted_revision_history, errors);
    }
}
