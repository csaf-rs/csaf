use super::int_ver_version::IntVerVersion;
use super::sem_ver_version::SemVerVersion;
use crate::csaf::types::version_number::CsafVersionNumberError;
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
    Invalid(String),
}

pub const INTEGER_VER_ZERO: CsafVersionNumber = CsafVersionNumber::IntVer(IntVerVersion::new(0));
pub const INTEGER_VER_ONE: CsafVersionNumber = CsafVersionNumber::IntVer(IntVerVersion::new(1));
pub const SEMANTIC_VER_ZERO: CsafVersionNumber = CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(0, 0, 0)));
pub const SEMANTIC_VER_ONE: CsafVersionNumber = CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(1, 0, 0)));

impl CsafVersionNumber {
    /// Parses a version string into a `CsafVersionNumber`, trying integer versioning and semantic versioning.
    ///
    /// Integer versions must be in the inclusive range `0..=u64::MAX`.
    /// Semantic version core identifiers are parsed by `semver::Version`, which applies the same
    /// `u64` range restriction to major, minor, and patch.
    /// If both parses fail, this function returns `Invalid`.
    fn parse_str(s: &str) -> CsafVersionNumber {
        if s.chars().all(|c| c.is_ascii_digit())
            && !(s.len() > 1 && s.starts_with('0'))
            && let Ok(num) = s.parse::<u64>()
        {
            return CsafVersionNumber::IntVer(IntVerVersion::new(num));
        }

        if let Ok(semver) = Version::parse(s) {
            return CsafVersionNumber::SemVer(SemVerVersion::new(semver));
        }

        return CsafVersionNumber::Invalid(s.to_string());
    }

    /// Helper function to get the major version number, which is either the integer version or the major version of the semantic version.
    pub fn get_major(&self) -> Result<u64, CsafVersionNumberError> {
        match &self {
            CsafVersionNumber::IntVer(intver) => Ok(intver.get()),
            CsafVersionNumber::SemVer(semver) => Ok(semver.get_major()),
            CsafVersionNumber::Invalid(v) => Err(CsafVersionNumberError::Invalid(v.clone())),
        }
    }

    /// Converts this version to a `semver::Version` for comparison purposes.
    ///
    /// An `IntVer(n)` is treated as `n.0.0`, consistent with the CSAF spec note in 6.1.14:
    /// *"non-semantic versioning numbers are interpreted as semantic versioning numbers"*.
    /// This allows mixed-variant revision histories to be sorted and compared correctly.
    pub(crate) fn to_comparable_semver(&self) -> Result<Version, CsafVersionNumberError> {
        match self {
            CsafVersionNumber::IntVer(intver) => Ok(Version::new(intver.get(), 0, 0)),
            CsafVersionNumber::SemVer(semver) => Ok(semver.get_version().clone()),
            CsafVersionNumber::Invalid(v) => Err(CsafVersionNumberError::Invalid(v.clone())),
        }
    }

    /// Returns the next version number.
    ///
    /// Integer versions are incremented by 1.
    /// Semantic versions perform a major bump, producing `x+1.0.0`.
    /// This returns an error if the next version would overflow `u64::MAX` for either integer or semantic versions.
    pub fn get_next_major_version(&self) -> Result<CsafVersionNumber, CsafVersionNumberError> {
        match self {
            CsafVersionNumber::IntVer(intver) => {
                let next_major = intver
                    .get()
                    .checked_add(1)
                    .ok_or_else(|| CsafVersionNumberError::Overflow)?;
                Ok(CsafVersionNumber::IntVer(IntVerVersion::new(next_major)))
            },
            CsafVersionNumber::SemVer(semver) => {
                let next_major = semver
                    .get_major()
                    .checked_add(1)
                    .ok_or_else(|| CsafVersionNumberError::Overflow)?;
                Ok(CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(
                    next_major, 0, 0,
                ))))
            },
            CsafVersionNumber::Invalid(v) => Err(CsafVersionNumberError::Invalid(v.clone())),
        }
    }

    /// Returns the previous version number up to including 1 or 1.0.0.
    ///
    /// Integer versions are decremented by 1.
    /// Semantic versions perform a major drop, producing `x-1.0.0`.
    pub fn get_previous_major_version(&self) -> Result<Option<CsafVersionNumber>, CsafVersionNumberError> {
        match self {
            CsafVersionNumber::IntVer(intver) => {
                let previous_major = match intver.get() {
                    1 => return Ok(None),
                    c => c - 1,
                };
                Ok(Some(CsafVersionNumber::IntVer(IntVerVersion::new(previous_major))))
            },
            CsafVersionNumber::SemVer(semver) => {
                let previous_major = match semver.get_major() {
                    1 => return Ok(None),
                    c => c - 1,
                };
                Ok(Some(CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(
                    previous_major,
                    0,
                    0,
                )))))
            },
            CsafVersionNumber::Invalid(v) => Err(CsafVersionNumberError::Invalid(v.clone())),
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
            CsafVersionNumber::Invalid(s) => write!(f, "{s}"),
        }
    }
}

