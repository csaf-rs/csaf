use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::{CsafVersionNumber, ValidCsafVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

/// 6.1.21 Missing Item in Revision History
///
/// It MUST be tested that items of the revision history do not omit a version number when the items are sorted ascending by date.
/// In the case of semantic versioning, this applies only to the Major version.
/// It MUST also be tested that the first item in such a sorted list has either the version number 0 or 1 in
/// the case of integer versioning or a Major version of 0 or 1 in the case of semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    /// A major version that was skipped at some point in the revision history.
    struct Gap<'a> {
        /// date of the item before the version was skipped, `None` if it was skipped by the first item
        from: Option<&'a CsafDateTime>,
        /// date of the item that skipped the version
        to: &'a CsafDateTime,
        /// decides whether the missing version is rendered as integer or semantic version
        template: &'a CsafVersionNumber,
        /// the version showed up later in the revision history, so it is out of order rather than missing at all
        found: bool,
    }

    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = doc.get_document().get_tracking().aggregate_revision_history();
    rev_history_tuples.inplace_sort_by_date_then_number();

    // resolve the major version once, ignoring invalid version numbers
    let mut items = rev_history_tuples
        .iter()
        .filter_map(|item| item.number.as_valid().map(|valid| (item, valid.get_major(), valid)));

    let Some((first, first_major, first_valid)) = items.next() else {
        return Ok(()); // ToDo #409 this should be Skipped: Precondition failed
    };

    let mut errors = Vec::new();
    let mut gaps: BTreeMap<u64, Gap> = BTreeMap::new();

    if first_major > 1 {
        errors.push(test_6_1_21_err_wrong_first_version(&first_valid));
        for major in 1..first_major {
            gaps.insert(
                major,
                Gap {
                    from: None,
                    to: &first.date,
                    template: &first.number,
                    found: false,
                },
            );
        }
    }

    // the first item which reached the highest major version seen so far
    let (mut head, mut head_major) = (first, first_major);
    for (item, major, _) in items {
        match major.cmp(&head_major) {
            Ordering::Less => {
                // a lower version showing up later only matters if it was skipped before
                if let Some(gap) = gaps.get_mut(&major) {
                    gap.found = true;
                }
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                // skip(1) instead of `head_major + 1`, so this can't overflow
                for missing in (head_major..major).skip(1) {
                    gaps.insert(
                        missing,
                        Gap {
                            from: Some(&head.date),
                            to: &item.date,
                            template: &head.number,
                            found: false,
                        },
                    );
                }
                (head, head_major) = (item, major);
            },
        }
    }

    // ToDo aggregate consecutive missing versions into one error message, e.g. "missing revision history items with numbers 2,3,4 between 2026-03-01T11:00:00.000Z and 2026-03-03T11:00:00.000Z"
    for (major, gap) in gaps {
        let missing_version = match gap.template {
            CsafVersionNumber::SemVer(_) => CsafVersionNumber::from(format!("{major}.0.0").as_str()),
            _ => CsafVersionNumber::from(major.to_string().as_str()),
        };
        errors.push(match (gap.found, gap.from) {
            (false, _) => test_6_1_21_err_missing_version_at_all(&missing_version),
            (true, Some(from)) => test_6_1_21_err_missing_version_between(&missing_version, from, gap.to),
            (true, None) => test_6_1_21_err_missing_version_before(&missing_version, gap.to),
        });
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_21, test_6_1_21_missing_item_in_revision_history);

const REVISION_HISTORY_PATH: &str = "/document/tracking/revision_history";

fn test_6_1_21_err_wrong_first_version(version: &ValidCsafVersionNumber) -> TestFinding {
    let expected_version = match version {
        ValidCsafVersionNumber::IntVer(_) => "`0` or `1`",
        ValidCsafVersionNumber::SemVer(_) => "`0.y.z` or `1.y.z`",
    }
    .to_string();

    TestFinding::Error(TestFindingData {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!(
            "revision history does not start with a version of {expected_version} when sorted by date (was `{version}`)"
        ),
    })
}

fn test_6_1_21_err_missing_version_at_all(missing_version: &CsafVersionNumber) -> TestFinding {
    TestFinding::Error(TestFindingData {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` at all"),
    })
}

fn test_6_1_21_err_missing_version_before(missing_version: &CsafVersionNumber, start: &CsafDateTime) -> TestFinding {
    let start = start.get_raw_string();
    TestFinding::Error(TestFindingData {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` before `{start}`"),
    })
}

