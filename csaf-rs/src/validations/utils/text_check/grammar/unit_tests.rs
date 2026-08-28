/// Tests that verify the behavior shared by every [`TextChecker`] implementation
use crate::validations::utils::text_check::grammar::mock_grammar;
use crate::validations::utils::text_check::{TextCheckKind, TextChecker};
use rstest::rstest;
use rstest_reuse::{self, apply, template};

/// Shared set of [`TextChecker`] implementations that every test in this module runs against.
#[template]
#[rstest]
#[case::mock(Box::new(mock_grammar::MockGrammarChecker))]
fn all_checkers(#[case] checker: Box<dyn TextChecker>) {}

#[apply(all_checkers)]
fn detects_grammar_mistake(#[case] checker: Box<dyn TextChecker>) {
    let text = "The security hardening guide must followed.";
    let findings = checker.check_text(TextCheckKind::Grammar, text);
    assert_eq!(findings.len(), 1, "expected exactly one findings, got: {findings:?}");
    let finding = findings.first().unwrap();
    assert_eq!(
        &text[finding.start..finding.end],
        "must followed",
        "expected a grammar finding 'must followed'"
    );
}

#[apply(all_checkers)]
fn detects_multiple_grammar_mistake(#[case] checker: Box<dyn TextChecker>) {
    let text = "The security hardening guide must followed for ensure secure operations of a products.";
    let findings = checker.check_text(TextCheckKind::Grammar, text);
    assert_eq!(findings.len(), 3, "expected exactly three findings, got: {findings:?}");
    let finding_1 = findings.get(0).unwrap();
    assert_eq!(
        &text[finding_1.start..finding_1.end],
        "must followed",
        "expected a grammar finding 'must followed'"
    );
    let finding_2 = findings.get(1).unwrap();
    assert_eq!(
        &text[finding_2.start..finding_2.end],
        "for ensure",
        "expected a grammar finding 'for ensure'"
    );
    let finding_3 = findings.get(2).unwrap();
    assert_eq!(
        &text[finding_3.start..finding_3.end],
        "a products",
        "expected a grammar finding for 'a products'"
    );
}

#[apply(all_checkers)]
fn does_not_flag_correct_grammar(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(
        TextCheckKind::Grammar,
        "The security hardening guide must be followed to ensure secure operations of the products.",
    );
    assert!(findings.is_empty(), "expected no grammar findings, got: {findings:?}");
}

#[apply(all_checkers)]
fn grammar_check_ignores_pure_spelling_issues(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Grammar, "Secruity researchers");
    assert!(
        findings.is_empty(),
        "grammar check should not flag pure spelling issues, got: {findings:?}"
    );
}
