//! English spell-checker backed by the SymSpell algorithm and static word-frequency
//! dictionaries (TODO: only english for now)

use super::{SpellTextChecker, TextChecker};
use crate::validations::utils::text_check::utils::{TemporaryTextCheckQuality, tokenize_words};
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind};
use std::sync::LazyLock;
use symspell::{SymSpell, SymSpellBuilder, UnicodeStringStrategy, Verbosity};

/// Word/frequency dictionary of the 50,000 most common English words, used to seed the
/// SymSpell spell-checker. Each line has the form `<word> <frequency>`.
const EN_50000_DICTIONARY: &str = include_str!("../../../../../assets/wordfreq/en_50000.txt");

/// Maximum edit distance considered when looking up suggestions for a word that is not
/// found verbatim in the dictionary.
const MAX_EDIT_DISTANCE: i64 = 2;

/// Lazy built SymSpell instance(s)
/// TODO: currently only english
static SYMSPELL: LazyLock<SymSpell<UnicodeStringStrategy>> = LazyLock::new(|| {
    let mut symspell: SymSpell<UnicodeStringStrategy> = SymSpellBuilder::default()
        .max_dictionary_edit_distance(MAX_EDIT_DISTANCE)
        .build()
        .expect("SymSpell builder configuration should be valid");

    for line in EN_50000_DICTIONARY.lines() {
        symspell.load_dictionary_line(line, 0, 1, " ");
    }

    symspell
});

/// A SymSpell-backed spell-checker for English text, seeded with the 50,000 most common
/// English words and their usage frequencies. (TODO: Only english for now)
///
/// Behavior:
/// - Only [`TextCheckKind::Spell`] findings are produced; grammar checking is not implemented.
/// - Strings are tokenized, see [`tokenize_words`].
/// - Words found verbatim in the dictionary are considered correctly spelled.
/// - Words not found are looked up via SymSpell: if a close match is found within
///   [`MAX_EDIT_DISTANCE`] edits it is offered as `replacement`. Otherwise the word is
///   reported as misspelled without a suggested fix.
///   TODO: symspell also supports merging compound words with errornous spaces in between
///   (exp. "foot ball" -> "football". This is currently ignored due to our naive tokenization.
#[derive(Default, Clone, Copy)]
pub struct EnglishSymspellChecker;

impl TextChecker for EnglishSymspellChecker {
    fn get_quality(&self) -> TemporaryTextCheckQuality {
        TemporaryTextCheckQuality::Poor
    }

    fn get_available_languages(&self) -> Vec<&str> {
        vec!["en"]
    }

    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding> {
        if kind != TextCheckKind::Spell {
            return vec![];
        }

        let symspell = &*SYMSPELL;
        let mut findings = Vec::new();

        for (word, start, end) in tokenize_words(text) {
            // TODO: Until custom dictionaries are implemented, all-uppercase chars are treated
            // as "known" acronyms.
            if word.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
                continue;
            }

            let lower = word.to_lowercase();
            // Runs the symspell spellchecker on a single word, using the dictionary and a MAX_DISTANCE
            // specifying the amount of edits we still consider a "correction".
            let suggestion = symspell
                .lookup(&lower, Verbosity::Top, MAX_EDIT_DISTANCE)
                .into_iter()
                .next();

            match suggestion {
                // word is in dictionary
                Some(s) if s.distance == 0 => continue,
                // similar word is in dictionary, provide report with correction
                Some(s) => findings.push(TextCheckFinding {
                    fragment: word,
                    start,
                    end,
                    replacement: Some(s.term),
                }),
                // no similar word in dictionary, report without correction
                None => findings.push(TextCheckFinding {
                    fragment: word,
                    start,
                    end,
                    replacement: None,
                }),
            }
        }

        findings
    }
}

/// Marker trait implementation: this checker performs spell checking.
/// For now, only relevant to integration tests.
impl SpellTextChecker for EnglishSymspellChecker {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validations::utils::text_check::TextCheckKind;

    #[test]
    fn does_not_flag_dictionary_words() {
        let findings = EnglishSymspellChecker.check_text(TextCheckKind::Spell, "security research");
        assert!(findings.is_empty(), "expected no findings, got: {findings:?}");
    }

    #[test]
    fn flags_misspelled_word_with_suggestion() {
        let findings = EnglishSymspellChecker.check_text(TextCheckKind::Spell, "Secruity researchers");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fragment, "Secruity");
        assert_eq!(findings[0].start, 0);
        assert_eq!(findings[0].end, 8);
        assert_eq!(findings[0].replacement.as_deref(), Some("security"));
    }

    #[test]
    fn flags_unknown_word_without_close_match() {
        let findings = EnglishSymspellChecker.check_text(TextCheckKind::Spell, "xyzabc");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fragment, "xyzabc");
        assert_eq!(findings[0].start, 0);
        assert_eq!(findings[0].end, 6);
        assert!(findings[0].replacement.is_none());
    }

    #[test]
    fn non_spell_kind_produces_no_findings() {
        let findings = EnglishSymspellChecker.check_text(TextCheckKind::Grammar, "Secruity");
        assert!(findings.is_empty());
    }
}
