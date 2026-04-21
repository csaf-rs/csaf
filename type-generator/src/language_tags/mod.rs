mod extract;

pub(crate) use extract::*;
use std::collections::HashMap;

mod generate;
#[cfg(test)]
mod tests;

use crate::build_helper::BuildError;
use crate::file_helper::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt};
use generate::generate_kind_section;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt;
use std::fs;
use std::path::Path;

/// The kinds of subtags we extract from the language subtag registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SubtagKind {
    Language,
    Region,
    Script,
    Grandfathered,
}

impl SubtagKind {
    /// All variants in a fixed order.
    pub const ALL: &[SubtagKind] = &[
        SubtagKind::Language,
        SubtagKind::Region,
        SubtagKind::Script,
        SubtagKind::Grandfathered,
    ];

    /// Returns the registry `Type:` value that corresponds to this kind.
    pub fn registry_key(self) -> &'static str {
        match self {
            SubtagKind::Language => "language",
            SubtagKind::Region => "region",
            SubtagKind::Script => "script",
            SubtagKind::Grandfathered => "grandfathered",
        }
    }
}

impl fmt::Display for SubtagKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.registry_key())
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
    let registry_path = Path::new(target_folder).join("assets/language-subtag-registry.txt");
    let registry = fs::read_to_string(&registry_path).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!(
                "Failed to read language subtag registry at {}: {e}",
                registry_path.display()
            ),
        )
    })?;

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
    let out_path = Path::new(target_folder)
        .join("src")
        .join("csaf")
        .join("types")
        .join("language")
        .join("language_subtags.generated.rs");
    // This is only none if outpath starts at root.
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!(
                    "Failed to create output directory for language subtags at {}: {e}",
                    parent.display()
                ),
            )
        })?;
    }
    println!("Writing generated language subtags to: {}", out_path.display());
    fs::write(&out_path, code)?;

    Ok(())
}
