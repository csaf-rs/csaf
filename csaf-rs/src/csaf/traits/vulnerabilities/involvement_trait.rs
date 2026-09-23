use crate::csaf::traits::util::impl_optional_str_field_getter;
use crate::csaf::traits::util::not_present_21::NotPresentInCsaf21;
use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Involvement as Involvement20, PartyCategory as PartyCategory20};

/// Trait for accessing vulnerability involvement information
pub trait InvolvementTrait: WithOptionalGroupIds + WithOptionalDate + WithOptionalProductIds {
    /// Returns the party associated with this vulnerability involvement
    fn get_party(&self) -> PartyCategory20;
    fn get_summary(&self) -> Option<&str>;
}

impl InvolvementTrait for Involvement20 {
    fn get_party(&self) -> PartyCategory20 {
        self.party
    }

    impl_optional_str_field_getter!(get_summary, summary);
}

crate::csaf::traits::impl_with_optional_date!(Involvement20);
crate::csaf::traits::impl_optional_ids!(Involvement20, WithOptionalGroupIds, ReturnsEmpty);
crate::csaf::traits::impl_optional_ids!(Involvement20, WithOptionalProductIds, ReturnsEmpty);



impl InvolvementTrait for NotPresentInCsaf21 {
    fn get_party(&self) -> PartyCategory20 {
        self.into_any()
    }

    fn get_summary(&self) -> Option<&str> {
        self.into_any()
    }
}

