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

mod grammar;
mod spell;
#[cfg(test)]
mod test_utils;
mod utils;

use crate::validation::TestFindingData;
use crate::validations::utils::text_check::TextCheckerMatchingError::{NoCheckerAvailable, UnsupportedLanguage};
use crate::validations::utils::text_check::grammar::all_grammar_checkers;
#[cfg(test)]
use crate::validations::utils::text_check::grammar::mock_grammar::MockGrammarChecker;
use crate::validations::utils::text_check::spell::all_spell_checkers;
#[cfg(test)]
use crate::validations::utils::text_check::spell::mock_spell::MockSpellChecker;
use crate::validations::utils::text_check::utils::TemporaryTextCheckQuality;

pub trait TextChecker {
    /// Temporary measure of quality, will be replaced later.
    fn get_quality(&self) -> TemporaryTextCheckQuality;

    /// Get the lowercases primary language tags
    fn get_available_languages(&self) -> Vec<&str>;

    /// Checks a single text snippet for issues of the given [`TextCheckKind`].
    ///
    /// Returns a (possibly empty) vector of findings. Each finding corresponds to a
    /// single lint that matches the requested check kind.
    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding>;
}

/// The kind of text check to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCheckKind {
    /// Spell checking only.
    Spell,
    /// Grammar checking only.
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

pub enum TextCheckerMatchingError {
    UnsupportedLanguage(String),
    NoCheckerAvailable(TextCheckKind),
}

impl From<TextCheckerMatchingError> for TestFindingData {
    fn from(err: TextCheckerMatchingError) -> Self {
        match err {
            TextCheckerMatchingError::UnsupportedLanguage(lang) => TestFindingData {
                message: format!("No text checker available for valid language '{lang}'"),
                instance_path: "".to_string(),
            },
            TextCheckerMatchingError::NoCheckerAvailable(kind) => {
                let message = match kind {
                    TextCheckKind::Spell => "There are no spell checkers available on your setup".to_string(),
                    TextCheckKind::Grammar => "There are no grammar checkers available on your setup".to_string(),
                };
                TestFindingData {
                    message,
                    instance_path: "".to_string(),
                }
            },
        }
    }
}

/// Selects the single best-quality [`TextChecker`] able to handle the given [`TextCheckKind`]
/// and [`ValidCsafLanguage`].
///
/// Matching only depends on `kind`/`lang`, not on any particular text.
pub fn select_checker(
    kind: TextCheckKind,
    lang: &ValidCsafLanguage,
) -> Result<Box<dyn TextChecker>, TextCheckerMatchingError> {
    // Unit tests get the mock checkers
    #[cfg(test)]
    if kind == TextCheckKind::Spell {
        return Ok(Box::new(MockSpellChecker));
    }
    #[cfg(test)]
    if kind == TextCheckKind::Grammar {
        return Ok(Box::new(MockGrammarChecker));
    }

    // Prod code gets matching
    let checkers = filter_checkers(kind, lang)?;

    // pick the best quality among the available checkers
    let best_quality = checkers
        .iter()
        .map(|checker| checker.get_quality())
        // temporary measure, good = 0, poor = 2, min means take the best available
        .min()
        .expect("filter_checkers should have returned at least one checker (or an error that none were found)");
    // pick any of that quality
    let checker = checkers
        .into_iter()
        .find(|checker| checker.get_quality() == best_quality)
        .expect("a checker with this quality should exist");

    Ok(checker)
}

/// TODO: Add unit tests once grammar checkers are implemented
pub(crate) fn filter_checkers(
    kind: TextCheckKind,
    lang: &ValidCsafLanguage,
) -> Result<Vec<Box<dyn TextChecker>>, TextCheckerMatchingError> {
    let all_checkers = match kind {
        TextCheckKind::Spell => all_spell_checkers(),
        TextCheckKind::Grammar => all_grammar_checkers(),
    };

    if all_checkers.is_empty() {
        return Err(NoCheckerAvailable(kind));
    }

    let matches = all_checkers
        .into_iter()
        .filter(|checker| {
            checker
                .get_available_languages()
                .iter()
                .any(|avail_lang| avail_lang.eq_ignore_ascii_case(lang.primary_language()))
        })
        .collect::<Vec<_>>();

    if matches.is_empty() {
        return Err(UnsupportedLanguage(lang.primary_language().to_string()));
    }

    Ok(matches)
}
