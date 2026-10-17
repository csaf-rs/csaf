use crate::schema::csaf2_1::schema::SharingGroupId as SharingGroupId21;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SharingGroupId {
    Max,
    Nil,
    Other(String),
}

impl SharingGroupId {
    pub const MAX_UUID: &str = "ffffffff-ffff-ffff-ffff-ffffffffffff";
    pub const NIL_UUID: &str = "00000000-0000-0000-0000-000000000000";

    pub fn is_max(&self) -> bool {
        matches!(self, Self::Max)
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }
}

impl From<&SharingGroupId21> for SharingGroupId {
    fn from(value: &SharingGroupId21) -> Self {
        match value.to_string().as_str() {
            Self::MAX_UUID => Self::Max,
            Self::NIL_UUID => Self::Nil,
            value => Self::Other(value.to_owned()),
        }
    }
}
