use crate::schema::csaf2_0::schema::CategoryOfTheBranch as CategoryOfTheBranch20;
use crate::schema::csaf2_1::schema::CategoryOfTheBranch as CategoryOfTheBranch21;
use strum::{AsRefStr, Display};

/// Enum representing the category of a branch in a product tree.
///
/// We need a shared type on the trait, as CSAF version 2.0 have fully divergent definitions.
/// CSAF 2.0 has legacy, which 2.1 has not.
/// CSAF 2.1 has platform, which 2.0 has not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr)]
pub enum CategoryOfTheBranch {
    #[strum(serialize = "architecture")]
    Architecture,
    #[strum(serialize = "host_name")]
    HostName,
    #[strum(serialize = "language")]
    Language,
    #[strum(serialize = "legacy")]
    Legacy,
    #[strum(serialize = "patch_level")]
    PatchLevel,
    #[strum(serialize = "platform")]
    Platform,
    #[strum(serialize = "product_family")]
    ProductFamily,
    #[strum(serialize = "product_name")]
    ProductName,
    #[strum(serialize = "product_version")]
    ProductVersion,
    #[strum(serialize = "product_version_range")]
    ProductVersionRange,
    #[strum(serialize = "service_pack")]
    ServicePack,
    #[strum(serialize = "specification")]
    Specification,
    #[strum(serialize = "vendor")]
    Vendor,
}

impl From<CategoryOfTheBranch20> for CategoryOfTheBranch {
    fn from(value: CategoryOfTheBranch20) -> Self {
        match value {
            CategoryOfTheBranch20::Architecture => Self::Architecture,
            CategoryOfTheBranch20::HostName => Self::HostName,
            CategoryOfTheBranch20::Language => Self::Language,
            CategoryOfTheBranch20::Legacy => Self::Legacy,
            CategoryOfTheBranch20::PatchLevel => Self::PatchLevel,
            CategoryOfTheBranch20::ProductFamily => Self::ProductFamily,
            CategoryOfTheBranch20::ProductName => Self::ProductName,
            CategoryOfTheBranch20::ProductVersion => Self::ProductVersion,
            CategoryOfTheBranch20::ProductVersionRange => Self::ProductVersionRange,
            CategoryOfTheBranch20::ServicePack => Self::ServicePack,
            CategoryOfTheBranch20::Specification => Self::Specification,
            CategoryOfTheBranch20::Vendor => Self::Vendor,
        }
    }
}

impl From<CategoryOfTheBranch21> for CategoryOfTheBranch {
    fn from(value: CategoryOfTheBranch21) -> Self {
        match value {
            CategoryOfTheBranch21::Architecture => Self::Architecture,
            CategoryOfTheBranch21::HostName => Self::HostName,
            CategoryOfTheBranch21::Language => Self::Language,
            CategoryOfTheBranch21::PatchLevel => Self::PatchLevel,
            CategoryOfTheBranch21::ProductFamily => Self::ProductFamily,
            CategoryOfTheBranch21::ProductName => Self::ProductName,
            CategoryOfTheBranch21::ProductVersion => Self::ProductVersion,
            CategoryOfTheBranch21::ProductVersionRange => Self::ProductVersionRange,
            CategoryOfTheBranch21::ServicePack => Self::ServicePack,
            CategoryOfTheBranch21::Specification => Self::Specification,
            CategoryOfTheBranch21::Vendor => Self::Vendor,
            CategoryOfTheBranch21::Platform => Self::Platform,
        }
    }
}
