use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::WithDate;
use crate::schema::csaf2_0::schema::Revision as Revision20;
use crate::schema::csaf2_1::schema::Revision as Revision21;

/// Trait for accessing revision history entry information
pub trait RevisionTrait: WithDate {
    /// Returns the number/identifier of this revision
    fn get_number(&self) -> CsafVersionNumber;

    /// Returns the summary of changes in this revision
    fn get_summary(&self) -> &String;
}

impl RevisionTrait for Revision20 {
    fn get_number(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.number)
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }
}

impl RevisionTrait for Revision21 {
    fn get_number(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.number)
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }
}

impl WithDate for Revision20 {
    fn get_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.date)
    }
}

impl WithDate for Revision21 {
    fn get_date(&self) -> CsafDateTime {
        CsafDateTime::from(&self.date)
    }
}
