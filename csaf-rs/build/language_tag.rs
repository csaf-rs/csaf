use quote::{format_ident, quote};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::BuildError;
use super::util::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt};

const LANGUAGE_REGISTRY: &str = include_str!("../assets/language-subtag-registry.txt");

/// Convert a character to a 0-based index relative to the base of its case
#[inline]
fn char_to_index(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        c as u8 - b'a'
    } else {
        c as u8 - b'A'
    }
}

/// Convert a 0-based index back to a character, preserving the case of the
/// reference character.
#[inline]
fn index_to_char(idx: u8, lowercase: bool) -> char {
    if lowercase {
        char::from(b'a' + idx)
    } else {
        char::from(b'A' + idx)
    }
}

// If you want to extract other tags: just add their "key" here!
const SUBTAG_KINDS: &[&str] = &["language", "region", "script", "grandfathered"];

/// Creates an empty subtag map pre-populated with all [`SUBTAG_KINDS`] as keys.
#[inline]
fn make_subtags_map() -> HashMap<&'static str, Vec<(String, bool)>> {
    SUBTAG_KINDS.iter().map(|&k| (k, Vec::new())).collect()
}

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

/// Parses registry line-by-line and populates `map` with the subtags found.
///
/// The registry uses `%%` as a block separator. For each block the fields
/// `Type:`, `Subtag:`, and `Description: Private use` are extracted.
fn parse_registry(registry: &str, map: &mut HashMap<&str, Vec<(String, bool)>>) {
    // init
    let mut current_entry_type: Option<String> = None;
    let mut current_subtag: Option<String> = None;
    let mut current_is_private_use = false;

    // parse file line by line
    for line in registry.lines() {
        let line = line.trim();

        // new block starts
        if line.starts_with("%%") {
            push_block_into_map(&current_entry_type, &current_subtag, current_is_private_use, map);
            current_entry_type = None;
            current_subtag = None;
            current_is_private_use = false;
            continue;
        }

        // extract values
        if let Some(type_value) = line.strip_prefix("Type: ") {
            current_entry_type = Some(type_value.to_string());
        } else if let Some(subtag) = line.strip_prefix("Subtag: ") {
            current_subtag = Some(subtag.to_string());
        } else if let Some(tag) = line.strip_prefix("Tag: ") {
            // Grandfathered (and redundant) entries use "Tag:" instead of "Subtag:"
            current_subtag = Some(tag.to_string());
        } else if line == "Description: Private use" {
            current_is_private_use = true;
        }
    }

    // flush the last block, as the file doesn't end with %%
    push_block_into_map(&current_entry_type, &current_subtag, current_is_private_use, map);
}

/// Handle a parsed registry block and put it into `map`.
///
/// If both `entry_type` and `subtag` are `Some` and the type matches a subtag type
/// we want to extract, the subtag (or expanded range) are put into `map`.
fn push_block_into_map(
    entry_type: &Option<String>,
    subtag: &Option<String>,
    is_private_use: bool,
    map: &mut HashMap<&str, Vec<(String, bool)>>,
) {
    // if all fields are filled and the entry type is one of the types we want to extract
    if let (Some(entry_type), Some(subtag)) = (entry_type, subtag)
        && let Some(&kind) = SUBTAG_KINDS.iter().find(|&&k| k == entry_type.as_str())
    {
        // the subtag is a range ("qaa..qtz"), expand it
        let entries: Vec<(String, bool)> = if let Some((start, end)) = subtag.split_once("..") {
            expand_subtag_range(start, end)
                .into_iter()
                .map(|t| (t, is_private_use))
                .collect()
        }
        // its a single subtag
        else {
            vec![(subtag.clone(), is_private_use)]
        };
        // put the entries in the map
        map.get_mut(kind)
            .expect("SubtagKind registry_key must be present in map – all keys from SUBTAG_KINDS should have been inserted during initialization")
            .extend(entries);
    }
}

