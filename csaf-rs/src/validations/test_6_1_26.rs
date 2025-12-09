use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentCategory, DocumentTrait};
use crate::validation::ValidationError;

/// 6.1.26 Prohibited Document Category Name
pub fn test_6_1_26_prohibited_document_category(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let version = doc.get_document().get_csaf_version();
    let doc_category = doc.get_document().get_category();

    // skip test for known profiles and categories
    if doc_category.is_known_profile(version) {
        return Ok(());
    }

    // throw error, as only known profiles are allowed to start with "csaf_"
    if doc_category.starts_with_csaf_underscore() {
        return Err(vec![test_6_1_27_6_err_generator_starts_with_csaf(
            &doc_category,
            version,
        )]);
    }

    // throw error if document category is too similar to known categories
    // this is done by comparing normalized versions of the categories
    for normalized_known_category in DocumentCategory::known_profiles_normalized(version) {
        println!(
            "Comparing '{}' with known normalized category '{}'",
            doc_category.normalize(),
            normalized_known_category.0
        );
        if doc_category.normalize() == normalized_known_category.0 {
            return Err(vec![test_6_1_27_6_err_generator_too_similar(
                &doc_category,
                &normalized_known_category.1,
            )]);
        }
    }

    Ok(())
}

fn test_6_1_27_6_err_generator_starts_with_csaf(
    doc_category: &DocumentCategory,
    version: &CsafVersion,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document category '{}' is prohibited. Only the following values are allowed to starting with 'csaf_' are allowed: {}",
            doc_category,
            DocumentCategory::known_profile_concat(version)
        ),
        instance_path: "/document/category".to_string(),
    }
}

fn test_6_1_27_6_err_generator_too_similar(
    doc_category: &DocumentCategory,
    known_category: &DocumentCategory,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document category '{}' is prohibited. It is too similar to the known category: {}",
            doc_category,
            known_category
        ),
        instance_path: "/document/category".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf_traits::DocumentCategory;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_26::{
        test_6_1_26_prohibited_document_category, test_6_1_27_6_err_generator_too_similar,
    };
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_26() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_27_6_err_generator_too_similar(
                    &DocumentCategory::from_string("Security_Incident_Response"),
                    &DocumentCategory::CsafSecurityIncidentResponse,
                )],
            ),
            (
                "02",
                vec![test_6_1_27_6_err_generator_too_similar(
                    &DocumentCategory::from_string("Deprecated Security Advisory"),
                    &DocumentCategory::CsafDeprecatedSecurityAdvisory,
                )],
            ),
            (
                "03",
                vec![test_6_1_27_6_err_generator_too_similar(
                    &DocumentCategory::from_string("withdrawn"),
                    &DocumentCategory::CsafWithdrawn,
                )],
            ),
            (
                "04",
                vec![test_6_1_27_6_err_generator_too_similar(
                    &DocumentCategory::from_string("superseded"),
                    &DocumentCategory::CsafSuperseded,
                )],
            ),
        ]);
        run_csaf20_tests("26", test_6_1_26_prohibited_document_category, errors.clone());
        run_csaf21_tests("26", test_6_1_26_prohibited_document_category, errors);
    }
}
