pub mod config;
pub mod extract;
pub mod generate;

use serde_json::Value;

use crate::build_errors::BuildError;

pub struct SchemaConfig {
    pub input: &'static str,
    pub output: &'static str,
    pub patch: Option<&'static dyn Fn(&mut Value)>,
}

/// Loads, patches, and generates types for a given schema configuration.
pub fn build_schema(config: &SchemaConfig, target_folder: &str) -> Result<(), BuildError> {
    let schema = extract::load_and_patch_schema(config.input, config.patch)?;
    generate::generate_from_schema(schema, config.output, target_folder)?;
    Ok(())
}
