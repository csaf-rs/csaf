use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_rustfmt};
use crate::utils::write_to_fs::write_generated_file;
use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate_from_schema(
    schema: schemars::schema::RootSchema,
    output: &str,
    target_folder: &str,
) -> Result<(), BuildError> {
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
    let relative_path = format!("src/schema/{output}");

    write_generated_file(target_folder, &relative_path, &content, "generated types")?;

    Ok(())
}
