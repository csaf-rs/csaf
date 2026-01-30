use std::fmt::{Debug};
use crate::csaf::types::csaf_datetime::{CsafDateTime};
use crate::csaf::types::csaf_version_number::{CsafVersionNumber};
use crate::csaf_traits::RevisionTrait;

// ===========================================================================
// Raw Revision History
// ===========================================================================

#[derive(Clone, Debug)]
pub struct RevisionHistory {
    pub items: Vec<CsafRevisionHistoryItem>,
}

impl<'c, T: RevisionTrait> From<&'c Vec<T>> for RevisionHistory {
    fn from(vec: &'c Vec<T>) -> Self {
        let mut items = Vec::with_capacity(vec.len());
        for (i_r, revision) in vec.iter().enumerate() {
            items.push(CsafRevisionHistoryItem {
                path_index: i_r,
                date: revision.get_date(),
                number: revision.get_number(),
            });
        }
        RevisionHistory { items }
    }
}


#[derive(Clone, Debug)]
pub struct CsafRevisionHistoryItem {
    pub path_index: usize,
    pub date: CsafDateTime,
    pub number: CsafVersionNumber,
}

impl IntoIterator for RevisionHistory {
    type Item = CsafRevisionHistoryItem;
    type IntoIter = std::vec::IntoIter<CsafRevisionHistoryItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}



