use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait, VersionNumber};
use crate::validation::ValidationError;
use chrono::{DateTime, Utc};

pub fn test_6_1_14_sorted_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let revision_history = doc.get_document().get_tracking().get_revision_history();

    let mut path_date_number_vec: Vec<(usize, DateTime<Utc>, VersionNumber)> = Vec::new();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let date = DateTime::parse_from_rfc3339(revision.get_date()).map(|dt| dt.with_timezone(&Utc));
        if let Ok(date) = date {
            let rev_num = revision.get_number();
            path_date_number_vec.push((i_r, date, rev_num));
        }
    }
    path_date_number_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut path_date_number_vec_sorted_by_number = path_date_number_vec.clone();
    path_date_number_vec_sorted_by_number.sort_by(|a, b| a.2.cmp(&b.2));

    let mut errors = Vec::new();
    for i in 0..path_date_number_vec.len() {
        if path_date_number_vec[i].1 != path_date_number_vec_sorted_by_number[i].1 {
            errors.push(ValidationError {
                message: format!(
                    "Revision history is not sorted by date, revision with number {} is out of place",
                    path_date_number_vec[i].2
                )
                .to_string(),
                instance_path: format!("/document/tracking/revision_history/{}", path_date_number_vec[i].0),
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
