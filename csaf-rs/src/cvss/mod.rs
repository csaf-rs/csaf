pub mod v2;
pub mod v3;
pub mod v4;

use std::fmt;

use crate::validation::ValidationError;
use cvss_rs::Cvss;
use cvss_rs::Severity;
use cvss_rs::Version;
use serde_json::Value;

/// Deserializes and extracts the expected CVSS variant, and validates scores and severities from a
/// CVSS JSON map.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
pub fn validate(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
    expected_version: Version,
) {
    let Some(cvss_deserialized) = deserialize_cvss(cvss_map, instance_path, errors) else {
        return;
    };
    match (expected_version, cvss_deserialized) {
        (Version::V2, Cvss::V2(cvss2)) => {
            v2::validate_scores(&cvss2, cvss_map, instance_path, errors);
        },
        (Version::V3_0, Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3)) => {
            v3::validate_scores(&cvss3, instance_path, errors);
        },
        (Version::V4, Cvss::V4(cvss4)) => {
            v4::validate_scores(&cvss4, instance_path, errors);
        },
        (expected, found) => {
            errors.get_or_insert_default().push(create_deserialization_error(
                format!(
                    "Deserialized CVSS metric {} does not match expected version {expected}",
                    found.version()
                ),
                instance_path.to_string(),
            ));
        },
    }
}

/// The type of CVSS score being validated, use for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
}

impl fmt::Display for ScoreType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreType::Base => write!(f, "Base"),
            ScoreType::Temporal => write!(f, "Temporal"),
            ScoreType::Environmental => write!(f, "Environmental"),
        }
    }
}

pub fn create_deserialization_error(error_message: String, instance_path: String) -> ValidationError {
    ValidationError {
        message: format!("Error deserializing CVSS metric: {error_message}"),
        instance_path,
    }
}

pub fn create_vector_parse_error(
    vector_string: &str,
    version: Version,
    parse_error: &cvss_rs::ParseError,
    instance_path: &str,
) -> ValidationError {
    let version_str = match version {
        Version::V2 => "2.0",
        Version::V4 => "4.0",
        _ => "3.x",
    };
    ValidationError {
        message: format!("Could not parse vector string \"{vector_string}\" as CVSS {version_str}: {parse_error}"),
        instance_path: instance_path.to_string(),
    }
}

/// Attempts to deserialize a csaf-rs/csaf CVSS JSON map into a scm-rs/cvss-rs [Cvss] enum.
/// Returns `None` and adds a deserialization error if parsing fails.
pub fn deserialize_cvss(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) -> Option<Cvss> {
    match serde_json::from_value(Value::Object(cvss_map.to_owned())) {
        Ok(cvss) => Some(cvss),
        Err(e) => {
            errors
                .get_or_insert_default()
                .push(create_deserialization_error(e.to_string(), instance_path.to_string()));
            None
        },
    }
}

pub fn create_score_mismatch_error(
    calculated: f64,
    actual: f64,
    score_type: ScoreType,
    instance_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "{score_type} score does not match the expected value calculated from the vector. \
             Expected: {calculated}, found: {actual}"
        ),
        instance_path: instance_path.to_string(),
    }
}

/// Compares an actual score against a calculated score and adds a validation error if they differ.
pub fn check_score_mismatch(
    actual: f64,
    calculated: f64,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    // compare scores as scaled integers
    if (actual * 10.0).round() as i8 != (calculated * 10.0).round() as i8 {
        errors.get_or_insert_default().push(create_score_mismatch_error(
            calculated,
            actual,
            score_type,
            instance_path,
        ));
    }
}

pub fn create_severity_mismatch_error(
    calculated: &Severity,
    actual: &Severity,
    score_type: ScoreType,
    instance_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "{score_type} severity does not match the expected value calculated from the vector. \
             Expected: {calculated:?}, found: {actual:?}"
        ),
        instance_path: instance_path.to_string(),
    }
}

/// Compares an actual severity against a calculated severity and adds a validation error if they
/// differ.
pub fn check_severity_mismatch(
    actual: &Severity,
    calculated: &Severity,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    if actual != calculated {
        errors.get_or_insert_default().push(create_severity_mismatch_error(
            calculated,
            actual,
            score_type,
            instance_path,
        ));
    }
}

/// Maps a CVSS score to its severity rating.
///
/// The severity ranges follow the CVSS v3/v4 specification.
pub fn map_score_to_severity(score: Option<f64>) -> Option<Severity> {
    let scaled = (score? * 10.0).round() as i8;
    Some(match scaled {
        0 => Severity::None,
        1..=39 => Severity::Low,
        40..=69 => Severity::Medium,
        70..=89 => Severity::High,
        90..=100 => Severity::Critical,
        _ => return None,
    })
}
