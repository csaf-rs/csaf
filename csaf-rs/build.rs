use std::io;
use thiserror::Error;

#[path = "build/language_tag.rs"]
mod language_tag;
#[path = "build/schema.rs"]
mod schema;
#[path = "build/util.rs"]
mod util;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("JSON schema error")]
    SchemaError(#[from] typify::Error),
    #[error("Rust syntax error")]
    SyntaxError(#[from] syn::Error),
    #[error("JSON parsing error")]
    JsonError(#[from] serde_json::Error),
    #[error("other error")]
    Other,
}

fn main() -> Result<(), BuildError> {
    println!("cargo:rerun-if-changed=build.rs");

    // All schema files for change watching
    let schema_configs = [
        (
            "assets/decision_point_json_schema.json",
            "csaf2_1/ssvc_dp.generated.rs",
            None,
        ),
        (
            "assets/decision_point_selection_list_json_schema.json",
            "csaf2_1/ssvc_dp_selection_list.generated.rs",
            None,
        ),
    ];

    // Register watching for all inputs
    for (input, _, _) in &schema_configs {
        println!("cargo:rerun-if-changed={input}");
    }

    // Execute all listed schema builds
    for (input, output, schema_patch) in &schema_configs {
        schema::build(input, output, schema_patch)?;
    }

    language_tag::generate()?;

    Ok(())
}
