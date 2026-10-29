use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Defined;

enum ComparisonValue<'a, T> {
    MetricValueDefined(&'a T),
    MetricValueNotDefined,
    MetricNotPresent
}

/// Compares an optional field from the deserialized JSON object against the value parsed from the
/// vector string. `Some(NotDefined)` is treated as equivalent to `None`.
///
/// For metrics without a `NotDefined` variant, use [check_required_field_mismatch] instead.
pub fn check_field_mismatch_with_not_defined<T: PartialEq + std::fmt::Display + Defined>(
    field_name: &str,
    json_value: &Option<T>,
    vector_value: &Option<T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    // Normalize: treat Some(NotDefined) as None
    let json = match json_value {
        Some(value) if value.is_defined() => ComparisonValue::MetricValueDefined(value),
        Some(_) => ComparisonValue::MetricValueNotDefined,
        None => ComparisonValue::MetricNotPresent,
    };
    let vector = match vector_value {
        Some(value) if value.is_defined() => ComparisonValue::MetricValueDefined(value),
        Some(_) => ComparisonValue::MetricValueNotDefined,
        None => ComparisonValue::MetricNotPresent,
    };
    compare_fields(
        field_name,
        json,
        vector,
        instance_path,
        errors,
    );
}

/// Compares an optional field from the deserialized JSON object against the value parsed from the
/// vector string, i.e. metrics that have no `NotDefined` variant (e.g. base metrics).
///
/// For metrics with a `NotDefined` variant, use [check_field_mismatch_with_not_defined] instead.
pub fn check_required_field_mismatch<T: PartialEq + std::fmt::Display>(
    field_name: &str,
    json_value: &Option<T>,
    vector_value: &Option<T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    let json = match json_value {
        Some(value) => ComparisonValue::MetricValueDefined(value),
        None => ComparisonValue::MetricNotPresent,
    };
    let vector = match vector_value {
        Some(value) => ComparisonValue::MetricValueDefined(value),
        None => ComparisonValue::MetricNotPresent,
    };
    compare_fields(
        field_name,
        json,
        vector,
        instance_path,
        errors,
    );
}

fn compare_fields<T: PartialEq + std::fmt::Display>(
    field_name: &str,
    json_effective: ComparisonValue<T>,
    vector_effective: ComparisonValue<T>,
    instance_path: &str,
    errors: &mut Option<Vec<TestFinding>>,
) {
    match (json_effective, vector_effective) {
        // both fields exist and are defined, but with different values
        (ComparisonValue::MetricValueDefined(json_val), ComparisonValue::MetricValueDefined(vec_val)) => {
            if json_val != vec_val {
                errors.get_or_insert_default().push(create_field_value_mismatch_error(
                    field_name,
                    json_val,
                    vec_val,
                    instance_path,
                ));
            }
        },
        // both fields exist, but one field has a Defined value, while the other has the value NotDefined
        (ComparisonValue::MetricValueDefined(json_val), ComparisonValue::MetricValueNotDefined) => {
            errors.get_or_insert_default().push(create_field_not_defined_in_vec_but_defined_in_json_error(
                field_name,
                json_val,
                instance_path,
            ));
        }
        (ComparisonValue::MetricValueNotDefined, ComparisonValue::MetricValueDefined(vec_val)) => {
            errors.get_or_insert_default().push(create_field_defined_in_vec_not_defined_in_json_error(
                field_name,
                vec_val,
                instance_path,
            ));
        }
        // both sides agree the value is "not defined"
        (ComparisonValue::MetricValueNotDefined, ComparisonValue::MetricValueNotDefined) => {}
        // if either side is not present, and the other is not not present
        (ComparisonValue::MetricNotPresent, ComparisonValue::MetricValueDefined(_)) |
        (ComparisonValue::MetricNotPresent, ComparisonValue::MetricValueNotDefined) |
        (ComparisonValue::MetricValueDefined(_), ComparisonValue::MetricNotPresent ) |
        (ComparisonValue::MetricValueNotDefined, ComparisonValue::MetricNotPresent ) => {
        }
        // if both sides are not present, it's just not present
        (ComparisonValue::MetricNotPresent, ComparisonValue::MetricNotPresent) => {}
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

pub fn create_field_defined_in_vec_not_defined_in_json_error<T: std::fmt::Display>(
    field_name: &str,
    vec_val: &T,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Property \"{field_name}\" is present in the vector string with value `{vec_val}` but has the value `NOT_DEFINED` in the object"
        ),
        instance_path: instance_path.to_string(),
    })
}

pub fn create_field_not_defined_in_vec_but_defined_in_json_error<T: std::fmt::Display>(
    field_name: &str,
    json_val: &T,
    instance_path: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Property \"{field_name}\" is present in the object with value `{json_val}` but has the value `ND` in the vector string"
        ),
        instance_path: instance_path.to_string(),
    })
}