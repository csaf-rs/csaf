use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait, VersionNumber};
use chrono::{DateTime, Utc};

/// Checks whether the intver version is zero, always `false` for semver
pub fn is_intver_is_zero(version: &VersionNumber) -> bool {
    if let VersionNumber::Integer(version) = version {
        return *version == 0;
    }
    false
}

/// Checks whether the semver major version is zero, always `false` for intver
pub fn is_semver_is_major_zero(version: &VersionNumber) -> bool {
    if let VersionNumber::Semver(version) = version {
        return version.major == 0;
    }
    false
}

/// Checks whether the semver has a pre-release part, always `false` for intver
pub fn is_semver_has_prerelease(version: &VersionNumber) -> bool {
    if let VersionNumber::Semver(version) = version {
        return !version.pre.is_empty();
    }
    false
}

/// A tuple of (revision history path index, date, number)
pub type RevisionHistoryTupleType = (usize, DateTime<Utc>, VersionNumber);

/// Extracts the revision history of the document into a vector of tuples
/// containing (revision history path index, date, number)
pub fn generate_revision_history_tuples(doc: &impl CsafTrait) -> Vec<RevisionHistoryTupleType> {
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    let mut path_date_number_vec: Vec<RevisionHistoryTupleType> = Vec::new();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let date = DateTime::parse_from_rfc3339(revision.get_date()).map(|dt| dt.with_timezone(&Utc));
        if let Ok(date) = date {
            let rev_num = revision.get_number();
            path_date_number_vec.push((i_r, date, rev_num));
        }
    }
    path_date_number_vec
}

/// Sorts the revision history tuples first by date, second by number
pub fn sort_revision_history_tuples_by_date_by_number(tuples: &mut Vec<RevisionHistoryTupleType>) {
    tuples.sort_unstable_by_key(|item| (item.1, item.2.clone()));
}

/// Sorts the revision history tuples by number
pub fn sort_revision_history_tuples_by_number(tuples: &mut Vec<RevisionHistoryTupleType>) {
    tuples.sort_unstable_by(|a, b| a.2.cmp(&b.2));
}
