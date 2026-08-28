use crate::csaf::types::language::ValidCsafLanguage;
use crate::validations::utils::text_check::checkers::TextCheckerMatchingError::{
    NoCheckerAvailable, UnsupportedLanguage,
};
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind, TextCheckerMatchingError};
use std::cmp::PartialEq;

#[cfg(test)]
pub(crate) mod mock_spell;
pub(crate) mod symspell_spell;
mod utils;

/// A backend capable of checking text for spelling / grammar issues.
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

/// Returns all known [`TextChecker`] implementations.
pub(crate) fn all_spell_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![
        #[cfg(test)]
        Box::new(mock_spell::MockSpellChecker),
        Box::new(symspell_spell::EnglishSymspellChecker),
    ]
}

pub(crate) fn all_grammar_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemporaryTextCheckQuality {
    #[allow(unused)]
    Good,
    #[allow(unused)]
    Medium,
    Poor,
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
