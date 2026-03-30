use std::fmt::{Display, Formatter};

/// Enum representing product status groups
#[derive(PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum ProductStatusGroup {
    // first_affected, known_affected, last_affected
    Affected,
    // known_not_affected
    NotAffected,
    // first_fixed, fixed
    Fixed,
    // under_investigation
    UnderInvestigation,
    // unknown
    Unknown,
    // recommended
    Recommended,
}

impl Display for ProductStatusGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatusGroup::Affected => write!(f, "affected"),
            ProductStatusGroup::NotAffected => write!(f, "not affected"),
            ProductStatusGroup::Fixed => write!(f, "fixed"),
            ProductStatusGroup::UnderInvestigation => write!(f, "under investigation"),
            ProductStatusGroup::Unknown => write!(f, "unknown"),
            ProductStatusGroup::Recommended => write!(f, "recommended"),
        }
    }
}
