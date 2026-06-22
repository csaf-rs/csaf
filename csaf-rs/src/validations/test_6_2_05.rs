use crate::csaf::types::csaf_datetime::CsafDateTime::Valid;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_older_initial_release_date_error(
    initial_release_date: impl std::fmt::Display,
    earliest_rev_history_release_date: impl std::fmt::Display,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Initial release date '{initial_release_date}' is older than the earliest revision history date '{earliest_rev_history_release_date}'"
        ),
        instance_path: "/document/tracking/initial_release_date".to_string(),
    }
}

/// 6.2.5 Older Initial Release Date than Revision History
///
pub fn test_6_2_05_older_init_release_than_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let initial_release_date = doc.get_document().get_tracking().get_initial_release_date();
    let rev_history = doc.get_document().get_tracking().aggregate_revision_history();

    // Find the true oldest revision by iterating and finding the minimum UTC timestamp
    let mut earliest_rev_item = None;
    for item in rev_history.iter() {
        if let Valid(item_date) = &item.date {
            match earliest_rev_item {
                None => earliest_rev_item = Some((item, item_date.get_as_utc())),
                Some((_, current_min_utc)) => {
                    if item_date.get_as_utc() < current_min_utc {
                        earliest_rev_item = Some((item, item_date.get_as_utc()));
                    }
                },
            }
        }
    }

    // If there are no valid tracking revisions, we cannot perform this comparison
    let Some((earliest_item, _)) = earliest_rev_item else {
        return Ok(()); // TODO #409 return a precondition failed here
    };

    let Valid(initial_date_val) = initial_release_date else {
        return Ok(()); // TODO #409 return a precondition failed here
    };

    let Valid(earliest_rev_date_val) = &earliest_item.date else {
        return Ok(()); // TODO #409 return a precondition failed here
    };

    // Strict chronological check: error if initial release is strictly before the oldest revision
    if initial_date_val.get_as_utc() < earliest_rev_date_val.get_as_utc() {
        return Err(vec![create_older_initial_release_date_error(
            initial_date_val.get_raw_string(),
            earliest_rev_date_val.get_raw_string(),
        )]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_5, test_6_2_05_older_init_release_than_rev_history);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_05() {
        // CSAF 2.0 case 01
        TESTS_2_0
            .test_6_2_5
            .expect(Err(vec![create_older_initial_release_date_error(
                "2021-04-22T10:00:00.000Z",
                "2021-05-06T10:00:00.000Z",
            )]));

        // CSAF 2.1 cases
        TESTS_2_1.test_6_2_5.expect(
            Err(vec![create_older_initial_release_date_error(
                "2023-08-22T10:00:00.000Z",
                "2023-09-06T10:00:00.000Z",
            )]),
            Err(vec![create_older_initial_release_date_error(
                "2023-09-06T10:00:00.000+10:00",
                "2023-09-06T10:00:00.000-01:00",
            )]),
            Ok(()),
            Ok(()),
        );
    }
}
