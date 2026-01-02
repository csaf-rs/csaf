use csaf_macros::profile_test_applies_to_category;
use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentReferenceTrait};
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::ValidationError;

fn create_missing_external_reference_error(doc_category: &DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have at least one reference with category 'external'",
            doc_category
        ),
        instance_path: "/document/references".to_string(),
    }
}

/// 6.1.27.2 Document References
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory`
/// or `csaf_security_incident_response`.
///
/// Documents with these categories must have at least one entry in `/document/notes` with `category` values
/// of `description`, `details`, `general` or `summary`.
#[profile_test_applies_to_category(
    all = [CsafInformationalAdvisory, CsafSecurityIncidentResponse],
)]
pub fn test_6_1_27_02_document_references(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // check if there is a document reference with category 'external'
    let mut found_external_reference = false;
    if let Some(references) = doc.get_document().get_references() {
        for reference in references.iter() {
            if CategoryOfReference::External == *reference.get_category() {
                found_external_reference = true;
                break;
            };
        }
    }

    // if there isn't a reference with category 'external', return an error
    if !found_external_reference {
        return Err(vec![create_missing_external_reference_error(&doc.get_document().get_category())]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_02() {
        let errors = HashMap::from([(
            "01",
            vec![create_missing_external_reference_error(
                &DocumentCategory::CsafInformationalAdvisory,
            )],
        )]);
        run_csaf20_tests("27-02", test_6_1_27_02_document_references, errors.clone());
        run_csaf21_tests("27-02", test_6_1_27_02_document_references, errors);
    }
}
