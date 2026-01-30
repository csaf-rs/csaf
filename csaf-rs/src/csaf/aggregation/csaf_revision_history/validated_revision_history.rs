use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Index;
use crate::csaf::aggregation::csaf_revision_history::revision_history::{RevisionHistory};
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_dates::{RevisionHistoryDateErrors, ValidatedRevisionHistoryDates};
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_numbers::{RevisionHistoryVersionParsingErrors, RevisionHistoryVersionTypeMismatchErrors, ValidatedRevisionHistoryNumbers};
use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf::types::csaf_version_number::{CsafVersionNumber, IntVerVersion, SemVerVersion, ValidVersionNumber};
use crate::validation::ValidationError;

#[derive(Clone, Debug)]
pub enum ValidatedRevisionHistory<'a> {
    Valid(ValidRevisionHistory<'a>),
    Invalid(RevisionHistoryParseErrors<'a>),
}

impl<'a> From<&'a RevisionHistory> for ValidatedRevisionHistory<'a> {
    fn from(raw: &'a RevisionHistory) -> Self {

        let date_errors= ValidatedRevisionHistoryDates::from(raw);
        let version_errors= ValidatedRevisionHistoryNumbers::from(raw);

        if let (ValidatedRevisionHistoryDates::Invalid(date_errors), ValidatedRevisionHistoryNumbers::Invalid(version_errors)) = (date_errors, version_errors) {
            let parse_errors = RevisionHistoryParseErrors {
                date_parsing_errors: Some(date_errors),
                version_parsing_errors: version_errors.parsing_errors,
                version_mismatch_errors: version_errors.type_mismatch_errors,
            };
            return ValidatedRevisionHistory::Invalid(parse_errors);
        }

        let mut revision_history: Option<ValidRevisionHistory> = None;
        for (revision_index, revision) in raw.items.iter().enumerate() {
            match (&revision.date, &revision.number) {
                (CsafDateTime::Valid(date), CsafVersionNumber::Valid(number)) => {
                    // Both date and number are valid, continue checking
                    match revision_history {
                        None => {
                            // Initialize the revision history based on the version number type
                            match &number {
                                ValidVersionNumber::IntVer(_) => {
                                    revision_history = Some(ValidRevisionHistory::IntVer(
                                        TypedValidCsafRevisionHistory::new(),
                                    ));
                                }
                                ValidVersionNumber::SemVer(_) => {
                                    revision_history = Some(ValidRevisionHistory::SemVer(
                                        TypedValidCsafRevisionHistory::new(),
                                    ));
                                }
                            }
                        }
                        Some(_) => {
                            // Revision History already initialized
                        }
                    }
                    let Some(revision_history) = &mut revision_history else {
                        unreachable!("Initialized above, this should not happen");
                    };
                    revision_history.push(revision_index, date, number);
                }
                _ => {
                    unreachable!("This should not happen, as invalid dates or version numbers should have been caught above and resulted in an Invalid ValidatedRevisionHistory.");
                }
            }
        }

        match revision_history {
            None => {
                unreachable!("This should not happen, as there should be at least one revision history item by definition of the schema, if that item contained an error, it should have been caught above and resulted in an Invalid ValidatedRevisionHistory.");
            }
            Some(revision_history) => {
                ValidatedRevisionHistory::Valid(revision_history)
            }
        }
    }
}


/// Collection of errors encountered while parsing revision history
#[derive(Debug, Clone)]
pub struct RevisionHistoryParseErrors<'a> {
    /// RFC3339 date parsing errors
    pub date_parsing_errors: Option<RevisionHistoryDateErrors<'a>>,
    /// Version number parsing errors
    pub version_parsing_errors: Option<RevisionHistoryVersionParsingErrors<'a>>,
    /// Version type mismatch errors
    pub version_mismatch_errors: Option<RevisionHistoryVersionTypeMismatchErrors>,
}

impl<'a> From<RevisionHistoryParseErrors<'a>> for Vec<ValidationError> {
    fn from(value: RevisionHistoryParseErrors<'a>) -> Self {
        value.date_parsing_errors
            .into_iter()
            .flat_map(|errs| errs.0.into_iter().map(|e| e.into()))
            .chain(
                value.version_parsing_errors
                    .into_iter()
                    .flat_map(|errs| errs.0.into_iter().map(|e| e.into()))
            )
            .chain(
                value.version_mismatch_errors
                    .into_iter()
                    .flat_map(|errs| errs.0.into_iter().map(|e| e.into()))
            )
            .collect()
    }
}



// ===========================================================================
// Valid CsafRevisionHistory
// ===========================================================================

#[derive(Clone, Debug)]
pub enum ValidRevisionHistory<'a> {
    IntVer(TypedValidCsafRevisionHistory<'a, IntVerVersion>),
    SemVer(TypedValidCsafRevisionHistory<'a, SemVerVersion>),
}

impl<'a> ValidRevisionHistory<'a> {
    pub fn push(&mut self, path_index: usize, date: &'a ValidCsafDateTime, number: &'a ValidVersionNumber) {
        match self {
            ValidRevisionHistory::IntVer(intver_history) => {
                let item = ValidRevisionHistoryItem {
                    path_index,
                    date,
                    number: match number {
                        ValidVersionNumber::IntVer(number) => number,
                        _ => panic!("Tried to push a non-IntVer version into an IntVer revision history. This should be prevented by the revision parser. (This looks like a dev error)."),
                    },
                };
                intver_history.push(item);
            }
            ValidRevisionHistory::SemVer(semver_history) => {
                let item = ValidRevisionHistoryItem {
                    path_index,
                    date,
                    number: match number {
                        ValidVersionNumber::SemVer(number) => number,
                        _ => panic!("Tried to push a non-SemVer version into a SemVer revision history. This should be prevented by the revision parser. (This looks like a dev error)."),
                    },
                };
                semver_history.push(item);
            }
        }
    }
}

// Marker traits for the version types, so I can use them as generics
pub trait VersionNumberKind:
Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Hash {
    fn get_major(&self) -> u64;

}

impl VersionNumberKind for IntVerVersion {
    fn get_major(&self) -> u64 {
        self.get()
    }
}

impl VersionNumberKind for SemVerVersion {
    fn get_major(&self) -> u64 {
        self.get_major()
    }
}

#[derive(Clone, Debug)]
pub struct TypedValidCsafRevisionHistory<'a, V: VersionNumberKind> {
    pub items: Vec<ValidRevisionHistoryItem<'a, V>>,
}

