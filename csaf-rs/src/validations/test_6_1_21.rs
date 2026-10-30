use std::collections::{BTreeMap, BTreeSet};

use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::{CsafVersionNumber, ValidCsafVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

/// The version core `(Major, Minor, Patch)` of a revision history item.
///
/// Pre-release and build metadata are not part of the version core and are ignored by this test.
// TODO: Take pre-release and build metadata into account as in 3.1.19.2.
type VersionCore = (u64, u64, u64);

/// The highest Major version a revision history may start with.
const MAX_FIRST_MAJOR_VERSION: u64 = 1;

/// Upper bound of individually reported missing revision history items.
///
/// A single gap can span an arbitrary number of versions (e.g. `1.0.0` followed by `999999.0.0`),
/// so the versions found beyond this limit are replaced by a single summarizing error.
// TODO #923: Clarify with the upstream.
const MAX_MISSING_ITEMS: usize = 100;

/// A revision history item with a valid version number, reduced to the data this test operates on.
struct RevHistoryItem<'a> {
    date: &'a CsafDateTime,
    // the actual version number for rendering in the error messages
    number: ValidCsafVersionNumber<'a>,
    // the normalized version number for comparison and gap detection
    core: VersionCore,
}

impl<'a> RevHistoryItem<'a> {
    fn new(date: &'a CsafDateTime, number: ValidCsafVersionNumber<'a>) -> Self {
        Self {
            date,
            number,
            core: number.get_version_core(),
        }
    }
}

/// A version that is omitted in front of or between items of the revision history sorted by date.
struct Gap<'a> {
    /// date of the item before the gap, `None` if the version is missing in front of the first item
    from: Option<&'a CsafDateTime>,
    /// date of the item after the gap
    to: &'a CsafDateTime,
    /// decides whether the missing version is rendered as integer or semantic version
    template: ValidCsafVersionNumber<'a>,
}

/// Outcome of the gap search over a revision history, limited to `MAX_MISSING_ITEMS` findings.
struct GapSearch<'a> {
    /// the missing versions that were found, at most `MAX_MISSING_ITEMS` entries
    gaps: BTreeMap<VersionCore, Gap<'a>>,
    /// `true` if the search stopped at the limit, so further versions are missing but not listed
    limit_reached: bool,
}

/// 6.1.21 Missing Item in Revision History
///
/// It MUST be tested that items of the revision history do not omit a version number when the items are
/// sorted ascending by `date` and as a second level criteria by `number`.
///
/// Items with integer versioning are mapped to semantic versioning by appending `.0.0`, so Major, Minor and
/// Patch versions all have to be contiguous. Minor and Patch versions MAY end at any point if the next item
/// has a higher Major respectively Minor version.
///
/// It MUST also be tested that the first item in such a sorted list has either the version number 0 or 1 in
/// the case of integer versioning or a Major version of 0 or 1 in the case of semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = doc.get_document().get_tracking().aggregate_revision_history();
    if rev_history_tuples.inplace_sort_by_date_then_number().is_err() {
        return Ok(()); // ToDo #409 precondition failed: invalid revision date, reported by 6.1.37 / schema
    }

    // resolve the version cores once, ignoring invalid version numbers
    let items = rev_history_tuples
        .iter()
        .filter_map(|item| {
            item.number
                .as_valid()
                .map(|number| RevHistoryItem::new(&item.date, number))
        })
        .collect::<Vec<_>>();

    let Some(first) = items.first() else {
        return Ok(()); // ToDo #409 this should be Skipped: Precondition failed
    };

    let mut errors = Vec::new();
    if first.core.0 > MAX_FIRST_MAJOR_VERSION {
        errors.push(test_6_1_21_err_wrong_first_version(&first.number));
    }

    let search = test_6_1_21_collect_gaps(&items);

    // Every version core documented somewhere. Versions published in front of their gap were already
    // filtered out, so a hit here means the version exists but only later in the sorted list.
    let documented = items.iter().map(|item| item.core).collect::<BTreeSet<_>>();

    for (core, gap) in search.gaps {
        let missing_version = test_6_1_21_missing_version_number(core, gap.template);
        errors.push(match (documented.contains(&core), gap.from) {
            (false, _) => test_6_1_21_err_missing_version_at_all(&missing_version),
            (true, Some(from)) => test_6_1_21_err_missing_version_between(&missing_version, from, gap.to),
            (true, None) => test_6_1_21_err_missing_version_before(&missing_version, gap.to),
        });
    }

    // the summary comes last so it reads as a continuation of the listed items
    if search.limit_reached {
        errors.push(test_6_1_21_err_too_many_missing_versions());
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_21, test_6_1_21_missing_item_in_revision_history);

