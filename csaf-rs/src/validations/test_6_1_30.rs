use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use std::mem::discriminant;

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
/// For this test, we take the document version as authoritative for the versioning scheme used in the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_version = doc.get_document().get_tracking().get_version();
    let doc_version_disc = discriminant(&doc_version);

    let mut errors = Vec::new();
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let rev_number = revision.get_number();
        if doc_version_disc != discriminant(&rev_number) {
            errors.push(ValidationError {
                message: format!(
                    "The document version '{}' and revision history number '{}' use different versioning schemes",
                    doc_version, rev_number
                ),
                instance_path: format!("/document/tracking/revision_history/{}/number", i_r),
            });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_30::test_6_1_30_mixed_integer_and_semantic_versioning;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_30() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message:
                    "The document version '2' and revision history number '1.0.0' use different versioning schemes"
                        .to_string(),
                instance_path: "/document/tracking/revision_history/0/number".to_string(),
            }],
        )]);
        run_csaf20_tests("30", test_6_1_30_mixed_integer_and_semantic_versioning, errors.clone());
        run_csaf21_tests("30", test_6_1_30_mixed_integer_and_semantic_versioning, errors);
    }
}
