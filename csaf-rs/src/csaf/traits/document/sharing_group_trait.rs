use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::schema::csaf2_1::schema::SharingGroup as SharingGroup21;
use uuid::Uuid;

/// Special name for public sharing groups
pub const SG_NAME_PUBLIC: &str = "Public";
/// Special name for private sharing groups
pub const SG_NAME_PRIVATE: &str = "No sharing allowed";

/// Trait representing sharing group information
pub trait SharingGroupTrait {
    /// Returns the ID of the sharing group
    fn get_id(&self) -> &Uuid;

    /// Returns the optional name of the sharing group
    fn get_name(&self) -> Option<&String>;

    /// Utility function to check if the sharing group name is "Public"
    fn is_name_public(&self) -> bool {
        self.get_name().is_some_and(|name| name == SG_NAME_PUBLIC)
    }

    /// Utility function to check if the sharing group name is "No sharing allowed"
    fn is_name_private(&self) -> bool {
        self.get_name().is_some_and(|name| name == SG_NAME_PRIVATE)
    }
}

impl SharingGroupTrait for NotPresentInCsaf20 {
    fn get_id(&self) -> &Uuid {
        self.into_any()
    }

    fn get_name(&self) -> Option<&String> {
        self.into_any()
    }
}

impl SharingGroupTrait for SharingGroup21 {
    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_name(&self) -> Option<&String> {
        self.name.as_deref()
    }
}
