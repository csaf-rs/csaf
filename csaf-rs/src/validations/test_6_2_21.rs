use std::collections::HashMap;

use crate::csaf::types::csaf_datetime::ValidCsafDateTime;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_same_timestamp_error(
    index: usize,
    date: &ValidCsafDateTime,
    conflicting_indices: &[usize],
) -> ValidationError {
    // join the duplicate revision history date indices excluding the current one
    let conflicting_indices_not_current = conflicting_indices
        .iter()
        .filter(|idx| *idx != &index)
        .map(|idx| idx.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    ValidationError {
        message: format!(
            "The timestamp '{date}' of this revision history item is also used by item at the position(s) {conflicting_indices_not_current}."
        ),
        instance_path: format!("/document/tracking/revision_history/{index}/date"),
    }
}

/// 6.2.21 Same Timestamps in Revision History
///
/// It MUST be tested that the timestamps of all items in the revision history are pairwise disjoint,
/// taking timezones into account.
pub fn test_6_2_21_same_timestamps_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let revision_history = doc.get_document().get_tracking().get_revision_history_tuples();

    // lookup of ValidCsafDateTime (hash function uses normalized utc)
    // to a vec containing each occurrence of that normalized utc
    // with its path index and "original" ValidCsafDateTime to preserve timezones etc.
    let mut datetime_path_lookup: HashMap<&ValidCsafDateTime, Vec<(usize, &ValidCsafDateTime)>> = HashMap::new();
    let mut errors: Option<Vec<ValidationError>> = None;

    // does the lookup already contain the datetime
    for item in &revision_history {
        match datetime_path_lookup.get_mut(&item.valid_date) {
            // push the path and original datetime into the vec
            Some(entries) => {
                entries.push((item.path_index, &item.valid_date));
            },
            // create a vec with this path and datetime
            None => {
                datetime_path_lookup.insert(&item.valid_date, vec![(item.path_index, &item.valid_date)]);
            },
        }
    }

    // filter out all date times that appeared more than once, generate an error for each
    for (_, entries) in datetime_path_lookup.iter().filter(|(_, entries)| entries.len() > 1) {
        let indices: Vec<usize> = entries.iter().map(|(idx, _)| *idx).collect();
        for (index, original_datetime) in entries.iter() {
            errors
                .get_or_insert_default()
                .push(create_same_timestamp_error(*index, original_datetime, &indices));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_21,
    test_6_2_21_same_timestamps_in_revision_history
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_21() {
        let date_01 = ValidCsafDateTime::from_str("2024-01-21T10:00:00.000Z").unwrap();
        let conflicting_indices_01: &[usize] = &[0, 1];
        let case_01_two_items_with_same_date = Err(vec![
            create_same_timestamp_error(0, &date_01, conflicting_indices_01),
            create_same_timestamp_error(1, &date_01, conflicting_indices_01),
        ]);

        let date_02a = ValidCsafDateTime::from_str("2024-01-21T10:00:00.000Z").unwrap();
        let date_02b = ValidCsafDateTime::from_str("2024-01-22T11:00:00.000Z").unwrap();
        let conflicting_indices_02a: &[usize] = &[0, 4];
        let conflicting_indices_02b: &[usize] = &[1, 3, 5];
        let case_02_two_groups_with_same_date = Err(vec![
            create_same_timestamp_error(0, &date_02a, conflicting_indices_02a),
            create_same_timestamp_error(4, &date_02a, conflicting_indices_02a),
            create_same_timestamp_error(1, &date_02b, conflicting_indices_02b),
            create_same_timestamp_error(3, &date_02b, conflicting_indices_02b),
            create_same_timestamp_error(5, &date_02b, conflicting_indices_02b),
        ]);

        let conflicting_indices_03: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7];
        let case_03 = Err(vec![
            create_same_timestamp_error(
                0,
                &ValidCsafDateTime::from_str("2024-01-21T10:00:00.000Z").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                1,
                &ValidCsafDateTime::from_str("2024-01-21T11:00:00.000+01:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                2,
                &ValidCsafDateTime::from_str("2024-01-21T20:00:00.000+10:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                3,
                &ValidCsafDateTime::from_str("2024-01-21T05:00:00.000-05:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                4,
                &ValidCsafDateTime::from_str("2024-01-21T13:00:00.000+03:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                5,
                &ValidCsafDateTime::from_str("2024-01-21T07:00:00.000-03:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                6,
                &ValidCsafDateTime::from_str("2024-01-21T00:00:00.000-10:00").unwrap(),
                conflicting_indices_03,
            ),
            create_same_timestamp_error(
                7,
                &ValidCsafDateTime::from_str("2024-01-22T00:00:00.000+14:00").unwrap(),
                conflicting_indices_03,
            ),
        ]);

        let conflicting_indices_04: &[usize] = &[0, 1];
        let case_04_subsecond_precision = Err(vec![
            create_same_timestamp_error(
                0,
                &ValidCsafDateTime::from_str("2024-01-21T10:00:00.000000Z").unwrap(),
                conflicting_indices_04,
            ),
            create_same_timestamp_error(
                1,
                &ValidCsafDateTime::from_str("2024-01-21T10:00:00.000Z").unwrap(),
                conflicting_indices_04,
            ),
        ]);

        let conflicting_indices_05: &[usize] = &[0, 1];
        let case_05_empty_timezone_expr = Err(vec![
            create_same_timestamp_error(
                0,
                &ValidCsafDateTime::from_str("2024-01-21T10:00:00.000+00:00").unwrap(),
                conflicting_indices_05,
            ),
            create_same_timestamp_error(
                1,
                &ValidCsafDateTime::from_str("2024-01-21T10:00:00.000Z").unwrap(),
                conflicting_indices_05,
            ),
        ]);

        // Cases 11-13: Valid (all timestamps are distinct, with subsecond precision and timezones)
        TESTS_2_1.test_6_2_21.expect(
            case_01_two_items_with_same_date,
            case_02_two_groups_with_same_date,
            case_03,
            case_04_subsecond_precision,
            case_05_empty_timezone_expr,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
