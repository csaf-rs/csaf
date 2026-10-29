use crate::validation::{TestFinding, TestFindingData};

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
