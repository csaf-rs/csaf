// This code is shared between the build script (via `#[path]`) and the library
// (for testing). The functions are not called from library code at runtime.
// Without this attribute, cargo will complain here.
#![allow(dead_code)]

use std::collections::HashMap;

/// Convert a character to a 0-based index relative to the base of its case
#[inline]
pub(super) fn char_to_index(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        c as u8 - b'a'
    } else {
        c as u8 - b'A'
    }
}

/// Convert a 0-based index back to a character, preserving the case of the
/// reference character.
#[inline]
pub(super) fn index_to_char(idx: u8, lowercase: bool) -> char {
    if lowercase {
        char::from(b'a' + idx)
    } else {
        char::from(b'A' + idx)
    }
}

// If you want to extract other tags: just add their "key" here!
pub(super) const SUBTAG_KINDS: &[&str] = &["language", "region", "script", "grandfathered"];

/// Creates an empty subtag map pre-populated with all [`SUBTAG_KINDS`] as keys.
#[inline]
pub(super) fn make_subtags_map() -> HashMap<&'static str, Vec<(String, bool)>> {
    SUBTAG_KINDS.iter().map(|&k| (k, Vec::new())).collect()
}

/// Parses registry line-by-line and populates `map` with the subtags found.
///
/// The registry uses `%%` as a block separator. For each block the fields
/// `Type:`, `Subtag:`, and `Description: Private use` are extracted.
pub(super) fn parse_registry(registry: &str, map: &mut HashMap<&str, Vec<(String, bool)>>) {
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
pub(super) fn expand_subtag_range(start: &str, end: &str) -> Vec<String> {
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
