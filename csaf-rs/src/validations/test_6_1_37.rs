use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};
use crate::csaf_traits::{ContentTrait, EpssTrait};
use crate::csaf_traits::{
    CsafTrait, DocumentTrait, FirstKnownExploitationDatesTrait, MetricTrait, TrackingTrait, VulnerabilityTrait,
    WithDate, WithOptionalDate,
};
use crate::validation::{TestFinding, TestFindingData};
use regex::Regex;
use std::sync::LazyLock;

static CSAF_RFC3339_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^((\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:(?:[0-4]\d|5[0-9])(?:\.\d+)?)(Z|[+-]\d{2}:\d{2}))$").unwrap()
});

fn get_dates(doc: &impl CsafTrait) -> Vec<(CsafDateTime, String)> {
    let mut dates = Vec::new();

    let tracking = doc.get_document().get_tracking();
    dates.push((
        tracking.get_initial_release_date(),
        "/document/tracking/initial_release_date".to_string(),
    ));
    dates.push((
        tracking.get_current_release_date(),
        "/document/tracking/current_release_date".to_string(),
    ));
    if let Some(generator) = tracking.get_generator()
        && let Some(date) = generator.get_date()
    {
        dates.push((date, "/document/tracking/generator/date".to_string()));
    }
    for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
        dates.push((
            revision.get_date().clone(),
            format!("/document/tracking/revision_history/{i_r}/date").to_string(),
        ));
    }

    let vulnerabilities = doc.get_vulnerabilities();
    for (i_v, vuln) in vulnerabilities.iter().enumerate() {
        if let Some(date) = vuln.get_disclosure_date() {
            dates.push((
                date.clone(),
                format!("/vulnerabilities/{i_v}/disclosure_date").to_string(),
            ));
        }
        if let Some(date) = vuln.get_discovery_date() {
            dates.push((
                date.clone(),
                format!("/vulnerabilities/{i_v}/discovery_date").to_string(),
            ));
        }
        if let Some(flags) = vuln.get_flags() {
            for (i_f, flag) in flags.iter().enumerate() {
                if let Some(date) = flag.get_date() {
                    dates.push((
                        date.clone(),
                        format!("/vulnerabilities/{i_v}/flags/{i_f}/date").to_string(),
                    ));
                }
            }
        }
        // ToDo refactor after merging latest revision history -> moved to document instead of vulnerability
        // also there are actions now
        if let Some(involvements) = vuln.get_involvements() {
            for (i_i, involvement) in involvements.iter().enumerate() {
                if let Some(date) = involvement.get_date() {
                    dates.push((
                        date.clone(),
                        format!("/vulnerabilities/{i_v}/involvements/{i_i}/date").to_string(),
                    ));
                }
            }
        }
        for (i_r, remediation) in vuln.get_remediations().iter().enumerate() {
            if let Some(date) = remediation.get_date() {
                dates.push((
                    date.clone(),
                    format!("/vulnerabilities/{i_v}/remediations/{i_r}/date").to_string(),
                ));
            }
        }
        for (i_t, threat) in vuln.get_threats().iter().enumerate() {
            if let Some(date) = threat.get_date() {
                dates.push((
                    date.clone(),
                    format!("/vulnerabilities/{i_v}/threats/{i_t}/date").to_string(),
                ));
            }
        }
        if let Some(first_known_exploitation_dates) = vuln.get_first_known_exploitation_dates() {
            for (i_d, date) in first_known_exploitation_dates.iter().enumerate() {
                dates.push((
                    date.get_date().clone(),
                    format!("/vulnerabilities/{i_v}/first_known_exploitation_dates/{i_d}/date").to_string(),
                ));
                dates.push((
                    date.get_exploitation_date().clone(),
                    format!("/vulnerabilities/{i_v}/first_known_exploitation_dates/{i_d}/exploitation_date")
                        .to_string(),
                ));
            }
        }
        if let Some(metrics) = vuln.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                if let Some(epss) = metric.get_content().get_epss() {
                    dates.push((
                        epss.get_timestamp().clone(),
                        format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/epss/timestamp").to_string(),
                    ));
                }
                if let Some(ssvc_map) = metric.get_content().get_ssvc_v2_raw()
                    && let Some(timestamp) = ssvc_map.get("timestamp").and_then(serde_json::Value::as_str)
                {
                    dates.push((
                        CsafDateTime::from(timestamp),
                        format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/timestamp").to_string(),
                    ))
                }
            }
        }
    }

    dates
}

/// Validates that all date/time fields in the CSAF document conform to the required format
/// (ISO 8601 format with time zone or UTC).
///
/// This function checks all date/time fields in the document, including tracking dates,
/// vulnerability disclosure/discovery dates, remediation dates, threat dates, etc.
pub fn test_6_1_37_date_and_time(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Vec<TestFinding> = Vec::new();

    for (date, instance_path) in get_dates(doc) {
        check_datetime(&date, instance_path.as_str(), &mut errors);
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn create_invalid_format_error(date_time: &str, instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Invalid date-time string {date_time}, expected RFC3339-compliant format with non-empty timezone and no leap seconds"
        ),
        instance_path: instance_path.to_string(),
    })
}

