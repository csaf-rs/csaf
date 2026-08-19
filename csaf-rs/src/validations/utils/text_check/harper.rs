use super::{TextCheckFinding, TextCheckKind, TextChecker};
use harper_core::linting::{LintGroup, Linter};
use harper_core::spell::FstDictionary;
use harper_core::{Dialect, Document, Lrc};
use std::sync::LazyLock;

/// [`TextChecker`] implementation backed by [`harper-core`].
pub struct HarperTextChecker;

impl TextChecker for HarperTextChecker {
    /// Checks a single text snippet for issues of the given [`TextCheckKind`] with harper-core.
    /// harper-core only supports English.
    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding> {
        if text.trim().is_empty() {
            return Vec::new();
        }

        let mut linter = LintGroup::new_curated(HARPER_DICTIONARY.clone(), Dialect::American);
        let document = Document::new_curated(text, &harper_core::parsers::PlainEnglish);

        let lints = linter.lint(&document);
        let source = document.get_full_content();

        let mut findings = Vec::new();
        for lint in lints {
            if !kind.matches(lint.lint_kind) {
                continue;
            }
            let word = lint.get_str(source);
            /// TODO: This will be replaced with an allow-list in the future. For now, this should ignore
            /// OASIS, CSAF, ...
            if kind == TextCheckKind::Spell && is_probable_acronym(&word) {
                continue;
            }
            findings.push(TextCheckFinding {
                word,
                start: lint.span.start,
                end: lint.span.end,
            });
        }
        findings
    }
}

/// Lazily-initialized, shared curated dictionary for running harper-core checks.
///
/// The curated dictionary is relatively expensive to construct, so it is created
/// once when needed via LazyLock and reused across all checks. It is reference-counted internally (via
/// [`Lrc`]), so cloning it to hand ownership to a fresh [`LintGroup`] per check is
/// cheap.
static HARPER_DICTIONARY: LazyLock<Lrc<FstDictionary>> = LazyLock::new(FstDictionary::curated);

/// Allows filtering harper-core findings by the provided TextCheckKind
impl TextCheckKind {
    /// Returns `true` if the given lint kind should be reported for this check kind.
    fn matches(self, lint_kind: harper_core::linting::LintKind) -> bool {
        match self {
            TextCheckKind::Spell => lint_kind.is_spelling() || lint_kind.is_typo(),
            TextCheckKind::Grammar => lint_kind.is_grammar(),
        }
    }
}

/// Returns `true` if `word` consists of two or more uppercase letters (ignoring any
/// non-alphabetic characters).
///
/// TODO: This will be replaced with an allow-list in the future. For now, this should ignore
/// OASIS, CSAF, ...
fn is_probable_acronym(word: &str) -> bool {
    let letters: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
    letters.len() >= 2 && letters.iter().all(|c| c.is_uppercase())
}