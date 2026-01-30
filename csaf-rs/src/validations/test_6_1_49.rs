use std::fmt::Display;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_dates::ValidatedRevisionHistoryDates;

fn create_ssvc_timestamp_too_late_error(
    ssvc_timestamp: impl Display,
    i_v: usize,
    newest_revision_date: impl Display,
    i_m: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "SSVC timestamp ({ssvc_timestamp}) for vulnerability at index {i_v} is later than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/timestamp"),
    }
}

fn create_invalid_ssvc_error(error: impl Display, i_v: usize, i_m: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {error}"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2"),
    }
}

/// 6.1.49 Inconsistent SSVC Timestamp
///
/// For each vulnerability, it is tested that the SSVC `timestamp` is earlier or equal to the `date`
/// of the newest item in the `revision_history` if the document status is `final` or `interim`.
pub fn test_6_1_49_inconsistent_ssvc_timestamp(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Check if the document status is "final" or "interim"
    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    // Get valid revision history dates, if there are invalid dates, return the errors and skip this test
    // As correct sorting can not be guaranteed if there are invalid dates
    let revision_history = tracking.get_revision_history();
    let mut valid_revision_dates = match ValidatedRevisionHistoryDates::from(&revision_history) {
        ValidatedRevisionHistoryDates::Invalid(err) => {return Err(err.into())}
        ValidatedRevisionHistoryDates::Valid(dates) => {dates}
    };

    // sort the valid revision dates and get the newest one
    valid_revision_dates.sort();
    let newest_revision_date = valid_revision_dates.get_newest().date_time;


    // Check each vulnerability's SSVC timestamp
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                if metric.get_content().has_ssvc() {
                    match metric.get_content().get_ssvc() {
                        Ok(ssvc) => {
                            if ssvc.timestamp > newest_revision_date.get_as_utc() {
                                return Err(vec![create_ssvc_timestamp_too_late_error(
                                    ssvc.timestamp.fixed_offset(),
                                    i_v,
                                    newest_revision_date.get_as_fixed_offset(),
                                    i_m,
                                )]);
                            }
                        },
                        Err(err) => {
                            return Err(vec![create_invalid_ssvc_error(err, i_v, i_m)]);
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_49
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_49_inconsistent_ssvc_timestamp(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_49() {
        // Only CSAF 2.1 has this test with 6 test cases (3 error cases, 3 success cases)
        TESTS_2_1.test_6_1_49.expect(
            Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-07-13 10:00:00 +00:00",
                0,
                "2024-01-24 10:00:00 +00:00",
                0,
            )]),
            Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29 10:30:00 +00:00",
                0,
                "2024-02-29 10:00:00 +00:00",
                0,
            )]),
            Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29 10:30:00 +00:00",
                0,
                "2024-02-29 10:00:00 +00:00",
                0,
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
