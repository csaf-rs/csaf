use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait, VersionNumber};
use crate::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use crate::version_helpers::{is_intver_is_zero, is_semver_has_prerelease, is_semver_is_major_zero};

/// 6.1.17 Document Status Draft
///
/// For `/document/version` to be 0, 0.y.z or contain a pre-release part,`/document/status` must be "draft".
/// This checks the inverse: If the document status is not "draft", the version must not be 0, 0.y.z or contain a pre-release part.
pub fn test_6_1_17_document_status_draft(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let status = doc.get_document().get_tracking().get_status();

    // Test does not apply if document status is "draft"
    if DocumentStatus::Draft == status {
        return Ok(());
    }

    // Check if the version is 0, 0.y.z or contains a pre-release part
    let mut error_message: Option<String> = None;
    let version = doc.get_document().get_tracking().get_version();
    if is_intver_is_zero(&version) {
        error_message = Some(format!(
            "The document version is '{}' but the document status is '{}'",
            version, status
        ));
    }
    if is_semver_is_major_zero(&version) || is_semver_has_prerelease(&version) {
        error_message = Some(format!(
            "The document version is '{}' but the document status is '{}'",
            version, status
        ));
    }

    if let Some(error_message) = error_message {
        return Err(vec![ValidationError {
            message: error_message,
            instance_path: "/document/tracking/version".to_string(),
        }]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_17::test_6_1_17_document_status_draft;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_14() {
        let errors = HashMap::from([(
            "01",
            vec![ValidationError {
                message: "The document version is '0.9.5' but the document status is 'final'".to_string(),
                instance_path: "/document/tracking/version".to_string(),
            }],
        )]);
        run_csaf20_tests("17", test_6_1_17_document_status_draft, errors.clone());
        run_csaf21_tests("17", test_6_1_17_document_status_draft, errors);
    }
}
