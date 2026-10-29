pub mod deserialize_into_cvss;
pub mod errors;
pub mod validate_consistency;
pub mod validate_metrics;
pub mod validate_scores;

pub use validate_metrics::{validate_content_consistency, validate_content_scores};

use serde::Deserialize;
use strum::{AsRefStr, Display};

/// The type of CVSS score being validated, use for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
}
