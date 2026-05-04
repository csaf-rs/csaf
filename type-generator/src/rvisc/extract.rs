use regex::Regex;
use serde::de::Error;
use std::path::Path;

use crate::build_errors::BuildError;
use crate::rvisc::JsonRegistry;
use crate::utils::read_write_fs::read_file_to_string;

/// Reads and parses the rvisc JSON file into a `JsonRegistry`
/// Validates that every `text_pattern` is a valid regex
pub(crate) fn parse_registry(registry_path: &str) -> Result<JsonRegistry, BuildError> {
    let content = read_file_to_string(Path::new(registry_path))?;
    let registry: JsonRegistry = serde_json::from_str(&content)?;

    for entry in &registry.entries {
        Regex::new(&entry.text_pattern).map_err(|e| {
            BuildError::Json(serde_json::Error::custom(format!(
                "Invalid regex in RVISC entry '{}': {e}",
                entry.text_pattern
            )))
        })?;
    }

    Ok(registry)
}
