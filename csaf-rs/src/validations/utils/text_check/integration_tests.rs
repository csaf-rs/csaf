//! Tests that run the full 6.3.8 spell-check validation while forcing a single, specific
//! [`TextChecker`] implementation instead of letting [`check_text`] select one.

#[cfg(test)]
mod tests {
    use crate::csaf_traits::CsafTrait;
    use crate::csaf2_0::testcases::Test6_3_8 as Test6_3_8_2_0;
    use crate::csaf2_1::testcases::Test6_3_8 as Test6_3_8_2_1;
    use crate::validation::TestFinding;
    use crate::validations::test_6_3_08::{EXPECTED_RESULTS_2_0, EXPECTED_RESULTS_2_1, test_6_3_8_spell_check_impl};
    use crate::validations::utils::text_check::checkers::TemporaryTextCheckQuality;
    use crate::validations::utils::text_check::checkers::mock_spell::MockSpellChecker;
    use crate::validations::utils::text_check::checkers::symspell_spell::EnglishSymspellChecker;
    use crate::validations::utils::text_check::{TextCheckFinding, TextCheckKind, TextChecker};
    use rstest::rstest;

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

    /// Runs the full 6.3.8 test suite against a single, specific checker.
    #[rstest]
    // MockSpellChecker is also already used during the "regular" test run, keeping it in here
    // for the time of test development / test isolation.
    #[case::mock_checker(MockSpellChecker)]
    #[case::symspell_checker(EnglishSymspellChecker)]
    fn test_test_6_3_8_checker_only<C: TextChecker + Default + Copy + 'static>(#[case] _checker: C) {
        Test6_3_8_2_0::<C>::new().expect(EXPECTED_RESULTS_2_0.clone());
        Test6_3_8_2_1::<C>::new().expect(EXPECTED_RESULTS_2_1.clone());
    }
}
