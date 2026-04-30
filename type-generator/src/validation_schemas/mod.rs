mod config;
mod generate;

use std::path::Path;

use crate::build_errors::BuildError;
use crate::utils::read_write_fs::read_file_to_string;
use config::VALIDATION_SCHEMAS;
use generate::{generate_schema_file, generate_url_file};

/// A validation schema entry that will be embedded in the `csaf-rs` binary
pub struct ValidationSchemaConfig {
    /// Base name for the generated constant and static
    /// (e.g. `"CSAF_2_0_SCHEMA"` → `CSAF_2_0_SCHEMA_URL` + `CSAF_2_0_SCHEMA`).
    var_name: &'static str,
    /// The canonical upstream URL for the schema
    source_url: &'static str,
    /// Path to the JSON file relative to the target folder
    relative_asset_path: &'static str,
}

/// A schema name/URL pair used to generate URL constants
pub struct SchemaUrlEntry {
    pub name: &'static str,
    pub source_url: &'static str,
}

/// A schema entry used to generate lazy-static schema statics
pub struct SchemaEntry {
    pub name: &'static str,
    pub asset_path: &'static str,
}

/// Validate every embedded JSON asset and generate `validation_schema_urls`
/// and `validation_schemas` inside the target folder
pub fn generate_validation_schemas(target_folder: &str) -> Result<(), BuildError> {
    let mut url_entries: Vec<SchemaUrlEntry> = Vec::new();
    let mut schema_entries: Vec<SchemaEntry> = Vec::new();

    for schema in VALIDATION_SCHEMAS {
        let relative_asset_path = schema.relative_asset_path;

        // Validate that the file exists and contains valid JSON
        let content = read_file_to_string(&Path::new(target_folder).join(relative_asset_path))?;
        let _: serde_json::Value = serde_json::from_str(&content)?;

        url_entries.push(SchemaUrlEntry {
            name: schema.var_name,
            source_url: schema.source_url,
        });
        schema_entries.push(SchemaEntry {
            name: schema.var_name,
            asset_path: relative_asset_path,
        });
    }

    generate_url_file(&url_entries, target_folder)?;
    generate_schema_file(&schema_entries, target_folder)?;

    Ok(())
}
