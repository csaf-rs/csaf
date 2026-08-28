//! Shared helpers and structs for spell-checking and grammar-checking.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemporaryTextCheckQuality {
    #[allow(unused)]
    Good,
    #[allow(unused)]
    Medium,
    Poor,
}

/// Splits `text` into alphanumeric word tokens suitable for spell-checking.
///
/// Returns `(word, start, end)` triples where `start`/`end` are character (not byte!)
/// offsets of `word` within `text`.
/// Each whitespace-separated token has its leading/trailing non-alphanumeric characters stripped.
/// After stripping, empty tokens and tokens with only numeric+punctuation characters are removed.
pub(crate) fn tokenize_words(text: &str) -> Vec<(String, usize, usize)> {
    let mut tokens = Vec::new();
    let mut search_from = 0;

    for token in text.split_whitespace() {
        // Locate the token's byte offset in the remaining text
        let offset = text[search_from..]
            .find(token)
            .expect("token should be found in remaining text");
        let token_start = search_from + offset;
        search_from = token_start + token.len();

        // Strip leading/trailing non-alphanumeric characters to get the bare word
        let trimmed = token.trim_matches(|c: char| !c.is_alphanumeric());

        // Skip empty and only-numeric+punctuation tokens
        if trimmed.is_empty() || trimmed.chars().all(|c| c.is_numeric() || !c.is_alphabetic()) {
            continue;
        }

        // Byte offset of trimmed within the original token (and so within the text)
        let word_offset = trimmed.as_ptr() as usize - token.as_ptr() as usize;
        // Convert to character counts so start/end are char, not byte, offsets
        let word_start = text[..token_start + word_offset].chars().count();
        let word_end = word_start + trimmed.chars().count();

        tokens.push((trimmed.to_string(), word_start, word_end));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::tokenize_words;
    use rstest::rstest;

    #[rstest]
    #[case::single_word("hello", vec![("hello", 0, 5)])]
    #[case::multiple_words("hello world foo", vec![("hello", 0, 5), ("world", 6, 11), ("foo", 12, 15)])]
    #[case::single_digit("5", vec![])]
    #[case::date("15.01.2024", vec![])]
    #[case::test_id("Test 6.3.8", vec![("Test", 0, 4)])]
    #[case::typo("vu1lnerability", vec![("vu1lnerability", 0, 14)])]
    #[case::acronym("OASIS TC", vec![("OASIS", 0, 5), ("TC", 6, 8)])]
    #[case::cve_id("CVE-2024-1234", vec![("CVE-2024-1234", 0, 13)])]
    fn tokenizes_expected_words(#[case] text: &str, #[case] expected: Vec<(&str, usize, usize)>) {
        let tokens = tokenize_words(text);
        let actual: Vec<(&str, usize, usize)> = tokens
            .iter()
            .map(|(word, start, end)| (word.as_str(), *start, *end))
            .collect();
        assert_eq!(actual, expected);
    }
}
