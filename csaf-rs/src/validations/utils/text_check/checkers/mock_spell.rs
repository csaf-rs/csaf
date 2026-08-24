//! Mock spell-checker implementation.
//!
//! This module provides a basic English spell-checker backed by a static word list.

use std::collections::HashSet;
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind};
use crate::validations::utils::text_check::checkers::{TemporaryTextCheckQuality, TextChecker};
use crate::validations::utils::text_check::checkers::utils::tokenize_words;

/// A mock spell-checker for English text.
///
/// Behavior:
/// - Only [`TextCheckKind::Spell`] findings are produced; grammar checking is not implemented.
/// - Strings are tokenized by whitespace, punctuation is stripped.
/// - Tokens that are entirely uppercase are treated as acronyms and skipped.
/// - Tokens that appear (case-insensitively) in the built-in minimal word list are considered correctly spelled.
/// - All other tokens are reported as misspellings without providing a replacement.
pub struct MockSpellChecker;

impl TextChecker for MockSpellChecker {
    fn get_quality(&self) -> TemporaryTextCheckQuality {
        TemporaryTextCheckQuality::Poor
    }

    fn get_available_check_kinds(&self) -> Vec<TextCheckKind> {
        vec![TextCheckKind::Spell]
    }

    fn get_available_languages(&self) -> Vec<&str> {
        vec!["en"]
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

    for (word, start, end) in tokenize_words(text) {
        // TODO: Until custom dictionaries are implemented, all-uppercase chars are treated
        // as "known" acronyms.
        if word.chars().all(|c| c.is_uppercase()) {
            continue;
        }

        if !dict.contains(word.to_lowercase().as_str()) {
            findings.push(TextCheckFinding {
                fragment: word,
                start,
                end,
                replacement: None,
            });
        }
    }

    findings
}

// just the words contained in the tests
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_flag_known_word() {
        let findings = MockSpellChecker.check_text(TextCheckKind::Spell, "security");
        assert!(findings.is_empty());
    }

    #[test]
    fn does_flag_unknown_word() {
        let findings = MockSpellChecker.check_text(TextCheckKind::Spell, "Secruity");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fragment, "Secruity");
    }

    #[test]
    fn non_spell_kind_produces_no_findings() {
        let findings = MockSpellChecker.check_text(TextCheckKind::Grammar, "Secruity");
        assert!(findings.is_empty());
    }
}