/// Walks the sorted revision history pairwise and maps each missing version to the first gap it occurred in.
///
/// The walk starts with the virtual gap in front of the first item. Items that are not greater than their
/// predecessor do not open a gap themselves, their wrong placement is reported by 6.1.14 and 6.1.22.
///
/// A version that was already published at an earlier position of the sorted list is not missing from a
/// later gap either, it is merely out of order, so it is skipped as well.
///
/// Stops as soon as `MAX_MISSING_ITEMS` missing versions have been collected. The result is then a prefix
/// in walk order, rendered in version order, which is not necessarily the lowest missing versions overall,
/// but is deterministic and keeps the walk bounded.
fn test_6_1_21_collect_gaps<'a>(items: &[RevHistoryItem<'a>]) -> GapSearch<'a> {
    let mut gaps = BTreeMap::new();
    // the version cores of all items in front of the current one
    let mut published = BTreeSet::new();
    let predecessors = std::iter::once(None).chain(items.iter().map(Some));

    for (predecessor, item) in predecessors.zip(items) {
        if let Some(predecessor) = predecessor {
            published.insert(predecessor.core);
            if predecessor.core >= item.core {
                continue;
            }
        }
        for missing in test_6_1_21_missing_versions(predecessor.map(|predecessor| predecessor.core), item.core) {
            if published.contains(&missing) {
                continue;
            }
            // a version that is already recorded does not grow the map, so it never reaches the limit
            if gaps.len() >= MAX_MISSING_ITEMS && !gaps.contains_key(&missing) {
                return GapSearch {
                    gaps,
                    limit_reached: true,
                };
            }
            gaps.entry(missing).or_insert(Gap {
                from: predecessor.map(|predecessor| predecessor.date),
                to: item.date,
                template: predecessor.unwrap_or(item).number,
            });
        }
    }

    GapSearch {
        gaps,
        limit_reached: false,
    }
}

/// Returns the version cores that have to be documented between two items of the sorted revision history.
///
/// `previous` is `None` for the gap in front of the first item. A revision history may start at any version
/// with Major version 0 or 1, so nothing is omitted in that case. Any other entry point omits the whole
/// chain from the initial release `1.0.0` up to the first item.
///
/// The caller has to ensure that `next` is greater than `previous`.
fn test_6_1_21_missing_versions(previous: Option<VersionCore>, next: VersionCore) -> impl Iterator<Item = VersionCore> {
    let (major, minor, patch) = next;
    let (majors, minors, patches) = match previous {
        // a Patch release only omits the Patch versions in between
        Some((prev_major, prev_minor, prev_patch)) if (prev_major, prev_minor) == (major, minor) => {
            (0..0, 0..0, prev_patch.saturating_add(1)..patch)
        },
        // a Minor release may end the Patch versions of the previous Minor version at any point
        Some((prev_major, prev_minor, _)) if prev_major == major => {
            (0..0, prev_minor.saturating_add(1)..minor, 0..patch)
        },
        // a Major release may end the Minor and Patch versions of the previous Major version at any point
        Some((prev_major, _, _)) => (prev_major.saturating_add(1)..major, 0..minor, 0..patch),
        // initial development (`0.y.z`) and the initial release (`1.y.z`) are both valid entry points
        None if major <= MAX_FIRST_MAJOR_VERSION => (0..0, 0..0, 0..0),
        // any other entry point omits the whole chain from the initial release `1.0.0` onwards
        None => (1..major, 0..minor, 0..patch),
    };

    majors
        .map(|missing| (missing, 0, 0))
        .chain(minors.map(move |missing| (major, missing, 0)))
        .chain(patches.map(move |missing| (major, minor, missing)))
}

/// Renders a missing version core as integer version if the neighboring item uses integer versioning and the
/// missing version has no Minor or Patch part, as semantic version otherwise.
fn test_6_1_21_missing_version_number(core: VersionCore, template: ValidCsafVersionNumber<'_>) -> CsafVersionNumber {
    let (major, minor, patch) = core;
    match template {
        ValidCsafVersionNumber::IntVer(_) if (minor, patch) == (0, 0) => {
            CsafVersionNumber::from(major.to_string().as_str())
        },
        _ => CsafVersionNumber::from(format!("{major}.{minor}.{patch}").as_str()),
    }
}

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

