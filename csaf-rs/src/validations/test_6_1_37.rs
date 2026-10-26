use crate::csaf::types::csaf_datetime::CsafDateTime::Invalid;
use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};
use regex::Regex;
use std::sync::LazyLock;

/// This regex realizes the requirements for timestamps in CSAF 2.1. See [`test_6_1_37_date_and_time`] for details.
///
/// The letter T separating the date and time SHALL be upper case: Ensured by `T` in `...\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}...`
/// The separator between date and time SHALL be the letter T: Same as above.
/// The letter Z indicating the timezone UTC SHALL be upper case: Ensured by `Z` in `...(Z|[+-]\d{2}...`
/// Fractions of seconds are allowed as specified in the standards mention above with the full stop (.) as separator: Ensured by `.` in `...(?:\.\d+)...`
/// Empty timezones SHALL NOT be used: Ensured by `(Z|[+-]\d{2}:\d{2})`, i.e. either a timezone or a offset must be present.
/// Leap seconds SHALL NOT be used: The expression `...5[0-9]...` ensures that leap seconds are rejected.
static CSAF_RFC3339_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(r"^((\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:(?:[0-4]\d|5[0-9])(?:\.\d+)?)(Z|[+-]\d{2}:\d{2}))$").unwrap()
});

/// 6.1.37 Date and Time
///
/// For each item of type string and format date-time it SHALL be tested that it conforms to the rules given in section 2.3.
///
/// Section 2.3:
/// This standard uses the date-time format as defined in JSON Schema Draft 2020-12 Section 7.3.1. In accordance with RFC3339 and ISO8601-1, the following rules apply:
//
/// * The letter T separating the date and time SHALL be upper case.
/// * The separator between date and time SHALL be the letter T.
/// * The letter Z indicating the timezone UTC SHALL be upper case.
/// * Fractions of seconds are allowed as specified in the standards mention above with the full stop (.) as separator.
/// * Empty timezones SHALL NOT be used.
/// * The ABNF of RFC 3339, section 5.6 applies.
///
/// In contrast to the aforementioned standards, leap seconds SHALL NOT be used.
pub fn test_6_1_37_date_and_time(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (date_parsed, instance_path) in doc.get_all_dates() {
        let date_raw = date_parsed.get_raw_string();
        // regex that ensures the additional section 2.3 requirements
        if !CSAF_RFC3339_REGEX.is_match(date_raw) {
            errors
                .get_or_insert_default()
                .push(create_invalid_format_error(date_raw, &instance_path));
            continue;
        }
        // Add chrono-based check for RFC 3339 requirements not covered by regex
        if let Invalid(err) = &date_parsed {
            errors
                .get_or_insert_default()
                .push(create_parsing_error(date_raw, err, &instance_path));
        }
    }

    errors.map_or(Ok(()), Err)
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
            case_21: Err(vec![
                create_invalid_format_error(
                    "2024-01-24t10:00:00.000Z",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/date",
                ),
                create_invalid_format_error(
                    "2024-01-22t12:34:56.789Z",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/exploitation_date",
                ),
            ]),
            case_22: Err(vec![
                create_parsing_error(
                    "2024-01-24T10:00:00.000+25:59",
                    "Failed to parse '2024-01-24T10:00:00.000+25:59' as RFC3339 with reason 'input is out of range'",
                    "/document/tracking/initial_release_date",
                ),
                create_parsing_error(
                    "2024-01-24T10:80:00.000Z",
                    "Failed to parse '2024-01-24T10:80:00.000Z' as RFC3339 with reason 'input is out of range'",
                    "/document/tracking/current_release_date",
                ),
                create_invalid_format_error("2024-01-24T10:00:80.000Z", "/document/tracking/generator/date"),
                create_invalid_format_error("2024-01-24T10:00:00", "/document/tracking/revision_history/0/date"),
                create_parsing_error(
                    "2023-02-30T09:00:00Z",
                    "Failed to parse '2023-02-30T09:00:00Z' as RFC3339 with reason 'input is out of range'",
                    "/document/involvement/actions/0/date",
                ),
                create_parsing_error(
                    "2023-03-00T15:00:00Z",
                    "Failed to parse '2023-03-00T15:00:00Z' as RFC3339 with reason 'input is out of range'",
                    "/document/involvement/actions/1/date",
                ),
                create_invalid_format_error("2024-01-24T10:00:00.000z", "/vulnerabilities/0/flags/0/date"),
                create_parsing_error(
                    "2024-01-24T10:00:00.000-24:00",
                    "Failed to parse '2024-01-24T10:00:00.000-24:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/remediations/0/date",
                ),
                create_parsing_error(
                    "2024-01-24T10:00:00.000+24:00",
                    "Failed to parse '2024-01-24T10:00:00.000+24:00' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/threats/0/date",
                ),
                create_invalid_format_error(
                    "2024-01-24t10:00:00.000Z",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/date",
                ),
                create_parsing_error(
                    "2023-12-32T12:34:56.789Z",
                    "Failed to parse '2023-12-32T12:34:56.789Z' as RFC3339 with reason 'input is out of range'",
                    "/vulnerabilities/0/first_known_exploitation_dates/0/exploitation_date",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
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
