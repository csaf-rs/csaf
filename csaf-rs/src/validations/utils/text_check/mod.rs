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
mod symspell_spell;
mod utils;

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
    if lang.is_english() && mock_spell::MockSpellChecker.get_available_check_kinds().contains(&kind) {
        return mock_spell::MockSpellChecker.check_text(kind, text);
    }

    if lang.is_english() {
        let checker = symspell_spell::EnglishSymspellChecker;
        if checker.get_available_check_kinds().contains(&kind) {
            return checker.check_text(kind, text);
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

/// Tests that verify the behavior shared by every [`TextChecker`] implementation
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn detects_misspelling(#[case] checker: Box<dyn TextChecker>) {
        let text = "Secruity researchers";
        let findings = checker.check_text(TextCheckKind::Spell, text);
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 0);
        assert_eq!(finding.end, 8);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn detects_misspelling_not_at_start(#[case] checker: Box<dyn TextChecker>) {
        let text = "A Secruity test";
        let findings = checker.check_text(TextCheckKind::Spell, text);
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 2);
        assert_eq!(finding.end, 10);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    /// Validates that start/end are character indices, not byte offsets.
    /// 'é' is a two-byte UTF-8 character; if bytes were used the start would be 3
    /// instead of the correct character index 2.
    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn detects_misspelling_after_multibyte_char(#[case] checker: Box<dyn TextChecker>) {
        let text = "é Secruity";
        let findings = checker.check_text(TextCheckKind::Spell, text);
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 2);
        assert_eq!(finding.end, 10);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn does_not_flag_correct_spelling(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "Security researchers");
        assert!(findings.is_empty(), "expected no spell findings, got: {findings:?}");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn handles_known_acronyms(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "OASIS CSAF TC");
        assert!(
            findings.is_empty(),
            "expected acronyms to be ignored, got: {findings:?}"
        );
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn empty_text_produces_no_findings(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "");
        assert!(findings.is_empty());
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn spell_check_ignores_grammar_issues(#[case] checker: Box<dyn TextChecker>) {
        // "He are going" is a grammar issue, not a spelling issue.
        let findings = checker.check_text(TextCheckKind::Spell, "He are going");
        assert!(
            findings.is_empty(),
            "spell check should not flag grammar issues, got: {findings:?}"
        );
    }
}
