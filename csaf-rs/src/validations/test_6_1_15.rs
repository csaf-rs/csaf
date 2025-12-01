use crate::csaf_traits::{CsafTrait, DocumentTrait, PublisherTrait};
use crate::csaf2_1::schema::CategoryOfPublisher;
use crate::validation::ValidationError;

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
        return Err(vec![ValidationError {
            message: "source_lang is required when the publisher category is 'translator'".to_string(),
            instance_path: "/document/source_lang".to_string(),
        }]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_15::test_6_1_15_translator;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_15() {
        let errors = HashMap::from([
            (
                "01",
                vec![ValidationError {
                    message: "source_lang is required when the publisher category is 'translator'".to_string(),
                    instance_path: "/document/source_lang".to_string(),
                }],
            ),
            (
                "02",
                vec![ValidationError {
                    message: "source_lang is required when the publisher category is 'translator'".to_string(),
                    instance_path: "/document/source_lang".to_string(),
                }],
            ),
        ]);
        run_csaf20_tests("15", test_6_1_15_translator, errors.clone());
        run_csaf21_tests("15", test_6_1_15_translator, errors);
    }
}
