use vers_rs::{Comparator, VersionConstraint, VersionRange as VersRange, schemes::semver::SemVer};

#[derive(Debug)]
pub enum VersionRange {
    /// A generic semver VERS version range
    GenericVers(vers_rs::GenericVersionRange<SemVer>),
    /// A dynamic VERS version range
    DynamicVers(vers_rs::DynamicVersionRange),
    /// A vers-like version range
    Vls(vers_like_specifier::Vls),
    /// An invalid version range
    Invalid(String),
}

impl VersionRange {
    pub fn new(s: &str) -> Self {
        // Trying dynamic first, because this would give the most accurate vers result.
        if let Ok(vers) = s.parse() {
            Self::DynamicVers(vers)
        } else if let Ok(vers) = s.parse() {
            Self::GenericVers(vers)
        } else if let Ok(vers_like) = s.parse() {
            Self::Vls(vers_like)
        } else {
            Self::Invalid(s.into())
        }
    }

    /// Returns `Some(true)` if the given version range only matches to a single version.
    pub fn is_single_version(&self) -> Option<bool> {
        match self {
            VersionRange::GenericVers(vers) => Some(is_single_version_constraint(&vers.constraints)),
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
