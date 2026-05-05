use super::{LanguageTranslationLookup, Translations};
use quote::{format_ident, quote};

/// Generates the complete token stream for all translation arrays
pub(crate) fn generate_translation_lookups(translations: &Translations) -> proc_macro2::TokenStream {
    let per_term_sections = [
        ("license", &translations.license),
        ("product_description", &translations.product_description),
        ("reasoning_for_supersession", &translations.reasoning_for_supersession),
        ("reasoning_for_withdrawal", &translations.reasoning_for_withdrawal),
        ("superseding_document", &translations.superseding_document),
    ]
    .into_iter()
    .map(|(field_name, lookup)| generate_term_section(field_name, lookup));

    quote! {
        #(#per_term_sections)*
    }
}

/// Generates the token stream for a single term key
fn generate_term_section(term_key: &str, lookup: &LanguageTranslationLookup) -> proc_macro2::TokenStream {
    let array_ident = format_ident!("{}_TRANSLATIONS", term_key.to_uppercase());

    let entries = lookup.iter().map(|(lang, value)| {
        let lang = lang.as_str();
        let value = value.as_str();
        quote! { (#lang, #value) }
    });

    quote! {
        pub static #array_ident: &[(&str, &str)] = &[
            #(#entries),*
        ];
    }
}
