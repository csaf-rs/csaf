use std::sync::LazyLock;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityTrait, WithDate,
};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use chrono::{DateTime, FixedOffset};
use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};

fn create_invalid_revision_date_error(date_str: &str, i_r: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid date format in revision history: {date_str}"),
        instance_path: format!("/document/tracking/revision_history/{i_r}/date"),
    }
}

static EMPTY_REVISION_HISTORY_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Revision history must not be empty for status final or interim".to_string(),
    instance_path: "/document/tracking/revision_history".to_string(),
});

fn create_ssvc_timestamp_too_late_error(
    ssvc_timestamp: &str,
    i_v: usize,
    newest_revision_date: &str,
    i_m: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "SSVC timestamp ({ssvc_timestamp}) for vulnerability at index {i_v} is later than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/timestamp"),
    }
}

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> ValidationError {
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

    // Parse the date of each revision and find the newest one
    let mut newest_revision_date: Option<DateTime<FixedOffset>> = None;
    for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
        // TODO: Rewrite this after revision history refactor
        let date = match revision.get_date() {
            Valid(date) => date.get_raw_string().to_owned(),
            Invalid(err) => err.get_raw_string().to_owned()
        };
        match DateTime::parse_from_rfc3339(date.as_str()) {
            Ok(parsed_date) => {
                newest_revision_date = match newest_revision_date {
                    None => Some(parsed_date),
                    Some(newest_date) => Some(newest_date.max(parsed_date)),
                };
            },
            Err(_) => {
                return Err(vec![create_invalid_revision_date_error(date.as_str(), i_r)]);
            },
        }
    }

    let newest_revision_date = match newest_revision_date {
        Some(date) => date,
        // No entries in revision history
        None => {
            return Err(vec![EMPTY_REVISION_HISTORY_ERROR.clone()]);
        },
    };

    // Check each vulnerability's SSVC timestamp
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                if metric.get_content().has_ssvc() {
                    match metric.get_content().get_ssvc() {
                        Ok(ssvc) => {
                            if ssvc.timestamp.fixed_offset() > newest_revision_date {
                                return Err(vec![create_ssvc_timestamp_too_late_error(
                                    &ssvc.timestamp.to_rfc3339(),
                                    i_v,
                                    &newest_revision_date.to_rfc3339(),
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
                "2024-07-13T10:00:00+00:00",
                0,
                "2024-01-24T10:00:00+00:00",
                0,
            )]),
            Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29T10:30:00+00:00",
                0,
                "2024-02-29T10:00:00+00:00",
                0,
            )]),
            Err(vec![create_ssvc_timestamp_too_late_error(
                "2024-02-29T10:30:00+00:00",
                0,
                "2024-02-29T10:00:00+00:00",
                0,
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
