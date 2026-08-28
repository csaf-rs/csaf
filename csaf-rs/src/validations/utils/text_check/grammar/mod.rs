use crate::validations::utils::text_check::TextChecker;

#[cfg(test)]
pub(crate) mod mock_grammar;
#[cfg(test)]
mod unit_tests;
#[cfg(test)]
mod integration_tests;

/// Marker trait for [`TextChecker`] implementations that perform grammar checking.
pub trait GrammarTextChecker: TextChecker {}

pub(crate) fn all_grammar_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![
        #[cfg(test)]
        Box::new(mock_grammar::MockGrammarChecker),
    ]
}
