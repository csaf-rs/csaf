use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Involvement as Involvement20, PartyCategory as PartyCategory20};
use crate::schema::csaf2_1::schema::{Involvement as Involvement21, PartyCategory as PartyCategory21};

/// Trait for accessing vulnerability involvement information
pub trait InvolvementTrait: WithOptionalGroupIds + WithOptionalDate + WithOptionalProductIds {
    /// Returns the party associated with this vulnerability involvement
    fn get_party(&self) -> PartyCategory21;
}

impl InvolvementTrait for Involvement20 {
    fn get_party(&self) -> PartyCategory21 {
        match self.party {
            PartyCategory20::Coordinator => PartyCategory21::Coordinator,
            PartyCategory20::Discoverer => PartyCategory21::Discoverer,
            PartyCategory20::Other => PartyCategory21::Other,
            PartyCategory20::User => PartyCategory21::User,
            PartyCategory20::Vendor => PartyCategory21::Vendor,
        }
    }
}

crate::csaf::traits::impl_with_optional_date!(Involvement20);
crate::csaf::traits::impl_optional_ids!(Involvement20, WithOptionalGroupIds, ReturnsEmpty);
crate::csaf::traits::impl_optional_ids!(Involvement20, WithOptionalProductIds, ReturnsEmpty);

impl InvolvementTrait for Involvement21 {
    fn get_party(&self) -> PartyCategory21 {
        self.party
    }
}

crate::csaf::traits::impl_with_optional_date!(Involvement21);
crate::csaf::traits::impl_optional_ids!(Involvement21, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Involvement21, WithOptionalProductIds, ReturnsValues);
