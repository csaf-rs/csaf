use crate::csaf::types::csaf_datetime::{CsafDateTimeParseError, ValidCsafDateTime};
use crate::schema::csaf2_1::schema::{Epss, Percentile, Probability};

/// A parsed EPSS value with validated timestamp
///
/// Wraps the raw [`Epss`] schema type, parsing the timestamp string into a
/// [`ValidCsafDateTime`] on construction. If the timestamp cannot be parsed,
/// the `Invalid` variant contains the parse error.
#[derive(Debug, Clone)]
pub enum CsafEpss {
    Valid(ValidCsafEpss),
    Invalid(CsafDateTimeParseError),
}

impl From<&Epss> for CsafEpss {
    fn from(epss: &Epss) -> Self {
        match epss.timestamp.parse::<ValidCsafDateTime>() {
            Ok(timestamp) => CsafEpss::Valid(ValidCsafEpss {
                percentile: epss.percentile.clone(),
                probability: epss.probability.clone(),
                timestamp,
            }),
            Err(err) => CsafEpss::Invalid(err),
        }
    }
}

/// A valid EPSS with a successfully parsed timestamp.
#[derive(Debug, Clone)]
pub struct ValidCsafEpss {
    pub percentile: Percentile,
    pub probability: Probability,
    pub timestamp: ValidCsafDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn build_epss(timestamp: &str) -> Epss {
        Epss::builder()
            .percentile("0.999999999")
            .probability("0.999999999")
            .timestamp(timestamp)
            .try_into()
            .unwrap()
    }

    #[test]
    fn test_valid_timestamp() {
        let csaf_epss = CsafEpss::from(&build_epss("2024-07-13T10:00:00.000Z"));
        assert!(matches!(csaf_epss, CsafEpss::Valid(_)));
    }

    #[test]
    fn test_invalid_timestamp() {
        let csaf_epss = CsafEpss::from(&build_epss("not-a-date"));
        assert!(matches!(csaf_epss, CsafEpss::Invalid(_)));
    }
}