impl Hash for CsafVersionNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash via the comparable semver representation so that IntVer(n) and SemVer(n.0.0)
        // always produce the same hash, consistent with their PartialEq implementation.
        match self {
            CsafVersionNumber::Invalid(s) => s.hash(state),
            valid => valid.to_comparable_semver().ok().hash(state),
        }
    }
}

impl Eq for CsafVersionNumber {}

/// Two version numbers are equal when their comparable semver representations are equal.
/// This means `IntVer(n)` equals `SemVer(n.0.0)` but not `SemVer(n.x.y)` for `x > 0` or `y > 0`.
/// If both versions are invalid, their original strings are compared for equality.
/// If one is invalid and the other is valid, they are not equal.
impl PartialEq for CsafVersionNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CsafVersionNumber::Invalid(s1), CsafVersionNumber::Invalid(s2)) => return s1 == s2,
            (CsafVersionNumber::Invalid(_), _) | (_, CsafVersionNumber::Invalid(_)) => return false,
            (v1, v2) => v1.to_comparable_semver().ok() == v2.to_comparable_semver().ok(),
        }
    }
}

impl Ord for CsafVersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (CsafVersionNumber::Invalid(s1), CsafVersionNumber::Invalid(s2)) => return s1.cmp(s2),
            (CsafVersionNumber::Invalid(i), v) => return i.cmp(&v.to_string()), // fall back to string comparison for invalid vs valid
            (v, CsafVersionNumber::Invalid(i)) => return v.to_string().cmp(i), // fall back to string comparison for valid vs invalid
            (v1, v2) => v1.to_comparable_semver().ok().cmp(&v2.to_comparable_semver().ok()),
        }
    }
}

