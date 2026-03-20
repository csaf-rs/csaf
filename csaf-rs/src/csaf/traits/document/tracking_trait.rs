use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::csaf_datetime::CsafDateTime::{Invalid, Valid};
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{GeneratorTrait, RevisionTrait, WithDate};
use crate::schema::csaf2_0::schema::{
    DocumentGenerator as DocumentGenerator20, DocumentStatus as DocumentStatus20, Revision as Revision20,
    Tracking as Tracking20,
};
use crate::schema::csaf2_1::schema::{
    DocumentGenerator as DocumentGenerator21, DocumentStatus as DocumentStatus21, Revision as Revision21,
    Tracking as Tracking21,
};
use chrono::{DateTime, Utc};
use std::ops::Deref;

/// Type alias for a vector of revision history items
pub type RevisionHistory = Vec<RevisionHistoryItem>;

/// Struct representing a revision history item
/// Includes the path index in the original revision history, the date, and the version number
#[derive(Clone)]
pub struct RevisionHistoryItem {
    pub path_index: usize,
    pub date_string: String,
    pub date: DateTime<Utc>,
    pub number: CsafVersionNumber,
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
    fn get_generator(&self) -> &Option<Self::GeneratorType>;

    /// Returns the revision history for this document
    fn get_revision_history(&self) -> &Vec<Self::RevisionType>;

    /// Utility function to get revision history as structs containing revision history path index, date and number
    fn get_revision_history_tuples(&self) -> RevisionHistory {
        let mut revision_history: RevisionHistory = Vec::new();
        for (i_r, revision) in self.get_revision_history().iter().enumerate() {
            match revision.get_date() {
                Valid(valid_date) => {
                    revision_history.push(RevisionHistoryItem {
                        path_index: i_r,
                        date: valid_date.get_as_utc().to_owned(),
                        date_string: valid_date.get_raw_string().to_string(),
                        number: revision.get_number(),
                    });
                },
                Invalid(error) => {
                    panic!("{}", error)
                },
            }
        }
        revision_history
    }

    /// Returns the status of this document
    fn get_status(&self) -> DocumentStatus21;

    /// Returns the tracking ID of this document
    fn get_id(&self) -> &String;

    fn get_version(&self) -> CsafVersionNumber;
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

    fn get_generator(&self) -> &Option<Self::GeneratorType> {
        &self.generator
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

    fn get_id(&self) -> &String {
        self.id.deref()
    }

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

    fn get_generator(&self) -> &Option<Self::GeneratorType> {
        &self.generator
    }

    fn get_revision_history(&self) -> &Vec<Self::RevisionType> {
        &self.revision_history
    }

    fn get_status(&self) -> DocumentStatus21 {
        self.status
    }

    fn get_id(&self) -> &String {
        self.id.deref()
    }

    fn get_version(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.version)
    }
}
