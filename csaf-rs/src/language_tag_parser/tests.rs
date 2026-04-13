use super::parser::{char_to_index, expand_subtag_range, index_to_char, make_subtags_map, parse_registry};
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
