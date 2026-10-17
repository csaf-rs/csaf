use crate::schema::csaf2_1::schema::SharingGroupName as SharingGroupName21;

/// Special name for public sharing groups.
pub const SG_NAME_PUBLIC: &str = "Public";
/// Special name for private sharing groups.
pub const SG_NAME_PRIVATE: &str = "No sharing allowed";

/// A sharing-group name with the defined CSAF semantics, or another name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SharingGroupName {
    Public,
    Private,
    Other(String),
}

impl SharingGroupName {
    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public)
    }

    pub fn is_private(&self) -> bool {
        matches!(self, Self::Private)
    }
}

impl From<&SharingGroupName21> for SharingGroupName {
    fn from(name: &SharingGroupName21) -> Self {
        match name.as_str() {
            SG_NAME_PUBLIC => Self::Public,
            SG_NAME_PRIVATE => Self::Private,
            _ => Self::Other(name.as_str().to_owned()),
        }
    }
}
