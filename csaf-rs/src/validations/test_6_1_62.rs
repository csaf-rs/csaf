use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::{TestFinding, TestFindingData};

/// 6.1.62 Inconsistent Discovery Date
///
/// For each vulnerability, it SHALL be tested that the `discovery_date` is
/// earlier than or equal to the `date` of the newest item in the `revision_history`
/// if the document status is `final` or `interim`.
///
/// It SHALL also be tested that the `discovery_date` is earlier than or equal to the
/// `disclosure_date` of the same vulnerability.
///
/// As the timestamps might use different timezones, the sorting SHALL take timezones into account.
pub fn test_6_1_62_inconsistent_discovery_date(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Get the revision history only if the document status is "final" or "interim"
    let revision_history = match status {
        DocumentStatus::Final | DocumentStatus::Interim => Some(tracking.aggregate_revision_history()),
        _ => None,
    };

    // Calculate the newest revision date if the revision history is present
    // For document statuses other than "final" or "interim", the newest revision date is not relevant and will be None
    let newest_revision_date = revision_history.as_ref().and_then(|revision_history| {
        revision_history
            .iter()
            .filter_map(|revision| match &revision.date {
                CsafDateTime::Valid(date) => Some(date),
                CsafDateTime::Invalid(_) => None, // TODO: This should result in a precondition finding (#409)
            })
            .max()
    });

    let mut errors = Vec::new();

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // Only proceed if the discovery date is present and valid
        let discovery_date = match vulnerability.get_discovery_date() {
            Some(CsafDateTime::Valid(date)) => date,
            Some(CsafDateTime::Invalid(_)) | None => continue, // TODO: Some(CsafDateTime::Invalid(_)) will be a collectable NonDeterminable #409
        };

        // If the newest revision date is present and the discovery date is later than it, push an error
        if let Some(newest_revision_date) = newest_revision_date
            && &discovery_date > newest_revision_date
        {
            errors.push(create_discovery_date_too_late_for_revision_error(
                &status,
                &discovery_date,
                newest_revision_date,
                vulnerability_index,
            ));
        }

        // Get the disclosure date only if it is present and valid
        let disclosure_date = match vulnerability.get_disclosure_date() {
            Some(CsafDateTime::Valid(date)) => Some(date),
            Some(CsafDateTime::Invalid(_)) | None => None, // TODO: Some(CsafDateTime::Invalid(_)) will be a collectable NonDeterminable #409
        };

        // If the disclosure date is present and valid, and the discovery date is later than it, push an error
        if let Some(disclosure_date) = disclosure_date
            && discovery_date > disclosure_date
        {
            errors.push(create_discovery_date_too_late_for_disclosure_error(
                &discovery_date,
                &disclosure_date,
                vulnerability_index,
            ));
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn create_discovery_date_too_late_for_revision_error(
    doc_status: &DocumentStatus,
    discovery_date: &ValidCsafDateTime,
    newest_revision_date: &ValidCsafDateTime,
    vulnerability_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Discovery date `{discovery_date}` is newer than the newest revision date \
             `{newest_revision_date}` on a document with status `{doc_status}`"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/discovery_date"),
    })
}

fn create_discovery_date_too_late_for_disclosure_error(
    discovery_date: &ValidCsafDateTime,
    disclosure_date: &ValidCsafDateTime,
    vulnerability_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Discovery date `{discovery_date}` is newer than its disclosure date `{disclosure_date}`"),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/discovery_date"),
    })
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_62, test_6_1_62_inconsistent_discovery_date);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_62 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_62() {
        let case_01 = Err(vec![create_discovery_date_too_late_for_revision_error(
            &DocumentStatus::Final,
            &ValidCsafDateTime::from_str("2024-02-24T10:00:00.000Z").unwrap(),
            &ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap(),
            0,
        )]);

        let case_02 = Err(vec![
            create_discovery_date_too_late_for_disclosure_error(
                &ValidCsafDateTime::from_str("2024-02-29T10:00:00.000Z").unwrap(),
                &ValidCsafDateTime::from_str("2024-02-26T10:00:00.000Z").unwrap(),
                0,
            ),
            create_discovery_date_too_late_for_revision_error(
                &DocumentStatus::Final,
                &ValidCsafDateTime::from_str("2026-09-09T10:00:00.000Z").unwrap(),
                &ValidCsafDateTime::from_str("2024-02-29T10:00:00.000Z").unwrap(),
                1,
            ),
            create_discovery_date_too_late_for_disclosure_error(
                &ValidCsafDateTime::from_str("2026-09-09T10:00:00.000Z").unwrap(),
                &ValidCsafDateTime::from_str("2022-11-18T10:00:00.000Z").unwrap(),
                1,
            ),
        ]);

        TESTS_2_1.test_6_1_62.expect(ExpectedResults {
            case_01,
            case_02,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
