use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use crate::version_helpers::is_semver_has_prerelease;

/// 6.1.20 Non-draft Document Version
///
/// For documents with status "final" or "interim", the `/document/version` field must not contain
/// a pre-release part (e.g. "1.0.0-alpha").
pub fn test_6_1_20_non_draft_document_version(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();
    let status = tracking.get_status();

    // Check if the document status is not "final" or "interim"
    if !(status == DocumentStatus::Final || status == DocumentStatus::Interim) {
        return Ok(());
    }

    // Check if there is a pre-release part
    let version = tracking.get_version();
    if is_semver_has_prerelease(&version) {
        return Err(vec![ValidationError {
            message: format!(
                "The document status is {} but the document version {} contains a pre-release part",
                status, version
            ),
            instance_path: "/document/version".to_string(),
        }]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_20::test_6_1_20_non_draft_document_version;

    #[test]
    fn test_test_6_1_20() {
        let errors = std::collections::HashMap::from([(
            "01",
            vec![crate::validation::ValidationError {
                message:
                    "The document status is interim but the document version 1.0.0-alpha contains a pre-release part"
                        .to_string(),
                instance_path: "/document/version".to_string(),
            }],
        )]);
        run_csaf20_tests("20", test_6_1_20_non_draft_document_version, errors.clone());
        run_csaf21_tests("20", test_6_1_20_non_draft_document_version, errors);
    }
}
