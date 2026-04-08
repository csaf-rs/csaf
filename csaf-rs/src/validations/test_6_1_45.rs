use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf_traits::{
    CsafTrait, DistributionTrait, DocumentTrait, RevisionHistorySortable, TlpTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::schema::csaf2_1::schema::{DocumentStatus, LabelOfTlp};
use crate::validation::ValidationError;

fn create_disclosure_date_too_late_error(
    doc_status: &DocumentStatus,
    disclosure_date: &ValidCsafDateTime,
    i_v: usize,
    newest_revision_date: &ValidCsafDateTime,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Disclosure date ({disclosure_date}) for vulnerability at index {i_v} is newer than the newest revision date ({newest_revision_date}) on a document with TLP:CLEAR and document status {doc_status}"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/disclosure_date"),
    }
}

/// 6.1.45 Inconsistent Disclosure Date
///
/// For each vulnerability, it is tested that the `disclosure_date` is earlier or equal to the `date`
/// of the newest item in the `revision_history` (taking timezones into consideration)
/// if the document is TLP:CLEAR and the document status is `final` or `interim`.
pub fn test_6_1_45_inconsistent_disclosure_date(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Check if the document status is "final" or "interim"
    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    // Check if the document is TLP:CLEAR
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

    // Get sorted revision history and find the newest entry
    let mut revision_history = tracking.get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let newest_revision = match revision_history.last() {
        Some(rev) => rev,
        None => return Ok(()), // TODO this should be a #409 precondition failed
    };

    // Check each vulnerability's disclosure date
    let mut errors: Option<Vec<ValidationError>> = None;
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(disclosure_date) = vulnerability.get_disclosure_date() {
            match disclosure_date {
                CsafDateTime::Valid(valid_date) => {
                    if valid_date > newest_revision.valid_date {
                        errors
                            .get_or_insert_default()
                            .push(create_disclosure_date_too_late_error(
                                &status,
                                &valid_date,
                                i_v,
                                &newest_revision.valid_date,
                            ));
                    }
                },
                CsafDateTime::Invalid(_) => {
                    // TODO: This will be a NonDeterminable (#409) later
                },
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_45,
    test_6_1_45_inconsistent_disclosure_date
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_45() {
        let case_01_disclosure_date_too_late = Err(vec![create_disclosure_date_too_late_error(
            &DocumentStatus::Final,
            &ValidCsafDateTime::from_str("2024-02-24T10:00:00.000Z").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
        )]);
        let case_02_disclosure_date_too_late = Err(vec![create_disclosure_date_too_late_error(
            &DocumentStatus::Final,
            &ValidCsafDateTime::from_str("2025-02-26T10:00:00.000Z").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-02-29T10:00:00.000Z").unwrap(),
        )]);
        let case_03_disclosure_date_too_late_with_timezone = Err(vec![create_disclosure_date_too_late_error(
            &DocumentStatus::Final,
            &ValidCsafDateTime::from_str("2024-01-24T09:00:00.000-06:00").unwrap(),
            0,
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
        )]);

        // Case 11: disclosure_date equals newest revision date
        // Case 12: disclosure_date before newest revision date
        // Case 13: Not TLP:CLEAR, so test doesn't apply
        // Case 14: disclosure_date before newest revision date, with timezone

        TESTS_2_1.test_6_1_45.expect(
            case_01_disclosure_date_too_late,
            case_02_disclosure_date_too_late,
            case_03_disclosure_date_too_late_with_timezone,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
