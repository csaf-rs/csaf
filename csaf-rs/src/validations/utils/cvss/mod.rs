pub mod errors;
pub mod validate_consistency;
pub mod validate_metrics;
pub mod validate_scores;
pub use validate_metrics::{validate_content_consistency, validate_content_scores};

use crate::validation::TestFinding;
use crate::validations::utils::cvss::errors::create_deserialization_error;
use cvss_rs::Cvss;
use serde::Deserialize;
use serde_json::Value;
use strum::{AsRefStr, Display};

/// The type of CVSS score being validated, use for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
}

/// Attempts to deserialize a csaf-rs/csaf CVSS JSON map into a scm-rs/cvss-rs [Cvss] enum.
/// Returns `None` and adds a deserialization error if parsing fails.
pub fn deserialize_cvss(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) -> Option<Cvss> {
    match Cvss::deserialize(cvss_map) {
        Ok(cvss) => Some(cvss),
        Err(e) => {
            errors
                .get_or_insert_default()
                .push(create_deserialization_error(e.to_string(), instance_path.to_string()));
            None
        },
    }
}
