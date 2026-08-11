pub mod v2;
pub mod v3;
pub mod v4;

use crate::csaf_traits::ContentTrait;
use crate::validation::ValidationError;
use cvss_rs::Cvss;
use cvss_rs::Severity;
use cvss_rs::Version;
use serde::Deserialize;
use serde_json::Value;
use strum::{AsRefStr, Display};

/// Validates CVSS scores for all CVSS versions present.
pub fn validate_content_scores(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    if let Some(cvss_v2_map) = content.get_cvss_v2() {
        validate_scores(cvss_v2_map, instance_path, errors, Version::V2);
    }
    if let Some(cvss_v3_map) = content.get_cvss_v3() {
        validate_scores(cvss_v3_map, instance_path, errors, Version::V3_0);
    }
    if let Some(cvss_v4_map) = content.get_cvss_v4() {
        validate_scores(cvss_v4_map, instance_path, errors, Version::V4);
    }
}

/// Validates CVSS consistency for all CVSS versions present.
pub fn validate_content_consistency(
    content: &impl ContentTrait,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    if let Some(cvss_map) = content.get_cvss_v2() {
        validate_consistency(cvss_map, instance_path, errors, Version::V2);
    }
    if let Some(cvss_map) = content.get_cvss_v3() {
        validate_consistency(cvss_map, instance_path, errors, Version::V3_0);
    }
    if let Some(cvss_map) = content.get_cvss_v4() {
        validate_consistency(cvss_map, instance_path, errors, Version::V4);
    }
}

/// Deserializes and extracts the expected CVSS variant, and validates scores and severities.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
fn validate_scores(
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

/// Deserializes and extracts the expected CVSS variant, and validates the consistency of the JSON vs.
/// the CVSS vector.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
fn validate_consistency(
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
            v2::validate_consistency(&cvss2, instance_path, errors);
        },
        (Version::V3_0, Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3)) => {
            v3::validate_consistency(&cvss3, instance_path, errors);
        },
        (Version::V4, Cvss::V4(cvss4)) => {
            v4::validate_consistency(&cvss4, instance_path, errors);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
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

pub fn create_field_value_mismatch_error<T: std::fmt::Display>(
    field_name: &str,
    json_val: &T,
    vec_val: &T,
    instance_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Property \"{field_name}\" does not match the value from the vector string. \
             Expected: {vec_val}, found: {json_val}"
        ),
        instance_path: instance_path.to_string(),
    }
}

pub fn create_field_missing_in_vector_error<T: std::fmt::Display>(
    field_name: &str,
    json_val: &T,
    instance_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Property \"{field_name}\" is present in the object ({json_val}) but missing in the vector string"
        ),
        instance_path: instance_path.to_string(),
    }
}

pub fn create_field_missing_in_object_error<T: std::fmt::Display>(
    field_name: &str,
    vec_val: &T,
    instance_path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Property \"{field_name}\" is missing in the object but present in the vector string ({vec_val})"
        ),
        instance_path: instance_path.to_string(),
    }
}

/// Checks whether an enum variant represents "not defined" by inspecting its
/// `Debug` representation. All CVSS metric enums from `cvss-rs` derive `Debug`, and their
/// "not defined" variants are uniformly named `NotDefined`.
/// TODO: This is really hacky and should be cleanly implemented in cvss-rs.
/// Alternative to this, we can add a shared IsUndefined Trait to all metrics, but I considered that
/// overly bloated.
fn is_not_defined(val: &impl std::fmt::Debug) -> bool {
    format!("{val:?}") == "NotDefined"
}

