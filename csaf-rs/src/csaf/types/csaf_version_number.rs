use crate::schema::csaf2_0::schema::VersionT as VersionTCsaf20;
use crate::schema::csaf2_1::schema::VersionT as VersionTCsaf21;
use crate::validation::ValidationError;
use semver::{BuildMetadata, Prerelease, Version};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;


// ============================================================================
// CsafVersionNumber implementation
// ============================================================================

#[derive(Debug, Clone)]
pub enum CsafVersionNumber {
    Valid(ValidVersionNumber),
    Invalid(VersionNumberParsingError),
}

impl CsafVersionNumber {
    pub fn is_valid(&self) -> bool {
        match self {
            CsafVersionNumber::Valid(_) => true,
            CsafVersionNumber::Invalid(_) => false,
        }
    }
}

impl From<&str> for CsafVersionNumber {
    fn from(s: &str) -> Self {
        match s.parse() {
            Ok(v) => CsafVersionNumber::Valid(v),
            Err(e) => CsafVersionNumber::Invalid(e),
        }
    }
}

impl From<&VersionTCsaf20> for CsafVersionNumber {
    fn from(v: &VersionTCsaf20) -> Self {
        CsafVersionNumber::from(v.deref().as_str())
    }
}

impl From<&VersionTCsaf21> for CsafVersionNumber {
    fn from(v: &VersionTCsaf21) -> Self {
        CsafVersionNumber::from(v.deref().as_str())
    }
}

/// Valid version numbers are compared by their parsed values
/// If either or both values are invalid, they are considered unequal
///
/// Also, we do not implement Eq here, as invalid values are also not reflexive, i.e. some invalid value
/// is not equal to itself, or we rather do not care if they are.
impl PartialEq for CsafVersionNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CsafVersionNumber::Valid(a), CsafVersionNumber::Valid(b)) => a == b,
            _ => false,
        }
    }
}

/// Valid version numbers are ordered by their parsed values
/// If either or both values are invalid, there is no ordering
///
/// Also, we do not implement Ord here, as invalid values can't be ordered, or we rather do not care
/// about their ordering.
impl PartialOrd for CsafVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (CsafVersionNumber::Valid(a), CsafVersionNumber::Valid(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

// ===========================================================================
// Error Stuff
// ===========================================================================

/// Error type for version number parsing failures
#[derive(Debug, Clone)]
pub struct VersionNumberParsingError {
    /// The original version string that failed to parse
    pub raw_string: String,
    /// The error message why the parsing failed,
    pub source: String,
}

impl VersionNumberParsingError {
    fn new(raw_string: &str, source: &str) -> Self {
        VersionNumberParsingError {
            raw_string: raw_string.to_owned(),
            source: source.to_owned(),
        }
    }

    pub fn get_validation_error(&self, instance_path: &str) -> ValidationError {
        ValidationError {
            message: self.to_string(),
            instance_path: instance_path.to_string(),
        }
    }
}

impl Display for VersionNumberParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Failed to parse version number '{}': {}",
            self.raw_string, self.source
        )
    }
}

impl std::error::Error for VersionNumberParsingError {}

// ==========================================================================
// VersionNumber Implementation
// ==========================================================================

// --------------------------------------------------------------------------
// IntegerVersion Newtype Wrapper
// --------------------------------------------------------------------------

/// Newtype wrapper for integer versioning (e.g., "1", "42")
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntVerVersion(u64);

impl IntVerVersion {
    /// Returns the version number
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for IntVerVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for IntVerVersion {
    fn from(value: u64) -> Self {
        IntVerVersion(value)
    }
}

// --------------------------------------------------------------------------
// SemverVersion Newtype Wrapper
// --------------------------------------------------------------------------

/// Newtype wrapper for semantic versioning (e.g., "1.2.3-alpha+001")
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemVerVersion(Version);

impl SemVerVersion {
    pub fn get_version(&self) -> &Version {
        &self.0
    }

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

    pub fn get_build_metadata(&self) -> &BuildMetadata {
        &self.0.build
    }
}

impl Display for SemVerVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<Version> for SemVerVersion {
    fn from(value: Version) -> Self {
        SemVerVersion(value)
    }
}

// --------------------------------------------------------------------------
// VersionNumber Enum
// --------------------------------------------------------------------------

/// Enum representing version numbers (integer versioning or semantic versioning)
#[derive(Debug, Clone)]
pub enum ValidVersionNumber {
    IntVer(IntVerVersion),
    SemVer(SemVerVersion),
}

impl ValidVersionNumber {
    pub fn get_major(&self) -> u64 {
        match &self {
            ValidVersionNumber::IntVer(intver) => intver.get(),
            ValidVersionNumber::SemVer(semver) => semver.get_major(),
        }
    }
}

impl FromStr for ValidVersionNumber {
    type Err = VersionNumberParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse as intver, which needs to comply to regex ^(0|[1-9][0-9]*)$
        // This is implemented by:
        // 1. Checking that all chars are digits
        // 2. Checking for '0' as first char in strings with more than 1 char
        // 3. Parsing to u64
        if s.chars().all(|c| c.is_ascii_digit())
            && !(s.len() > 1 && s.starts_with('0'))
            && let Ok(num) = s.parse::<u64>()
        {
            return Ok(ValidVersionNumber::IntVer(IntVerVersion(num)));
        }

