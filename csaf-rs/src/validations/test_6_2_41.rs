use crate::csaf::macros::skip_if_document_status_is_not::skip_if_document_status_is_not;
use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, EpssTrait, MetricTrait, RevisionHistorySortable, TrackingTrait,
    VulnerabilityTrait,
};
use crate::validation::ValidationError;
use chrono::TimeDelta;

fn create_old_epss_timestamp_error(
    epss_timestamp: &ValidCsafDateTime,
    newest_revision_date: &ValidCsafDateTime,
    content_json_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "EPSS timestamp ({epss_timestamp}) is more than 15 days older than the newest revision date ({newest_revision_date}).",
        ),
        instance_path: format!("{content_json_path}/epss/timestamp"),
    }
}

/// 6.2.41 Old EPSS Timestamp
///
/// For each vulnerability, it MUST be tested that the youngest EPSS timestamp is not more than
/// 15 days older than the date of the newest item of the revision_history (taking timezones into account),
/// if the document status is `final` or `interim`.
pub fn test_6_2_41_old_epss_timestamp(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();

    // Only check for final or interim documents
    skip_if_document_status_is_not!(tracking.get_status(), Final, Interim);

    // Get sorted revision history and find the newest entry
    let mut revision_history = tracking.get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let newest_revision = match revision_history.last() {
        Some(rev) => rev,
        None => return Ok(()), // TODO this should be a #409 precondition failed
    };

    let vulnerabilities = doc.get_vulnerabilities();
    if vulnerabilities.is_empty() {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    let fifteen_days = TimeDelta::days(15);

    for (i_v, vulnerability) in vulnerabilities.iter().enumerate() {
        // Find the newest EPSS timestamp across all metrics
        if let Some(metrics) = vulnerability.get_metrics() {
            // the newest datetime, the metric index where it was encountered
            let mut newest_epss: Option<(ValidCsafDateTime, usize)> = None;

            for (i_m, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                // if the metric contains epss
                if let Some(epss) = content.get_epss() {
                    // if the timestamp is valid
                    match epss.get_timestamp() {
                        CsafDateTime::Valid(valid_timestamp) => {
                            // set newest_epss if is still none or
                            // replace newest_epss if the encountered timestamp is newer
                            if newest_epss.as_ref().is_none_or(|(prev, _)| valid_timestamp > *prev) {
                                newest_epss = Some((valid_timestamp, i_m));
                            }
                        },
                        CsafDateTime::Invalid(_) => {
                            // TODO: This will be a NonDeterminable (#409) later
                        },
                    }
                }
            }

            // if there was an epss metric
            if let Some((newest_epss_timestamp, path_metric_idx)) = newest_epss {
                // if it is 15 days older than the newest revision date, add an error
                let diff = newest_revision.valid_date.get_as_utc() - newest_epss_timestamp.get_as_utc();
                if diff > fifteen_days {
                    errors.get_or_insert_default().push(create_old_epss_timestamp_error(
                        &newest_epss_timestamp,
                        &newest_revision.valid_date,
                        &metrics[path_metric_idx]
                            .get_content()
                            .get_content_json_path(i_v, path_metric_idx),
                    ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_41, test_6_2_41_old_epss_timestamp);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_41() {
        let case_01_old_epss = Err(vec![create_old_epss_timestamp_error(
            &ValidCsafDateTime::from_str("2023-01-24T10:00:00.000Z").unwrap(),
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
            "/vulnerabilities/0/metrics/0/content",
        )]);
        let case_02_old_epss_with_timezone = Err(vec![create_old_epss_timestamp_error(
            &ValidCsafDateTime::from_str("2024-03-14T14:30:00.000-19:00").unwrap(),
            &ValidCsafDateTime::from_str("2024-03-30T10:00:00.000Z").unwrap(),
            "/vulnerabilities/0/metrics/0/content",
        )]);

        // Case 11: EPSS timestamp is same as newest revision history date
        // Case 12: Newest EPSS timestamp (with timezone) is within 15 days of newest revision

        TESTS_2_1
            .test_6_2_41
            .expect(case_01_old_epss, case_02_old_epss_with_timezone, Ok(()), Ok(()));
    }
}
