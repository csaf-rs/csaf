use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Defined;

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

pub fn create_field_value_mismatch_error<T: std::fmt::Display>(
    field_name: &str,
    json_val: &T,
    vec_val: &T,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Property \"{field_name}\" does not match the value from the vector string. \
             Expected: {vec_val}, found: {json_val}"
        ),
        instance_path: instance_path.to_string(),
    })
}

pub fn create_field_missing_in_vector_error<T: std::fmt::Display>(
    field_name: &str,
    json_val: &T,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Property \"{field_name}\" is present in the object ({json_val}) but missing in the vector string"
        ),
        instance_path: instance_path.to_string(),
    })
}

pub fn create_field_missing_in_object_error<T: std::fmt::Display>(
    field_name: &str,
    vec_val: &T,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Property \"{field_name}\" is missing in the object but present in the vector string ({vec_val})"
        ),
        instance_path: instance_path.to_string(),
    })
}
