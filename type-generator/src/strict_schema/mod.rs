mod config;
mod generate;

use crate::BuildError;
use config::get_strict_schemas;
use generate::generate_strict_schema;

pub struct StrictSchemaConfig {
    pub input: &'static str,
    pub output: &'static str,
}

pub fn generate_strict_schemas(target_folder: &str) -> Result<(), BuildError> {
    for schema in get_strict_schemas() {
        generate_strict_schema(schema.input, schema.output, target_folder)?;
    }
    Ok(())
}
