use crate::validation::ValidationError;
use chrono::{DateTime, FixedOffset, ParseError, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::str::FromStr;
// ============================================================================
// CsafDateTime Implementation
// ============================================================================

#[derive(Debug, Clone)]
pub enum CsafDateTime {
    Valid(ValidCsafDateTime),
    Invalid(CsafDateTimeParseError),
}

impl CsafDateTime {
    /// Returns `true` if the date was successfully parsed.
    pub fn is_valid(&self) -> bool {
        matches!(self, CsafDateTime::Valid(_))
    }
}

impl From<&str> for CsafDateTime {
    fn from(s: &str) -> Self {
        match s.parse() {
            Ok(valid) => CsafDateTime::Valid(valid),
            Err(err) => CsafDateTime::Invalid(err),
        }
    }
}

impl From<&String> for CsafDateTime {
    fn from(s: &String) -> Self {
        CsafDateTime::from(s.as_str())
    }
}

/// Comparison implementation
///
/// Valid dates are compared by their parsed values.
/// If either or both dates are invalid, they are always unequal.
///
/// Also, we do not implement Eq here, as invalid values are also not reflexive, i.e. some invalid value
/// is not equal to itself, or we rather do not care if they are.
impl PartialEq for CsafDateTime {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (CsafDateTime::Valid(a), CsafDateTime::Valid(b)) => a == b,
            _ => false,
        }
    }
}

/// PartialOrd implementation
///
/// Valid dates are ordered by their parsed values.
/// If either or both dates are invalid, there is no ordering.
///
/// Also, we do not implement Ord here, as invalid values can't be ordered, or we rather
/// do not care about their ordering.
impl PartialOrd for CsafDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self, &other) {
            (CsafDateTime::Valid(a), CsafDateTime::Valid(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

// ============================================================================
// Error Stuff
// ============================================================================

#[derive(Debug, Clone)]
pub struct CsafDateTimeParseError {
    pub raw_string: String,
    pub source: ParseError,
}

impl CsafDateTimeParseError {
    /// Creates a new CsafDateTimeParseError.
    pub fn new(raw_string: &str, source: ParseError) -> Self {
        CsafDateTimeParseError {
            raw_string: raw_string.to_owned(),
            source,
        }
    }

    pub fn get_raw_string(&self) -> &str {
        &self.raw_string
    }

    pub fn into_validation_error(self, instance_path: &str) -> ValidationError {
        ValidationError {
            message: self.to_string(),
            instance_path: instance_path.to_string(),
        }
    }
}

impl Display for CsafDateTimeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Failed to parse '{}' as RFC3339 with reason '{}'",
            self.raw_string, self.source
        )
    }
}

impl std::error::Error for CsafDateTimeParseError {}

// ---------------------------------------------------------------------------
// Trait Implementations for CsafDateTime
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ValidCsafDateTime {
    raw_string: String,
    parsed: DateTime<FixedOffset>,
}

impl ValidCsafDateTime {
    /// Creates a new ValidCsafDateTime from a parsed DateTime.
    fn new(raw_string: &str, parsed: DateTime<FixedOffset>) -> Self {
        ValidCsafDateTime {
            raw_string: raw_string.to_owned(),
            parsed,
        }
    }

    pub fn get_raw_string(&self) -> &str {
        &self.raw_string
    }

    /// Returns the parsed DateTime with the original timezone preserved, if valid.
    pub fn get_as_fixed_offset(&self) -> &DateTime<FixedOffset> {
        &self.parsed
    }

    /// Converts the parsed DateTime to UTC, if valid.
    pub fn get_as_utc(&self) -> DateTime<Utc> {
        self.parsed.with_timezone(&Utc)
    }
}

impl FromStr for ValidCsafDateTime {
    type Err = CsafDateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = DateTime::parse_from_rfc3339(s).map_err(|e| CsafDateTimeParseError::new(s, e))?;
        Ok(ValidCsafDateTime::new(s, parsed))
    }
}

