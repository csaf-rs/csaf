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

    /// Get the [`TextCheckKind`]s supported by this checker
    fn get_available_check_kinds(&self) -> Vec<TextCheckKind>;

    /// Get the [`TextCheckLanguage`]s supported by this checker
    fn get_available_languages(&self) -> Vec<&str>;

    /// Checks a single text snippet for issues of the given [`TextCheckKind`].
    ///
    /// Returns a (possibly empty) vector of findings. Each finding corresponds to a
    /// single lint that matches the requested check kind.
    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding>;
}

/// Returns all known [`TextChecker`] implementations.
pub(crate) fn all_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![
        #[cfg(test)]
        Box::new(mock_spell::MockSpellChecker),
        Box::new(symspell_spell::EnglishSymspellChecker),
    ]
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
    let mut matches: Option<Vec<Box<dyn TextChecker>>> = None;
    let mut no_checker_match = true;
    for checker in all_checkers() {
        if checker.get_available_check_kinds().contains(&kind) {
            no_checker_match = false;
            if checker.get_available_languages().iter().any(|avail_lang| avail_lang.eq_ignore_ascii_case(lang.primary_language())) {
                matches.get_or_insert_default().push(checker);
            }
        }
    }
    if no_checker_match {
        return Err(NoCheckerAvailable(kind));
    }
    if matches.is_none() {
        return Err(UnsupportedLanguage(lang.primary_language().to_string()));
    }
    Ok(matches.unwrap())
}
