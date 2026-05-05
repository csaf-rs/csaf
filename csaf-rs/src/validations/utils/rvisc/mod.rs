mod generated;
use generated::REGISTRY_ENTRIES;

use regress::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static REGISTRY_HASHMAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(|| {
    REGISTRY_ENTRIES
        .iter()
        .map(|&(sn, pattern)| {
            let re = Regex::new(pattern)
                .expect(
                    "The pattern should be parseable as a regex. This is validated during type generation. Please re-run type generation. (This looks like a dev error)",
                );
            (sn, re)
        })
        .collect()
});

/// Looks up a rvisc entry by `system_name` and returns its pattern as a compiled `Regex`.
pub(crate) fn lookup_regex(system_name: &str) -> Option<&'static Regex> {
    REGISTRY_HASHMAP.get(system_name)
}

/// Checks if the given `system_name` is a known rvisc entry
pub(crate) fn is_registered_id_system(system_name: &str) -> bool {
    REGISTRY_HASHMAP.contains_key(system_name)
}
