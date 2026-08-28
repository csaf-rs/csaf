use crate::validations::utils::text_check::TextChecker;

#[cfg(test)]
mod integration_tests;
#[cfg(test)]
pub(crate) mod mock_spell;
pub(crate) mod symspell_spell;
#[cfg(test)]
mod unit_tests;

pub(crate) fn all_spell_checkers() -> Vec<Box<dyn TextChecker>> {
    vec![
        #[cfg(test)]
        Box::new(mock_spell::MockSpellChecker),
        Box::new(symspell_spell::EnglishSymspellChecker),
    ]
}
