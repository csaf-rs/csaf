use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::schema::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;

/// Creates a ValidationError for missing source_lang when publisher category is "translator".
fn create_missing_source_lang_error() -> ValidationError {
    ValidationError {
        message: "source_lang is required when the publisher category is 'translator'".to_string(),
        instance_path: "/document/source_lang".to_string(),
    }
}

/// 6.1.15 Translator
///
/// If the `/document/publisher/category` is "translator", then the `/document/source_lang` must be present.
pub fn test_6_1_15_translator(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    // This test only applies if the publisher category is "translator"
    if CategoryOfPublisher::Translator != document.get_publisher().get_category() {
        return Ok(());
    }

    // Check if source_lang is present
    if document.get_source_lang().is_none() {
        return Err(vec![create_missing_source_lang_error()]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_15() {
        let error = create_missing_source_lang_error();
        let errors = HashMap::from([("01", vec![error.clone()]), ("02", vec![error.clone()])]);
        run_csaf20_tests("15", test_6_1_15_translator, errors.clone());
        run_csaf21_tests("15", test_6_1_15_translator, errors);
    }
}
