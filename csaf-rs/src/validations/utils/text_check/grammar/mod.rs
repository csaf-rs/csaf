use crate::validations::utils::text_check::TextChecker;

#[cfg(test)]
mod integration_tests;
#[cfg(test)]
pub(crate) mod mock_grammar;
#[cfg(test)]
mod unit_tests;

/// Marker trait for [`TextChecker`] implementations that perform grammar checking.
/// For now, only relevant to integration tests.
#[allow(unused)]
trait GrammarTextChecker: TextChecker {}

pub(crate) fn all_grammar_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![
        #[cfg(test)]
        Box::new(mock_grammar::MockGrammarChecker),
    ]
}
