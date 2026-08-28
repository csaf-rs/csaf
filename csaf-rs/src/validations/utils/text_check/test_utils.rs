use crate::validation::TestFinding;
use crate::validations::utils::text_check::utils::TemporaryTextCheckQuality;
use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind, TextChecker};

/// Returns the substring of `text` identified by the character-index span `[start, end)`.
pub(crate) fn char_slice(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

/// Wraps a [`TextChecker`], discarding any suggested `replacement` from its findings.
///
/// Different checkers may legitimately suggest different corrections for the same
/// misspelled word (or none at all). Wrapping a checker in this type lets the shared
/// `EXPECTED_RESULTS_*` fixtures (which only specify `replacement: None`) be reused
/// across checkers.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NoSuggestionChecker<C>(pub C);

impl<C: TextChecker> TextChecker for NoSuggestionChecker<C> {
    fn get_quality(&self) -> TemporaryTextCheckQuality {
        self.0.get_quality()
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
