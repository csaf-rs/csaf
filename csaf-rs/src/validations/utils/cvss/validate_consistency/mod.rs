use crate::validation::TestFinding;
use crate::validations::utils::cvss::validate_consistency::errors::{
    create_field_missing_in_object_error, create_field_missing_in_vector_error, create_field_value_mismatch_error,
    create_severity_mismatch_error,
};
use cvss_rs::{Defined, Severity};
use crate::validations::utils::cvss::errors::ScoreType;

pub mod errors;
pub mod validation;
/// Maps a CVSS v3 / v4 score to its severity rating.
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

/// Compares an actual severity against a calculated severity and adds a validation error if they
/// differ.
pub fn check_severity_mismatch(
    actual: &Severity,
    calculated: &Severity,
    score_type: ScoreType,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
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

/// Compares an optional field from the deserialized JSON object against the value parsed from the
/// vector string. `Some(NotDefined)` is treated as equivalent to `None`.
/// For metrics without a `NotDefined` variant, use [check_required_field_mismatch] instead.
pub fn check_optional_field_mismatch<T: PartialEq + std::fmt::Display + Defined>(
    field_name: &str,
    json_value: &Option<T>,
    vector_value: &Option<T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    // Normalize: treat Some(NotDefined) as None
    compare_fields(
        field_name,
        json_value.as_ref().and_then(Defined::as_defined),
        vector_value.as_ref().and_then(Defined::as_defined),
        instance_path,
        errors,
    );
}

/// Compares an optional field from the deserialized JSON object against the value parsed from the
/// vector string, for metrics that have no `NotDefined` variant (e.g. base metrics).
pub fn check_required_field_mismatch<T: PartialEq + std::fmt::Display>(
    field_name: &str,
    json_value: &Option<T>,
    vector_value: &Option<T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    compare_fields(
        field_name,
        json_value.as_ref(),
        vector_value.as_ref(),
        instance_path,
        errors,
    );
}

fn compare_fields<T: PartialEq + std::fmt::Display>(
    field_name: &str,
    json_effective: Option<&T>,
    vector_effective: Option<&T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
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