/// Compares an optional field from the deserialized JSON object against the value parsed from the
/// vector string. `Some(NotDefined)` is treated as equivalent to `None`.
pub fn check_optional_field_mismatch<T: PartialEq + std::fmt::Display + std::fmt::Debug>(
    field_name: &str,
    json_value: &Option<T>,
    vector_value: &Option<T>,
    instance_path: &str,
    errors: &mut Option<Vec<ValidationError>>,
) {
    // Normalize: treat Some(NotDefined) as None
    let json_effective = json_value.as_ref().filter(|v| !is_not_defined(*v));
    let vector_effective = vector_value.as_ref().filter(|v| !is_not_defined(*v));

    match (json_effective, vector_effective) {
        // both fields exist: compare values
        (Some(json_val), Some(vec_val)) if json_val != vec_val => {
            errors.get_or_insert_default().push(create_field_value_mismatch_error(
                field_name,
                json_val,
                vec_val,
                instance_path,
            ));
        },
        // field exists only in json
        (Some(json_val), None) => {
            errors
                .get_or_insert_default()
                .push(create_field_missing_in_vector_error(
                    field_name,
                    json_val,
                    instance_path,
                ));
        },
        // field exists only in vector
        (None, Some(vec_val)) => {
            errors
                .get_or_insert_default()
                .push(create_field_missing_in_object_error(field_name, vec_val, instance_path));
        },
        // field does not exist
        _ => {},
    }
}

// ---- Typed construction of score objects ----

use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;
use serde::Serialize;
use serde_json::Map;
use std::str::FromStr;

/// Renders a typed CVSS object into the JSON object that a score's
/// `cvss_v2`/`cvss_v3`/`cvss_v4` property carries. The CSAF schemas type these
/// properties as untyped objects; the typed models serialize to exactly the
/// shape the referenced FIRST CVSS schemas expect.
fn to_score_map<T: Serialize>(cvss: &T) -> Map<String, Value> {
    match serde_json::to_value(cvss) {
        Ok(Value::Object(map)) => map,
        // A derived struct serialization always yields an object.
        _ => Map::new(),
    }
}

/// Renders a typed CVSS v2.0 object for a score's `cvss_v2` property.
pub fn cvss_v2_to_score_map(cvss: &CvssV2) -> Map<String, Value> {
    to_score_map(cvss)
}

/// Renders a typed CVSS v3.0/v3.1 object for a score's `cvss_v3` property.
pub fn cvss_v3_to_score_map(cvss: &CvssV3) -> Map<String, Value> {
    to_score_map(cvss)
}

/// Renders a typed CVSS v4.0 object for a content's `cvss_v4` property (CSAF 2.1).
pub fn cvss_v4_to_score_map(cvss: &CvssV4) -> Map<String, Value> {
    to_score_map(cvss)
}

/// Parses a CVSS v2.0 vector and renders its score object with the base score
/// and severity recomputed from the parsed metrics, so the CSAF mandatory
/// tests 6.1.9 and 6.1.10 hold by construction. A vector whose base metrics
/// are incomplete keeps the parsed score as-is.
pub fn cvss_v2_score_map_from_vector(vector: &str) -> Result<Map<String, Value>, cvss_rs::ParseError> {
    let mut cvss = CvssV2::from_str(vector)?;
    if let Some(score) = cvss.calculated_base_score() {
        cvss.severity = Some(v2_severity(score));
        cvss.base_score = score;
    }
    Ok(cvss_v2_to_score_map(&cvss))
}

/// Parses a CVSS v3.0/v3.1 vector and renders its score object with the base
/// score and base severity recomputed from the parsed metrics, so the CSAF
/// mandatory tests 6.1.9 and 6.1.10 hold by construction. A vector whose base
/// metrics are incomplete keeps the parsed score as-is.
pub fn cvss_v3_score_map_from_vector(vector: &str) -> Result<Map<String, Value>, cvss_rs::ParseError> {
    let mut cvss = CvssV3::from_str(vector)?;
    if let Some(score) = cvss.calculated_base_score() {
        cvss.base_severity = v3_severity(score);
        cvss.base_score = score;
    }
    Ok(cvss_v3_to_score_map(&cvss))
}

