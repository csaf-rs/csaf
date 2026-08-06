use crate::csaf::aggregation::revision_history::UnvalidatedCsafRevisionHistory;
use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::types::csaf_datetime::{CsafDateTime, ValidCsafDateTime};
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{GeneratorTrait, RevisionTrait};
use crate::schema::csaf2_0::schema::{
    DocumentGenerator as DocumentGenerator20, DocumentStatus as DocumentStatus20, Revision as Revision20,
    Tracking as Tracking20,
};
use crate::schema::csaf2_1::schema::{
    DocumentGenerator as DocumentGenerator21, DocumentStatus as DocumentStatus21, Revision as Revision21,
    Tracking as Tracking21,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::sync::LazyLock;

/// Type alias for a vector of revision history items
pub type RevisionHistory = Vec<RevisionHistoryItem>;

/// Generates the canonical filename for a CSAF document from its tracking ID, per section 5.1:
/// lowercase, non-`[+\-a-z0-9]` sequences replaced with `_`, `.json` appended.
fn canonical_filename_from_id(tracking_id: &str) -> String {
    static INVALID_CHARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^+\-a-z0-9]+").unwrap());
    let lowercase_id = tracking_id.to_lowercase();
    let cleaned_id = INVALID_CHARS.replace_all(&lowercase_id, "_");
    format!("{cleaned_id}.json")
}

/// Struct representing a revision history item
/// Includes the path index in the original revision history, the date, and the version number
#[derive(Clone)]
pub struct RevisionHistoryItem {
    pub path_index: usize,
    pub date_string: String,
    pub date: DateTime<Utc>,
    pub number: CsafVersionNumber,
    pub valid_date: ValidCsafDateTime,
}

/// Trait providing sorting functionality for revision history
pub trait RevisionHistorySortable {
    /// Sorts the revision history items first by date, second by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long the second order key (revision history numbers) are unique
    fn inplace_sort_by_date_then_number(&mut self);

    /// Sorts the revision history items by number
    ///
    /// Uses unstable sorting, which might be faster, while not keeping the order of equal keys, which
    /// should be unique anyways, as long as the order key (revision history numbers) are unique
    fn inplace_sort_by_number(&mut self);
}

impl RevisionHistorySortable for RevisionHistory {
    fn inplace_sort_by_date_then_number(&mut self) {
        self.sort_unstable_by_key(|item| (item.date, item.number.clone()));
    }

    fn inplace_sort_by_number(&mut self) {
        self.sort_unstable_by(|a, b| a.number.cmp(&b.number));
    }
}

/// Trait representing tracking information for a CSAF document
pub trait TrackingTrait {
    /// Type representing document generator information
    type GeneratorType: GeneratorTrait;

    /// Type representing revision history entries
    type RevisionType: RevisionTrait;

    /// The release date of this document's latest version
    fn get_current_release_date(&self) -> CsafDateTime;

    /// The initial release date of this document
    fn get_initial_release_date(&self) -> CsafDateTime;

    /// Returns the generator information for this document
    fn get_generator(&self) -> Option<&Self::GeneratorType>;

    /// Returns the revision history for this document
    fn get_revision_history(&self) -> &Vec<Self::RevisionType>;

    /// Aggregate the revision history
    fn aggregate_revision_history(&self) -> UnvalidatedCsafRevisionHistory {
        UnvalidatedCsafRevisionHistory::from(self.get_revision_history())
    }

    /// Returns the status of this document
    fn get_status(&self) -> DocumentStatus21;

    /// Returns the tracking ID of this document
    fn get_id(&self) -> &str;

    /// Generates the canonical filename from this document's tracking ID, per section 5.1:
    /// lowercased, non-`[+\-a-z0-9]` sequences replaced with `_`, `.json` appended.
    fn get_canonical_filename(&self) -> String {
        canonical_filename_from_id(self.get_id())
    }

    fn get_version(&self) -> CsafVersionNumber;

    fn get_aliases(&self) -> Option<Vec<&str>>;
}

impl TrackingTrait for Tracking20 {
    type GeneratorType = DocumentGenerator20;
    type RevisionType = Revision20;

    fn get_current_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.current_release_date)
    }

    fn get_initial_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.initial_release_date)
    }

    fn get_aliases(&self) -> Option<Vec<&str>> {
        self.aliases
            .as_ref()
            .map(|aliases| aliases.iter().map(|a| a.as_str()).collect())
    }

    fn get_generator(&self) -> Option<&Self::GeneratorType> {
        self.generator.as_ref()
    }

    fn get_revision_history(&self) -> &Vec<Self::RevisionType> {
        &self.revision_history
    }

    fn get_status(&self) -> DocumentStatus21 {
        match self.status {
            DocumentStatus20::Draft => DocumentStatus21::Draft,
            DocumentStatus20::Final => DocumentStatus21::Final,
            DocumentStatus20::Interim => DocumentStatus21::Interim,
        }
    }

    impl_str_field_getter!(get_id, id);

    fn get_version(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.version)
    }
}

impl TrackingTrait for Tracking21 {
    type GeneratorType = DocumentGenerator21;
    type RevisionType = Revision21;

    fn get_current_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.current_release_date)
    }

    fn get_initial_release_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.initial_release_date)
    }

    fn get_generator(&self) -> Option<&Self::GeneratorType> {
        self.generator.as_ref()
    }

    fn get_revision_history(&self) -> &Vec<Self::RevisionType> {
        &self.revision_history
    }

    fn get_status(&self) -> DocumentStatus21 {
        self.status
    }

    impl_str_field_getter!(get_id, id);

    fn get_version(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.version)
    }

    fn get_aliases(&self) -> Option<Vec<&str>> {
        self.aliases
            .as_ref()
            .map(|aliases| aliases.iter().map(|a| a.as_str()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "OASIS_CSAF_TC-CSAF_2.0-2021-6-2-11-01",
        "oasis_csaf_tc-csaf_2_0-2021-6-2-11-01.json"
    )]
    #[case("2022_#01-A", "2022_01-a.json")]
    #[case("test###value", "test_value.json")]
    #[case("Test+123-456", "test+123-456.json")]
    fn test_canonical_filename_from_id(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(canonical_filename_from_id(input), expected);
    }
}
