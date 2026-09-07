use strum::{AsRefStr, Display};

/// Enum representing individual product statuses in a CSAF document.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd, Display, AsRefStr)]
pub enum ProductStatus {
    #[strum(serialize = "first_affected")]
    FirstAffected,
    #[strum(serialize = "first_fixed")]
    FirstFixed,
    #[strum(serialize = "fixed")]
    Fixed,
    #[strum(serialize = "known_affected")]
    KnownAffected,
    #[strum(serialize = "known_not_affected")]
    KnownNotAffected,
    #[strum(serialize = "last_affected")]
    LastAffected,
    #[strum(serialize = "recommended")]
    Recommended,
    #[strum(serialize = "under_investigation")]
    UnderInvestigation,
    #[strum(serialize = "unknown")]
    Unknown,
}