        // Try to parse as semver
        if let Ok(semver) = Version::parse(s) {
            return Ok(ValidVersionNumber::SemVer(SemVerVersion(semver)));
        }

        // If both fail, return error
        Err(VersionNumberParsingError::new(
            s,
            "Version could not be parsed as integer versioning or semantic versioning",
        ))
    }
}

impl Display for ValidVersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ValidVersionNumber::IntVer(num) => write!(f, "{num}"),
            ValidVersionNumber::SemVer(version) => write!(f, "{version}"),
        }
    }
}

impl Eq for ValidVersionNumber {}

impl Hash for ValidVersionNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ValidVersionNumber::IntVer(num) => {
                0u8.hash(state); // Discriminator for IntVer
                num.hash(state);
            },
            ValidVersionNumber::SemVer(version) => {
                1u8.hash(state); // Discriminator for SemVer
                version.hash(state);
            },
        }
    }
}

/// VersionNumbers are equal if they are of the same variant and their values are equal
/// Otherwise, they are unequal
/// Also, this relationship is reflexive
impl PartialEq for ValidVersionNumber {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (ValidVersionNumber::IntVer(a), ValidVersionNumber::IntVer(b)) => a == b,
            (ValidVersionNumber::SemVer(a), ValidVersionNumber::SemVer(b)) => a == b,
            // Integer and Semver are always unequal
            (ValidVersionNumber::IntVer(_), ValidVersionNumber::SemVer(_)) => false,
            (ValidVersionNumber::SemVer(_), ValidVersionNumber::IntVer(_)) => false,
        }
    }
}

// TODO: This can be removed once revisionhistory has been typified to only allow one variant
impl Ord for ValidVersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, &other) {
            (ValidVersionNumber::IntVer(a), ValidVersionNumber::IntVer(b)) => a.cmp(b),
            (ValidVersionNumber::SemVer(a), ValidVersionNumber::SemVer(b)) => a.cmp(b),
            _ => {
                panic!("TODO This can be removed once revisionhistory has been typified to only allow one variant");
            },
        }
    }
}

// TODO: This can be removed once revisionhistory has been typified to only allow one variant
impl PartialOrd for ValidVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
/*
TODO: Uncomment this once revisionhistory has been typified to only allow one variant
/// VersionNumbers can be ordered if they are of the same variant
/// Otherwise, there is no ordering
///
/// Also, we do not implement Ord here, as mixed variant comparisons can't be ordered, or we rather do not care
/// about their ordering.
impl PartialOrd for VersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self, &other) {
            (VersionNumber::IntVer(a), VersionNumber::IntVer(b)) => Some(a.cmp(b)),
            (VersionNumber::SemVer(a), VersionNumber::SemVer(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_number_intver_parsing() {
        let v_str = "42";
        let v = CsafVersionNumber::from(v_str);
        match v {
            CsafVersionNumber::Valid(ValidVersionNumber::IntVer(int_ver)) => {
                assert_eq!(int_ver.0, 42);
                assert_eq!(int_ver.get(), 42);
            },
            _ => panic!("Expected Integer variant with value 42"),
        }
    }

    #[test]
    fn test_version_number_semver_parsing() {
        let v_str = "1.2.3-alpha+001";
        let v = CsafVersionNumber::from(v_str);
        match v {
            CsafVersionNumber::Valid(ValidVersionNumber::SemVer(semver)) => {
                assert_eq!(semver.get_major(), 1);
                assert_eq!(semver.get_minor(), 2);
                assert_eq!(semver.get_patch(), 3);
                assert_eq!(semver.0.pre.as_str(), "alpha");
                assert_eq!(semver.0.build.as_str(), "001");
                assert!(semver.has_prerelease());
                assert!(semver.has_build_metadata());
            },
            _ => panic!("Expected Semver variant with value 1.2.3-alpha+001"),
        }
    }

    #[test]
    fn test_version_number_invalid_parsing() {
        let v_str = "invalid_version";
        let v = CsafVersionNumber::from(v_str);
        match v {
            CsafVersionNumber::Invalid(err) => {
                assert_eq!(err.raw_string, v_str);
            },
            _ => panic!("Expected Invalid variant"),
        }
    }

    #[test]
    fn test_version_number_invalid_intver_with_leading_0s() {
        let v_str = "01";
        let v = CsafVersionNumber::from(v_str);
        match v {
            CsafVersionNumber::Invalid(err) => {
                assert_eq!(err.raw_string, v_str);
            },
            _ => panic!("Expected Invalid variant"),
        }
    }
    #[test]
    fn test_version_number_invalid_intver_but_valid_u64() {
        let v_str = "+5";
        let v = CsafVersionNumber::from(v_str);
        match v {
            CsafVersionNumber::Invalid(err) => {
                assert_eq!(err.raw_string, v_str);
            },
            _ => panic!("Expected Invalid variant"),
        }
    }
}
