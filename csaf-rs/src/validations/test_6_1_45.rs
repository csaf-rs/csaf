use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};
use crate::csaf_traits::{
    CsafTrait, DistributionTrait, DocumentTrait, TlpTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_1::schema::{DocumentStatus, LabelOfTlp};
use crate::validation::ValidationError;
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_dates::ValidatedRevisionHistoryDates;

fn create_disclosure_date_too_late_error(i_v: usize) -> ValidationError {
    ValidationError {
        message: "Disclosure date must not be later than the newest revision history date for TLP:CLEAR documents with final or interim status".to_string(),
        instance_path: format!("/vulnerabilities/{i_v}/discovery_date"),
    }
}

pub fn test_6_1_45_inconsistent_disclosure_date(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let status = document.get_tracking().get_status();

    // Only run this test if the document status is final or interim
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

    // And if the TLP is not CLEAR, skip this test
    if !is_tlp_clear {
        return Ok(());
    }

    // Get valid revision history dates, if there are invalid dates, return the errors and skip this test
    // As correct sorting can not be guaranteed if there are invalid dates
    let revision_history = document.get_tracking().get_revision_history();
    let mut valid_revision_dates = match ValidatedRevisionHistoryDates::from(&revision_history) {
        ValidatedRevisionHistoryDates::Invalid(err) => {return Err(err.into())}
        ValidatedRevisionHistoryDates::Valid(dates) => {dates}
    };

    // sort the valid revision dates and get the newest one
    valid_revision_dates.sort();
    let newest_revision_date = valid_revision_dates.get_newest().date_time;

    let mut errors: Option<Vec<ValidationError>> = None;

    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        // Get the disclosure date, if provided and valid
        if let Some(disclosure_date) = v.get_disclosure_date() {
            let disclosure_date = match &disclosure_date {
                Valid(date) => date,
                Invalid(err) => {
                    // If the disclosure date is invalid, add an error and skip this vulnerability
                    errors.get_or_insert_default().push(err.get_validation_error(&format!("/vulnerabilities/{i_v}/disclosure_date")));
                    continue;
                }
            };
            // If the disclosure date is later than the newest revision history date, add an error
            if disclosure_date > newest_revision_date {
                errors.get_or_insert_default().push(create_disclosure_date_too_late_error(i_v));
            }
        }
    }
    errors.map_or(Ok(()), Err)
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