impl PartialOrd for CsafVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Transform a raw version string into a CsafVersionNumber.  As the schema validation is the same
// for CSAF 2.0 and 2.1, we can just push the raw string through either before parsing it into a CsafVersionNumber.
// This is only used for testing and not available on the public API
#[cfg(test)]
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
        assert_eq!(intver.get_major().ok(), Some(42));
    }

    #[test]
    fn test_parse_str_integer_boundaries() {
        let zero = CsafVersionNumber::parse_str("0");
        assert!(matches!(zero, CsafVersionNumber::IntVer(_)));
        assert_eq!(zero.get_major().ok(), Some(0));

        let max = CsafVersionNumber::parse_str("18446744073709551615");
        assert!(matches!(max, CsafVersionNumber::IntVer(_)));
        assert_eq!(max.get_major().ok(), Some(u64::MAX));
    }

    #[test]
    fn test_parse_str_semver_version() {
        let semver = CsafVersionNumber::parse_str("1.0.0");
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major().ok(), Some(1));
    }

    #[test]
    fn test_parse_str_semver_boundaries() {
        let zero = CsafVersionNumber::parse_str("0.0.0");
        assert!(matches!(zero, CsafVersionNumber::SemVer(_)));
        assert_eq!(zero.to_string(), "0.0.0");

        let max = CsafVersionNumber::parse_str("18446744073709551615.18446744073709551615.18446744073709551615");
        assert!(matches!(max, CsafVersionNumber::SemVer(_)));
        assert_eq!(
            max.to_string(),
            "18446744073709551615.18446744073709551615.18446744073709551615"
        );
    }

    #[test]
    fn test_parse_str_invalid_version() {
        let semver = CsafVersionNumber::parse_str("v234");
        assert!(matches!(semver, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_parse_str_leading_zero_invalid() {
        let semver = CsafVersionNumber::parse_str("0123");
        assert!(matches!(semver, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_parse_str_integer_overflow_invalid() {
        let version = CsafVersionNumber::parse_str("18446744073709551616");
        assert!(matches!(version, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_parse_str_semver_major_overflow_invalid() {
        let version = CsafVersionNumber::parse_str("18446744073709551616.0.0");
        assert!(matches!(version, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_parse_str_semver_minor_overflow_invalid() {
        let version = CsafVersionNumber::parse_str("0.18446744073709551616.0");
        assert!(matches!(version, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_parse_str_semver_patch_overflow_invalid() {
        let version = CsafVersionNumber::parse_str("0.0.18446744073709551616");
        assert!(matches!(version, CsafVersionNumber::Invalid(_)));
    }

    #[test]
    fn test_from_str() {
        let v42: CsafVersionNumber = "42".into();
        assert!(matches!(v42, CsafVersionNumber::IntVer(_)));
        assert_eq!(v42.get_major().ok(), Some(42));

        let v_1_0_0: CsafVersionNumber = "1.0.0".into();
        assert!(matches!(v_1_0_0, CsafVersionNumber::SemVer(_)));
        assert_eq!(v_1_0_0.get_major().ok(), Some(1));
    }

    #[test]
    fn test_from_version_t20() {
        let version_t_intver = VersionT20::from_str("42").unwrap();
        let intver = CsafVersionNumber::from(&version_t_intver);
        assert!(matches!(intver, CsafVersionNumber::IntVer(_)));
        assert_eq!(intver.get_major().ok(), Some(42));

        let version_t_semver = VersionT20::from_str("1.0.0").unwrap();
        let semver = CsafVersionNumber::from(&version_t_semver);
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major().ok(), Some(1));
    }

    #[test]
    fn test_from_version_t21() {
        let version_t_intver = VersionT21::from_str("42").unwrap();
        let intver = CsafVersionNumber::from(&version_t_intver);
        assert!(matches!(intver, CsafVersionNumber::IntVer(_)));
        assert_eq!(intver.get_major().ok(), Some(42));

        let version_t_semver = VersionT21::from_str("1.0.0").unwrap();
        let semver = CsafVersionNumber::from(&version_t_semver);
        assert!(matches!(semver, CsafVersionNumber::SemVer(_)));
        assert_eq!(semver.get_major().ok(), Some(1));
    }

    #[test]
    fn test_increment_intver() {
        let version = CsafVersionNumber::from("42");
        assert_eq!(
            version.get_next_major_version().ok().and_then(|v| v.get_major().ok()),
            Some(43)
        );
    }

    #[test]
    fn test_increment_semver_major() {
        let version = CsafVersionNumber::from("1.2.3-alpha+001");
        assert_eq!(
            version.get_next_major_version().ok().map(|v| v.to_string()),
            Some("2.0.0".to_string())
        );
    }

    #[test]
    fn test_increment_invalid_version_error() {
        let version = CsafVersionNumber::parse_str("v123");
        assert!(version.get_next_major_version().is_err());
    }

    #[test]
    fn test_increment_intver_overflow_error() {
        let version = CsafVersionNumber::IntVer(IntVerVersion::new(u64::MAX));
        assert!(version.get_next_major_version().is_err());
    }

    #[test]
    fn test_increment_semver_major_overflow_error() {
        let version = CsafVersionNumber::SemVer(SemVerVersion::new(Version::new(u64::MAX, 0, 0)));
        assert!(version.get_next_major_version().is_err());
    }

    #[test]
    fn test_decrement_intver() {
        let version = CsafVersionNumber::from("42");
        assert_eq!(
            version
                .get_previous_major_version()
                .ok()
                .flatten()
                .and_then(|v| v.get_major().ok()),
            Some(41)
        );
    }

    #[test]
    fn test_decrement_semver() {
        let version = CsafVersionNumber::from("2.2.3-alpha+001");
        assert_eq!(
            version
                .get_previous_major_version()
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            Some("1.0.0".to_string())
        );
    }

    #[test]
    fn test_decrement_invalid() {
        let version = CsafVersionNumber::parse_str("v123v123");
        assert!(version.get_previous_major_version().is_err());
    }

    #[test]
    fn test_decrement_to_none() {
        let version = CsafVersionNumber::from("1");
        assert!(matches!(version.get_previous_major_version(), Ok(None)));

        let version = CsafVersionNumber::from("1.2.3");
        assert!(matches!(version.get_previous_major_version(), Ok(None)));
    }

    #[test]
    fn test_cross_variant_equality() {
        // IntVer(n) equals SemVer(n.0.0)
        assert_eq!(CsafVersionNumber::from("1"), CsafVersionNumber::from("1.0.0"));
        assert_eq!(CsafVersionNumber::from("42"), CsafVersionNumber::from("42.0.0"));
        // IntVer(n) does not equal SemVer(n.x.y) when x > 0 or y > 0
        assert_ne!(CsafVersionNumber::from("1"), CsafVersionNumber::from("1.2.3"));
        assert_ne!(CsafVersionNumber::from("1"), CsafVersionNumber::parse_str("v1"));
        assert_ne!(CsafVersionNumber::parse_str("v1"), CsafVersionNumber::from("1.0.0"));
    }

    #[test]
    fn test_cross_variant_ordering() {
        // IntVer(1) < SemVer(2.0.0)
        assert!(CsafVersionNumber::from("1") < CsafVersionNumber::from("2.0.0"));
        // IntVer(2) > SemVer(1.0.0)
        assert!(CsafVersionNumber::from("2") > CsafVersionNumber::from("1.0.0"));
        // IntVer(1) == SemVer(1.0.0) in ordering
        assert_eq!(
            CsafVersionNumber::from("1").cmp(&CsafVersionNumber::from("1.0.0")),
            std::cmp::Ordering::Equal
        );
        // IntVer(1) < SemVer(1.2.3) because 1.0.0 < 1.2.3
        assert!(CsafVersionNumber::from("1") < CsafVersionNumber::from("1.2.3"));
    }
}
