//! Shared helpers for [`TextChecker`] implementations.

/// Splits `text` into alphanumeric word tokens suitable for spell-checking.
///
/// Returns `(word, start, end)` triples where `start`/`end` are character (not byte!)
/// offsets of `word` within `text`. Each whitespace-separated token has its
/// leading/trailing non-alphabetic characters stripped. Empty tokens are removed.
pub(crate) fn tokenize_words(text: &str) -> Vec<(String, usize, usize)> {
    let mut tokens = Vec::new();
    let mut search_from = 0;

    for token in text.split_whitespace() {
        // Locate the token's byte offset in the remaining text.
        let offset = text[search_from..].find(token).unwrap_or(0);
        let token_start = search_from + offset;
        search_from = token_start + token.len();

        // Strip leading/trailing non-alphanumeric characters to get the bare word.
        let trimmed = token.trim_matches(|c: char| !c.is_alphabetic());
        if trimmed.is_empty() {
            continue;
        }

        // Byte offset of trimmed within the original token (and so within the text).
        let word_offset = trimmed.as_ptr() as usize - token.as_ptr() as usize;
        // Convert to character counts so start/end are char, not byte, offsets.
        let word_start = text[..token_start + word_offset].chars().count();
        let word_end = word_start + trimmed.chars().count();

        tokens.push((trimmed.to_string(), word_start, word_end));
    }

    tokens
}
