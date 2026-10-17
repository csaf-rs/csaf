use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{SharingGroupId, SharingGroupName};
use crate::schema::csaf2_1::schema::SharingGroup as SharingGroup21;

/// Trait representing sharing group information
pub trait SharingGroupTrait {
    /// Returns the ID of the sharing group
    fn get_id(&self) -> SharingGroupId;

    /// Returns the optional name of the sharing group
    fn get_name(&self) -> Option<SharingGroupName>;
}

impl SharingGroupTrait for NotPresentInCsaf20 {
    fn get_id(&self) -> SharingGroupId {
        self.into_any()
    }

    fn get_name(&self) -> Option<SharingGroupName> {
        self.into_any()
    }
}

impl SharingGroupTrait for SharingGroup21 {
    fn get_id(&self) -> SharingGroupId {
        (&self.id).into()
    }

    fn get_name(&self) -> Option<SharingGroupName> {
        self.name.as_ref().map(SharingGroupName::from)
    }
}
