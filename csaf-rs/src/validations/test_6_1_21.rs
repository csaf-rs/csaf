use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::validation::ValidationError;

/// 6.1.21 Missing Item in Revision History
///
/// It MUST be tested that items of the revision history do not omit a version number when the items are sorted ascending by date.
/// In the case of semantic versioning, this applies only to the Major version.
/// It MUST also be tested that the first item in such a sorted list has either the version number 0 or 1 in
/// the case of integer versioning or a Major version of 0 or 1 in the case of semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = doc.get_document().get_tracking().get_revision_history_tuples();
    rev_history_tuples.inplace_sort_by_date_then_number();

    if rev_history_tuples.is_empty() {
        return Ok(()); // ToDo #409 this should be Skipped: Precondition failed
    };

    rev_history_tuples
        .iter()
        .fold(None::<&CsafVersionNumber>, |prev, current| {
            match prev {
                // checks first item
                None => {
                    if !(current.number.get_major() == 0 || current.number.get_major() == 1) {
                        errors.get_or_insert_default().push(test_6_1_21_err_wrong_first_version(
                            current.number.clone(),
                            &current.path_index,
                        ));
                    }
                },
                // checks subsequent items
                Some(prev_number) => {
                    // skip lesser versions in the sequence
                    if current.number.get_major() < prev_number.get_major() {
                        return prev;
                    }
                    let expected = prev_number.get_next_major_version();
                    // check if the current version is the expected next major version
                    if current.number.get_major() > expected.get_major() {
                        if expected.get_next_major_version() == current.number {
                            // only one version missing
                            errors
                                .get_or_insert_default()
                                .push(test_6_1_21_err_missing_version(expected));
                        } else {
                            // multiple versions missing
                            errors
                                .get_or_insert_default()
                                .push(test_6_1_21_err_missing_version_range(
                                    expected,
                                    current.number.get_previous_major_version(),
                                ));
                        }
                    }
                },
            }
            Some(&current.number)
        });

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_21, test_6_1_21_missing_item_in_revision_history);

fn test_6_1_21_err_wrong_first_version(version: CsafVersionNumber, revision_index: &usize) -> ValidationError {
    let version_error = match version {
        CsafVersionNumber::IntVer(_) => "integer version of 0 or 1",
        CsafVersionNumber::SemVer(_) => "semver version of 0.y.z or 1.y.z",
    }
    .to_string();
    ValidationError {
        message: format!("The first revision history item should have {version_error}, but was {version}"),
        instance_path: format!("/document/tracking/revision_history/{revision_index}"),
    }
}

fn test_6_1_21_err_missing_version(expected_version: CsafVersionNumber) -> ValidationError {
    let expected_number = expected_version.get_major();
    let version_error = match expected_version {
        CsafVersionNumber::IntVer(_) => format!("{expected_number}"),
        CsafVersionNumber::SemVer(_) => format!("{expected_number}.y.z"),
    };
    ValidationError {
        message: format!("Missing revision history item {version_error}"),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

fn test_6_1_21_err_missing_version_range(from: CsafVersionNumber, to: CsafVersionNumber) -> ValidationError {
    ValidationError {
        message: format!("Missing revision history items {from} to {to}."),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        let case_intver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2"))]);
        let case_intver_2_3_missing_1 = Err(vec![test_6_1_21_err_wrong_first_version(
            CsafVersionNumber::from("2"),
            &0,
        )]);
        let case_semver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0"))]);
        let case_semver_2_3_missing_1 = Err(vec![test_6_1_21_err_wrong_first_version(
            CsafVersionNumber::from("2.0.0"),
            &0,
        )]);
        let case_semver_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0"))]);
        let case_semver_multiple_single_versions_and_range_missing = Err(vec![
            test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0")),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("4.0.0")),
            test_6_1_21_err_missing_version_range(CsafVersionNumber::from("6.0.0"), CsafVersionNumber::from("7.0.0")),
        ]);
        let case_big_range_missing = Err(vec![test_6_1_21_err_missing_version_range(
            CsafVersionNumber::from("2.0.0"),
            CsafVersionNumber::from("99.0.0"),
        )]);
        let case_semver_first_version_mismatch_multiple_versions_missing = Err(vec![
            test_6_1_21_err_wrong_first_version(CsafVersionNumber::from("3.0.0"), &1),
            test_6_1_21_err_missing_version_range(CsafVersionNumber::from("4.0.0"), CsafVersionNumber::from("7.0.0")),
        ]);
        let case_intver_first_version_mismatch_range_missing = Err(vec![test_6_1_21_err_wrong_first_version(
            CsafVersionNumber::from("4"),
            &0,
        )]);

        // Valid cases for both 2.0 and 2.1
        // case 11: valid intver final start with 1
        // case 12: valid intver draft start with 0
        // case 13: valid semver final start with 1.0.0
        // case s11: empty revision history
        // case s12: valid intver interim start with 1

        TESTS_2_0.test_6_1_21.expect(
            case_intver_1_3_missing_2.clone(),
            case_intver_2_3_missing_1.clone(),
            case_semver_1_3_missing_2.clone(),
            case_semver_2_3_missing_1.clone(),
            case_semver_missing_2.clone(),
            case_semver_multiple_single_versions_and_range_missing.clone(),
            case_big_range_missing.clone(),
            case_semver_first_version_mismatch_multiple_versions_missing.clone(),
            case_intver_first_version_mismatch_range_missing.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        let case_intver_1_3_4_with_timezone_missing_2 =
            Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2"))]);

        TESTS_2_1.test_6_1_21.expect(
            case_intver_1_3_missing_2,
            case_intver_2_3_missing_1,
            case_intver_1_3_4_with_timezone_missing_2,
            case_semver_1_3_missing_2,
            case_semver_2_3_missing_1,
            case_semver_missing_2,
            case_semver_multiple_single_versions_and_range_missing,
            case_big_range_missing,
            case_semver_first_version_mismatch_multiple_versions_missing,
            case_intver_first_version_mismatch_range_missing,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
