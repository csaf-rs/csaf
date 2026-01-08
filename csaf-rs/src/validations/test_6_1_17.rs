use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

fn generate_status_version_error(version: &str, status: &DocumentStatus) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version is '{}' but the document status is '{}'",
            version, status
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

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
    let version = doc.get_document().get_tracking().get_version();
    if version.is_intver_is_zero() || version.is_semver_is_major_zero() || version.is_semver_has_prerelease() {
        return Err(vec![generate_status_version_error(&version.to_string(), &status)]);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_17
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_17_document_status_draft(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_17
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_17_document_status_draft(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::DocumentStatus;

    #[test]
    fn test_test_6_1_17() {
        let case_01 = Err(vec![generate_status_version_error("0.9.5", &DocumentStatus::Final)]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_17.expect(case_01.clone());
        TESTS_2_1.test_6_1_17.expect(case_01);
    }
}
