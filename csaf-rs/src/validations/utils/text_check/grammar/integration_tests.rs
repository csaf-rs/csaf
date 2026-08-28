use crate::csaf_traits::CsafTrait;
use crate::validation::TestFinding;
use crate::validations::test_6_3_16::{EXPECTED_RESULTS_2_1, test_6_3_16_grammar_check_impl};
use crate::validations::utils::text_check::TextChecker;
use crate::validations::utils::text_check::grammar::GrammarTextChecker;
use crate::validations::utils::text_check::grammar::mock_grammar::MockGrammarChecker;
use crate::validations::utils::text_check::test_utils::NoSuggestionChecker;
use rstest::rstest;

/// Test-only entry point that forces the given checker instead of `select_checker`'s matching.
fn test_6_3_16_grammar_check_with_checker(
    doc: &impl CsafTrait,
    checker: impl TextChecker + 'static,
) -> Result<(), Vec<TestFinding>> {
    test_6_3_16_grammar_check_impl(doc, |_| Ok(Box::new(checker) as Box<dyn TextChecker>))
}

/// Wraps a [`GrammarTextChecker`] so it can act as its own `TestValidator`, forcing
/// `check_text` to use exactly that checker. This allows us to run the CSAF 2.0 / 2.1 test suites
/// separately against each of the grammar checkers. Suggested replacements are discarded (see
/// `NoSuggestionChecker`) since they can vary between checker implementations.
#[derive(Debug, Clone, Copy, Default)]
struct GrammarCheckerValidator<C>(C);

impl<C, Doc> crate::test_validation::TestValidator<Doc> for GrammarCheckerValidator<C>
where
    C: GrammarTextChecker + Default + Copy + 'static,
    Doc: CsafTrait,
{
    fn validate(&self, doc: &Doc) -> Result<(), Vec<TestFinding>> {
        test_6_3_16_grammar_check_with_checker(doc, NoSuggestionChecker(self.0))
    }
}

/// Runs the full 6.3.16 test suite against a single, specific checker.
///
/// `MockGrammarChecker` is also already used during the "regular" (non-integration) test run;
/// keeping it in here too for test development / test isolation.
#[rstest]
#[case::mock_checker(MockGrammarChecker, EXPECTED_RESULTS_2_1.clone())]
fn test_test_6_3_16_checker_only<C: GrammarTextChecker + Default + Copy + 'static>(
    #[case] _checker: C,
    #[case] expected_2_1: crate::csaf2_1::testcases::ExpectedResults_6_3_16,
) {
    crate::csaf2_1::testcases::Test6_3_16::<GrammarCheckerValidator<C>>::new().expect(expected_2_1);
}
