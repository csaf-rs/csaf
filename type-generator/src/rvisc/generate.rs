use crate::rvisc::JsonRegistryEntry;
use proc_macro2::TokenStream;
use quote::quote;

/// Generates a static lookup array and lookup functions from rvisc entries
pub(crate) fn generate(entries: &[JsonRegistryEntry]) -> TokenStream {
    // Sort entries by system_name for binary search
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| a.system_name.cmp(&b.system_name));

    let array_entries = sorted_entries.iter().map(|e| {
        let sn = e.system_name.as_str();
        let tp = e.text_pattern.as_str();
        quote! { (#sn, #tp) }
    });

    quote! {
        /// Lookup array of `(system_name, text_pattern)`
        pub(super) static REGISTRY_ENTRIES: &[(&str, &str)] = &[
            #(#array_entries),*
        ];
    }
}
