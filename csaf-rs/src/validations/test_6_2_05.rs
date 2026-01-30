use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_dates::ValidatedRevisionHistoryDates;
use crate::csaf::types::csaf_datetime::CsafDateTime;
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
    let tracking = doc.get_document().get_tracking();

    // get initial release date
    let initial_release_date = match tracking.get_initial_release_date() {
        CsafDateTime::Valid(initial_release_date) => initial_release_date,
        // if initial release date is invalid, return an error and skip this test
        CsafDateTime::Invalid(err) => return Err(vec![err.get_validation_error("/document/tracking/initial_release_date")]),
    };

    // get revision history dates
    let revision_history = tracking.get_revision_history();
    let mut revision_history_dates = match ValidatedRevisionHistoryDates::from(&revision_history) {
        ValidatedRevisionHistoryDates::Valid(dates) => dates,
        ValidatedRevisionHistoryDates::Invalid(err) => {return Err(err.into())}
    };

    // sort revision history
    revision_history_dates.sort();
    let oldest_revision_history_date = revision_history_dates.get_oldest().date_time;

    // if initial release date is older than the oldest revision history date, return an error
    if &initial_release_date < oldest_revision_history_date {
        return Err(vec![create_older_initial_release_date_error(
            initial_release_date.get_raw_string(),
            oldest_revision_history_date.get_raw_string()
        )]);
    }
    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_05_older_init_release_than_rev_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_05_older_init_release_than_rev_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_05() {
        // Both CSAF 2.0 and 2.1 have test cases
        TESTS_2_0
            .test_6_2_5
            .expect(Err(vec![create_older_initial_release_date_error(
                "2021-04-22T10:00:00.000Z",
                "2021-05-06T10:00:00.000Z",
            )]));
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
