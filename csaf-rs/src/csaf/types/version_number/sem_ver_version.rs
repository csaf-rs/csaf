use semver::{BuildMetadata, Prerelease, Version};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Newtype wrapper for semantic versioning (e.g., "1.2.3-alpha+001") around
/// `semver::Version`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemVerVersion(Version);

impl SemVerVersion {
    /// Get the semver::Version instance
    pub fn get_version(&self) -> &Version {
        &self.0
    }

    /// Returns the major version number
    pub fn get_major(&self) -> u64 {
        self.0.major
    }

    /// Returns the minor version number
    pub fn get_minor(&self) -> u64 {
        self.0.minor
    }

    /// Returns the patch version number
    pub fn get_patch(&self) -> u64 {
        self.0.patch
    }

    /// Returns true if the version has a prerelease tag
    pub fn has_prerelease(&self) -> bool {
        !self.0.pre.is_empty()
    }

    /// Returns the prerelease tag
    pub fn get_prerelease(&self) -> &Prerelease {
        &self.0.pre
    }

    /// Returns true if the version has build metadata
    pub fn has_build_metadata(&self) -> bool {
        !self.0.build.is_empty()
    }

    // Returns the build metadata
    pub fn get_build_metadata(&self) -> &BuildMetadata {
        &self.0.build
    }
}

impl Display for SemVerVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl SemVerVersion {
    /// Creates a new `SemVerVersion` from a `semver::Version`.
    /// Only available within the version_number module.
    pub(super) fn new(value: Version) -> Self {
        SemVerVersion(value)
    }
}

#[cfg(test)]
// This is only used for testing and not available on the public API
impl From<Version> for SemVerVersion {
    fn from(value: Version) -> Self {
        SemVerVersion::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_version() {
        let version_str = "1.2.3-alpha+001";
        let version = Version::parse(version_str).unwrap();
        let semver_version = SemVerVersion::from(version);

        assert_eq!(semver_version.get_major(), 1);
        assert_eq!(semver_version.get_minor(), 2);
        assert_eq!(semver_version.get_patch(), 3);
        assert!(semver_version.has_prerelease());
        assert_eq!(semver_version.get_prerelease().to_string(), "alpha");
        assert!(semver_version.has_build_metadata());
        assert_eq!(semver_version.get_build_metadata().to_string(), "001");
        assert_eq!(semver_version.to_string(), version_str);
    }
}
