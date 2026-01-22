use crate::csaf_traits::{
    CsafTrait, DocumentTrait, FirstKnownExploitationDatesTrait, FlagTrait, GeneratorTrait, InvolvementTrait,
    RemediationTrait, RevisionTrait, ThreatTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::validation::ValidationError;
use regex::Regex;
use std::sync::LazyLock;

static CSAF_RFC3339_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^((\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:(?:[0-4]\d|5[0-9])(?:\.\d+)?)(Z|[+-]\d{2}:\d{2}))$").unwrap()
});

/// Validates that all date/time fields in the CSAF document conform to the required format
/// (ISO 8601 format with time zone or UTC).
///
/// This function checks all date/time fields in the document, including tracking dates,
/// vulnerability disclosure/discovery dates, remediation dates, threat dates, etc.
pub fn test_6_1_37_date_and_time(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    // Check the initial release date
    check_datetime(
        tracking.get_initial_release_date_string(),
        "/document/tracking/initial_release_date",
    )?;

    // Check the current release date
    check_datetime(
        tracking.get_current_release_date_string(),
        "/document/tracking/current_release_date",
    )?;

    // Check the generator date if present
    if let Some(generator) = tracking.get_generator() {
        if let Some(date) = generator.get_date() {
            check_datetime(date, "/document/tracking/generator/date")?;
        }
    }

    // Check revision history dates if present
    for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
        check_datetime(
            revision.get_date(),
            &format!("/document/tracking/revision_history/{i_r}/date"),
        )?;
    }

    // Check vulnerability-related dates
    for (i_v, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // Check disclosure date if present
        if let Some(date) = vuln.get_disclosure_date() {
            check_datetime(date, &format!("/vulnerabilities/{i_v}/disclosure_date"))?;
        }

        // Check the discovery date if present
        if let Some(date) = vuln.get_discovery_date() {
            check_datetime(date, &format!("/vulnerabilities/{i_v}/discovery_date"))?;
        }

        // Check flags dates if present
        if let Some(flags) = vuln.get_flags() {
            for (i_f, flag) in flags.iter().enumerate() {
                if let Some(date) = flag.get_date() {
                    check_datetime(date, &format!("/vulnerabilities/{i_v}/flags/{i_f}/date"))?;
                }
            }
        }

        // Check involvement dates if present
        if let Some(involvements) = vuln.get_involvements() {
            for (i_i, involvement) in involvements.iter().enumerate() {
                if let Some(date) = involvement.get_date() {
                    check_datetime(date, &format!("/vulnerabilities/{i_v}/involvements/{i_i}/date"))?;
                }
            }
        }

        // Check remediation dates if present
        for (i_r, remediation) in vuln.get_remediations().iter().enumerate() {
            if let Some(date) = remediation.get_date() {
                check_datetime(date, &format!("/vulnerabilities/{i_v}/remediations/{i_r}/date"))?;
            }
        }

        // Check threat dates if present
        for (i_t, threat) in vuln.get_threats().iter().enumerate() {
            if let Some(date) = threat.get_date() {
                check_datetime(date, &format!("/vulnerabilities/{i_v}/threats/{i_t}/date"))?;
            }
        }

        if let Some(first_known_exploitation_dates) = vuln.get_first_known_exploitation_dates() {
            for (i_d, date) in first_known_exploitation_dates.iter().enumerate() {
                check_datetime(
                    date.get_date(),
                    &format!("/vulnerabilities/{i_v}/first_known_exploitation_dates/{i_d}/date"),
                )?;
            }
        }
    }

    Ok(())
}

fn create_invalid_format_error(date_time: &str, instance_path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Invalid date-time string {date_time}, expected RFC3339-compliant format with non-empty timezone and no leap seconds"
        ),
        instance_path: instance_path.to_string(),
    }
}

fn create_parsing_error(date_time: &str, error: impl std::fmt::Display, instance_path: &str) -> ValidationError {
    ValidationError {
        message: format!("Date-time string {date_time} matched RFC3339 regex but failed chrono parsing: {error}"),
        instance_path: instance_path.to_string(),
    }
}

fn check_datetime(date_time: &str, instance_path: &str) -> Result<(), Vec<ValidationError>> {
    if CSAF_RFC3339_REGEX.is_match(date_time) {
        // Add chrono-based plausibility check
        match chrono::DateTime::parse_from_rfc3339(date_time) {
            Ok(_) => Ok(()), // Successfully parsed as a valid RFC3339 datetime
            Err(e) => Err(vec![create_parsing_error(date_time, e, instance_path)]),
        }
    } else {
        Err(vec![create_invalid_format_error(date_time, instance_path)])
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_37
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_37_date_and_time(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_37() {
        // Only CSAF 2.1 has this test with 16 test cases (10 error cases, 6 success cases)
        TESTS_2_1.test_6_1_37.expect(
            Err(vec![create_invalid_format_error(
                "2024-01-24 10:00:00.000Z",
                "/document/tracking/initial_release_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2024-01-24T10:00:00.000z",
                "/document/tracking/initial_release_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2017-01-01T02:59:60+04:00",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_parsing_error(
                "2023-04-31T00:00:00+01:00",
                "input is out of range",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_parsing_error(
                "2023-02-29T00:00:00+01:00",
                "input is out of range",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2016-12-31T00:00:60+23:59",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2015-06-30T10:29:60-13:30",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2015-06-30T10:29:60-13:30",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2016-12-31T23:59:60.0123+00:00",
                "/vulnerabilities/0/disclosure_date",
            )]),
            Err(vec![create_invalid_format_error(
                "2024-01-24t10:00:00.000Z",
                "/vulnerabilities/0/first_known_exploitation_dates/0/date",
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
