use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::schema::csaf2_1::schema::{Epss, Percentile, Probability};

/// Trait representing an abstract EPSS metric in a CSAF document.
///
/// Provides access to the EPSS percentile, probability, and timestamp fields.
/// Implemented for [`Epss`] (CSAF 2.1) and [`NotPresentInCsaf20`] (CSAF 2.0).
pub trait EpssTrait {
    /// Returns the EPSS percentile value.
    fn get_percentile(&self) -> &Percentile;

    /// Returns the EPSS probability value.
    fn get_probability(&self) -> &Probability;

    /// Returns the EPSS timestamp as a [`CsafDateTime`].
    fn get_timestamp(&self) -> CsafDateTime;
}

impl EpssTrait for NotPresentInCsaf20 {
    fn get_percentile(&self) -> &Percentile {
        self.into_any()
    }

    fn get_probability(&self) -> &Probability {
        self.into_any()
    }

    fn get_timestamp(&self) -> CsafDateTime {
        self.into_any()
    }
}

// CSAF 2.1 implementation
impl EpssTrait for Epss {
    fn get_percentile(&self) -> &Percentile {
        &self.percentile
    }

    fn get_probability(&self) -> &Probability {
        &self.probability
    }

    fn get_timestamp(&self) -> CsafDateTime {
        CsafDateTime::from(&self.timestamp)
    }
}

