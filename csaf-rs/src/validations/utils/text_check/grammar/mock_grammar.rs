//! Mock grammar-checker implementation.
//!
//! This module provides a basic English grammar-checker backed by a static list of
//! known incorrect two-word sequences.

use super::{GrammarTextChecker, TextChecker};
use crate::validations::utils::text_check::utils::{tokenize_words, TemporaryTextCheckQuality};
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind};
use std::collections::HashSet;

/// A mock grammar-checker for English text.
///
/// Behavior:
/// - Only [`TextCheckKind::Grammar`] findings are produced; spell checking is not implemented.
/// - Strings are tokenized, see [`tokenize_words`].
/// - Consecutive word pairs are matched against a built-in list of known incorrect sequences.
/// - When a bad sequence is detected, the first word of the pair is reported as the problematic fragment.
#[derive(Default, Clone, Copy)]
pub struct MockGrammarChecker;

impl TextChecker for MockGrammarChecker {
    fn get_quality(&self) -> TemporaryTextCheckQuality {
        TemporaryTextCheckQuality::Poor
    }

    fn get_available_languages(&self) -> Vec<&str> {
        vec!["en"]
    }

    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding> {
        if kind != TextCheckKind::Grammar {
            return vec![];
        }
        grammar_check(text)
    }
}

/// Marker trait for [`TextChecker`] implementations that perform grammar checking.
/// For now, only relevant to integration tests.
#[cfg(test)]
impl GrammarTextChecker for MockGrammarChecker {}

fn grammar_check(text: &str) -> Vec<TextCheckFinding> {
    let known_bad_sequences = known_bad_sequences();
    let mut findings = Vec::new();
    let mut search_from = 0;

    // Collect (lowercase, original, char_start, char_end) for each token
    let mut tokens: Vec<(String, String, usize, usize)> = Vec::new();

    for token in text.split_whitespace() {
        let offset = text[search_from..].find(token).unwrap_or(0);
        let token_start = search_from + offset;
        search_from = token_start + token.len();

        let trimmed = token.trim_matches(|c: char| !c.is_alphabetic());
        if trimmed.is_empty() {
            continue;
        }
        let word_offset = trimmed.as_ptr() as usize - token.as_ptr() as usize;
        let word_start = text[..token_start + word_offset].chars().count();
        let word_end = word_start + trimmed.chars().count();

        tokens.push((trimmed.to_lowercase(), trimmed.to_string(), word_start, word_end));
    }

    // Check each consecutive pair against the bad-sequence list.
    for window in tokens.windows(2) {
        let (lower1, original1, start, _) = &window[0];
        let (lower2, original2, _, end) = &window[1];
        let key = format!("{lower1} {lower2}");
        if known_bad_sequences.contains(key.as_str()) {
            findings.push(TextCheckFinding {
                fragment: format!("{original1} {original2}"),
                start: *start,
                end: *end,
                replacement: None,
            });
        }
    }

    findings
}

fn known_bad_sequences() -> HashSet<&'static str> {
    ["must followed", "for ensure", "a products"].iter().copied().collect()
}