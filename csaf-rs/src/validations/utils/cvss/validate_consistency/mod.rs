use crate::validation::TestFinding;
use crate::validations::utils::cvss::dispatch::validate_metric;
use crate::validations::utils::cvss::validate_consistency::errors::{
    create_field_missing_in_object_error, create_field_missing_in_vector_error, create_field_value_mismatch_error,
};
use cvss_rs::{Cvss, Defined, Severity, Version};
use serde_json::Value;

pub mod errors;
pub mod validation;

/// Deserializes and extracts the expected CVSS variant, and validates the consistency of the JSON vs.
/// the CVSS vector.
///
/// The `expected_version` parameter determines which version-specific validation is
/// applied. [Version::V3_0] is used as a placeholder to convey that a CVSS v3 validation should be done.
pub(crate) fn validate_consistency(
    cvss_map: &serde_json::Map<String, Value>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
    expected_version: Version,
) {
    validate_metric(
        cvss_map,
        instance_path,
        errors,
        expected_version,
        |cvss, _, instance_path, errors| match cvss {
            Cvss::V2(cvss2) => {
                validation::validate_v2_consistency(&cvss2, instance_path, errors);
            },
            Cvss::V3_0(cvss3) | Cvss::V3_1(cvss3) => {
                validation::validate_v3_consistency(&cvss3, instance_path, errors);
            },
            Cvss::V4(cvss4) => {
                validation::validate_v4_consistency(&cvss4, instance_path, errors);
            },
        },
    );
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