/// Struct representing a revision history item
/// Includes the path index in the original revision history and the date
#[derive(Clone, Debug)]
pub struct ValidRevisionHistoryItem<'a, V: VersionNumberKind> {
    pub path_index: usize,
    pub date: &'a ValidCsafDateTime,
    pub number: &'a V,
}

impl<'a, V: VersionNumberKind> TypedValidCsafRevisionHistory<'a, V> {
    pub fn new() -> Self {
        TypedValidCsafRevisionHistory { items: Vec::new() }
    }

    /// Adds a revision history item
    pub fn push(&mut self, item: ValidRevisionHistoryItem<'a, V>) {
        self.items.push(item);
    }

    /// Returns an iterator over the items
    pub fn iter(&self) -> impl Iterator<Item = &ValidRevisionHistoryItem<'a, V>> {
        self.items.iter()
    }

    /// Returns a sorted vector of references to the revision history items by date
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys
    ///
    /// ATTENTION: Only use this function if you DO NOT CARE about the order of items by number for the same date.
    /// That's what the function sorted_by_date_by_number is for.
    pub fn get_sorted_by_date(&self) -> SortedRevisionHistory<'_, V> {
        let mut refs: Vec<_> = self.items.iter().collect();
        refs.sort_unstable_by(|a, b| a.date.cmp(b.date));
        SortedRevisionHistory::new(refs)
    }

    /// Returns a sorted vector of references to the revision history items, first by date, second by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyway, as long the second order key (revision history numbers) are unique
    ///
    /// Note: Items with invalid version numbers are sorted last
    pub fn get_sorted_by_date_by_number(&self) -> SortedRevisionHistory<'_, V> {
        let mut refs: Vec<_> = self.items.iter().collect();
        refs.sort_unstable_by(|a, b| {
            a.date.cmp(b.date).then_with(|| a.number.cmp(b.number))
        });
        SortedRevisionHistory::new(refs)
    }

    /// Returns a sorted vector of references to the revision history items by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyway, as long as the order key (revision history numbers) are unique
    ///
    /// Note: Items with invalid version numbers are sorted last
    pub fn get_sorted_by_number(&self) -> SortedRevisionHistory<'_, V> {
        let mut refs: Vec<_> = self.items.iter().collect();
        refs.sort_unstable_by(|a, b| a.number.cmp(b.number));
        SortedRevisionHistory::new(refs)
    }
}

impl<'a, V: VersionNumberKind> Default for TypedValidCsafRevisionHistory<'a, V> {
     fn default() -> Self {
         Self::new()
     }
}

/// Allow iterating over RevisionHistory with for loops
impl<'a, V: VersionNumberKind> IntoIterator for TypedValidCsafRevisionHistory<'a, V> {
    type Item = ValidRevisionHistoryItem<'a, V>;
    type IntoIter = std::vec::IntoIter<ValidRevisionHistoryItem<'a, V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// A sorted view of revision history items
///
/// This struct wraps a sorted vector of references to revision history items
/// and provides `first()` and `last()` methods that panic on empty (which should
/// never happen as revision history can't be empty by schema definition).
#[derive(Clone, Debug)]
pub struct SortedRevisionHistory<'a, V: VersionNumberKind>(Vec<&'a ValidRevisionHistoryItem<'a, V>>);

impl<'a, V: VersionNumberKind> SortedRevisionHistory<'a, V> {
    /// Creates a new SortedRevisionHistory from a vector of references
    fn new(items: Vec<&'a ValidRevisionHistoryItem<'a, V>>) -> Self {
        SortedRevisionHistory(items)
    }

    /// Returns an iterator over the items
    pub fn iter(&self) -> impl Iterator<Item = &&'a ValidRevisionHistoryItem<'a, V>> {
        self.0.iter()
    }

    /// Returns a reference to the first item
    pub fn first(&self) -> &ValidRevisionHistoryItem<'a, V> {
        self.0.first().unwrap_or_else(|| panic!("You tried to get the first item of an empty SortedRevisionHistory. RevisionHistory can't be empty by the schema. (This looks like a dev error)."))
    }

    /// Returns a reference to the last item
    pub fn last(&self) -> &ValidRevisionHistoryItem<'a, V> {
        self.0.last().unwrap_or_else(|| panic!("You tried to get the last item of an empty SortedRevisionHistory. RevisionHistory can't be empty by the schema. (This looks like a dev error)."))
    }
}

impl<'a, V: VersionNumberKind> IntoIterator for SortedRevisionHistory<'a, V> {
    type Item = &'a ValidRevisionHistoryItem<'a, V>;
    type IntoIter = std::vec::IntoIter<&'a ValidRevisionHistoryItem<'a, V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, V: VersionNumberKind> Index<usize> for SortedRevisionHistory<'a, V> {
    type Output = ValidRevisionHistoryItem<'a, V>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0[index]
    }
}

