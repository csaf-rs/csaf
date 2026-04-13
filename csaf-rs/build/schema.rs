use serde_json::Value;
use std::fs;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

use super::BuildError;
use super::util::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_rustfmt};

pub fn build(input: &str, output: &str, schema_patch: &Option<&dyn Fn(&mut Value)>) -> Result<(), BuildError> {
    let content = fs::read_to_string(input)?;
    let mut schema_value = serde_json::from_str(&content)?;
    // Execute a schema patch function, if provided.
    if let Some(patch_fn) = schema_patch {
        patch_fn(&mut schema_value);
    }
    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_value)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    // Convert the TypeSpace token stream into a syn::File so we can inject a file-level doc attribute
    let mut file = syn::parse2::<syn::File>(type_space.to_stream())?;

    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    // Parse the GENERATED_CODE_HEADER as a doc attribute
    let doc_attr = syn::parse_quote! { #![doc = #GENERATED_CODE_HEADER] };
    file.attrs.insert(0, doc_attr);

    // Unparse the modified syn::File into Rust source code
    let content = prettyplease::unparse(&file);

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push(output);
    Ok(fs::write(out_file, content)?)
}
