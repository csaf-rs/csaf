use crate::csaf::macros::skip_if_document_status_is_not::skip_if_document_status_is_not;
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

fn create_revision_history_error(status: &DocumentStatus, number: &CsafVersionNumber, index: usize) -> ValidationError {
    let reason = match number {
        CsafVersionNumber::IntVer(_) => "Version 0 is",
        CsafVersionNumber::SemVer(_) => "Versions 0.y.z are",
    };
    ValidationError {
        message: format!(
            "Document with status '{status}' contains a revision history item with number '{number}', {reason} forbidden"
        ),
        instance_path: format!("/document/tracking/revision_history/{index}/number"),
    }
}

/// 6.1.18 Released Revision History
///
/// For documents with `/document/status` "final" or "interim", no item in `/document/tracking/revision_history[]`
/// may have the version 0 or 0.y.z.
pub fn test_6_1_18_released_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    // This test is only relevant for documents with status 'interim' and 'final'
    let status = tracking.get_status();
    skip_if_document_status_is_not!(status, Final, Interim);

    // Check that no revision history item has version 0 or 0.y.z
    let mut errors: Option<Vec<ValidationError>> = None;
    let revision_history = tracking.get_revision_history();
    for (revision_index, revision) in revision_history.iter().enumerate() {
        let number = revision.get_number();
        if number.get_major() == 0 {
            errors
                .get_or_insert_default()
                .push(create_revision_history_error(&status, &number, revision_index));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_18, test_6_1_18_released_revision_history);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_18() {
        let case_intver_zero_status_final = Err(vec![create_revision_history_error(
            &DocumentStatus::Final,
            &CsafVersionNumber::from("0"),
            0,
        )]);
        let case_semver_zero_status_final = Err(vec![create_revision_history_error(
            &DocumentStatus::Final,
            &CsafVersionNumber::from("0.9.0"),
            0,
        )]);

        // Case S11: Document status draft, revision history item with version 0.y.z
        TESTS_2_0.test_6_1_18.expect(
            case_intver_zero_status_final.clone(),
            case_semver_zero_status_final.clone(),
            Ok(()),
        );
        TESTS_2_1
            .test_6_1_18
            .expect(case_intver_zero_status_final, case_semver_zero_status_final, Ok(()));
    }
}
