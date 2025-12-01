use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

/// 6.1.28 Translation
///
/// `/document/lang` and `/document/source_lang` must have different values
pub fn test_6_1_28_translation(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    if let Some(lang) = document.get_lang() {
        if let Some(source_lang) = document.get_source_lang() {
            if lang.to_lowercase() == source_lang.to_lowercase() {
                return Err(vec![ValidationError {
                    message: format!("document language and source language have the same value {}", lang),
                    instance_path: "/document/source_lang".to_string(),
                }]);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use std::collections::HashMap;
    use crate::validations::test_6_1_28::test_6_1_28_translation;

    #[test]
    fn test_test_6_1_28() {
        let errors = HashMap::from([
            (
                "01",
                vec![ValidationError {
                    message: "document language and source language have the same value en-US".to_string(),
                    instance_path: "/document/source_lang".to_string(),
                }],
            )
        ]);
        run_csaf20_tests("28", test_6_1_28_translation, errors.clone());
        run_csaf21_tests("28", test_6_1_28_translation, errors);
    }
}
