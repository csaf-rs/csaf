use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::RevisionTrait;
use std::fmt::Debug;

/// Newtype wrapper around a `Vec<CsafRevisionHistoryItem>`
///
/// Allows for this aggregation to be constructed from a vectors of revisions.
///
/// This wrapper adds no validation of the revision history items (i.e. validity of CsafDateTime)
/// and no validation of the correct version number typing of the revision history as a whole.
#[derive(Clone, Debug)]
pub struct UnvalidatedCsafRevisionHistory(pub Vec<CsafRevisionHistoryItem>);

// TODO: These sorting steps will be moved to the ValidatedCsafRevisionHistory after the refactor
// is done. For now, we need them here to pass our test coverage.
impl UnvalidatedCsafRevisionHistory {
    /// Sorts the revision history items first by date, second by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long the second order key (revision history numbers) are unique
    pub(crate) fn inplace_sort_by_date_then_number(&mut self) {
        self.0
            .sort_unstable_by_key(|item| (item.date.clone(), item.number.clone()));
    }

    /// Sorts the revision history items by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long as the order key (revision history numbers) are unique
    pub(crate) fn inplace_sort_by_number(&mut self) {
        self.0.sort_unstable_by(|a, b| a.number.cmp(&b.number));
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, index: usize) -> Option<&CsafRevisionHistoryItem> {
        self.0.get(index)
    }

    pub(crate) fn first(&self) -> Option<&CsafRevisionHistoryItem> {
        self.0.first()
    }

    pub(crate) fn last(&self) -> Option<&CsafRevisionHistoryItem> {
        self.0.last()
    }
}

impl<T: RevisionTrait> From<&Vec<T>> for UnvalidatedCsafRevisionHistory {
    fn from(vec: &Vec<T>) -> Self {
        let mut items = Vec::with_capacity(vec.len());
        for (i_r, revision) in vec.iter().enumerate() {
            items.push(CsafRevisionHistoryItem {
                path_index: i_r,
                date: revision.get_date(),
                number: revision.get_number(),
            });
        }
        UnvalidatedCsafRevisionHistory(items)
    }
}

impl IntoIterator for UnvalidatedCsafRevisionHistory {
    type Item = CsafRevisionHistoryItem;
    type IntoIter = std::vec::IntoIter<CsafRevisionHistoryItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Simple struct that encapsulates a revision history items:
/// - version number,
/// - date
/// - path index in the json
#[derive(Clone, Debug)]
pub struct CsafRevisionHistoryItem {
    pub path_index: usize,
    pub date: CsafDateTime,
    pub number: CsafVersionNumber,
}
