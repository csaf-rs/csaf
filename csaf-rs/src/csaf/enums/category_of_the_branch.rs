use strum::{AsRefStr, Display};

/// Enum representing the category of a branch in a product tree.
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
