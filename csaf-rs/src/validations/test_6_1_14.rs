use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait, RevisionHistorySortable};
use crate::validation::ValidationError;


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
            errors.push(ValidationError {
                message: format!(
                    "Revision history is not sorted by date, revision with number {} is out of place",
                    rev_history_tuples_sort_by_date[i].number
                ),
                instance_path: format!(
                    "/document/tracking/revision_history/{}",
                    rev_history_tuples_sort_by_date[i].path_index
                ),
            });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_14::test_6_1_14_sorted_revision_history;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_14() {
        let errors = HashMap::from([
            (
                "01",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                ],
            ),
            (
                "02",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                ],
            ),
            (
                "03",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                ],
            ),
            (
                "04",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2.0.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1.0.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                ],
            ),
            (
                "05",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2.0.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1.0.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                ],
            ),
            (
                "06",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 10 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/9".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 9 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/8".to_string(),
                    },
                ],
            ),
            (
                "07",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1.10.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/10".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1.9.0 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/9".to_string(),
                    },
                ],
            ),
            (
                "08",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                ],
            ),
            (
                "09",
                vec![
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 2 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/0".to_string(),
                    },
                    ValidationError {
                        message: "Revision history is not sorted by date, revision with number 1 is out of place"
                            .to_string(),
                        instance_path: "/document/tracking/revision_history/1".to_string(),
                    },
                ],
            ),
        ]);
        run_csaf20_tests("14", test_6_1_14_sorted_revision_history, errors.clone());
        run_csaf21_tests("14", test_6_1_14_sorted_revision_history, errors);
    }
}
