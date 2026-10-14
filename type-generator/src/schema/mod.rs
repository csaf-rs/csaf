pub mod config;
pub mod extract;
pub mod generate;

use core::fmt;

use serde_json::Value;
use typify::TypeSpace;

use crate::build_errors::BuildError;

pub type SchemaPatchFn = dyn Fn(&mut Value) -> Result<(), BuildError>;

pub struct SchemaConfig {
    pub input: &'static str,
    pub output: &'static str,
    pub patch: Option<&'static SchemaPatchFn>,
    pub version: SchemaVersion,
}

/// Loads, patches, and generates types for a given schema configuration.
pub fn build_schema(config: &SchemaConfig, target_folder: &str) -> Result<TypeSpace, BuildError> {
    let schema = extract::load_and_patch_schema(config.input, config.patch)?;
    let type_space = generate::generate_from_schema(schema, config.output, target_folder)?;
    Ok(type_space)
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaVersion {
    V20,
    V21,
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::V20 => "v20",
            Self::V21 => "v21",
        };
        f.write_str(s)
    }
}
