use vers_rs::{Comparator, schemes::semver::SemVer};

#[derive(Debug)]
pub enum VersionRange {
    /// A VERS version range
    Vers(vers_rs::GenericVersionRange<SemVer>),
    /// A vers-like version range
    Vls(vers_like_specifier::Vls),
    /// An invalid version range
    Invalid(String),
}

impl VersionRange {
    pub fn new(s: &str) -> Self {
        if let Ok(vers) = s.parse() {
            Self::Vers(vers)
        } else if let Ok(vers_like) = s.parse() {
            Self::Vls(vers_like)
        } else {
            Self::Invalid(s.into())
        }
    }

    pub fn is_single_version(&self) -> Option<bool> {
        match self {
            VersionRange::Vers(vers) => {
                let constraints = &vers.constraints;
                let is_single_version =
                    constraints.len() == 1 && matches!(constraints[0].comparator, Comparator::Equal);
                Some(is_single_version)
            },
            VersionRange::Vls(vls) => Some(vls.is_single_version()),
            VersionRange::Invalid(_) => None,
        }
    }
}
