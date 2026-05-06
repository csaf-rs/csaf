use super::int_ver_version::IntVerVersion;
use super::sem_ver_version::SemVerVersion;
use crate::schema::csaf2_0::schema::VersionT as VersionT20;
use crate::schema::csaf2_1::schema::VersionT as VersionT21;
use semver::Version;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::ops::Deref;

/// Enum representing the version number of a CSAF document, which can be either be integer or semantic versioning
///
/// This type is parsed into from VersionT, which is already schema validated with the regex:
/// `^(0|[1-9][0-9]*)$|^((0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?)$`
///
/// The regex consists of two parts:
/// 1. The first part `(0|[1-9][0-9]*)$` matches integer versioning, which is either "0" or a non-zero digit followed by any number of digits (e.g., "1", "42", but not "01").
/// 2. The second part matches semantic versioning, which consists of three dot-separated numeric identifiers (major, minor, patch), optionally followed by a prerelease tag (prefixed with '-') and build metadata (prefixed with '+').
#[derive(Debug, Clone)]
pub enum CsafVersionNumber {
    IntVer(IntVerVersion),
    SemVer(SemVerVersion),
}

impl CsafVersionNumber {
    /// Parses a version string into a `CsafVersionNumber`, trying integer versioning and semantic versioning.
    ///
    /// This function assumes that the input string has already passed schema validation.
    /// If the string does not match either format, this function will panic, as it indicates a
    /// dev error (either the schema regex is wrong, or this function was called on an unvalidated string).
    fn parse_str(s: &str) -> CsafVersionNumber {
        // Try parse as u64 for intver
        if s.chars().all(|c| c.is_ascii_digit())
            && !(s.len() > 1 && s.starts_with('0'))
            && let Ok(num) = s.parse::<u64>()
        {
            return CsafVersionNumber::IntVer(IntVerVersion::new(num));
        }

        // Try to parse as semver
        if let Ok(semver) = Version::parse(s) {
            return CsafVersionNumber::SemVer(SemVerVersion::new(semver));
        }

        // Panic if both fail
        panic!(
            "Version string '{s}' does not match either integer or semantic versioning. (Either the schema regex is wrong, or this is a dev error)"
        );
    }

    /// Helper function to get the major version number, which is either the integer version or the major version of the semantic version.
    pub fn get_major(&self) -> u64 {
        match &self {
            CsafVersionNumber::IntVer(intver) => intver.get(),
            CsafVersionNumber::SemVer(semver) => semver.get_major(),
        }
    }

    /// Returns the next version number.
    ///
    /// Integer versions are incremented by 1.
    /// Semantic versions perform a major bump, producing `x+1.0.0`.
    pub fn get_next_major_version(&self) -> CsafVersionNumber {
        match self {
            CsafVersionNumber::IntVer(intver) => CsafVersionNumber::IntVer(IntVerVersion::new(
                intver
                    .get()
                    .checked_add(1)
                    .expect("Integer version overflow while incrementing"),
            )),
            CsafVersionNumber::SemVer(semver) => CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(
                semver
                    .get_major()
                    .checked_add(1)
                    .expect("Semantic version major overflow while incrementing"),
                0,
                0,
            ))),
        }
    }

    /// Returns the previous version number.
    ///
    /// Integer versions are decremented by 1.
    /// Semantic versions perform a major drop, producing `x-1.0.0`.
    pub fn get_previous_major_version(&self) -> CsafVersionNumber {
        match self {
            CsafVersionNumber::IntVer(intver) => CsafVersionNumber::IntVer(IntVerVersion::new(
                intver
                    .get()
                    .checked_sub(1)
                    .expect("Integer version underflow while decrementing"),
            )),
            CsafVersionNumber::SemVer(semver) => CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(
                semver
                    .get_major()
                    .checked_sub(1)
                    .expect("Semantic version major underflow while decrementing"),
                0,
                0,
            ))),
        }
    }
}

// Transform an already schema-validated version string (VersionT) from CSAF 2.0 into a CsafVersionNumber
impl From<&VersionT20> for CsafVersionNumber {
    fn from(v: &VersionT20) -> Self {
        CsafVersionNumber::parse_str(v.deref().as_str())
    }
}

// Transform an already schema-validated version string (VersionT) from CSAF 2.1 into a CsafVersionNumber
impl From<&VersionT21> for CsafVersionNumber {
    fn from(v: &VersionT21) -> Self {
        CsafVersionNumber::parse_str(v.deref().as_str())
    }
}

impl Display for CsafVersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CsafVersionNumber::IntVer(num) => write!(f, "{num}"),
            CsafVersionNumber::SemVer(version) => write!(f, "{version}"),
        }
    }
}

impl Hash for CsafVersionNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            CsafVersionNumber::IntVer(num) => {
                0u8.hash(state); // Discriminator for IntVer
                num.hash(state);
            },
            CsafVersionNumber::SemVer(version) => {
                1u8.hash(state); // Discriminator for SemVer
                version.hash(state);
            },
        }
    }
}

