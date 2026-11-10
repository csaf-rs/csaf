use crate::csaf::macros::skip_if_document_status_is_not::skip_if_document_status_is_not;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::{INTEGER_VER_ONE, SEMANTIC_VER_ONE};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_date_mismatch_error(initial: &str, revision: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Initial release date `{initial}` does not match the date `{revision}` of revision 1 or 1.0.0"
        ),
        instance_path: "/document/tracking/initial_release_date".to_string(),
    })
}

fn create_missing_initial_revision_error() -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: "Could not determine the initial release date: revision history contains no item numbered 1 or 1.0.0"
            .to_string(),
        instance_path: "/document/tracking/revision_history".to_string(),
    })
}

fn create_invalid_initial_revision_date_error(index: usize, date: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Could not determine the initial release date: invalid date '{date}' in revision 1 or 1.0.0"),
        instance_path: format!("/document/tracking/revision_history/{index}/date"),
    })
}

/// 6.1.66 Non-Draft Initial Release Date
///
/// For documents with status `final` or `interim`, the initial release date SHALL match
/// the date of the revision history item numbered `1` or `1.0.0`.
///
/// Comparison SHALL take timezones into account.
/// The test SHALL fail with a different error message if the `date` of the revision history item
/// that has a `number` of `1` or `1.0.0` cannot be determined.
pub fn test_6_1_66_non_draft_initial_release_date(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let tracking = doc.get_document().get_tracking();

    skip_if_document_status_is_not!(tracking.get_status(), Final, Interim);

    let revision_history = tracking.aggregate_revision_history();

    let Some(initial_revision) = revision_history
        .iter()
        .find(|item| item.number == INTEGER_VER_ONE || item.number == SEMANTIC_VER_ONE)
    else {
        return Err(vec![create_missing_initial_revision_error()]);
    };

    let CsafDateTime::Valid(revision_date) = &initial_revision.date else {
        return Err(vec![create_invalid_initial_revision_date_error(
            initial_revision.path_index,
            initial_revision.date.get_raw_string(),
        )]);
    };

    let CsafDateTime::Valid(initial_release_date) = tracking.get_initial_release_date() else {
        return Ok(()); // TODO #409: Return `PreconditionFailed` once supported
    };

    if &initial_release_date == revision_date {
        Ok(())
    } else {
        Err(vec![create_date_mismatch_error(
            initial_release_date.get_raw_string(),
            revision_date.get_raw_string(),
        )])
    }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_66,
    test_6_1_66_non_draft_initial_release_date
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_66 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_66() {
        let status_final_mismatching_initial_release_date_with_integer_version = Err(vec![create_date_mismatch_error(
            "2024-01-22T10:00:00.000Z",
            "2024-01-21T10:00:00.000Z",
        )]);

        let status_final_mismatching_initial_release_date_with_semantic_version =
            Err(vec![create_date_mismatch_error(
                "2024-01-21T10:00:00.000Z",
                "2024-01-22T10:00:00.000Z",
            )]);

        let status_interim_mismatching_initial_release_date_with_integer_version =
            Err(vec![create_date_mismatch_error(
                "2024-01-20T10:00:00.000Z",
                "2024-01-21T10:00:00.000Z",
            )]);

        let status_interim_mismatching_initial_release_date_with_semantic_version =
            Err(vec![create_date_mismatch_error(
                "2024-01-21T10:00:00.000Z",
                "2024-01-20T10:00:00.000Z",
            )]);

        let missing_initial_revision = Err(vec![create_missing_initial_revision_error()]);

        // Case 11: Status final, matching initial release date with integer version
        // Case 12: Status final, matching initial release date with semantic version
        // Case 15: Status draft
        TESTS_2_1.test_6_1_66.expect(ExpectedResults {
            case_01: status_final_mismatching_initial_release_date_with_integer_version,
            case_02: status_final_mismatching_initial_release_date_with_semantic_version,
            case_03: status_interim_mismatching_initial_release_date_with_integer_version.clone(),
            case_04: status_interim_mismatching_initial_release_date_with_semantic_version.clone(),
            case_05: missing_initial_revision,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: status_interim_mismatching_initial_release_date_with_integer_version, // TODO: Discuss with OASIS. OASIS marks these cases as valid despite mismatching dates
            case_14: status_interim_mismatching_initial_release_date_with_semantic_version, // TODO: Discuss with OASIS. OASIS marks these cases as valid despite mismatching dates
            case_15: Ok(()),
        });
    }
}
