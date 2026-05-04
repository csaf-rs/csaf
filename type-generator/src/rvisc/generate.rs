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
        use regex::Regex;
        use std::collections::HashMap;
        use std::sync::LazyLock;

        /// Lookup array of `(system_name, text_pattern)`
        static REGISTRY_ENTRIES: &[(&str, &str)] = &[
            #(#array_entries),*
        ];

        /// Lazy regex cache
        static REGEX_CACHE: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(|| {
            REGISTRY_ENTRIES
                .iter()
                .map(|&(sn, pattern)| {
                    let re = Regex::new(pattern).expect(
                        "The pattern should be parseable as a regex. This is validated during type generation. Please re-run type generation. (This looks like a dev error)",
                    );
                    (sn, re)
                })
                .collect()
        });

        /// Looks up a rvisc entry by `system_name`
        /// Returns `(system_name, text_pattern)` if found
        pub fn lookup(system_name: &str) -> Option<(&'static str, &'static str)> {
            REGISTRY_ENTRIES
                .binary_search_by(|&(sn, _)| sn.cmp(system_name))
                .ok()
                .map(|idx| REGISTRY_ENTRIES[idx])
        }

        /// Looks up a rvisc entry by `system_name` and returns its pattern as a compiled `Regex`.
        pub fn lookup_regex(system_name: &str) -> Option<&'static Regex> {
            REGEX_CACHE.get(system_name)
        }

        /// Checks if the given `system_name` is a known rvisc entry
        pub fn is_registered_id_system(system_name: &str) -> bool {
            lookup(system_name).is_some()
        }
    }
}