fn create_parsing_error(date_time: &str, error: impl std::fmt::Display, instance_path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Date-time string {date_time} matched RFC3339 regex but failed chrono parsing: {error}"),
        instance_path: instance_path.to_string(),
    })
}

fn check_datetime(date_time: &CsafDateTime, instance_path: &str, errors: &mut Vec<TestFinding>) {
    let date = match date_time {
        Valid(date) => date.get_raw_string(),
        Invalid(err) => err.get_raw_string(),
    };
    if CSAF_RFC3339_REGEX.is_match(date) {
        // Add chrono-based plausibility check
        if let CsafDateTime::Invalid(err) = date_time {
            errors.push(create_parsing_error(date, err, instance_path));
        }
    } else {
        errors.push(create_invalid_format_error(date, instance_path));
    }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_37, test_6_1_37_date_and_time);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_37 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_37() {
        TESTS_2_1.test_6_1_37.expect(ExpectedResults {
            case_01: Err(vec![
                create_invalid_format_error("2024-01-24 10:00:00.000Z", "/document/tracking/initial_release_date"),
                create_invalid_format_error("2024-01-24 10:00:00.000Z", "/document/tracking/current_release_date"),
                create_invalid_format_error("2024-01-24 10:00:00.000Z", "/document/tracking/revision_history/0/date"),
            ]),
            case_02: Err(vec![
                create_invalid_format_error("2024-01-24T10:00:00.000", "/document/tracking/current_release_date"),
                create_invalid_format_error("2024-01-24T10:00:00.000z", "/document/tracking/initial_release_date"),
                create_invalid_format_error(
                    "2024-01-24T10:00:00.000+00:10:21",
                    "/document/tracking/revision_history/0/date",
                ),
            ]),
            case_03: Err(vec![
                create_invalid_format_error("2017-01-01T02:59:60+04:00", "/vulnerabilities/0/disclosure_date"),
                create_parsing_error(
                    "2014-13-31T00:00:00+01:00",
                    "Failed to parse '2014-13-31T00:00:00+01:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/discovery_date",
                ),
            ]),
            case_04: Err(vec![
                create_parsing_error(
                    "2023-04-31T00:00:00+01:00",
                    "Failed to parse '2023-04-31T00:00:00+01:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/disclosure_date",
                ),
                create_parsing_error(
                    "2023-02-30T00:00:00+01:00",
                    "Failed to parse '2023-02-30T00:00:00+01:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/discovery_date",
                ),
            ]),
            case_05: Err(vec![
                create_parsing_error(
                    "2023-02-29T00:00:00+01:00",
                    "Failed to parse '2023-02-29T00:00:00+01:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/disclosure_date",
                ),
                create_parsing_error(
                    "1900-02-29T00:00:00+01:00",
                    "Failed to parse '1900-02-29T00:00:00+01:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/discovery_date",
                ),
            ]),
            case_06: Err(vec![
                create_invalid_format_error("2016-12-31T00:00:60+23:59", "/vulnerabilities/0/disclosure_date"),
                create_invalid_format_error("2015-07-01T06:59:60-07:00", "/vulnerabilities/0/discovery_date"),
            ]),
            case_07: Err(vec![
                create_invalid_format_error("2015-06-30T10:29:60-13:30", "/vulnerabilities/0/disclosure_date"),
                create_invalid_format_error("2015-06-30T23:59:60+00:00", "/vulnerabilities/0/discovery_date"),
            ]),
            case_08: Err(vec![
                create_invalid_format_error("2015-06-30T10:29:60-13:30", "/vulnerabilities/0/disclosure_date"),
                create_invalid_format_error("2015-06-30T23:59:60+00:00", "/vulnerabilities/0/discovery_date"),
            ]),
            case_09: Err(vec![
                create_invalid_format_error("2016-12-31T23:59:60.0123+00:00", "/vulnerabilities/0/disclosure_date"),
                create_invalid_format_error("2017-01-01T02:59:60.999999+03:00", "/vulnerabilities/0/discovery_date"),
            ]),
            case_20: Err(vec![
                create_invalid_format_error(
                    "2024-01-24t10:00:00.000Z",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/date",
                ),
                create_invalid_format_error(
                    "2024-01-22t12:34:56.789Z",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/exploitation_date",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_s01: Err(vec![create_invalid_format_error(
                "2024-02-29t14:30:00.000Z",
                "/vulnerabilities/0/metrics/0/content/epss/timestamp",
            )]),
            case_s02: Err(vec![create_invalid_format_error(
                "2024-02-29t14:30:00.000Z",
                "/vulnerabilities/0/metrics/0/content/ssvc_v2/timestamp",
            )]),
        });
    }
}
