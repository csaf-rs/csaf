use strum::{AsRefStr, Display};

use crate::csaf::enums::product_status::ProductStatus;

/// Enum representing product status groups, as defined in CSAF 2.0 (see 6.1.6) and CSAF 2.1 (see 3.2.4.11).
///
/// This enum is shared between CSAF 2.0 and 2.1.
/// CSAF 2.0 and 2.1 have divergent definitions, with 2.1 adding the `Unknown` group, which does not exist in 2.0.
///
/// For the purpose of this implementation, we also have a "status group" `Recommended`.
/// This group does not exist in either version of CSAF, but allows us to have exhaustive matching and filtering on product group mappings.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd, Display, AsRefStr)]
pub enum ProductStatusGroup {
    /// Product status group containing product status: first_affected, known_affected, last_affected
    #[strum(serialize = "affected")]
    Affected,
    /// Product status group containing product status: known_not_affected
    #[strum(serialize = "not affected")]
    NotAffected,
    /// Product status group containing product status: first_fixed, fixed
    #[strum(serialize = "fixed")]
    Fixed,
    /// Product status group containing product status: under_investigation
    #[strum(serialize = "under investigation")]
    UnderInvestigation,
    /// Product status group containing product status: unknown
    #[strum(serialize = "unknown")]
    Unknown,
    /// Product status group containing product status: recommended
    /// This group does not exist in either version of CSAF, but allows us to have exhaustive matching and filtering on product group mappings.
    #[strum(serialize = "recommended")]
    Recommended,
}

impl From<&ProductStatus> for ProductStatusGroup {
    fn from(status: &ProductStatus) -> Self {
        match status {
            ProductStatus::FirstAffected | ProductStatus::KnownAffected | ProductStatus::LastAffected => Self::Affected,
            ProductStatus::KnownNotAffected => Self::NotAffected,
            ProductStatus::Fixed | ProductStatus::FirstFixed => Self::Fixed,
            ProductStatus::UnderInvestigation => Self::UnderInvestigation,
            ProductStatus::Unknown => Self::Unknown,
            ProductStatus::Recommended => Self::Recommended,
        }
    }
}
