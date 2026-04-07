use quote::{format_ident, quote};
use std::fs;
use std::path::Path;

use super::BuildError;
use super::language_tag_parser::{SUBTAG_KINDS, make_subtags_map, parse_registry};
use super::util::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt};

const LANGUAGE_REGISTRY: &str = include_str!("../assets/language-subtag-registry.txt");

/// Generates the language subtags array from the build-embedded text file.
pub fn generate() -> Result<(), BuildError> {
    let mut subtags_by_kind = make_subtags_map();

    parse_registry(LANGUAGE_REGISTRY, &mut subtags_by_kind);

    // Sort all subtag lists by tag.
    for list in subtags_by_kind.values_mut() {
        // When the tests that check subtag casing get added, this will need to be removed.
        // We'll probably need a tuple (original_cased_tag, lower_cased_tag, is_private_use) then.
        for (tag, _) in list.iter_mut() {
            *tag = tag.to_ascii_lowercase();
        }
        list.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    // Generate code for each subtag kind in a loop.
    let per_kind_sections: Vec<_> = SUBTAG_KINDS
        .iter()
        .map(|&kind| generate_kind_section(kind, &subtags_by_kind[kind]))
        .collect();

    let tokens = quote! {
        #![doc = #GENERATED_CODE_HEADER]

        /// Looks up a subtag in a sorted `&[(&str, bool)]` array by key.
        /// Returns the matching `(tag, is_private_use)` tuple if found.
        fn lookup(array: &'static [(&'static str, bool)], key: &str) -> Option<(&'static str, bool)> {
            array
                .binary_search_by_key(&key, |(tag, _)| tag)
                .ok()
                .map(|idx| array[idx])
        }

        #(#per_kind_sections)*
    };

    let mut file: syn::File = syn::parse2(tokens)?;
    // add headers
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    // TODO: This should be removed in the future, i.e. we should only generate needed code.
    add_ignore_dead_code(&mut file);

    // Pretty-print the generated code.
    let code = prettyplease::unparse(&file);

    // write the file
    let out_path = Path::new("src")
        .join("csaf")
        .join("types")
        .join("language")
        .join("language_subtags.generated.rs");
    fs::write(&out_path, code)?;

    println!("cargo:rerun-if-changed=assets/language-subtag-registry.txt");
    Ok(())
}

/// Generates the token stream for a single subtag kind section.
fn generate_kind_section(kind: &str, subtags: &[(String, bool)]) -> impl quote::ToTokens {
    let tags: Vec<&str> = subtags.iter().map(|(s, _)| s.as_str()).collect();
    let privs: Vec<bool> = subtags.iter().map(|(_, p)| *p).collect();

    let array_ident = format_ident!("{}_SUBTAGS_ARRAY", kind.to_uppercase());
    let is_valid_fn = format_ident!("is_valid_{}_subtag", kind);
    let is_private_fn = format_ident!("is_{}_private_use", kind);

    let is_valid_doc =
        format!("Checks if a given subtag is a valid {kind} subtag. Lower cases the input before checking.");
    let is_private_doc =
        format!("Checks if a given {kind} subtag is registered as private use. Lower cases the input before checking.");

    quote! {
        pub static #array_ident: &[(&str, bool)] = &[
            #((#tags, #privs)),*
        ];

        #[doc = #is_valid_doc]
        pub fn #is_valid_fn(subtag: &str) -> bool {
            lookup(#array_ident, &subtag.to_ascii_lowercase()).is_some()
        }

        #[doc = #is_private_doc]
        pub fn #is_private_fn(subtag: &str) -> bool {
            lookup(#array_ident, &subtag.to_ascii_lowercase()).is_some_and(|(_, is_private_use)| is_private_use)
        }
    }
}
