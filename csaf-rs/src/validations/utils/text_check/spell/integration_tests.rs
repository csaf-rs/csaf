//! Tests that run the full 6.3.8 spell-check validation while forcing a single, specific
//! [`TextChecker`] implementation.

use crate::csaf_traits::CsafTrait;
use crate::csaf2_0::testcases::Test6_3_8 as Test6_3_8_2_0;
use crate::csaf2_1::testcases::Test6_3_8 as Test6_3_8_2_1;
use crate::validation::TestFinding;
use crate::validations::test_6_3_08::test_6_3_8_spell_check_impl;
use crate::validations::test_6_3_08::{EXPECTED_RESULTS_2_0, EXPECTED_RESULTS_2_1, create_misspelling_finding_info};
use crate::validations::utils::text_check::TextChecker;
use crate::validations::utils::text_check::spell::SpellTextChecker;
use crate::validations::utils::text_check::spell::mock_spell::MockSpellChecker;
use crate::validations::utils::text_check::spell::symspell_spell::EnglishSymspellChecker;
use crate::validations::utils::text_check::test_utils::{ExpectedResultExt, NoSuggestionChecker};
use rstest::rstest;

/// Test-only entry point that forces the given checker instead of `select_checker`'s matching.
fn test_6_3_8_spell_check_with_checker(
    doc: &impl CsafTrait,
    checker: impl TextChecker + 'static,
) -> Result<(), Vec<TestFinding>> {
    test_6_3_8_spell_check_impl(doc, |_| Ok(Box::new(checker) as Box<dyn TextChecker>))
}

/// Wraps a [`SpellTextChecker`] so it can act as its own `TestValidator`, forcing
/// `check_text` to use exactly that checker. This allows us to run the CSAF 2.0 / 2.1 test suites
/// separately against each of the spell checkers. Suggested replacements are discarded (see
/// `NoSuggestionChecker`) since they can vary between checker implementations.
#[derive(Debug, Clone, Copy, Default)]
struct SpellCheckerValidator<C>(C);

impl<C, Doc> crate::test_validation::TestValidator<Doc> for SpellCheckerValidator<C>
where
    C: SpellTextChecker + Default + Copy + 'static,
    Doc: CsafTrait,
{
    fn validate(&self, doc: &Doc) -> Result<(), Vec<TestFinding>> {
        test_6_3_8_spell_check_with_checker(doc, NoSuggestionChecker(self.0))
    }
}

fn expected_results_2_0_symspell() -> crate::csaf2_0::testcases::ExpectedResults_6_3_8 {
    let base = EXPECTED_RESULTS_2_0.clone();
    crate::csaf2_0::testcases::ExpectedResults_6_3_8 {
        case_02: base
            .case_02
            .clone()
            .with_finding(create_misspelling_finding_info(
                "unauthenticated",
                66,
                81,
                &None,
                "/vulnerabilities/0/notes/0/text",
            ))
            .without_finding("rood"),
        case_12: base.case_12.clone().with_finding(create_misspelling_finding_info(
            "unauthenticated",
            66,
            81,
            &None,
            "/vulnerabilities/0/notes/0/text",
        )),
        ..base
    }
}

fn expected_results_2_1_symspell() -> crate::csaf2_1::testcases::ExpectedResults_6_3_8 {
    let base = EXPECTED_RESULTS_2_1.clone();
    crate::csaf2_1::testcases::ExpectedResults_6_3_8 {
        case_02: base.case_02.clone().with_finding(create_misspelling_finding_info(
            "unauthenticated",
            66,
            81,
            &None,
            "/vulnerabilities/0/notes/0/text",
        )),
        case_12: base.case_12.clone().with_finding(create_misspelling_finding_info(
            "unauthenticated",
            66,
            81,
            &None,
            "/vulnerabilities/0/notes/0/text",
        )),
        ..base
    }
}

/// Runs the full 6.3.8 test suite against a single, specific checker.
///
/// `MockSpellChecker` is also already used during the "regular" (non-integration) test run;
/// keeping it in here too for test development / test isolation.
///
/// `EnglishSymspellChecker` is a real, basic spell checker. Its findings differ slightly from
/// the expected results. See `expected_results_2_0_symspell` / `expected_results_2_1_symspell`
/// for the diff: "rood" is an old-english word, "unauthenticated" is not in the dictionary.
#[rstest]
#[case::mock_checker(MockSpellChecker, EXPECTED_RESULTS_2_0.clone(), EXPECTED_RESULTS_2_1.clone())]
#[case::symspell_checker(
    EnglishSymspellChecker,
    expected_results_2_0_symspell(),
    expected_results_2_1_symspell()
)]
fn test_test_6_3_8_checker_only<C: SpellTextChecker + Default + Copy + 'static>(
    #[case] _checker: C,
    #[case] expected_2_0: crate::csaf2_0::testcases::ExpectedResults_6_3_8,
    #[case] expected_2_1: crate::csaf2_1::testcases::ExpectedResults_6_3_8,
) {
    Test6_3_8_2_0::<SpellCheckerValidator<C>>::new().expect(expected_2_0);
    Test6_3_8_2_1::<SpellCheckerValidator<C>>::new().expect(expected_2_1);
}
