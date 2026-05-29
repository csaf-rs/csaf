use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::WithDate;
use crate::schema::csaf2_0::schema::Revision as Revision20;
use crate::schema::csaf2_1::schema::Revision as Revision21;

/// Trait for accessing revision history entry information
pub trait RevisionTrait: WithDate {
    /// Returns the number/identifier of this revision
    fn get_number(&self) -> CsafVersionNumber;

    /// Returns the summary of changes in this revision
    fn get_summary(&self) -> &str;
}

impl RevisionTrait for Revision20 {
    fn get_number(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.number)
    }

    impl_str_field_getter!(get_summary, summary);
}

impl RevisionTrait for Revision21 {
    fn get_number(&self) -> CsafVersionNumber {
        CsafVersionNumber::from(&self.number)
    }

    impl_str_field_getter!(get_summary, summary);
}

crate::csaf::traits::impl_with_date!(Revision20);
crate::csaf::traits::impl_with_date!(Revision21);
