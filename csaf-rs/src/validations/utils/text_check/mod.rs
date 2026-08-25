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
use crate::validations::utils::text_check::checkers::{filter_checkers};

pub(crate) mod checkers;
mod unit_tests;
mod integration_tests;

pub use checkers::TextChecker;
use crate::validation::TestFindingData;
#[cfg(test)]
use crate::validations::utils::text_check::checkers::mock_spell::MockSpellChecker;

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

pub enum TextCheckerMatchingError {
    #[allow(unused)]
    UnsupportedLanguage(String),
    #[allow(unused)]
    NoCheckerAvailable(TextCheckKind),
}

impl From<TextCheckerMatchingError> for TestFindingData {
    fn from(err: TextCheckerMatchingError) -> Self {
        match err {
            TextCheckerMatchingError::UnsupportedLanguage(lang) => TestFindingData {
                message: format!("No text checker available for language '{lang}'"),
                instance_path: "".to_string(),
            },
            TextCheckerMatchingError::NoCheckerAvailable(kind) => TestFindingData {
                message: format!("No text checker available for check kind '{kind:?}'"),
                instance_path: "".to_string(),
            },
        }
    }
}

/// Selects the single best-quality [`TextChecker`] able to handle the given [`TextCheckKind`]
/// and [`ValidCsafLanguage`].
///
/// Matching only depends on `kind`/`lang`, not on any particular text.
pub fn select_checker(kind: TextCheckKind, lang: &ValidCsafLanguage) -> Result<Box<dyn TextChecker>, TextCheckerMatchingError> {
    // Unit tests get the mock checkers
    #[cfg(test)]
    if kind == TextCheckKind::Spell {
        return Ok(Box::new(MockSpellChecker));
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