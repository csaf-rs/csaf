//! Mock spell-checker implementation.
//!
//! This module provides a basic English spell-checker backed by a static word list.
//! It is intended as a test-setup only placeholder until a fully-featured library (e.g., harper-core)
//! is integrated.

use super::{TextCheckFinding, TextCheckKind, TextChecker};
use std::collections::HashSet;

/// A mock spell-checker for English text.
///
/// Behaviour:
/// - Only [`TextCheckKind::Spell`] findings are produced; grammar checking is not implemented.
/// - Strings are tokenized by whitespace, punctuation is stripped.
/// - Tokens that are entirely uppercase are treated as acronyms and skipped.
/// - Tokens that appear (case-insensitively) in the built-in minimal word list are considered correctly spelled.
/// - All other tokens are reported as misspellings without providing a replacement.
pub struct MockSpellChecker;

impl TextChecker for MockSpellChecker {
    fn get_available_check_kinds(&self) -> Vec<TextCheckKind> {
        vec![TextCheckKind::Spell]
    }

    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding> {
        if kind != TextCheckKind::Spell {
            return vec![];
        }
        spell_check(text)
    }
}

fn spell_check(text: &str) -> Vec<TextCheckFinding> {
    let dict = dictionary();
    let mut findings = Vec::new();
    let mut search_from = 0;

    for token in text.split_whitespace() {
        // Locate the token's byte offset in the remaining text.
        let offset = text[search_from..].find(token).unwrap_or(0);
        let token_start = search_from + offset;
        search_from = token_start + token.len();

        // Strip leading/trailing non-alphabetic characters to get the bare word.
        let trimmed = token.trim_matches(|c: char| !c.is_alphabetic());
        if trimmed.is_empty() {
            continue;
        }
        // Byte offset of trimmed within the original token (and thus within the text).
        let word_offset = trimmed.as_ptr() as usize - token.as_ptr() as usize;
        let word_start = token_start + word_offset;
        let word_end = word_start + trimmed.len();

        // All-uppercase tokens are treated as acronyms and are not spell-checked.
        if trimmed.chars().all(|c| c.is_uppercase()) {
            continue;
        }

        if !dict.contains(trimmed.to_lowercase().as_str()) {
            findings.push(TextCheckFinding {
                word: trimmed.to_string(),
                start: word_start,
                end: word_end,
                replacement: None,
            });
        }
    }

    findings
}

fn dictionary() -> HashSet<&'static str> {
    [
        "are",
        "going",
        "he",
        "researchers",
        "security",
        "check",
        "example",
        "failing",
        "informative",
        "spell",
        "test",
        "valid",
        "found",
        "in",
        "multiple",
        "vulnerabilities",
        "initial",
        "version",
        "company",
        "product",
        "allows",
        "an",
        "arbitrary",
        "attacker",
        "code",
        "component",
        "execute",
        "exists",
        "privileges",
        "remote",
        "root",
        "that",
        "to",
        "unauthenticated",
        "undisclosed",
        "vulnerability",
        "with",
        "summary",
    ]
    .iter()
    .copied()
    .collect()
}
