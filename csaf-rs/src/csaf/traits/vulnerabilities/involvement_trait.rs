use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Involvement as Involvement20, PartyCategory as PartyCategory20};
use crate::schema::csaf2_1::schema::{Involvement as Involvement21, PartyCategory as PartyCategory21};
use std::ops::Deref;

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

impl WithOptionalDate for Involvement20 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalGroupIds for Involvement20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithOptionalProductIds for Involvement20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl InvolvementTrait for Involvement21 {
    fn get_party(&self) -> PartyCategory21 {
        self.party
    }
}

impl WithOptionalDate for Involvement21 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalGroupIds for Involvement21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Involvement21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}
