//! Text checking utilities.
//!
//! This module provides a reusable interface for running language checks (spell
//! checking and grammar checking) against snippets of text extracted from a CSAF
//! document.
//!
//! The interface is intentionally small: callers provide the text to check and
//! receive back [`TextCheckFinding`]s.
//!
//! The [`TextChecker`] trait abstracts over the concrete language-checking engine.

use crate::csaf::types::language::ValidCsafLanguage;

#[cfg(test)]
mod mock_spell;

/// The kind of text check to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCheckKind {
    #[allow(dead_code)]
    /// Spell checking only.
    Spell,
    /// Grammar checking only (TODO not yet implemented)
    #[allow(dead_code)]
    Grammar,
}

/// A single finding produced by a text check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCheckFinding {
    /// The problematic text fragment (e.g. a misspelled word).
    pub fragment: String,
    /// The (character, not byte) index of the first character of `fragment` within the
    /// checked text.
    pub start: usize,
    /// The (character, not byte) index one past the last character of `fragment` within
    /// the checked text.
    pub end: usize,
    /// The suggested replacement for `fragment`, if any is available.
    pub replacement: Option<String>,
}

/// A backend capable of checking text for spelling / grammar issues.
#[allow(dead_code)]
pub trait TextChecker {
    fn get_available_check_kinds(&self) -> Vec<TextCheckKind>;

    /// Checks a single text snippet for issues of the given [`TextCheckKind`].
    ///
    /// Returns a (possibly empty) vector of findings. Each finding corresponds to a
    /// single lint that matches the requested check kind.
    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding>;
}

/// Checks a single text snippet for issues of the given [`TextCheckKind`] for the given [`ValidCsafLanguage`].
/// TODO: Provide some matching on which spellchecking / grammarchecking to use for which language
#[allow(dead_code)]
pub fn check_text(kind: TextCheckKind, text: &str, lang: &ValidCsafLanguage) -> Vec<TextCheckFinding> {
    #[cfg(test)]
    if lang.is_english() {
        if mock_spell::MockSpellChecker.get_available_check_kinds().contains(&kind) {
            return mock_spell::MockSpellChecker.check_text(kind, text);
        }
    }
    // happy linter
    let _ = lang;
    let _ = kind;
    let _ = text;
    vec![]
}

/// Returns the substring of `text` identified by the character-index span `[start, end)`.
#[cfg(test)]
fn char_slice(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

/// TODO: ensure these tests run for all implementers
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_misspelling() {
        let text = "Secruity researchers";
        let findings = check_text(TextCheckKind::Spell, text, &ValidCsafLanguage::new_for_tests("en-US"));
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 0);
        assert_eq!(finding.end, 8);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[test]
    fn detects_misspelling_not_at_start() {
        let text = "A Secruity test";
        let findings = check_text(TextCheckKind::Spell, text, &ValidCsafLanguage::new_for_tests("en-US"));
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 2);
        assert_eq!(finding.end, 10);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    /// Validates that start/end are character indices, not byte offsets.
    /// 'é' is a two-byte UTF-8 character; if bytes were used the start would be 3
    /// instead of the correct character index 2.
    #[test]
    fn detects_misspelling_after_multibyte_char() {
        let text = "é Secruity";
        let findings = check_text(TextCheckKind::Spell, text, &ValidCsafLanguage::new_for_tests("en-US"));
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 2);
        assert_eq!(finding.end, 10);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[test]
    fn does_not_flag_correct_spelling() {
        let findings = check_text(
            TextCheckKind::Spell,
            "Security researchers",
            &ValidCsafLanguage::new_for_tests("en-US"),
        );
        assert!(findings.is_empty(), "expected no spell findings, got: {findings:?}");
    }

    #[test]
    fn ignores_acronyms() {
        let findings = check_text(
            TextCheckKind::Spell,
            "OASIS CSAF TC",
            &ValidCsafLanguage::new_for_tests("en-US"),
        );
        assert!(
            findings.is_empty(),
            "expected acronyms to be ignored, got: {findings:?}"
        );
    }

    #[test]
    fn empty_text_produces_no_findings() {
        let findings = check_text(TextCheckKind::Spell, "", &ValidCsafLanguage::new_for_tests("en-US"));
        assert!(findings.is_empty());
    }

    #[test]
    fn spell_check_ignores_grammar_issues() {
        // "He are going" is a grammar issue, not a spelling issue.
        let findings = check_text(
            TextCheckKind::Spell,
            "He are going",
            &ValidCsafLanguage::new_for_tests("en-US"),
        );
        assert!(
            findings.is_empty(),
            "spell check should not flag grammar issues, got: {findings:?}"
        );
    }
}