impl Display for ValidCsafDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "CsafDateTime (raw: {}, parsed: {})", self.raw_string, self.parsed)
    }
}

impl Eq for ValidCsafDateTime {}

impl PartialEq for ValidCsafDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.parsed == other.parsed
    }
}

impl Hash for ValidCsafDateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parsed.hash(state);
    }
}

impl Ord for ValidCsafDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.parsed.cmp(&other.parsed)
    }
}

impl PartialOrd for ValidCsafDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    use chrono::format::ParseErrorKind;

    // From<> Stuff

    #[test]
    fn test_from_str_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00Z");
        assert!(dt.is_valid());
        match &dt {
            CsafDateTime::Valid(valid) => {
                assert_eq!(valid.get_raw_string(), "2024-01-15T10:30:00Z");
            },
            CsafDateTime::Invalid(err) => panic!("DateTime should have been valid, but returned invalid with: {err}"),
        }
    }

    #[test]
    fn test_from_str_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        assert!(!dt.is_valid());
        match &dt {
            CsafDateTime::Invalid(err) => {
                assert_eq!(err.get_raw_string(), "not-a-date");
                matches!(err.source.kind(), ParseErrorKind::Invalid);
            },
            CsafDateTime::Valid(valid) => {
                panic!("DateTime should have been invalid, but returned valid with: {valid}")
            },
        }
    }

    // Getter

    #[test]
    fn test_as_utc() {
        let dt = CsafDateTime::from("2024-01-15T12:00:00+02:00");
        assert!(dt.is_valid());
        match dt {
            CsafDateTime::Valid(valid) => {
                assert_eq!(valid.get_as_utc().hour(), 10); // 12:00 +02:00 = 10:00 UTC
            },
            CsafDateTime::Invalid(err) => panic!("DateTime should have been valid, but returned invalid with: {err}"),
        }
    }

    #[test]
    fn test_as_fixed_offset_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00+02:00");
        assert!(dt.is_valid());
        match dt {
            CsafDateTime::Valid(valid) => {
                let fixed = valid.get_as_fixed_offset();
                assert_eq!(fixed.hour(), 10);
                assert_eq!(fixed.minute(), 30);
            },
            CsafDateTime::Invalid(err) => panic!("DateTime should have been valid, but returned invalid with: {err}"),
        }
    }

    // Equality

    #[test]
    fn test_equality_equal() {
        let dt1 = CsafDateTime::from("2024-01-15T10:30:00Z");
        let dt2 = CsafDateTime::from("2024-01-15T10:30:00Z");
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_equality_unequal() {
        let dt1 = CsafDateTime::from("2024-01-15T10:30:00Z");
        let dt2 = CsafDateTime::from("2024-01-15T11:00:00Z");
        assert_ne!(dt1, dt2);
    }

    #[test]
    fn test_equality_one_invalid() {
        let dt1 = CsafDateTime::from("2024-01-15T10:30:00Z");
        let dt2 = CsafDateTime::from("not-a-date");
        assert_ne!(dt1, dt2);
    }

    #[test]
    fn test_equality_both_invalid() {
        let dt1 = CsafDateTime::from("not-a-date");
        let dt2 = CsafDateTime::from("not-a-date");
        assert_ne!(dt1, dt2);
    }

    // Ordering

    #[test]
    fn test_ordering_valid() {
        let dt1 = CsafDateTime::from("2024-01-15T10:00:00Z");
        let dt2 = CsafDateTime::from("2024-01-15T12:00:00Z");
        assert!(dt1 < dt2);
    }

    #[test]
    fn test_ordering_one_invalid() {
        let dt1 = CsafDateTime::from("not-a-date");
        let dt2 = CsafDateTime::from("2024-01-15T12:00:00Z");
        assert!(dt1.partial_cmp(&dt2).is_none());
    }

    #[test]
    fn test_ordering_both_invalid() {
        let dt1 = CsafDateTime::from("not-a-date");
        let dt2 = CsafDateTime::from("not-a-date");
        assert!(dt1.partial_cmp(&dt2).is_none());
    }
}
