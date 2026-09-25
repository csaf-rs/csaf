use crate::csaf::macros::skip_if_document_status_is_not::skip_if_document_status_is_not;
use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};
use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityTrait, WithDate,
};
use crate::validation::{TestFinding, TestFindingData};

fn create_ssvc_timestamp_too_late_error(
    ssvc_timestamp: &str,
    i_v: usize,
    newest_revision_date: &str,
    i_m: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "SSVC timestamp ({ssvc_timestamp}) for vulnerability at index {i_v} is later than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/timestamp"),
    })
}

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Invalid SSVC object: {error}"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2"),
    })
}

/// 6.1.49 Inconsistent SSVC Timestamp
///
/// For each vulnerability, it is tested that the SSVC `timestamp` is earlier or equal to the `date`
/// of the newest item in the `revision_history` if the document status is `final` or `interim`.
/// As the timestamps might use different timezones, the sorting SHALL take timezones into account.
pub fn test_6_1_49_inconsistent_ssvc_timestamp(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();

    skip_if_document_status_is_not!(tracking.get_status(), Final, Interim);

    let newest_revision_date = if let Some(newest_date) = tracking
        .get_revision_history()
        .iter()
        .filter_map(|revision| match revision.get_date() {
            Valid(date) => Some(date.get_as_utc()),
            Invalid(_) => None,
        })
        .max()
    {
        newest_date
    } else {
        // Tested in 6.1.16
        return Ok(());
    };

    let mut findings = Vec::new();
    // Check each vulnerability's SSVC timestamp
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                if let Some(ssvc_result) = content.get_ssvc_v2() {
                    match ssvc_result {
                        Ok(ssvc) => {
                            if ssvc.timestamp.fixed_offset() > newest_revision_date {
                                findings.push(create_ssvc_timestamp_too_late_error(
                                    &ssvc.timestamp.to_rfc3339(),
                                    i_v,
                                    &newest_revision_date.to_rfc3339(),
                                    i_m,
                                ));
                            }
                        },
                        Err(err) => {
                            findings.push(create_invalid_ssvc_error(err, i_v, i_m));
                        },
                    }
                }
            }
        }
    }

    // TODO: Refactor this with roll-out of finding collector see #1018
    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_49, test_6_1_49_inconsistent_ssvc_timestamp);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_49 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_49() {
        let two_vulnerabilities_alternating_valid_and_invalid_ssvc_timestamps = Err(vec![
            create_ssvc_timestamp_too_late_error("2024-07-13T10:00:00+00:00", 0, "2024-01-24T10:00:00+00:00", 0),
            create_ssvc_timestamp_too_late_error("2025-07-13T10:00:00+00:00", 0, "2024-01-24T10:00:00+00:00", 2),
            create_ssvc_timestamp_too_late_error("2024-07-13T10:00:00+00:00", 1, "2024-01-24T10:00:00+00:00", 0),
            create_ssvc_timestamp_too_late_error("2025-07-13T10:00:00+00:00", 1, "2024-01-24T10:00:00+00:00", 2),
        ]);

        // Only CSAF 2.1 has this test
        TESTS_2_1.test_6_1_49.expect(ExpectedResults {
            case_01: Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-07-13T10:00:00+00:00",
                0,
                "2024-01-24T10:00:00+00:00",
                0,
            )]),
            case_02: Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29T10:30:00+00:00",
                0,
                "2024-02-29T10:00:00+00:00",
                0,
            )]),
            case_03: Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29T10:30:00+00:00",
                0,
                "2024-02-29T10:00:00+00:00",
                0,
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_s01: two_vulnerabilities_alternating_valid_and_invalid_ssvc_timestamps,
        });
    }
}