fn test_6_1_21_err_missing_version_between(
    missing_version: &CsafVersionNumber,
    start: &CsafDateTime,
    end: &CsafDateTime,
) -> TestFinding {
    let start = start.get_raw_string();
    let end = end.get_raw_string();
    TestFinding::Error(TestFindingData {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!("missing revision history item with number `{missing_version}` between `{start}` and `{end}`"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_21 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_21 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        let case_intver_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from(
            "2",
        ))]);
        let case_intver_2_3_wrong_first_and_missing_1_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("2").as_valid().unwrap()),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("1")),
        ]);
        let case_semver_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from(
            "2.0.0",
        ))]);
        let case_semver_2_3_missing_1_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("2.0.0").as_valid().unwrap()),
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
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("3").as_valid().unwrap()),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1"),
                &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("2")),
        ]);
        let case_s06_semver_3_1_missing_1_before_2_at_all = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("3.0.0").as_valid().unwrap()),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1.0.0"),
                &CsafDateTime::from("2026-03-03T11:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("2.0.0")),
        ]);

        let case_mixed_versions_start_with_intver_missing_2_at_all = Err(vec![test_6_1_21_err_missing_version_at_all(
            &CsafVersionNumber::from("2"),
        )]);
        let case_intver_wrong_first_missing_1_and_2_before_4_between = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("3").as_valid().unwrap()),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("2"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_between(
                &CsafVersionNumber::from("4"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
                &CsafDateTime::from("2024-01-21T11:00:00.000Z"),
            ),
        ]);

        let case_semver_wrong_first_missing_1_and_2_before_4_between = Err(vec![
            test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from("4.0.0").as_valid().unwrap()),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("1.0.0"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_before(
                &CsafVersionNumber::from("2.0.0"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
            ),
            test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from("3.0.0")),
            test_6_1_21_err_missing_version_between(
                &CsafVersionNumber::from("5.0.0"),
                &CsafDateTime::from("2023-08-22T10:00:00.000Z"),
                &CsafDateTime::from("2024-01-21T11:00:00.000Z"),
            ),
        ]);

        // Valid cases for both 2.0 and 2.1
        // case 11: valid intver final start with 1
        // case 12: valid intver draft start with 0
        // case 13: valid semver final start with 1.0.0
        // case s11: empty revision history
        // case s12: valid intver interim start with 1
        // case s13: mixed versioning 1,2,3
        // case s14: repeated lower version 1,2,1

        TESTS_2_0.test_6_1_21.expect(ExpectedResults_2_0 {
            case_01: case_intver_missing_2_at_all.clone(),
            case_02: case_intver_2_3_wrong_first_and_missing_1_at_all.clone(),
            case_s01: case_semver_missing_2_at_all.clone(),
            case_s02: case_semver_2_3_missing_1_at_all.clone(),
            case_s03: case_s03_intver_1_3_2_missing_2_between,
            case_s04: case_s04_semver_1_3_2_missing_2_between,
            case_s05: case_s05_intver_3_1_missing_1_before_2_at_all,
            case_s06: case_s06_semver_3_1_missing_1_before_2_at_all,
            case_s07: case_mixed_versions_start_with_intver_missing_2_at_all.clone(),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });

        TESTS_2_1.test_6_1_21.expect(ExpectedResults_2_1 {
            case_01: case_intver_missing_2_at_all.clone(),
            case_02: case_intver_2_3_wrong_first_and_missing_1_at_all,
            case_03: case_intver_missing_2_at_all.clone(),
            case_04: case_semver_missing_2_at_all,
            case_05: case_intver_wrong_first_missing_1_and_2_before_4_between,
            case_06: case_semver_wrong_first_missing_1_and_2_before_4_between,
            case_07: case_intver_missing_2_at_all,
            case_s01: case_semver_2_3_missing_1_at_all,
            case_s02: case_mixed_versions_start_with_intver_missing_2_at_all,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()), // only wrong ordering in json
            case_15: Ok(()), // only wrong ordering in json due to timezones
            case_16: Ok(()), // only wrong ordering in json due to timezones
            case_17: Ok(()), // 1&2 have same date
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });
    }
}
