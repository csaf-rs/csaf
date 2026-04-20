use std::fmt::{Display, Formatter};

/// Enum representing individual product statuses in a CSAF document.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum ProductStatus {
    FirstAffected,
    FirstFixed,
    Fixed,
    KnownAffected,
    KnownNotAffected,
    LastAffected,
    Recommended,
    UnderInvestigation,
    Unknown,
}

impl Display for ProductStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatus::FirstAffected => write!(f, "first_affected"),
            ProductStatus::FirstFixed => write!(f, "first_fixed"),
            ProductStatus::Fixed => write!(f, "fixed"),
            ProductStatus::KnownAffected => write!(f, "known_affected"),
            ProductStatus::KnownNotAffected => write!(f, "known_not_affected"),
            ProductStatus::LastAffected => write!(f, "last_affected"),
            ProductStatus::Recommended => write!(f, "recommended"),
            ProductStatus::UnderInvestigation => write!(f, "under_investigation"),
            ProductStatus::Unknown => write!(f, "unknown"),
        }
    }
}
