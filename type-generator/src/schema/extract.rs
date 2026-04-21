use crate::build_errors::BuildError;
use crate::utils::read_write_fs::read_file_to_string;
use serde_json::Value;
use std::path::Path;

/// Loads a JSON schema from disk, optionally applies a patch function,
/// and returns the parsed `RootSchema`.
pub fn load_and_patch_schema(
    input: &str,
    schema_patch: Option<&dyn Fn(&mut Value)>,
) -> Result<schemars::schema::RootSchema, BuildError> {
    let content = read_file_to_string(Path::new(input))?;
    let mut schema_value: Value = serde_json::from_str(&content)?;

    if let Some(patch_fn) = schema_patch {
        patch_fn(&mut schema_value);
    }

    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_value)?;
    Ok(schema)
}
