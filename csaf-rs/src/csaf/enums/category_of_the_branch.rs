/// Enum representing the category of a branch in a product tree.
/// We need a shared type on the trait, as CSAF version 2.0 have fully divergent definitions.
/// CSAF 2.0 has legacy, which 2.1 has not.
/// CSAF 2.1 has platform, which 2.0 has not.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
