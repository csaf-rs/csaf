use crate::language_tags::{SubtagEntry, SubtagKind};
use quote::{format_ident, quote};

/// Generates the token stream for a single subtag kind section.
pub(crate) fn generate_kind_section(kind: &SubtagKind, subtags: &[SubtagEntry]) -> proc_macro2::TokenStream {
    let kind_str = kind.registry_key();
    let array_ident = format_ident!("{}_SUBTAGS_ARRAY", kind_str.to_uppercase());
    let is_valid_fn = format_ident!("is_valid_{}_subtag", kind_str);
    let is_private_fn = format_ident!("is_{}_private_use", kind_str);

    let is_valid_doc =
        format!("Checks if a given subtag is a valid {kind_str} subtag. Lower cases the input before checking.");
    let is_private_doc = format!(
        "Checks if a given {kind_str} subtag is registered as private use. Lower cases the input before checking."
    );

    let entries = subtags.iter().map(|e| {
        let tag = e.subtag.as_str();
        let is_private = e.is_private;
        quote! { (#tag, #is_private) }
    });

    quote! {
        pub static #array_ident: &[(&str, bool)] = &[
            #(#entries),*
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