impl Eq for CsafVersionNumber {}

/// VersionNumbers are equal if they are of the same variant and their values are equal
/// Otherwise, they are unequal
/// Also, this relationship is reflexive
impl PartialEq for CsafVersionNumber {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (CsafVersionNumber::IntVer(a), CsafVersionNumber::IntVer(b)) => a == b,
            (CsafVersionNumber::SemVer(a), CsafVersionNumber::SemVer(b)) => a == b,
            // Integer and Semver are always unequal
            (CsafVersionNumber::IntVer(_), CsafVersionNumber::SemVer(_))
            | (CsafVersionNumber::SemVer(_), CsafVersionNumber::IntVer(_)) => false,
        }
    }
}

// TODO: Review this after revision history refactor
impl Ord for CsafVersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, &other) {
            (CsafVersionNumber::IntVer(a), CsafVersionNumber::IntVer(b)) => a.cmp(b),
            (CsafVersionNumber::SemVer(a), CsafVersionNumber::SemVer(b)) => a.cmp(b),
            _ => {
                panic!(
                    "Cannot compare CsafVersionNumbers of different variants (IntVer vs SemVer). This looks like a dev error."
                );
            },
        }
    }
}

// TODO: Review this after revision history refactor.
impl PartialOrd for CsafVersionNumber {
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

// Transform a raw version string into a CsafVersionNumber.  As the schema validation is the same
// for CSAF 2.0 and 2.1, we can just push the raw string through either before parsing it into a CsafVersionNumber.
// This is only used for testing and not available on the public API
impl From<&str> for CsafVersionNumber {
    fn from(s: &str) -> Self {
        use std::str::FromStr;
        CsafVersionNumber::parse_str(
            &crate::schema::csaf2_1::schema::VersionT::from_str(s).unwrap_or_else(|err| {
                panic!("Raw version string '{s}' failed schema validation: {err}. This looks like a dev error.")
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_str_integer_version() {
        let intver = CsafVersionNumber::parse_str("42");
        assert!(matches!(intver, CsafVersionNumber::IntVer(_)));
        assert_eq!(intver.get_major(), 42);
    }

    #[test]
    fn test_parse_str_semver_version() {
        let semver = CsafVersionNumber::parse_str("1.0.0");
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major(), 1);
    }

    #[test]
    #[should_panic(expected = "does not match either integer or semantic versioning")]
    fn test_parse_str_invalid_version_panics() {
        // Invalid version strings should panic
        CsafVersionNumber::parse_str("invalid");
    }

    #[test]
    #[should_panic(expected = "does not match either integer or semantic versioning")]
    fn test_parse_str_leading_zero_panics() {
        // Leading zeros are invalid for integer versioning and should panic
        CsafVersionNumber::parse_str("01");
    }

    #[test]
    fn test_from_str() {
        let v42: CsafVersionNumber = "42".into();
        assert!(matches!(v42, CsafVersionNumber::IntVer(_)));
        assert_eq!(v42.get_major(), 42);

        let v: CsafVersionNumber = "1.0.0".into();
        assert!(matches!(v, CsafVersionNumber::SemVer(_)));
        assert_eq!(v.get_major(), 1);
    }

    #[test]
    #[should_panic(expected = "failed schema validation")]
    fn test_from_str_invalid_panics() {
        // Invalid version strings should fail schema validation
        let _: CsafVersionNumber = "invalid".into();
    }

    #[test]
    fn test_from_version_t20() {
        let version_t_intver = VersionT20::from_str("42").unwrap();
        let intver = CsafVersionNumber::from(&version_t_intver);
        assert!(matches!(intver, CsafVersionNumber::IntVer(_)));
        assert_eq!(intver.get_major(), 42);

        let version_t_semver = VersionT20::from_str("1.0.0").unwrap();
        let semver = CsafVersionNumber::from(&version_t_semver);
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major(), 1);
    }

    #[test]
    fn test_from_version_t21() {
        let version_t_intver = VersionT21::from_str("42").unwrap();
        let intver = CsafVersionNumber::from(&version_t_intver);
        assert!(matches!(intver, CsafVersionNumber::IntVer(_)));
        assert_eq!(intver.get_major(), 42);

        let version_t_semver = VersionT21::from_str("1.0.0").unwrap();
        let semver = CsafVersionNumber::from(&version_t_semver);
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major(), 1);
    }

    #[test]
    fn test_increment_intver() {
        let version = CsafVersionNumber::from("42");

        assert_eq!(version.get_next_major_version().get_major(), 43);
    }

    #[test]
    fn test_increment_semver_major() {
        let version = CsafVersionNumber::from("1.2.3-alpha+001");

        assert_eq!(version.get_next_major_version().to_string(), "2.0.0");
    }
}
