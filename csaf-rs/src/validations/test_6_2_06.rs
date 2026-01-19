use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::validation::ValidationError;

fn create_older_current_release_date_error(
    current_release_date: impl std::fmt::Display,
    newest_rev_history_release_date: impl std::fmt::Display,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Current release date '{}' is older than the newest revision history date '{}'",
            current_release_date, newest_rev_history_release_date
        ),
        instance_path: "/document/tracking/current_release_date".to_string(),
    }
}

/// 6.2.5 Older Current Release Date than Revision History
///
pub fn test_6_2_06_older_current_release_than_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let current_release_date = doc.get_document().get_tracking().get_current_release_date();

    let mut rev_history = doc.get_document().get_tracking().get_revision_history_tuples();
    rev_history.inplace_sort_by_date_then_number();
    // We can safely unwrap here because empty revision histories would not parse schema validation
    let newest_rev_history_item_date = rev_history.last().unwrap();
    if current_release_date < newest_rev_history_item_date.date {
        return Err(vec![create_older_current_release_date_error(
            doc.get_document().get_tracking().get_current_release_date_string(),
            &newest_rev_history_item_date.date_string,
        )]);
    }
    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_06_older_current_release_than_rev_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_06_older_current_release_than_rev_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_06() {
        // Both CSAF 2.0 and 2.1 have test cases
        TESTS_2_0
            .test_6_2_6
            .expect(Err(vec![create_older_current_release_date_error(
                "2021-05-06T10:00:00.000Z",
                "2021-07-21T11:00:00.000Z",
            )]));
        TESTS_2_1.test_6_2_6.expect(
            Err(vec![create_older_current_release_date_error(
                "2023-09-06T10:00:00.000Z",
                "2024-01-21T11:00:00.000Z",
            )]),
            Err(vec![create_older_current_release_date_error(
                "2024-01-21T11:00:00.000+11:00",
                "2024-01-21T11:00:00.000+10:00",
            )]),
            Err(vec![create_older_current_release_date_error(
                "2024-01-22T11:00:00.000+11:30",
                "2024-01-21T13:00:00.000-11:30",
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
