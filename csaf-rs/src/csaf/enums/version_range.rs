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

#[derive(Debug)]
pub enum VersionRangeError {
    InvalidVers(VersError),
    InvalidVls(VlsError),
}

impl VersionRange {
    pub fn parse(s: &str) -> Self {
        // parse as vers
        match s.parse::<DynamicVersionRange>() {
            // valid vers
            Ok(vers_parsed) => return Self::DynamicVers(vers_parsed),
            // the string was prefixed with "vers:", we report the version error
            Err(vers_error) if !matches!(vers_error, VersError::InvalidScheme) => {
                return Self::Invalid(VersionRangeError::InvalidVers(vers_error));
            },
            // the string was not prefix with "vers:", maybe its vls
            Err(_) => {},
        }
        // parse as vls
        match s.parse::<Vls>() {
            // valid vls
            Ok(vls_parsed) => Self::Vls(vls_parsed),
            // invalid vers, invalid vls, so its not a valid version range
            Err(vls_error) => Self::Invalid(VersionRangeError::InvalidVls(vls_error)),
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
