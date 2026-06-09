use json_dotpath::DotPaths;
use serde_json::{Value, json};

use super::SchemaConfig;
use crate::build_errors::BuildError;

pub fn get_schemas() -> Vec<SchemaConfig> {
    vec![
        SchemaConfig {
            input: "assets/csaf_2.0_json_schema.json",
            output: "csaf2_0/schema.rs",
            patch: Some(&fix_2_0_schema),
        },
        SchemaConfig {
            input: "assets/csaf_2.1_json_schema.json",
            output: "csaf2_1/schema.rs",
            patch: Some(&fix_2_1_schema),
        },
    ]
}

pub fn get_testcases_schemas() -> Vec<SchemaConfig> {
    vec![
        SchemaConfig {
            input: "assets/csaf_2.0_testcases_json_schema.json",
            output: "csaf2_0/testcases_schema.rs",
            patch: None,
        },
        SchemaConfig {
            input: "assets/csaf_2.1_testcases_json_schema.json",
            output: "csaf2_1/testcases_schema.rs",
            patch: None,
        },
    ]
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.0.
fn fix_2_0_schema(value: &mut Value) -> Result<(), BuildError> {
    let prefix = "properties.vulnerabilities.items.properties.scores.items.properties";
    let fix_paths = [format!("{prefix}.cvss_v2"), format!("{prefix}.cvss_v3")];
    for path in fix_paths {
        patch_dot_set(value, &path, json!({"type": "object"}))?;
    }
    remove_format(value, "date-time");
    remove_format(value, "uri");
    Ok(())
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.1.
fn fix_2_1_schema(value: &mut Value) -> Result<(), BuildError> {
    let prefix = "properties.vulnerabilities.items.properties.metrics.items.properties.content.properties";
    let fix_paths = [
        format!("{prefix}.cvss_v2"),
        format!("{prefix}.cvss_v3"),
        format!("{prefix}.cvss_v4"),
        format!("{prefix}.ssvc_v1"),
        format!("{prefix}.ssvc_v2"),
        "$defs.extensions_t.items".to_string(),
    ];
    for path in fix_paths {
        patch_dot_set(value, &path, json!({"type": "object"}))?;
    }
    remove_format(value, "date-time");
    remove_format(value, "uri");
    Ok(())
}

/// Applies `dot_set` and returns a structured error on failure.
fn patch_dot_set(value: &mut Value, path: &str, replacement: Value) -> Result<(), BuildError> {
    value.dot_set(path, replacement).map_err(|e| {
        BuildError::SchemaPatch(format!(
            "failed to patch schema at path '{path}': {e}. \
             The upstream schema structure may have changed."
        ))
    })
}

/// Recursively searches for a specific "format" value and removes it.
fn remove_format(value: &mut Value, format_value: &str) {
    if let Value::Object(map) = value {
        if let Some(format) = map.get("format")
            && format.as_str() == Some(format_value)
        {
            map.remove("format");
        }

        for (_, v) in map.iter_mut() {
            remove_format(v, format_value);
        }
    } else if let Value::Array(arr) = value {
        for item in arr.iter_mut() {
            remove_format(item, format_value);
        }
    }
}
