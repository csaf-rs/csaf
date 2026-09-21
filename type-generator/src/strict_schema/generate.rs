use crate::BuildError;
use crate::utils::read_write_fs::{read_file_to_string, write_generated_file};
use serde_json::Value;
use std::path::Path;

fn make_strict(schema_value: Value) -> Value {
    let mut schema_value = schema_value;
    make_strict_inplace(&mut schema_value);
    schema_value
}

fn make_strict_inplace(schema_value: &mut Value) {
    if let Some(obj) = schema_value.as_object_mut() {
        for value in obj.values_mut() {
            make_strict_inplace(value);
        }
        if obj.get("type").and_then(|t| t.as_str()) == Some("object") {
            obj.insert("unevaluatedProperties".to_string(), Value::Bool(false));
        }
        if obj.contains_key("oneOf") {
            obj.insert("unevaluatedProperties".to_string(), Value::Bool(false));
        }
    } else if let Some(array) = schema_value.as_array_mut() {
        for item in array {
            make_strict_inplace(item);
        }
    }
}

pub fn generate_strict_schema(input: &str, output: &str, target_folder: &str) -> Result<(), BuildError> {
    let schema = make_strict(serde_json::from_str(&read_file_to_string(
        &Path::new(target_folder).join(input),
    )?)?);
    write_generated_file(
        target_folder,
        output,
        &serde_json::to_string_pretty(&schema)?,
        "strict schema",
    )?;
    Ok(())
}
