use crate::csaf_traits::{
    CsafTrait, DistributionTrait, DocumentTrait, TlpTrait, TrackingTrait, VulnerabilityTrait, WithDate,
};
use crate::schema::csaf2_1::schema::{DocumentStatus, LabelOfTlp};
use crate::validation::ValidationError;
use chrono::{DateTime, FixedOffset};
use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};

fn create_invalid_revision_date_error(date: &str, i_rev: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid date format in revision history: {date}"),
        instance_path: format!("/document/tracking/revision_history/{i_rev}"),
    }
}

fn create_disclosure_date_too_late_error(i_v: usize) -> ValidationError {
    ValidationError {
        message: "Disclosure date must not be later than the newest revision history date for TLP:CLEAR documents with final or interim status".to_string(),
        instance_path: format!("/vulnerabilities/{i_v}/discovery_date"),
    }
}

fn create_invalid_disclosure_date_error(date: &str, i_v: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid disclosure date format: {date}"),
        instance_path: format!("/vulnerabilities/{i_v}/discovery_date"),
    }
}

pub fn test_6_1_45_inconsistent_disclosure_date(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Only check if document is TLP:CLEAR and status is final or interim
    let document = doc.get_document();
    let status = document.get_tracking().get_status();

    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    let is_tlp_clear = match document.get_distribution_21() {
        Ok(distribution) => match distribution.get_tlp_21() {
            Ok(tlp) => tlp.get_label() == LabelOfTlp::Clear,
            Err(_) => false,
        },
        Err(_) => false,
    };

    if !is_tlp_clear {
        return Ok(());
    }

    // Get the newest revision history date
    let mut newest_revision_date: Option<DateTime<FixedOffset>> = None;
    let revision_history = document.get_tracking().get_revision_history();
    for (i_rev, rev) in revision_history.iter().enumerate() {
        // TODO: Rewrite this after revision history refactor
        let date = rev.get_date();
        let date = match date {
            Valid(date) => date.get_raw_string().to_owned(),
            Invalid(err) => err.get_raw_string().to_owned()
        };
        chrono::DateTime::parse_from_rfc3339(&date)
            .map(|rev_datetime| {
                println!("rev_datetime: {rev_datetime:?}, newest_revision_date: {newest_revision_date:?}");
                newest_revision_date = match newest_revision_date {
                    None => Some(rev_datetime),
                    Some(prev_max) => Some(prev_max.max(rev_datetime)),
                }
            })
            .map_err(|_| vec![create_invalid_revision_date_error(&date, i_rev)])?;
    }

    if let Some(newest_date) = newest_revision_date {
        // Check each vulnerability's disclosure date
        for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
            if let Some(disclosure_date) = v.get_disclosure_date() {
                let disclosure_date = match disclosure_date {
                    Valid(date) => date.get_raw_string().to_owned(),
                    Invalid(err) => err.get_raw_string().to_owned()
                };
                match chrono::DateTime::parse_from_rfc3339(&disclosure_date) {
                    Ok(disclosure_datetime) => {
                        println!("disclosure_datetime: {disclosure_datetime:?}, newest_date: {newest_date:?}");
                        if disclosure_datetime > newest_date {
                            return Err(vec![create_disclosure_date_too_late_error(i_v)]);
                        }
                    },
                    Err(_) => {
                        return Err(vec![create_invalid_disclosure_date_error(
                            &disclosure_date,
                            i_v,
                        )]);
                    },
                }
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_45
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_45_inconsistent_disclosure_date(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_45() {
        // Only CSAF 2.1 has this test with 7 test cases (3 error cases, 4 success cases)
        TESTS_2_1.test_6_1_45.expect(
            Err(vec![create_disclosure_date_too_late_error(0)]),
            Err(vec![create_disclosure_date_too_late_error(0)]),
            Err(vec![create_disclosure_date_too_late_error(0)]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
