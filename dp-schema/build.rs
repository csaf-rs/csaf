use typify::{TypeSpace, TypeSpaceSettings};
use std::fs;

static SCHEMA_PATH: &str = "../ssvc/data/schema/v1/Decision_Point-1-0-1.schema.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_content = fs::read_to_string(SCHEMA_PATH)?;
    let schema_value: serde_json::Value = serde_json::from_str(&schema_content)?;
    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_value)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let generated_code = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);
    fs::write("src/decision_point.rs", generated_code)?;

    println!("cargo:rerun-if-changed={}", SCHEMA_PATH);
    Ok(())
}
