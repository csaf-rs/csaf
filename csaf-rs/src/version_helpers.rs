use crate::csaf_traits::VersionNumber;

pub fn is_intver_is_zero(version: &VersionNumber) -> bool {
    if let VersionNumber::Integer(version) = version {
        return *version == 0;
    }
    false
}

/// Checks whether the semver major version is zero, always False for intver
pub fn is_semver_is_major_zero(version: &VersionNumber) -> bool {
    if let VersionNumber::Semver(version) = version {
        return version.major == 0;
    }
    false
}

/// Checks whether the semver has a pre-release part, always false for intver
pub fn is_semver_has_prerelease(version: &VersionNumber) -> bool {
    if let VersionNumber::Semver(version) = version {
        return !version.pre.is_empty();
    }
    false
}