/// Expands a subtag range notation (e.g. "qaa..qtz") into all individual subtags.
///
/// Both `start` and `end` must be only ASCII letters and have the same length.
///
/// Panics if:
/// * `start` and `end` have different length
/// * `start` or `end` contain non-alphabetic ASCII chars
fn expand_subtag_range(start: &str, end: &str) -> Vec<String> {
    // validate input
    assert_eq!(start.len(), end.len(), "Range endpoints must have the same length");
    assert!(
        start.chars().all(|c| c.is_ascii_alphabetic()),
        "Range start must only contain ASCII letters (a-zA-Z), got: {start:?}"
    );
    assert!(
        end.chars().all(|c| c.is_ascii_alphabetic()),
        "Range end must only contain ASCII letters (a-zA-Z), got: {end:?}"
    );

    let mut result = Vec::new();

    // get char vecs
    let start_chars: Vec<char> = start.chars().collect();
    let end_chars: Vec<char> = end.chars().collect();

    // get indices
    let mut current: Vec<u8> = start_chars.iter().map(|&c| char_to_index(c)).collect();
    let end_indices: Vec<u8> = end_chars.iter().map(|&c| char_to_index(c)).collect();

    loop {
        // build the current subtag string
        let subtag: String = current
            .iter()
            .zip(start_chars.iter())
            .map(|(&idx, &ref_ch)| index_to_char(idx, ref_ch.is_ascii_lowercase()))
            .collect();
        result.push(subtag);

        // all positions done
        if current == end_indices {
            break;
        }

        // Increment the current subtag as if it were a big-endian base-26 number,
        // where index 0 = 'a'/'A' and index 25 = 'z'/'Z'.
        //
        // Example: iterating from "aay" to "abb"
        // 1) "aay" -> [0, 0, 24] -> +1 -> [0, 0, 25]
        // 2) "aaz" -> [0, 0, 25] -> +1 -> [0, 0, 26] -> carry -> [0, 1, 0]
        // 3) "aba" -> [0, 1, 0]  -> +1 -> [0, 1, 1] -> "abb"
        let mut carry = true;
        for i in (0..current.len()).rev() {
            if carry {
                current[i] += 1;
                if current[i] > 25 {
                    current[i] = 0;
                    // carry propagates to the next position
                } else {
                    carry = false;
                }
            }
        }
    }

    result
}

