//! Shared test-only helpers for building expected [`crate::validation::TestFinding`] results
//! and for forcing a specific [`TextChecker`] to be used during validation.

use crate::csaf_traits::CsafTrait;
use crate::validation::TestFinding;
use crate::validations::test_6_3_08::test_6_3_8_spell_check_impl;
use crate::validations::utils::text_check::checkers::TemporaryTextCheckQuality;
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind, TextChecker};

/// Extension trait to build checker-specific expected results `EXPECTED_RESULTS_*`.
pub(crate) trait ExpectedResultExt {
    /// Returns `self` with `finding` added to it, turning `Ok` into a `Err`
    fn with_finding(self, finding: TestFinding) -> Self;

    /// Returns `self` with any finding whose message contains `fragment` removed, possibly turning now-empty `Err` into `Ok`
    fn without_finding(self, fragment: &str) -> Self;
}

impl ExpectedResultExt for Result<(), Vec<TestFinding>> {
    fn with_finding(self, finding: TestFinding) -> Self {
        let mut findings = self.err().unwrap_or_default();
        findings.push(finding);
        Err(findings)
    }

    fn without_finding(self, fragment: &str) -> Self {
        match self {
            Ok(()) => Ok(()),
            Err(findings) => {
                let remaining: Vec<TestFinding> = findings
                    .into_iter()
                    .filter(|f| !f.get_data().message.contains(fragment))
                    .collect();
                if remaining.is_empty() { Ok(()) } else { Err(remaining) }
            },
        }
    }
}

/// Wraps a [`TextChecker`], discarding any suggested `replacement` from its findings.
///
/// Different checkers may legitimately suggest different corrections for the same
/// misspelled word (or none at all). Wrapping a checker in this type lets the shared
/// `EXPECTED_RESULTS_*` fixtures (which only specify `replacement: None`) be reused
/// across checkers.
#[derive(Debug, Clone, Copy, Default)]
struct NoSuggestionChecker<C>(C);

impl<C: TextChecker> TextChecker for NoSuggestionChecker<C> {
    fn get_quality(&self) -> TemporaryTextCheckQuality {
        self.0.get_quality()
    }

    fn get_available_check_kinds(&self) -> Vec<TextCheckKind> {
        self.0.get_available_check_kinds()
    }

    fn get_available_languages(&self) -> Vec<&str> {
        self.0.get_available_languages()
    }

    fn check_text(&self, kind: TextCheckKind, text: &str) -> Vec<TextCheckFinding> {
        self.0
            .check_text(kind, text)
            .into_iter()
            .map(|finding| TextCheckFinding {
                replacement: None,
                ..finding
            })
            .collect()
    }
}

/// Test-only entry point that forces the given checker instead of `select_checker`'s matching.
fn test_6_3_8_spell_check_with_checker(
    doc: &impl CsafTrait,
    checker: impl TextChecker + 'static,
) -> Result<(), Vec<TestFinding>> {
    test_6_3_8_spell_check_impl(doc, |_| Ok(Box::new(checker) as Box<dyn TextChecker>))
}

// Any `TextChecker` can act as its own `TestValidator`, forcing
// `check_text` to use exactly that checker. This allows us to run the CSAF 2.0 / 2.1 test suites
// separately against each of the spell checkers. Suggested replacements are discarded (see
// `NoSuggestionChecker`) since they can vary between checker implementations.
impl<C, Doc> crate::test_validation::TestValidator<Doc> for C
where
    C: TextChecker + Default + Copy + 'static,
    Doc: CsafTrait,
{
    fn validate(&self, doc: &Doc) -> Result<(), Vec<TestFinding>> {
        test_6_3_8_spell_check_with_checker(doc, NoSuggestionChecker(*self))
    }
}
