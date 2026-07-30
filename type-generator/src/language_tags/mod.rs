mod extract;

pub(crate) use extract::*;
use std::collections::HashMap;

mod generate;
#[cfg(test)]
mod tests;

use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt,
};
use crate::utils::read_write_fs::{read_file_to_string, write_generated_file};
use generate::generate_kind_section;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;
use strum::{AsRefStr, Display};

/// The kinds of subtags we extract from the language subtag registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, AsRefStr)]
pub(crate) enum SubtagKind {
    #[strum(serialize = "language")]
    Language,
    #[strum(serialize = "region")]
    Region,
    #[strum(serialize = "script")]
    Script,
    #[strum(serialize = "grandfathered")]
    Grandfathered,
}

impl SubtagKind {
    /// All variants in a fixed order.
    pub const ALL: &[SubtagKind] = &[
        Self::Language,
        Self::Region,
        Self::Script,
        Self::Grandfathered,
    ];

    /// Returns the registry `Type:` value that corresponds to this kind.
    pub fn registry_key(&self) -> &str {
        self.as_ref()
    }
}

/// A single subtag extracted from the language subtag registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubtagEntry {
    pub subtag: String,
    pub is_private: bool,
}

/// Generates the language subtags array from the registry text file.
pub fn generate_language_tags(target_folder: &str) -> Result<(), BuildError> {
    let registry_path = Path::new("assets/language-subtag-registry.txt");
    let registry = read_file_to_string(registry_path)?;

    let mut subtags_by_kind: HashMap<SubtagKind, Vec<SubtagEntry>> = make_subtags_map();

    parse_registry(&registry, &mut subtags_by_kind);

    // Sort all subtag lists by tag.
    for list in subtags_by_kind.values_mut() {
        // When the tests that check subtag casing get added, this will need to be removed.
        // We'll probably need a tuple (original_cased_tag, lower_cased_tag, is_private_use) then.
        for entry in list.iter_mut() {
            entry.subtag = entry.subtag.to_ascii_lowercase();
        }
        list.sort_unstable_by(|a, b| a.subtag.cmp(&b.subtag));
    }

    // Generate code for each subtag kind in a loop.
    let per_kind_sections: Vec<TokenStream> = SubtagKind::ALL
        .iter()
        .map(|kind| generate_kind_section(kind, &subtags_by_kind[kind]))
        .collect();

    let tokens = quote! {

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
    add_generated_code_header(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    // TODO: This should be removed in the future, i.e. we should only generate needed code.
    add_ignore_dead_code(&mut file);

    // Pretty-print the generated code.
    let code = prettyplease::unparse(&file);

    // write the file
    write_generated_file(
        target_folder,
        "src/csaf/types/language/language_subtags.generated.rs",
        &code,
        "generated language subtags",
    )?;

    Ok(())
}
