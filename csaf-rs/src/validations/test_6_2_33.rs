use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use chrono::Utc;

fn create_disclosure_date_newer_than_revision_error(
    disclosure_date: &ValidCsafDateTime,
    newest_revision_date: &ValidCsafDateTime,
    i_v: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Disclosure date ({disclosure_date}) is in the past and newer than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/disclosure_date"),
    }
}

/// 6.2.33 Disclosure Date Newer than Revision History
///
/// For each vulnerability, it MUST be tested that the `disclosure_date` is earlier or equal to the `date`
/// of the newest item of the `revision_history` if the `disclosure_date` is in the past at the time of
/// the test execution. As the timestamps might use different timezones, the sorting MUST take timezones
/// into account.
///
/// The result of the test is dependent upon the time of the execution of the test - it might change for
/// a given CSAF document over time. However, the latest version of a CSAF document should always pass
/// the test.
///
/// Unlike test 6.1.45, this test applies regardless of TLP/status, and additionally takes the datetime of test
/// execution into consideration.
pub fn test_6_2_33_disclosure_date_newer_than_revision(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Get sorted revision history and find the newest entry
    let mut revision_history = doc.get_document().get_tracking().get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let newest_revision_date = match revision_history.last() {
        Some(rev) => &rev.valid_date,
        None => return Ok(()), // TODO this should be a #409 precondition failed
    };

    let now = Utc::now();

    // Check each vulnerability's disclosure date
    let mut errors: Option<Vec<ValidationError>> = None;
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // If there is a disclosure date
        if let Some(disclosure_date) = vulnerability.get_disclosure_date() {
            // If the disclosure date is valid
            match disclosure_date {
                CsafDateTime::Valid(valid_disclosure_date) => {
                    // Check if the disclosure_date is in the past
                    // if so, check if the disclosure date is newer than the newest revision date
                    // if so, return an error
                    if valid_disclosure_date.get_as_utc() <= now && &valid_disclosure_date > newest_revision_date {
                        errors
                            .get_or_insert_default()
                            .push(create_disclosure_date_newer_than_revision_error(
                                &valid_disclosure_date,
                                newest_revision_date,
                                i_v,
                            ));
                    }
                }
                CsafDateTime::Invalid(_) => {
                    // TODO: This will be a NonDeterminable (#409) later
                },
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_33,
    test_6_2_33_disclosure_date_newer_than_revision
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_33() {
        let case_01_disclosure_date_newer_than_newest_rev = Err(vec![create_disclosure_date_newer_than_revision_error(
            &ValidCsafDateTime::from_str("2024-02-24T10:00:00.000Z").unwrap(),
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
            0,
        )]);

        let case_02_disclosure_date_newer_than_newest_rev_with_timezone = Err(vec![create_disclosure_date_newer_than_revision_error(
            &ValidCsafDateTime::from_str("2024-02-23T14:00:00.000-21:00").unwrap(),
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
            0,
        )]);

        let case_03_disclosure_date_newer_than_newest_rev_with_timezone = Err(vec![create_disclosure_date_newer_than_revision_error(
            &ValidCsafDateTime::from_str("2024-02-24T14:00:00.000-07:00").unwrap(),
            &ValidCsafDateTime::from_str("2024-02-24T10:00:00.000Z").unwrap(),
            0,
        )]);

        // Case 11: disclosure_date equals newest revision date
        // Case 12: disclosure_date is definitely not in the past (9999-12-31)
        // Case 13: disclosure_date is earlier than newest revision, with timezones

        TESTS_2_1.test_6_2_33.expect(
            case_01_disclosure_date_newer_than_newest_rev,
            case_02_disclosure_date_newer_than_newest_rev_with_timezone,
            case_03_disclosure_date_newer_than_newest_rev_with_timezone,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}

