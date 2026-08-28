/// Tests that verify the behavior shared by every [`TextChecker`] implementation
use crate::validations::utils::text_check::spell::mock_spell;
use crate::validations::utils::text_check::spell::symspell_spell;
use crate::validations::utils::text_check::test_utils::char_slice;
use crate::validations::utils::text_check::{TextCheckKind, TextChecker};
use rstest::rstest;
use rstest_reuse::{self, apply, template};

/// Shared set of [`TextChecker`] implementations that every test in this module runs against.
#[template]
#[rstest]
#[case::mock(Box::new(mock_spell::MockSpellChecker))]
#[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
fn all_checkers(#[case] checker: Box<dyn TextChecker>) {}

#[apply(all_checkers)]
fn detects_misspelling(#[case] checker: Box<dyn TextChecker>) {
    let text = "Secruity researchers";
    let findings = checker.check_text(TextCheckKind::Spell, text);
    let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
    let finding = finding.expect("expected a misspelling finding");
    assert_eq!(finding.start, 0);
    assert_eq!(finding.end, 8);
    assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
}

#[apply(all_checkers)]
fn detects_misspelling_not_at_start(#[case] checker: Box<dyn TextChecker>) {
    let text = "A Secruity test";
    let findings = checker.check_text(TextCheckKind::Spell, text);
    let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
    let finding = finding.expect("expected a misspelling finding");
    assert_eq!(finding.start, 2);
    assert_eq!(finding.end, 10);
    assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
}

/// Validates that start/end are character indices, not byte offsets.
/// 'é' is a two-byte UTF-8 character; if bytes were used the start would be 3
/// instead of the correct character index 2.
#[apply(all_checkers)]
fn detects_misspelling_after_multibyte_char(#[case] checker: Box<dyn TextChecker>) {
    let text = "é Secruity";
    let findings = checker.check_text(TextCheckKind::Spell, text);
    let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
    let finding = finding.expect("expected a misspelling finding");
    assert_eq!(finding.start, 2);
    assert_eq!(finding.end, 10);
    assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
}

#[apply(all_checkers)]
fn does_not_flag_correct_spelling(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Spell, "Security researchers");
    assert!(findings.is_empty(), "expected no spell findings, got: {findings:?}");
}

#[apply(all_checkers)]
fn handles_known_acronyms(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Spell, "OASIS CSAF TC");
    assert!(
        findings.is_empty(),
        "expected acronyms to be ignored, got: {findings:?}"
    );
}

// TODO: Currently custom dictionary is mocked
#[apply(all_checkers)]
fn does_not_flag_custom_dictionary_words(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Spell, "OASIS TC");
    assert!(
        findings.is_empty(),
        "expected custom dictionary words to be ignored, got: {findings:?}"
    );
}

// TODO: Currently CVE-ID like stuff is mocked
#[apply(all_checkers)]
fn does_not_flag_cve_ids(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Spell, "CVE-2024-1234");
    assert!(findings.is_empty(), "expected CVE IDs to be ignored, got: {findings:?}");
}

#[apply(all_checkers)]
fn empty_text_produces_no_findings(#[case] checker: Box<dyn TextChecker>) {
    let findings = checker.check_text(TextCheckKind::Spell, "");
    assert!(findings.is_empty());
}

#[apply(all_checkers)]
fn spell_check_ignores_grammar_issues(#[case] checker: Box<dyn TextChecker>) {
    // "He are going" is a grammar issue, not a spelling issue.
    let findings = checker.check_text(TextCheckKind::Spell, "He are going");
    assert!(
        findings.is_empty(),
        "spell check should not flag grammar issues, got: {findings:?}"
    );
}
