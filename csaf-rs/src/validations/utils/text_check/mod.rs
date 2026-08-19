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
//! TODO we currently only have harper-core, if more gets integrated, we'll have to implement some matching system
//! based on if spellchecking / grammar checking is requested on what setup this is run on.

mod harper;

use crate::csaf::types::language::ValidCsafLanguage;

/// The kind of text check to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCheckKind {
    /// Spell checking only.
    Spell,
    /// Grammar checking only (TODO not yet implemented)
    #[allow(dead_code)]
    Grammar,
}

/// A single finding produced by a text check.
///
/// `word` is the misspelled / problematic text fragment and `start`/`end` mark its
/// character span within the text that was checked (end-exclusive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCheckFinding {
    /// The problematic text fragment (e.g. a misspelled word).
    pub word: String,
    /// The (character, not byte) index of the first character of `word` within the
    /// checked text.
    pub start: usize,
    /// The (character, not byte) index one past the last character of `word` within
    /// the checked text.
    pub end: usize,
}

/// A backend capable of checking text for spelling / grammar issues.
///
/// Implementations are free to use whatever underlying engine they like (e.g.
/// harper-core as done by [`harper::HarperTextChecker`]). This trait exists so
/// that the engine can be swapped out without changing any callers.
pub trait TextChecker {
    /// Checks a single text snippet for issues of the given [`TextCheckKind`].
    ///
    /// Returns a (possibly empty) vector of findings. Each finding corresponds to a
    /// single lint that matches the requested check kind.
    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding>;
}

/// Checks a single text snippet for issues of the given [`TextCheckKind`] for the given [`ValidCsafLanguage`].
/// TODO: Provide some matching on which spellchecking / grammarchecking to use for which language
pub fn check_text(kind: TextCheckKind, text: &str, lang: &ValidCsafLanguage) -> Vec<TextCheckFinding> {
    if lang.is_english() {
        return harper::HarperTextChecker.check_text(kind, text);
    }
    vec![]
}

/// TODO: ensure these tests run for all implementers
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_misspelling() {
        let text = "Secruity researchers";
        let findings = check_text(TextCheckKind::Spell, text, &ValidCsafLanguage::new_for_tests("en-US"));
        let finding = findings.iter().find(|f| f.word.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(&text[finding.start..finding.end], "Secruity");
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

    #[test]
    fn non_english_language_produces_no_findings() {
        // Not yet supported: harper-core only checks English text.
        let findings = check_text(
            TextCheckKind::Spell,
            "Secruity researchers",
            &ValidCsafLanguage::new_for_tests("de-DE"),
        );
        assert!(
            findings.is_empty(),
            "expected no findings for non-English text, got: {findings:?}"
        );
    }
}
