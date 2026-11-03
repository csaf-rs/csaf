use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait, TrackingTrait, WithDate};
use crate::schema::csaf2_1::schema::ActionStatus;
use crate::validation::{TestFinding, TestFindingData};

fn create_future_action_date_warning(
    action_date: &ValidCsafDateTime,
    newest_revision_date: &ValidCsafDateTime,
    action_index: usize,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Action date `{action_date}` is newer than the newest revision date `{newest_revision_date}`"),
        instance_path: format!("/document/involvement/actions/{action_index}/date"),
    })
}

/// 6.2.56.5 Inconsistent Future Action Date
///
/// For each item in `$.document.involvement.actions` with one the `status` values `attempted`, `completed` or `discontinued`
/// it SHALL be tested that the `date` of the action is earlier than or equal to the newest item of the `revision_history`.
/// As the timestamps might use different timezones, the sorting and comparison SHALL take timezones into account.
pub fn test_6_2_56_5_inconsistent_future_action_date(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    // check if involvement is present, if not, skip the test
    let Some(involvement) = document.get_involvement() else {
        return Ok(()); // TODO: wasSkipped (#409)
    };

    // get most recent revision date, ignoring invalid dates and revision item version numbers
    let revision_history = document.get_tracking().aggregate_revision_history();
    let newest_revision_date = revision_history
        .iter()
        .filter_map(|revision| match &revision.date {
            CsafDateTime::Valid(date) => Some(date),
            CsafDateTime::Invalid(_) => None, // TODO: This should result in a precondition finding (#409)
        })
        .max();

    // if there is no valid revision date, skip the test
    let Some(newest_revision_date) = newest_revision_date else {
        return Ok(()); // TODO: non-determinable finding (#409)
    };

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each action
    for (action_index, action) in involvement.get_actions().enumerate() {
        // check if the status is one of the relevant ones
        if !matches!(
            action.get_status(),
            ActionStatus::Attempted | ActionStatus::Completed | ActionStatus::Discontinued
        ) {
            continue;
        }

        // check if the action date is valid
        let action_date = match action.get_date() {
            CsafDateTime::Valid(date) => date,
            CsafDateTime::Invalid(_) => continue, // TODO: non-determinable finding (#409)
        };

        // check if the action date is newer than the newest revision date, if so, add a finding
        if &action_date > newest_revision_date {
            findings.get_or_insert_default().push(create_future_action_date_warning(
                &action_date,
                newest_revision_date,
                action_index,
            ));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_56_5,
    test_6_2_56_5_inconsistent_future_action_date
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_datetime::ValidCsafDateTime;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_56_5 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_56_5() {
        TESTS_2_1.test_6_2_56_5.expect(ExpectedResults {
            // TODO: Improve test coverage, so far only status completed is tested, there are no tests
            // for attempted and discontinued, also no passing tests for a non-relevant status
            // TODO: Multiple warnings / findings, mixed between passing and failing
            // TODO: no unsorted revision histories
            // TODO: Trivial passes: no involvement
            // TODO: Further: Handling of really broken revision history (no valid dates)

            // Case 01: 1 rev history item, earlier than action date
            // Case 02: 2 rev history items, timezones, earlier than action date
            // Case 11: equal dates
            // Case 12: action date earlier than rev history date, timezones
            case_01: Err(vec![create_future_action_date_warning(
                &ValidCsafDateTime::from_str("2024-03-02T15:00:00Z").unwrap(),
                &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
                2,
            )]),
            case_02: Err(vec![create_future_action_date_warning(
                &ValidCsafDateTime::from_str("2024-03-02T10:00:00-02:29").unwrap(),
                &ValidCsafDateTime::from_str("2024-03-02T10:00:00+02:30").unwrap(),
                2,
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
