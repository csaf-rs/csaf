use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{add_generated_code_header, add_ignore_dead_code};
use crate::utils::read_write_fs::write_generated_file;

use super::{SchemaEntry, SchemaUrlEntry};


/// Generate `validation_schema_urls.rs` with a file-level `#![allow(dead_code)]`
pub fn generate_url_file(entries: &[SchemaUrlEntry], target_folder: &str) -> Result<(), BuildError> {
    // Generate code snippets
    let items: TokenStream = entries
        .iter()
        .map(|entry| {
            let ident = format_ident!("{}_URL", entry.name);
            let url = entry.source_url;
            quote! { pub const #ident: &str = #url; }
        })
        .collect();

    let mut file: syn::File =
        syn::parse2(items).expect("quote!-generated token stream should always be valid syntax");

    add_ignore_dead_code(&mut file);
    add_generated_code_header(&mut file);

    write_generated_file(
        target_folder,
        "src/validations/utils/validation_schema_urls.rs",
        &prettyplease::unparse(&file),
        "generated validation schema URLs",
    )
}

/// Generate `validation_schemas.rs` (lazy statics)
pub fn generate_schema_file(entries: &[SchemaEntry], target_folder: &str) -> Result<(), BuildError> {
    const OUTPUT_PATH: &str = "src/validations/utils/validation_schemas.rs";

    // Compute how many '../'s are needed to get from the output file
    // directory back to the target_folder root, from which we can go to the asset path
    // that is relative to the target_folder.
    let depth = Path::new(OUTPUT_PATH)
        .parent()
        .map_or(0, |p| p.components().count());
    let prefix = "../".repeat(depth);

    let preamble: TokenStream = quote! {
        use serde_json::Value;
        use std::sync::LazyLock;
    };
    let items: TokenStream = entries
        .iter()
        .map(|entry| {
            let ident = format_ident!("{}", entry.name);
            let asset_path = entry.asset_path;
            let include_path = format!("{prefix}{asset_path}");
            let expect_msg = format!(
                "embedded JSON schema {asset_path} was validated at build-time"
            );
            quote! {
                pub static #ident: LazyLock<Value> = LazyLock::new(|| {
                    serde_json::from_str(include_str!(#include_path))
                        .expect(#expect_msg)
                });
            }
        })
        .collect();
    let full: TokenStream = quote! { #preamble #items };

    let mut file: syn::File =
        syn::parse2(full).expect("quote!-generated token stream should always be valid syntax");

    add_generated_code_header(&mut file);

    write_generated_file(
        target_folder,
        OUTPUT_PATH,
        &prettyplease::unparse(&file),
        "generated validation schemas",
    )
}

