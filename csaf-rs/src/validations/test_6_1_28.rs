use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

/// Creates a ValidationError for when document language and source language have the same value
fn create_same_language_error(lang: &str) -> ValidationError {
    ValidationError {
        message: format!("document language and source language have the same value {}", lang),
        instance_path: "/document/source_lang".to_string(),
    }
}

/// 6.1.28 Translation
///
/// `/document/lang` and `/document/source_lang` must have different values
pub fn test_6_1_28_translation(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    if let Some(lang) = document.get_lang() {
        if let Some(source_lang) = document.get_source_lang() {
            if lang.to_lowercase() == source_lang.to_lowercase() {
                return Err(vec![create_same_language_error(lang)]);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_28() {
        let errors = HashMap::from([("01", vec![create_same_language_error("en-US")])]);
        run_csaf20_tests("28", test_6_1_28_translation, errors.clone());
        run_csaf21_tests("28", test_6_1_28_translation, errors);
    }
}
