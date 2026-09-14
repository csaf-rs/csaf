use crate::csaf::types::csaf_datetime::CsafDateTime::Valid;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_older_initial_release_date_error(
    initial_release_date: impl std::fmt::Display,
    earliest_rev_history_release_date: impl std::fmt::Display,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Initial release date '{initial_release_date}' is older than the earliest revision history date '{earliest_rev_history_release_date}'"
        ),
        instance_path: "/document/tracking/initial_release_date".to_string(),
    })
}

/// 6.2.5 Older Initial Release Date than Revision History
///
pub fn test_6_2_05_older_init_release_than_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let initial_release_date = doc.get_document().get_tracking().get_initial_release_date();
    // TODO: Check for invalid dates here, will be done after revision history refactor, which will introduce
    // generic parsing error handling

    let mut rev_history = doc.get_document().get_tracking().aggregate_revision_history();
    rev_history.inplace_sort_by_date_then_number();
    // We can safely unwrap here because empty revision histories would not parse schema validation
    let earliest_rev_history_item_date = match rev_history.first() {
        None => return Ok(()), // TODO #409 return a precondition failed here,
        Some(x) => x,
    };
    let Valid(initial_release_date) = initial_release_date else {
        return Ok(()); // TODO #409 return a precondition failed here,
    };
    let Valid(earliest_rev_history_item_date) = &earliest_rev_history_item_date.date else {
        return Ok(()); // TODO #409 return a precondition failed here,
    };
    if initial_release_date.get_as_utc() < earliest_rev_history_item_date.get_as_utc() {
        return Err(vec![create_older_initial_release_date_error(
            initial_release_date.get_raw_string(),
            earliest_rev_history_item_date.get_raw_string(),
        )]);
    }
    Ok(())
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_5, test_6_2_05_older_init_release_than_rev_history);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_5 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_5 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_05() {
        let initial_release_date_older_than_oldest_revision_2_0 = Err(vec![create_older_initial_release_date_error(
            "2021-04-22T10:00:00.000Z",
            "2021-05-06T10:00:00.000Z",
        )]);

        let initial_release_date_older_than_oldest_revision_2_1 = Err(vec![create_older_initial_release_date_error(
            "2023-08-22T10:00:00.000Z",
            "2023-09-06T10:00:00.000Z",
        )]);

        let initial_release_date_older_after_timezone_normalization =
            Err(vec![create_older_initial_release_date_error(
                "2023-09-06T10:00:00.000+10:00",
                "2023-09-06T10:00:00.000-01:00",
            )]);

        let initial_release_date_older_after_crossing_calendar_boundary =
            Err(vec![create_older_initial_release_date_error(
                "2024-01-01T00:00:00.000+01:30",
                "2023-12-31T23:00:00.000Z",
            )]);

        TESTS_2_0.test_6_2_5.expect(ExpectedResults_2_0 {
            case_01: initial_release_date_older_than_oldest_revision_2_0,
        });

        TESTS_2_1.test_6_2_5.expect(ExpectedResults_2_1 {
            case_01: initial_release_date_older_than_oldest_revision_2_1,
            case_02: initial_release_date_older_after_timezone_normalization,
            case_s01: initial_release_date_older_after_crossing_calendar_boundary,
            // initial_release_date equal to the oldest revision
            case_11: Ok(()),
            // initial_release_date equal to the oldest revision with a timezone offset
            case_12: Ok(()),
            // initial_release_date newer than the oldest revision
            case_s11: Ok(()),
            // initial_release_date equal to the oldest revision after normalizing different timezone offsets
            case_s12: Ok(()),
            // initial_release_date newer than the oldest revision selected timezone-aware independent of array order
            case_s13: Ok(()),
        });
    }
}
