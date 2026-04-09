use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

const LANG_INSTANCE_PATH: &str = "/document/lang";

fn create_missing_document_language_error() -> ValidationError {
    ValidationError {
        message: "The document language is not defined, but is required.".to_string(),
        instance_path: LANG_INSTANCE_PATH.to_string(),
    }
}

fn create_empty_document_language_error() -> ValidationError {
    ValidationError {
        message: "The document language is present with an empty value, but is required.".to_string(),
        instance_path: LANG_INSTANCE_PATH.to_string(),
    }
}

/// 6.2.12 Missing Document Language
///
/// It MUST be tested that the `/document/language` is present and set. 
/// 
/// A CSAF Validator SHALL differentiate in the error message between the key being present 
/// but having no or an empty value and not being present at all.
pub fn test_6_2_12_missing_document_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    match document.get_lang() {
        Some(lang) if lang.to_string().is_empty() => Err(vec![create_empty_document_language_error()]),
        Some(_) => Ok(()),
        None => Err(vec![create_missing_document_language_error()]),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_12, test_6_2_12_missing_document_language);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_12() {
        let err = Err(vec![create_missing_document_language_error()]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_12.expect(err.clone());
        TESTS_2_1.test_6_2_12.expect(err);
    }
}
