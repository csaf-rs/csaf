use strum::{AsRefStr, Display};

use crate::csaf::enums::product_status::ProductStatus;

/// Enum representing product status groups
#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd, Display, AsRefStr)]
pub enum ProductStatusGroup {
    // first_affected, known_affected, last_affected
    #[strum(serialize = "affected")]
    Affected,
    // known_not_affected
    #[strum(serialize = "not affected")]
    NotAffected,
    // first_fixed, fixed
    #[strum(serialize = "fixed")]
    Fixed,
    // under_investigation
    #[strum(serialize = "under investigation")]
    UnderInvestigation,
    // unknown
    #[strum(serialize = "unknown")]
    Unknown,
    // recommended
    #[strum(serialize = "recommended")]
    Recommended,
}

impl From<&ProductStatus> for ProductStatusGroup {
    fn from(status: &ProductStatus) -> Self {
        match status {
            ProductStatus::FirstAffected | ProductStatus::KnownAffected | ProductStatus::LastAffected => {
                Self::Affected
            },
            ProductStatus::KnownNotAffected => Self::NotAffected,
            ProductStatus::Fixed | ProductStatus::FirstFixed => Self::Fixed,
            ProductStatus::UnderInvestigation => Self::UnderInvestigation,
            ProductStatus::Unknown => Self::Unknown,
            ProductStatus::Recommended => Self::Recommended,
        }
    }
}
