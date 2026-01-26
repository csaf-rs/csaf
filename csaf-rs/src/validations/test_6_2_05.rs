use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
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
    // TODO: Check for invalid dates here, will be done after revision history refactor, which will introduce
    // generic parsing error handling

    let mut rev_history = doc.get_document().get_tracking().get_revision_history_tuples();
    rev_history.inplace_sort_by_date_then_number();
    // We can safely unwrap here because empty revision histories would not parse schema validation
    let earliest_rev_history_item_date = rev_history.first().unwrap();
    if initial_release_date.get_as_utc().unwrap() < earliest_rev_history_item_date.date {
        return Err(vec![create_older_initial_release_date_error(
            initial_release_date.get_str(),
            &earliest_rev_history_item_date.date_string,
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
