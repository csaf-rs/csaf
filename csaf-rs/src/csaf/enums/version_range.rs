use std::fmt::{Debug, Display};
use vers_like_specifier::{Vls, VlsError};
use vers_rs::{Comparator, DynamicVersionRange, VersError, VersionConstraint, VersionRange as VersRange};

#[derive(Debug)]
pub enum VersionRange {
    /// A dynamic VERS version range
    DynamicVers(DynamicVersionRange),
    /// A vers-like version range
    Vls(Vls),
    /// An invalid version range, containing the error
    Invalid(VersionRangeError),
}

impl Display for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionRange::DynamicVers(vers) => write!(f, "{}", vers),
            VersionRange::Vls(vls) => write!(f, "{}", vls),
            VersionRange::Invalid(err) => write!(f, "Invalid version range: {:?}", err),
        }
    }
}

#[derive(Debug)]
pub enum VersionRangeError {
    InvalidVers(VersError),
    InvalidVls(VlsError),
}

impl Display for VersionRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionRangeError::InvalidVers(err) => write!(f, "Invalid vers version range: {}", err),
            VersionRangeError::InvalidVls(err) => write!(f, "Invalid vls version range: {}", err),
        }
    }
}

impl VersionRange {
    pub fn parse(s: &str) -> Self {
        if s.starts_with("vers:") {
            // parse as vers
            match s.parse::<DynamicVersionRange>() {
                // valid vers
                Ok(vers_parsed) => Self::DynamicVers(vers_parsed),
                // invalid vers
                Err(vers_error) => Self::Invalid(VersionRangeError::InvalidVers(vers_error)),
            }
        } else {
            // parse as vls
            match s.parse::<Vls>() {
                // valid vls
                Ok(vls_parsed) => Self::Vls(vls_parsed),
                // invalid vls
                Err(vls_error) => Self::Invalid(VersionRangeError::InvalidVls(vls_error)),
            }
        }
    }

    /// Returns `Some(true)` if the given version range only matches to a single version.
    pub fn is_single_version(&self) -> Option<bool> {
        match self {
            VersionRange::DynamicVers(vers) => Some(is_single_version_constraint(vers.constraints())),
            VersionRange::Vls(vls) => Some(vls.is_single_version()),
            VersionRange::Invalid(_) => None,
        }
    }
}

/// Returns `true` if the given vers constraints only match with a single version.
fn is_single_version_constraint<V: vers_rs::constraint::VersionType>(constraints: &[VersionConstraint<V>]) -> bool {
    constraints.len() == 1 && matches!(constraints[0].comparator, Comparator::Equal)
}