/// Parses a CVSS v4.0 vector and renders its score object with the base score
/// and base severity recomputed from the parsed metrics, so the CSAF mandatory
/// tests 6.1.9 and 6.1.10 hold by construction. A vector whose base metrics
/// are incomplete keeps the parsed score as-is.
pub fn cvss_v4_score_map_from_vector(vector: &str) -> Result<Map<String, Value>, cvss_rs::ParseError> {
    let mut cvss = CvssV4::from_str(vector)?;
    if let Some(score) = cvss.calculated_base_score() {
        cvss.base_severity = v4_severity(score);
        cvss.base_score = score;
    }
    Ok(cvss_v4_to_score_map(&cvss))
}

/// The CVSS v2.0 severity band of a base score.
fn v2_severity(score: f64) -> cvss_rs::v2_0::Severity {
    use cvss_rs::v2_0::Severity;
    if score < 4.0 {
        Severity::Low
    } else if score < 7.0 {
        Severity::Medium
    } else {
        Severity::High
    }
}

/// The CVSS v3.x severity band of a base score.
fn v3_severity(score: f64) -> cvss_rs::v3::Severity {
    use cvss_rs::v3::Severity;
    if score == 0.0 {
        Severity::None
    } else if score < 4.0 {
        Severity::Low
    } else if score < 7.0 {
        Severity::Medium
    } else if score < 9.0 {
        Severity::High
    } else {
        Severity::Critical
    }
}

/// The CVSS v4.0 severity band of a base score.
fn v4_severity(score: f64) -> cvss_rs::v4_0::Severity {
    use cvss_rs::v4_0::Severity;
    if score == 0.0 {
        Severity::None
    } else if score < 4.0 {
        Severity::Low
    } else if score < 7.0 {
        Severity::Medium
    } else if score < 9.0 {
        Severity::High
    } else {
        Severity::Critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn v3_from_vector_recomputes_score_and_severity() {
        let map = cvss_v3_score_map_from_vector("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").expect("vector parses");
        assert_eq!(map.get("baseScore"), Some(&json!(9.8)));
        assert_eq!(map.get("baseSeverity"), Some(&json!("CRITICAL")));
        assert_eq!(map.get("attackVector"), Some(&json!("NETWORK")));
        assert_eq!(
            map.get("vectorString"),
            Some(&json!("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"))
        );
    }

    #[test]
    fn v2_from_vector_recomputes_score_and_severity() {
        let map = cvss_v2_score_map_from_vector("AV:N/AC:L/Au:N/C:C/I:C/A:C").expect("vector parses");
        assert_eq!(map.get("baseScore"), Some(&json!(10.0)));
        assert_eq!(map.get("severity"), Some(&json!("High")));
        assert_eq!(map.get("accessVector"), Some(&json!("NETWORK")));
    }

    #[test]
    fn v4_from_vector_recomputes_score_and_severity() {
        let map = cvss_v4_score_map_from_vector("CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N")
            .expect("vector parses");
        assert_eq!(map.get("baseScore"), Some(&json!(9.3)));
        assert_eq!(map.get("baseSeverity"), Some(&json!("CRITICAL")));
    }

    #[test]
    fn score_maps_parse_back_into_the_typed_models() {
        let vector = "CVSS:3.0/AV:A/AC:H/PR:L/UI:R/S:C/C:L/I:L/A:N";
        let map = cvss_v3_score_map_from_vector(vector).expect("vector parses");
        let parsed = CvssV3::deserialize(&map).expect("round-trips");
        assert_eq!(parsed.vector_string, vector);
    }

    #[test]
    fn invalid_vectors_are_rejected() {
        assert!(cvss_v3_score_map_from_vector("CVSS:9.9/AV:N").is_err());
        assert!(cvss_v2_score_map_from_vector("").is_err());
        assert!(cvss_v4_score_map_from_vector("CVSS:9.9/AV:N").is_err());
    }
}
