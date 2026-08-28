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

    let tokens = tokenize_words(text);

    // Check each consecutive pair against the bad-sequence list.
    for window in tokens.windows(2) {
        let (word1, start, _) = &window[0];
        let (word2, _, end) = &window[1];
        let key = format!("{} {}", word1.to_lowercase(), word2.to_lowercase());
        if known_bad_sequences.contains(key.as_str()) {
            findings.push(TextCheckFinding {
                fragment: format!("{word1} {word2}"),
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