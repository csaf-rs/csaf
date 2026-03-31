use std::fmt::{Display, Formatter};

/// Enum representing the category of a branch in a product tree.
/// We need a shared type on the trait, as CSAF version 2.0 have fully divergent definitions.
/// CSAF 2.0 has legacy, which 2.1 has not.
/// CSAF 2.1 has platform, which 2.0 has not.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CategoryOfTheBranch {
    Architecture,
    HostName,
    Language,
    Legacy,
    PatchLevel,
    Platform,
    ProductFamily,
    ProductName,
    ProductVersion,
    ProductVersionRange,
    ServicePack,
    Specification,
    Vendor,
}

impl Display for CategoryOfTheBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryOfTheBranch::Architecture => write!(f, "architecture"),
            CategoryOfTheBranch::HostName => write!(f, "host_name"),
            CategoryOfTheBranch::Language => write!(f, "language"),
            CategoryOfTheBranch::Legacy => write!(f, "legacy"),
            CategoryOfTheBranch::PatchLevel => write!(f, "patch_level"),
            CategoryOfTheBranch::Platform => write!(f, "platform"),
            CategoryOfTheBranch::ProductFamily => write!(f, "product_family"),
            CategoryOfTheBranch::ProductName => write!(f, "product_name"),
            CategoryOfTheBranch::ProductVersion => write!(f, "product_version"),
            CategoryOfTheBranch::ProductVersionRange => write!(f, "product_version_range"),
            CategoryOfTheBranch::ServicePack => write!(f, "service_pack"),
            CategoryOfTheBranch::Specification => write!(f, "specification"),
            CategoryOfTheBranch::Vendor => write!(f, "vendor"),
        }
    }
}
