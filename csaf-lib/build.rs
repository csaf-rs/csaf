use std::path::Path;
use std::{fs, io};
use thiserror::Error;
use typify::{TypeSpace, TypeSpaceSettings};

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
    build(
        "./src/csaf/csaf2_0/csaf_json_schema.json",
        "csaf/csaf2_0/schema.rs",
    )?;
    build(
        "./src/csaf/csaf2_1/csaf_json_schema.json",
        "csaf/csaf2_1/schema.rs",
    )?;

    Ok(())
}

fn build(input: &str, output: &str) -> Result<(), BuildError> {
    let content = fs::read_to_string(input)?;
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let content = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push(output);
    Ok(fs::write(out_file, content)?)
}
