use std::collections::HashMap;

use crate::csaf::aggregation::revision_history::CsafRevisionHistoryItem;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

/// 6.1.21 Missing Item in Revision History
///
/// It MUST be tested that items of the revision history do not omit a version number when the items are sorted ascending by date.
/// In the case of semantic versioning, this applies only to the Major version.
/// It MUST also be tested that the first item in such a sorted list has either the version number 0 or 1 in
/// the case of integer versioning or a Major version of 0 or 1 in the case of semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    struct MissingVersionMetadata {
        found: bool,
        from: Option<CsafDateTime>,
        to: Option<CsafDateTime>,
    }
    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = doc.get_document().get_tracking().aggregate_revision_history();
    rev_history_tuples.inplace_sort_by_date_then_number();

    if rev_history_tuples.is_empty() {
        return Ok(()); // ToDo #409 this should be Skipped: Precondition failed
    };
    let mut missing_versions: HashMap<CsafVersionNumber, MissingVersionMetadata> = HashMap::new();
    rev_history_tuples
        .iter()
        .fold(None::<&CsafRevisionHistoryItem>, |prev, current| {
            match prev {
                // checks first item
                None => {
                    let mut first_item = current.number.clone();
                    if !(first_item.get_major() == 0 || first_item.get_major() == 1) {
                        errors
                            .get_or_insert_default()
                            .push(test_6_1_21_err_wrong_first_version(&first_item));
                        while let Some(previous_version) = first_item.get_previous_major_version() {
                            missing_versions.insert(
                                previous_version.clone(),
                                MissingVersionMetadata {
                                    found: false,
                                    from: None,
                                    to: Some(current.date.clone()),
                                },
                            );
                            first_item = previous_version;
                        }
                    }
                },
                // checks subsequent items
                Some(prev_item) => {
                    if current.number.get_major() < prev_item.number.get_major() {
                        // check if the current number was already marked as missing
                        if let Some(previously_missing_version) = missing_versions.get_mut(&current.number) {
                            // mark as found so we can distinguish between missing at all or not
                            // example squence: 3, 2 -> 1,2 would be already marked as missing, now we found 2 and can add a before or between error
                            previously_missing_version.found = true;
                        } else {
                            missing_versions.insert(
                                current.number.clone(),
                                MissingVersionMetadata {
                                    found: false,
                                    from: None,
                                    to: Some(prev_item.date.clone()),
                                },
                            );
                        }
                        return prev;
                    }
                    if current.number.get_major() == prev_item.number.get_major() {
                        // we don't care about successive items with the same version number, we only care about missing versions
                        return prev;
                    }

                    let expected = prev_item.number.get_next_major_version();
                    if current.number.get_major() == expected.get_major() {
                        // the current version is the expected next major version, so we can continue checking the next item
                        return Some(current);
                    }
                    // check if the current version is the expected next major version
                    if current.number.get_major() > expected.get_major() {
                        // there is at least one missing version between the previous and current version
                        let mut start = prev_item.number.clone();
                        while let next_version = start.get_next_major_version()
                            && next_version < current.number
                        {
                            missing_versions.insert(
                                next_version.clone(),
                                MissingVersionMetadata {
                                    found: false,
                                    from: Some(prev_item.date.clone()),
                                    to: Some(current.date.clone()),
                                },
                            );
                            start = next_version;
                        }
                    }
                },
            }
            Some(current)
        });
    for (missing_version, version_metadata) in missing_versions {
        if !version_metadata.found {
            errors
                .get_or_insert_default()
                .push(test_6_1_21_err_missing_version_at_all(&missing_version))
        } else {
            match (version_metadata.from, version_metadata.to) {
                (Some(from), Some(to)) => errors
                    .get_or_insert_default()
                    .push(test_6_1_21_err_missing_version_between(&missing_version, &from, &to)),
                (None, Some(to)) => errors
                    .get_or_insert_default()
                    .push(test_6_1_21_err_missing_version_before(&missing_version, &to)),
                _ => {},
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_21, test_6_1_21_missing_item_in_revision_history);

const REVISION_HISTORY_PATH: &str = "/document/tracking/revision_history";

fn test_6_1_21_err_wrong_first_version(version: &CsafVersionNumber) -> ValidationError {
    let expected_version = match version {
        CsafVersionNumber::IntVer(_) => "`0` or `1`",
        CsafVersionNumber::SemVer(_) => "`0.y.z` or `1.y.z`",
    }
    .to_string();

    ValidationError {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!(
            "revision history does not start with a version of {expected_version} when sorted by date (was `{version}`)"
        ),
    }
}

fn test_6_1_21_err_missing_version_at_all(missing_version: &CsafVersionNumber) -> ValidationError {
    ValidationError {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` at all"),
    }
}

fn test_6_1_21_err_missing_version_before(
    missing_version: &CsafVersionNumber,
    start: &CsafDateTime,
) -> ValidationError {
    let start = start.get_raw_string();
    ValidationError {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` before `{start}`"),
    }
}

fn test_6_1_21_err_missing_version_between(
    missing_version: &CsafVersionNumber,
    start: &CsafDateTime,
    end: &CsafDateTime,
) -> ValidationError {
    let start = start.get_raw_string();
    let end = end.get_raw_string();
    ValidationError {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` between `{start}` and `{end}`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        let case_01_intver_1_3_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(
            &CsafVersionNumber::from("2"),
        )]);
        let case_02_intver_2_3_missing_1_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("2")),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("1")),
        ]);
        let case_s01_semver_1_3_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(
            &CsafVersionNumber::from("2.0.0"),
        )]);
        let case_s02_semver_2_3_missing_1_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("2.0.0")),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("1.0.0")),
        ]);
        let case_s03_intver_1_3_2_missing_2_between = Err(vec![test_6_1_21_err_missing_version_between(
            &CsafVersionNumber::from("2"),
            &CsafDateTime::from("2026-03-01T11:00:00.000Z"),
            &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
        )]);
        let case_s04_semver_1_3_2_missing_2_between = Err(vec![test_6_1_21_err_missing_version_between(
            &CsafVersionNumber::from("2.0.0"),
            &CsafDateTime::from("2026-03-01T11:00:00.000Z"),
            &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
        )]);

        let case_s05_intver_3_1_missing_1_before_2_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("3")),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1"),
                &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("2")),
        ]);
        let case_s06_semver_3_1_missing_1_before_2_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("3.0.0")),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1.0.0"),
                &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("2.0.0")),
        ]);

        let case_s07_mixed_versions_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(
            &CsafVersionNumber::from("2"),
        )]);
        let case_intver_3_1_2_5_4_wrong_first_missing_1_2_4 = Err(vec![
            test_6_1_21_err_wrong_first_version(CsafVersionNumber::from("3"), &0),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("4")),
        ]);
        let case_06 = Err(vec![
            test_6_1_21_err_wrong_first_version(CsafVersionNumber::from("4.0.0"), &0),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("5.0.0")),
        ]);
        let case_07 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2"))]);

        // Valid cases for both 2.0 and 2.1
        // case 11: valid intver final start with 1
        // case 12: valid intver draft start with 0
        // case 13: valid semver final start with 1.0.0
        // case s11: empty revision history
        // case s12: valid intver interim start with 1
        // case s13: mixed versioning 1,2,3

        TESTS_2_0.test_6_1_21.expect(
            case_01_intver_1_3_missing_2_at_all.clone(),
            case_02_intver_2_3_missing_1_at_all.clone(),
            case_s01_semver_1_3_missing_2_at_all.clone(),
            case_s02_semver_2_3_missing_1_at_all.clone(),
            case_s03_intver_1_3_2_missing_2_between.clone(),
            case_s04_semver_1_3_2_missing_2_between.clone(),
            case_s05_intver_3_1_missing_1_before_2_at_all.clone(),
            case_s06_semver_3_1_missing_1_before_2_at_all.clone(),
            case_s07_mixed_versions_missing_2_at_all.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        let case_intver_1_3_4_with_timezone_missing_2 = Err(vec![test_6_1_21_err_missing_version_at_all(
            &CsafVersionNumber::from("2"),
        )]);

        TESTS_2_1.test_6_1_21.expect(
            case_intver_1_3_missing_2,
            case_intver_2_3_missing_1,
            case_intver_1_3_4_with_timezone_missing_2,
            case_semver_1_3_missing_2.clone(),
            case_intver_3_1_2_5_4_wrong_first_missing_1_2_4,
            case_06,
            case_07,
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
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
