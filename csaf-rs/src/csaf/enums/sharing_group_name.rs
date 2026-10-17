use crate::schema::csaf2_1::schema::SharingGroupName as SharingGroupName21;
use std::fmt;

/// A sharing-group name with the defined CSAF semantics, or another name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SharingGroupName {
    Public,
    Private,
    Other(String),
}

impl SharingGroupName {
    const SG_NAME_PUBLIC: &str = "Public";
    const SG_NAME_PRIVATE: &str = "No sharing allowed";

    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public)
    }

    pub fn is_private(&self) -> bool {
        matches!(self, Self::Private)
    }
}

impl fmt::Display for SharingGroupName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => formatter.write_str(Self::SG_NAME_PUBLIC),
            Self::Private => formatter.write_str(Self::SG_NAME_PRIVATE),
            Self::Other(name) => formatter.write_str(name),
        }
    }
}

impl From<&SharingGroupName21> for SharingGroupName {
    fn from(name: &SharingGroupName21) -> Self {
        match name.as_str() {
            Self::SG_NAME_PUBLIC => Self::Public,
            Self::SG_NAME_PRIVATE => Self::Private,
            _ => Self::Other(name.as_str().to_owned()),
        }
    }
}
