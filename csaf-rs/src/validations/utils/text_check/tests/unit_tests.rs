/// Tests that verify the behavior shared by every [`TextChecker`] implementation
#[cfg(test)]
mod tests {
    use crate::validations::utils::text_check::checkers::mock_spell;
    use crate::validations::utils::text_check::checkers::symspell_spell;
    use crate::validations::utils::text_check::{TextCheckKind, TextChecker};
    use rstest::rstest;

    /// Returns the substring of `text` identified by the character-index span `[start, end)`.
    fn char_slice(text: &str, start: usize, end: usize) -> String {
        text.chars().skip(start).take(end - start).collect()
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn detects_misspelling(#[case] checker: Box<dyn TextChecker>) {
        let text = "Secruity researchers";
        let findings = checker.check_text(TextCheckKind::Spell, text);
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 0);
        assert_eq!(finding.end, 8);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
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
    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn detects_misspelling_after_multibyte_char(#[case] checker: Box<dyn TextChecker>) {
        let text = "é Secruity";
        let findings = checker.check_text(TextCheckKind::Spell, text);
        let finding = findings.iter().find(|f| f.fragment.eq_ignore_ascii_case("secruity"));
        let finding = finding.expect("expected a misspelling finding");
        assert_eq!(finding.start, 2);
        assert_eq!(finding.end, 10);
        assert_eq!(char_slice(text, finding.start, finding.end), "Secruity");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn does_not_flag_correct_spelling(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "Security researchers");
        assert!(findings.is_empty(), "expected no spell findings, got: {findings:?}");
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn handles_known_acronyms(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "OASIS CSAF TC");
        assert!(
            findings.is_empty(),
            "expected acronyms to be ignored, got: {findings:?}"
        );
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn empty_text_produces_no_findings(#[case] checker: Box<dyn TextChecker>) {
        let findings = checker.check_text(TextCheckKind::Spell, "");
        assert!(findings.is_empty());
    }

    #[rstest]
    #[case::mock(Box::new(mock_spell::MockSpellChecker))]
    #[case::symspell(Box::new(symspell_spell::EnglishSymspellChecker))]
    fn spell_check_ignores_grammar_issues(#[case] checker: Box<dyn TextChecker>) {
        // "He are going" is a grammar issue, not a spelling issue.
        let findings = checker.check_text(TextCheckKind::Spell, "He are going");
        assert!(
            findings.is_empty(),
            "spell check should not flag grammar issues, got: {findings:?}"
        );
    }
}
