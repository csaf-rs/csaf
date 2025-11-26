use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::generated::language_subtags::is_valid_language_subtag;
use crate::validation::ValidationError;

pub fn test_6_1_12_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    let mut errors: Option<Vec<ValidationError>> = None;
    // Check /document/lang if it exists
    if let Some(lang) = document.get_lang()
        && let Err(e) = validate_language_code(lang, "/document/lang")
    {
        errors.get_or_insert_default().extend(e);
    }

    // Check /document/source_lang if it exists
    if let Some(source_lang) = document.get_source_lang() 
        && let Err(e) = validate_language_code(source_lang, "/document/source_lang")
    {   
        errors.get_or_insert_default().extend(e);
    }

    errors.map_or(Ok(()), Err)
}

fn validate_language_code(lang_code: &str, json_path: &str) -> Result<(), Vec<ValidationError>> {
    // Extract the primary language subtag (everything before the first hyphen)
    let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);

    // ToDo casing
    if !is_valid_language_subtag(primary_subtag) {
        return Err(vec![ValidationError {
            message: format!(
                "Invalid language code '{}': primary language subtag '{}' is not a valid language subtag",
                //EZ is not a valid language. It is the subtag for the region "Eurozone".
                lang_code, primary_subtag
            ),
            instance_path: json_path.to_string(),
        }]);
    }

    Ok(())
}

fn create_error_message(language: String) -> String {
    format!(
        "Invalid language code '{}': primary language subtag '{}' is not a valid language subtag",
        language,
        language.split('-').next().unwrap_or(&language)
    )
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_12::test_6_1_12_language;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_12() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "Invalid language code 'EZ': primary language subtag 'EZ' is not a valid language subtag"
                    .to_string(),
                instance_path: "/document/lang".to_string(),
            }],
        )]);
        run_csaf20_tests("12", test_6_1_12_language, errors.clone());
        run_csaf21_tests("12", test_6_1_12_language, errors);
    }
}