// TODO #923: Clarify with the upstream.
fn test_6_1_21_err_too_many_missing_versions() -> TestFinding {
    TestFinding::Error(TestFindingData {
        instance_path: REVISION_HISTORY_PATH.to_string(),
        message: format!(
            "more than {MAX_MISSING_ITEMS} revision history items are missing, only the first \
            {MAX_MISSING_ITEMS} are listed individually to avoid flooding the report."
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_21 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_21 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    // Test helpers
    fn wrong_first(version: &str) -> TestFinding {
        test_6_1_21_err_wrong_first_version(&CsafVersionNumber::from(version).as_valid().unwrap())
    }

    fn at_all(version: &str) -> TestFinding {
        test_6_1_21_err_missing_version_at_all(&CsafVersionNumber::from(version))
    }

    fn before(version: &str, start: &str) -> TestFinding {
        test_6_1_21_err_missing_version_before(&CsafVersionNumber::from(version), &CsafDateTime::from(start))
    }

    fn between(version: &str, start: &str, end: &str) -> TestFinding {
        test_6_1_21_err_missing_version_between(
            &CsafVersionNumber::from(version),
            &CsafDateTime::from(start),
            &CsafDateTime::from(end),
        )
    }

    #[test]
    fn test_6_1_21_missing_versions_entry_point() {
        let missing = |previous, next| test_6_1_21_missing_versions(previous, next).collect::<Vec<_>>();

        // a revision history may start at any version with Major version 0 or 1
        assert!(missing(None, (0, 0, 0)).is_empty());
        assert!(missing(None, (0, 5, 0)).is_empty());
        assert!(missing(None, (1, 0, 0)).is_empty());
        assert!(missing(None, (1, 99, 9)).is_empty());

        // any other entry point omits the whole chain from the initial release `1.0.0` onwards
        assert_eq!(missing(None, (2, 0, 0)), [(1, 0, 0)]);
        assert_eq!(missing(None, (3, 0, 0)), [(1, 0, 0), (2, 0, 0)]);
        assert_eq!(
            missing(None, (3, 2, 2)),
            [(1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 1, 0), (3, 2, 0), (3, 2, 1)]
        );
    }

    #[test]
    fn test_6_1_21_missing_versions_chain() {
        let missing = |previous, next| test_6_1_21_missing_versions(previous, next).collect::<Vec<_>>();

        // consecutive Major, Minor and Patch releases have no gap
        assert!(missing(Some((1, 0, 0)), (2, 0, 0)).is_empty());
        assert!(missing(Some((1, 0, 0)), (1, 1, 0)).is_empty());
        assert!(missing(Some((1, 0, 0)), (1, 0, 1)).is_empty());

        // Minor and Patch versions may end at any point if the next version has a higher Major resp. Minor
        assert!(missing(Some((1, 2, 3)), (2, 0, 0)).is_empty());
        assert!(missing(Some((1, 2, 3)), (1, 3, 0)).is_empty());
        assert!(missing(Some((0, 9, 3)), (1, 0, 0)).is_empty());

        // Major, Minor and Patch versions have to be contiguous
        assert_eq!(missing(Some((4, 0, 1)), (4, 0, 3)), [(4, 0, 2)]);
        assert_eq!(missing(Some((4, 0, 3)), (5, 2, 0)), [(5, 0, 0), (5, 1, 0)]);
        assert_eq!(
            missing(Some((2, 0, 0)), (4, 1, 2)),
            [(3, 0, 0), (4, 0, 0), (4, 1, 0), (4, 1, 1)]
        );
        // initial development versions have to be contiguous as well
        assert_eq!(missing(Some((0, 9, 0)), (0, 9, 3)), [(0, 9, 1), (0, 9, 2)]);
        assert_eq!(missing(Some((0, 1, 0)), (0, 3, 0)), [(0, 2, 0)]);
    }

    #[test]
    fn test_6_1_21_collect_gaps_entry_points() {
        let dates = [
            "2026-03-09T11:00:00.000Z",
            "2026-03-09T12:00:00.000Z",
            "2026-03-09T13:00:00.000Z",
        ]
        .map(CsafDateTime::from);
        let collect_gaps = |numbers: [&str; 3]| {
            let numbers = numbers.map(CsafVersionNumber::from);
            let items = [
                RevHistoryItem::new(&dates[0], numbers[0].as_valid().unwrap()),
                RevHistoryItem::new(&dates[1], numbers[1].as_valid().unwrap()),
                RevHistoryItem::new(&dates[2], numbers[2].as_valid().unwrap()),
            ];
            let search = test_6_1_21_collect_gaps(&items);
            // no pair opens a gap here, so everything is missing in front of the first item
            assert!(search.gaps.values().all(|gap| gap.from.is_none()));
            search.gaps.into_keys().collect::<Vec<_>>()
        };

        assert_eq!(
            collect_gaps(["3.2.2", "0.5.0", "1.0.0"]),
            [(1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 1, 0), (3, 2, 0), (3, 2, 1)]
        );
        assert!(collect_gaps(["1.99.9", "0.5.0", "1.0.0"]).is_empty());
    }

    #[test]
    fn test_6_1_21_collect_gaps_is_limited() {
        let date = CsafDateTime::from("2026-03-09T11:00:00.000Z");
        let first = CsafVersionNumber::from("1.0.0");
        // `1.0.0` followed by `missing + 2` omits exactly `missing` Major versions in between
        let gap_of = |missing: usize| {
            let major = missing + 2;
            CsafVersionNumber::from(format!("{major}.0.0").as_str())
        };
        // the highest Major version that is still listed individually
        let last_listed = MAX_MISSING_ITEMS as u64 + 1;
        let collect_gaps = |last: &CsafVersionNumber| {
            let items = [
                RevHistoryItem::new(&date, first.as_valid().unwrap()),
                RevHistoryItem::new(&date, last.as_valid().unwrap()),
            ];
            let search = test_6_1_21_collect_gaps(&items);
            (search.gaps.len(), search.limit_reached)
        };

        // every missing version is reported up to and including the limit
        assert_eq!(
            collect_gaps(&gap_of(MAX_MISSING_ITEMS - 1)),
            (MAX_MISSING_ITEMS - 1, false)
        );
        assert_eq!(collect_gaps(&gap_of(MAX_MISSING_ITEMS)), (MAX_MISSING_ITEMS, false));
        // the versions beyond the limit are left out, the ones found so far are still reported
        assert_eq!(collect_gaps(&gap_of(MAX_MISSING_ITEMS + 1)), (MAX_MISSING_ITEMS, true));
        // an arbitrarily wide gap stays bounded by the limit
        let widest = CsafVersionNumber::from(format!("{}.0.0", u64::MAX).as_str());
        assert_eq!(collect_gaps(&widest), (MAX_MISSING_ITEMS, true));

        // the collected versions are the ones directly following the last documented item
        let items = [
            RevHistoryItem::new(&date, first.as_valid().unwrap()),
            RevHistoryItem::new(&date, widest.as_valid().unwrap()),
        ];
        let gaps = test_6_1_21_collect_gaps(&items).gaps;
        assert_eq!(gaps.keys().next(), Some(&(2, 0, 0)));
        assert_eq!(gaps.keys().next_back(), Some(&(last_listed, 0, 0)));
    }

    #[test]
    fn test_6_1_21_collect_gaps_skips_already_published_versions() {
        let dates = [
            "2026-03-09T11:00:00.000Z",
            "2026-03-09T12:00:00.000Z",
            "2026-03-09T13:00:00.000Z",
            "2026-03-09T14:00:00.000Z",
        ]
        .map(CsafDateTime::from);
        let collect_gaps = |numbers: [&str; 4]| {
            let numbers = numbers.map(CsafVersionNumber::from);
            let items = [
                RevHistoryItem::new(&dates[0], numbers[0].as_valid().unwrap()),
                RevHistoryItem::new(&dates[1], numbers[1].as_valid().unwrap()),
                RevHistoryItem::new(&dates[2], numbers[2].as_valid().unwrap()),
                RevHistoryItem::new(&dates[3], numbers[3].as_valid().unwrap()),
            ];
            test_6_1_21_collect_gaps(&items).gaps.into_keys().collect::<Vec<_>>()
        };

        // 1.0.0 and 2.0.0 are missing in front of 3.0.0, and 3.0.0 is already published when 1.0.0 jumps to 5.0.0
        assert_eq!(
            collect_gaps(["3.0.0", "2.0.0", "1.0.0", "5.0.0"]),
            [(1, 0, 0), (2, 0, 0), (4, 0, 0)]
        );
        // a later gap below an already published version is still reported
        assert_eq!(
            collect_gaps(["1.0.0", "1.3.0", "1.1.0", "1.1.3"]),
            [(1, 1, 0), (1, 1, 1), (1, 1, 2), (1, 2, 0)]
        );
    }

    #[test]
    fn test_test_6_1_21() {
        TESTS_2_0.test_6_1_21.expect(ExpectedResults_2_0 {
            // 1, 3
            case_01: Err(vec![at_all("2")]),
            // 2, 3
            case_02: Err(vec![wrong_first("2"), at_all("1")]),
            // 1.0.0, 3.0.0
            case_s01: Err(vec![at_all("2.0.0")]),
            // 2.0.0, 3.0.0
            case_s02: Err(vec![wrong_first("2.0.0"), at_all("1.0.0")]),
            // 1, 3, 2 - 2 is documented, but dated after 3
            case_s03: Err(vec![between(
                "2",
                "2026-03-01T11:00:00.000Z",
                "2026-03-03T11:00:00.000Z",
            )]),
            // 1.0.0, 3.0.0, 2.0.0
            case_s04: Err(vec![between(
                "2.0.0",
                "2026-03-01T11:00:00.000Z",
                "2026-03-03T11:00:00.000Z",
            )]),
            // 3, 1 - 1 is documented, but dated after 3
            case_s05: Err(vec![
                wrong_first("3"),
                before("1", "2026-03-03T11:00:00.000Z"),
                at_all("2"),
            ]),
            // 3.0.0, 1.0.0
            case_s06: Err(vec![
                wrong_first("3.0.0"),
                before("1.0.0", "2026-03-03T11:00:00.000Z"),
                at_all("2.0.0"),
            ]),
            // 1, 3.0.0 - the integer predecessor decides the rendering of the missing version
            case_s07: Err(vec![at_all("2")]),

            // Case 11: 1, 2
            // Case 12: 0, 1
            // Case 13: 1.0.0, 1.1.0
            // Case S11: empty revision history
            // Case S12: 1, 2, 3
            // Case S13: mixed versioning 1.1.0, 2, 2.0.1, 3.0.0
            // Case S14: 1, 2, 1 - a repeated lower version is not missing
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });

        // 1, 999999 - only the first MAX_MISSING_ITEMS missing versions are listed individually,
        // everything beyond is replaced by a single summarizing error
        let gap_beyond_limit = (2..=MAX_MISSING_ITEMS + 1)
            .map(|version| at_all(&version.to_string()))
            .chain(std::iter::once(test_6_1_21_err_too_many_missing_versions()))
            .collect::<Vec<_>>();
        TESTS_2_1.test_6_1_21.expect(ExpectedResults_2_1 {
            // 1, 3
            case_01: Err(vec![at_all("2")]),
            // 2, 3
            case_02: Err(vec![wrong_first("2"), at_all("1")]),
            // 1, 3, 4
            case_03: Err(vec![at_all("2")]),
            // 1.0.0, 3.0.0
            case_04: Err(vec![at_all("2.0.0")]),
            // 3, 2, 1, 5, 4 - 1 and 2 are missing in front of 3, only 4 is missing between 1 and 5,
            // because 2 and 3 were already published earlier in the sorted list
            case_05: Err(vec![
                wrong_first("3"),
                before("1", "2023-08-22T10:00:00.000Z"),
                before("2", "2023-08-22T10:00:00.000Z"),
                between("4", "2024-01-21T10:00:00.000Z", "2024-01-21T11:00:00.000Z"),
            ]),
            // 4.0.0, 1.0.0, 2.0.0, 6.0.0, 5.0.0 - 4.0.0 was already published in front of 6.0.0
            case_06: Err(vec![
                wrong_first("4.0.0"),
                before("1.0.0", "2023-08-22T10:00:00.000Z"),
                before("2.0.0", "2023-08-22T10:00:00.000Z"),
                at_all("3.0.0"),
                between("5.0.0", "2024-01-21T10:00:00.000Z", "2024-01-21T11:00:00.000Z"),
            ]),
            // 1, 3
            case_07: Err(vec![at_all("2")]),
            // 1.0.0, 2, 3.0.0, 4.0.0, 4.0.1, 4.0.3, 5.2.0 - the Patch and Minor chains are interrupted
            case_08: Err(vec![at_all("4.0.2"), at_all("5.0.0"), at_all("5.1.0")]),
            // 2.0.0, 3.0.0
            case_s01: Err(vec![wrong_first("2.0.0"), at_all("1.0.0")]),
            // 1, 3.0.0 on the same date
            case_s02: Err(vec![at_all("2")]),
            // 1, 999999 - 2 to 101 are missing, everything beyond the limit is summarized
            case_s03: Err(gap_beyond_limit),

            // Case 11: 1, 2
            // Case 12: 0, 1
            // Case 13: 1.0.0, 1.1.0
            // Case 14: only wrong ordering in json
            // Case 15: only wrong ordering in json due to timezones
            // Case 16: only wrong ordering in json due to timezones
            // Case 17: 1&2 have same date
            // Case 18: complete Major, Minor and Patch chain
            // Case S11: empty revision history
            // Case S12: 1, 2, 3
            // Case S13: mixed versioning 1.1.0, 2, 2.0.1, 3.0.0
            // Case S14: 1, 2, 1 - a repeated lower version is not missing
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
            case_18: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });
    }
}
