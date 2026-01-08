use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

fn create_validation_error(status: &DocumentStatus, version: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "The document status is {} but the document version {} contains a pre-release part",
            status, version
        ),
        instance_path: "/document/version".to_string(),
    }
}

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
    if version.is_semver_has_prerelease() {
        return Err(vec![create_validation_error(&status, &version.to_string())]);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_20
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_20_non_draft_document_version(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_20
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_20_non_draft_document_version(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::DocumentStatus;

    #[test]
    fn test_test_6_1_20() {
        let case_01 = Err(vec![create_validation_error(&DocumentStatus::Interim, "1.0.0-alpha")]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_20.expect(case_01.clone());
        TESTS_2_1.test_6_1_20.expect(case_01);
    }
}