/// Generates the token stream for a single subtag kind section.
fn generate_kind_section(kind: &str, subtags: &[(String, bool)]) -> impl quote::ToTokens {
    let tags: Vec<&str> = subtags.iter().map(|(s, _)| s.as_str()).collect();
    let privs: Vec<bool> = subtags.iter().map(|(_, p)| *p).collect();

    let array_ident = format_ident!("{}_SUBTAGS_ARRAY", kind.to_uppercase());
    let is_valid_fn = format_ident!("is_valid_{}_subtag", kind);
    let is_private_fn = format_ident!("is_{}_private_use", kind);

    let is_valid_doc = format!(
        "Checks if a given subtag is a valid {} subtag. Lower cases the input before checking.",
        kind,
    );
    let is_private_doc = format!(
        "Checks if a given {} subtag is registered as private use. Lower cases the input before checking.",
        kind,
    );

    quote! {
        pub static #array_ident: &[(&str, bool)] = &[
            #((#tags, #privs)),*
        ];

        #[doc = #is_valid_doc]
        pub fn #is_valid_fn(subtag: &str) -> bool {
            lookup(#array_ident, &subtag.to_lowercase()).is_some()
        }

        #[doc = #is_private_doc]
        pub fn #is_private_fn(subtag: &str) -> bool {
            lookup(#array_ident, &subtag.to_lowercase()).is_some_and(|(_, is_private_use)| is_private_use)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{char_to_index, expand_subtag_range, index_to_char, make_subtags_map, parse_registry};
    use rstest::rstest;

    #[test]
    fn char_a_incremented_to_b() {
        // Convert 'a' → index, add 1, convert back → 'b'
        let idx = char_to_index('a');
        let next = index_to_char(idx + 1, /* lowercase = */ true);
        assert_eq!(next, 'b');
    }

    #[rstest]
    // single char range
    #[case("a", "e", vec!["a", "b", "c", "d", "e"])]
    // not really a range
    #[case("a", "a", vec!["a"])]
    // two char range
    #[case("aa", "ac", vec!["aa", "ab", "ac"])]
    // three char range + carry over
    #[case("aay", "abb", vec!["aay", "aaz", "aba", "abb"])]
    // uppercase also works
    #[case("AA", "AC", vec!["AA", "AB", "AC"])]
    // mixed case also works
    #[case("Aa", "Ac", vec!["Aa", "Ab", "Ac"])]
    fn expand_range_exact(#[case] start: &str, #[case] end: &str, #[case] expected: Vec<&str>) {
        assert_eq!(expand_subtag_range(start, end), expected);
    }

    #[rstest]
    // single char full alphabet
    #[case("a", "z", 26, "a", "z")]
    // same first char, full alphabet
    #[case("aa", "az", 26, "aa", "az")]
    // first and second same char, full alphabet
    #[case("aaa", "aaz", 26, "aaa", "aaz")]
    // primary language subtag range
    #[case("qaa", "qtz", 20 * 26, "qaa", "qtz")]
    fn expand_range_count(
        #[case] start: &str,
        #[case] end: &str,
        #[case] expected_len: usize,
        #[case] expected_first: &str,
        #[case] expected_last: &str,
    ) {
        let result = expand_subtag_range(start, end);
        assert_eq!(result.len(), expected_len);
        assert_eq!(result.first().unwrap(), expected_first);
        assert_eq!(result.last().unwrap(), expected_last);
    }

    #[test]
    #[should_panic(expected = "Range endpoints must have the same length")]
    fn panics_on_different_lengths() {
        expand_subtag_range("a", "ab");
    }

    #[test]
    #[should_panic(expected = "Range start must only contain ASCII letters")]
    fn panics_on_non_alpha_start() {
        expand_subtag_range("a1", "az");
    }

    #[test]
    #[should_panic(expected = "Range end must only contain ASCII letters")]
    fn panics_on_non_alpha_end() {
        expand_subtag_range("aa", "a9");
    }

    #[test]
    #[should_panic(expected = "Range start must only contain ASCII letters")]
    fn panics_on_whitespace_in_start() {
        expand_subtag_range("a ", "az");
    }

    #[rstest]
    // language subtag is inserted
    #[case("%%\nType: language\nSubtag: en\n%%\n", "language", vec![("en".to_string(), false)])]
    // region subtag is inserted
    #[case("%%\nType: region\nSubtag: DE\n%%\n", "region", vec![("DE".to_string(), false)])]
    // script subtag is inserted
    #[case("%%\nType: script\nSubtag: Latn\n%%\n", "script", vec![("Latn".to_string(), false)])]
    // private-use flag is detected
    #[case("%%\nType: language\nSubtag: qaa\nDescription: Private use\n%%\n", "language", vec![("qaa".to_string(), true)])]
    // range subtag is expanded
    #[case("%%\nType: language\nSubtag: aa..ac\n%%\n", "language", vec![("aa".to_string(), false), ("ab".to_string(), false), ("ac".to_string(), false)])]
    // grandfathered tag using "Tag:" is inserted
    #[case("%%\nType: grandfathered\nTag: i-default\nDescription: Default Language\n%%\n", "grandfathered", vec![("i-default".to_string(), false)])]
    fn parse_registry_single_block(
        #[case] input: &str,
        #[case] expected_key: &str,
        #[case] expected_entries: Vec<(String, bool)>,
    ) {
        let mut map = make_subtags_map();
        parse_registry(input, &mut map);
        assert_eq!(map[expected_key], expected_entries);
    }

    #[rstest]
    // unknown type is ignored
    #[case("%%\nType: extlang\nSubtag: aao\n%%\n")]
    // block without a type is ignored
    #[case("%%\nSubtag: en\n%%\n")]
    // block without a subtag is ignored
    #[case("%%\nType: language\n%%\n")]
    // empty input
    #[case("")]
    // redundant type is ignored
    #[case("%%\nType: redundant\nTag: en-scouse\n%%\n")]
    // variant type is ignored
    #[case("%%\nType: variant\nSubtag: rozaj\n%%\n")]
    fn parse_registry_skipped_blocks(#[case] input: &str) {
        let mut map = make_subtags_map();
        parse_registry(input, &mut map);
        assert!(map.values().all(|v| v.is_empty()));
    }

    #[test]
    fn parse_registry_multiple_blocks() {
        let input = "%%\nType: language\nSubtag: en\n%%\nType: region\nSubtag: DE\n%%\n";
        let mut map = make_subtags_map();
        parse_registry(input, &mut map);
        assert_eq!(map["language"], vec![("en".to_string(), false)]);
        assert_eq!(map["region"], vec![("DE".to_string(), false)]);
    }

    #[test]
    fn parse_registry_flushes_last_block_without_trailing_separator() {
        // The real registry does not end with %% – the last block must still be flushed.
        let input = "%%\nType: language\nSubtag: en";
        let mut map = make_subtags_map();
        parse_registry(input, &mut map);
        assert_eq!(map["language"], vec![("en".to_string(), false)]);
    }

    #[test]
    fn parse_registry_private_use_resets_between_blocks() {
        // private-use flag from one block must not bleed into the next
        let input = "%%\nType: language\nSubtag: qaa\nDescription: Private use\n%%\nType: language\nSubtag: en\n%%\n";
        let mut map = make_subtags_map();
        parse_registry(input, &mut map);
        assert_eq!(
            map["language"],
            vec![("qaa".to_string(), true), ("en".to_string(), false),]
        );
    }
}
