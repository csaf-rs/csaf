use crate::csaf::types::csaf_datetime::ValidCsafDateTime;
use crate::csaf::types::csaf_epss::CsafEpss;
use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, MetricTrait, RevisionHistorySortable, TrackingTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

fn create_epss_timestamp_too_new_error(
    epss_timestamp: &ValidCsafDateTime,
    i_v: usize,
    newest_revision_date: &ValidCsafDateTime,
    i_m: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "EPSS timestamp ({epss_timestamp}) for vulnerability at index {i_v} is newer than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/epss/timestamp"),
    }
}

/// 6.1.51 Inconsistent EPSS Timestamp
///
/// For each vulnerability, it is tested that the EPSS `timestamp` is earlier or equal to the `date`
/// of the newest item in the `revision_history` (taking timezones into consideration)
/// if the document status is `final` or `interim`.
pub fn test_6_1_51_inconsistent_epss_timestamp(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Check if the document status is "final" or "interim"
    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    // Get sorted revision history and find the newest entry
    let mut revision_history = tracking.get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let newest_revision = match revision_history.last() {
        Some(rev) => rev,
        None => return Ok(()), // TODO this should be a #409 precondition failed
    };

    // Check each vulnerability's EPSS timestamp
    let mut errors: Option<Vec<ValidationError>> = None;
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                if let Some(epss) = metric.get_content().get_epss() {
                    match CsafEpss::from(epss) {
                        CsafEpss::Valid(valid_epss) => {
                            // TODO fix this after #503
                            if valid_epss.timestamp > newest_revision.valid_date {
                                errors.get_or_insert_default().push(create_epss_timestamp_too_new_error(
                                    &valid_epss.timestamp,
                                    i_v,
                                    &newest_revision.valid_date,
                                    i_m,
                                ));
                            }
                        },
                        CsafEpss::Invalid(_) => {
                            // TODO: This will be a NonDeterminable (#409) later
                        },
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_51, test_6_1_51_inconsistent_epss_timestamp);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_51() {
        let case_01_too_late_new_timezone = Err(vec![create_epss_timestamp_too_new_error(
            &ValidCsafDateTime::from_str("2024-07-13T10:00:00.000Z").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
            0,
        )]);
        let case_02_too_new_neg_timezone_offset = Err(vec![create_epss_timestamp_too_new_error(
            &ValidCsafDateTime::from_str("2024-02-28T14:30:00.000-20:00").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-02-29T10:00:00.000Z").unwrap(),
            0,
        )]);
        let case_03_too_new_pos_timezone_offset = Err(vec![create_epss_timestamp_too_new_error(
            &ValidCsafDateTime::from_str("2024-02-29T14:30:00.000+04:00").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-02-29T10:00:00.000Z").unwrap(),
            0,
        )]);

        // Case 11: Same timestamp in newest rev history and EPSS
        // Case 12: EPSS timestamp before newest rev history, with negative timezone offset
        // Case 13: EPSS timestamp before newest rev history, with positive timezone offset

        TESTS_2_1.test_6_1_51.expect(
            case_01_too_late_new_timezone,
            case_02_too_new_neg_timezone_offset,
            case_03_too_new_pos_timezone_offset,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
