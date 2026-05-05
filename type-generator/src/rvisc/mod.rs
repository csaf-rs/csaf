mod extract;
mod generate;

use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt,
};
use crate::utils::read_write_fs::write_generated_file;
use extract::parse_registry;
use generate::generate as generate_registry_code;
use serde::Deserialize;

/// Represents a single entry in the registry JSON file
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct JsonRegistryEntry {
    pub system_name: String,
    pub text_pattern: String,
}

/// The top-level structure in the registry JSON file
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct JsonRegistry {
    pub entries: Vec<JsonRegistryEntry>,
}

/// Generates a lookup from the synced registry JSON file
pub fn generate_registry(target_folder: &str) -> Result<(), BuildError> {
    let registry = parse_registry("assets/rvisc/registry.json")?;

    let tokens = generate_registry_code(&registry.entries);

    let mut file: syn::File = syn::parse2(tokens)?;
    add_generated_code_header(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_ignore_dead_code(&mut file);

    let code = prettyplease::unparse(&file);

    write_generated_file(
        target_folder,
        "src/validations/utils/rvisc/generated.rs",
        &code,
        "generated RVISC lookup",
    )?;

    Ok(())
}
