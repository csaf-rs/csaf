use chrono::{DateTime, FixedOffset, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A wrapper type that stores both the raw RFC3339 string and its parsing result.
///
/// This type always contains the original string, regardless of whether parsing
/// succeeded or failed. The parsed value is stored as a `Result`.
///
/// # Example
/// ```
/// use csaf::csaf::types::csaf_datetime::CsafDateTime;
///
/// // Valid date
/// let dt = CsafDateTime::from("2024-01-15T10:30:00Z");
/// assert_eq!(dt.get_str(), "2024-01-15T10:30:00Z");
/// assert!(dt.is_valid());
/// println!("UTC: {}", dt.get_as_utc().unwrap());
///
/// // Invalid date - still stores the raw string
/// let invalid = CsafDateTime::from("not-a-date");
/// assert_eq!(invalid.get_str(), "not-a-date");
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone)]
pub struct CsafDateTime {
    /// The original RFC3339 string
    raw: String,
    /// The parsing result - Ok with parsed DateTime, or Err with parse error
    parsed: Result<DateTime<FixedOffset>, Rfc3339ParseError>,
}

// ============================================================================
// Error Stuff
// ============================================================================

/// Error type for RFC3339 date-time parsing failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rfc3339ParseError {
    /// The error message from chrono
    pub message: String,
}

impl Display for Rfc3339ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to parse as RFC3339: '{}'", self.message)
    }
}

impl std::error::Error for Rfc3339ParseError {}

// ============================================================================
// CsafDateTime Implementation
// ============================================================================

impl CsafDateTime {
    /// Creates a new CsafDateTime by parsing the given string.
    fn parse(raw: String) -> Self {
        let parsed = DateTime::parse_from_rfc3339(&raw).map_err(|e| Rfc3339ParseError { message: e.to_string() });
        CsafDateTime { raw, parsed }
    }

    /// Returns the original RFC3339 string.
    pub fn get_str(&self) -> &str {
        &self.raw
    }

    /// Returns `true` if the date was successfully parsed.
    pub fn is_valid(&self) -> bool {
        self.parsed.is_ok()
    }

    /// Returns the parsing result as a reference.
    pub fn get_parsed(&self) -> &Result<DateTime<FixedOffset>, Rfc3339ParseError> {
        &self.parsed
    }

    /// Returns the parsed DateTime with the original timezone preserved, if valid.
    pub fn get_as_fixed_offset(&self) -> Option<&DateTime<FixedOffset>> {
        self.parsed.as_ref().ok()
    }

    /// Converts the parsed DateTime to UTC, if valid.
    pub fn get_as_utc(&self) -> Option<DateTime<Utc>> {
        self.parsed.as_ref().ok().map(|dt| dt.with_timezone(&Utc))
    }

    /// Returns the parse error, if parsing failed.
    pub fn get_error(&self) -> Option<&Rfc3339ParseError> {
        self.parsed.as_ref().err()
    }
}

// ---------------------------------------------------------------------------
// Trait Implementations for CsafDateTime
// ---------------------------------------------------------------------------

/// From<&str> for parsing string slices
impl From<&str> for CsafDateTime {
    fn from(s: &str) -> Self {
        Self::parse(s.to_string())
    }
}

/// From<&String> for parsing borrowed Strings
impl From<&String> for CsafDateTime {
    fn from(s: &String) -> Self {
        Self::parse(s.clone())
    }
}

/// From<String> for parsing owned strings (avoids extra allocation)
impl From<String> for CsafDateTime {
    fn from(s: String) -> Self {
        Self::parse(s)
    }
}

/// Display shows raw string and either parsed value or parsing error
impl Display for CsafDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.is_valid() {
            write!(
                f,
                "raw: {}, parsing failed with: {}",
                self.raw,
                self.get_error().unwrap()
            )
        } else {
            write!(f, "raw: {}, parsed: {}", self.raw, self.get_as_fixed_offset().unwrap())
        }
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
        match (&self.parsed, &other.parsed) {
            (Ok(a), Ok(b)) => a == b,
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
        match (&self.parsed, &other.parsed) {
            (Ok(a), Ok(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    // From<> Stuff

    #[test]
    fn test_from_str_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00Z");
        assert_eq!(dt.get_str(), "2024-01-15T10:30:00Z");
        assert!(dt.is_valid());
    }

    #[test]
    fn test_from_string_valid() {
        let s = String::from("2024-01-15T10:30:00Z");
        let dt = CsafDateTime::from(s);
        assert_eq!(dt.get_str(), "2024-01-15T10:30:00Z");
        assert!(dt.is_valid());
    }

    #[test]
    fn test_from_string_ref_valid() {
        let s = String::from("2024-01-15T10:30:00Z");
        let dt = CsafDateTime::from(&s);
        assert_eq!(dt.get_str(), "2024-01-15T10:30:00Z");
        assert_eq!(s, "2024-01-15T10:30:00Z");
    }

    #[test]
    fn test_from_str_with_timezone() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00+02:00");
        assert_eq!(dt.get_str(), "2024-01-15T10:30:00+02:00");
        assert!(dt.is_valid());
    }

    #[test]
    fn test_from_str_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        assert_eq!(dt.get_str(), "not-a-date");
        assert!(!dt.is_valid());
        assert!(dt.get_error().is_some());
    }

    // Getter

    #[test]
    fn test_get_parsed_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00+02:00");
        let parsed = dt.get_parsed();
        assert!(parsed.is_ok());
        let inner = parsed.as_ref().unwrap();
        assert_eq!(inner.hour(), 10);
        assert_eq!(inner.minute(), 30);
    }

    #[test]
    fn test_get_parsed_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        let parsed = dt.get_parsed();
        assert!(parsed.is_err());
        assert!(!parsed.as_ref().unwrap_err().message.is_empty());
    }

    #[test]
    fn test_get_error_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00Z");
        assert!(dt.get_error().is_none());
    }

    #[test]
    fn test_get_error_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        let error = dt.get_error().unwrap();
        assert!(!error.message.is_empty());
    }

    #[test]
    fn test_as_utc() {
        let dt = CsafDateTime::from("2024-01-15T12:00:00+02:00");
        let utc = dt.get_as_utc().unwrap();
        assert_eq!(utc.hour(), 10); // 12:00 +02:00 = 10:00 UTC
    }

    #[test]
    fn test_as_utc_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        assert!(dt.get_as_utc().is_none());
    }

    #[test]
    fn test_as_fixed_offset_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00+02:00");
        let fixed = dt.get_as_fixed_offset().unwrap();
        assert_eq!(fixed.hour(), 10);
        assert_eq!(fixed.minute(), 30);
    }

    #[test]
    fn test_as_fixed_offset_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        assert!(dt.get_as_fixed_offset().is_none());
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

    // Display

    #[test]
    fn test_display_valid() {
        let dt = CsafDateTime::from("2024-01-15T10:30:00Z");
        let display = format!("{}", dt);
        assert!(display.contains("raw: 2024-01-15T10:30:00Z"));
        assert!(display.contains("parsed:"));
    }

    #[test]
    fn test_display_invalid() {
        let dt = CsafDateTime::from("not-a-date");
        let display = format!("{}", dt);
        assert!(display.contains("raw: not-a-date"));
        assert!(display.contains("parsing failed with:"));
    }

    #[test]
    fn test_display_error_message() {
        let dt = CsafDateTime::from("not-a-date");
        let display = format!("{}", dt.get_error().unwrap());
        assert!(display.contains("Failed to parse as RFC3339:"));
    }
}
