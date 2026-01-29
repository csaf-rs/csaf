use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use crate::csaf::types::version_number::{CsafVersionNumber, VersionNumber};
use std::fmt::{Display, Formatter};

pub enum DocumentStatusDraftErrorReason {
    IntVerZero,
    SemVerMajorZero,
    SemVerHasPre,
}

impl Display for DocumentStatusDraftErrorReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            DocumentStatusDraftErrorReason::IntVerZero => write!(f, "Version 0 is"),
            DocumentStatusDraftErrorReason::SemVerMajorZero => write!(f, "Versions 0.y.z are"),
            DocumentStatusDraftErrorReason::SemVerHasPre => write!(f, "Version with prerelease are"),
        }
    }
}

fn generate_status_version_error(
    version: &VersionNumber,
    status: &DocumentStatus,
    reason: &DocumentStatusDraftErrorReason,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version is '{version}' but the document status is '{status}'. {reason} reserved for document status 'Draft'"
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

/// 6.1.17 Document Status Draft
///
/// For `/document/version` to be 0, 0.y.z or contain a pre-release part,`/document/status` must be "draft".
/// This checks the inverse: If the document status is not "draft", the version must not be 0, 0.y.z or contain a pre-release part.
pub fn test_6_1_17_document_status_draft(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    // Test does not apply if document status is "draft"
    let doc_status = tracking.get_status();
    if DocumentStatus::Draft == doc_status {
        return Ok(());
    }

    // Check if doc version is valid
    let doc_version = match tracking.get_version() {
        CsafVersionNumber::Valid(version_number) => version_number,
        CsafVersionNumber::Invalid(err) => return Err(vec![err.get_validation_error("/document/version")]),
    };

    match &doc_version {
        VersionNumber::IntVer(intver) => {
            if intver.get() == 0 {
                Err(vec![generate_status_version_error(
                    &doc_version,
                    &doc_status,
                    &DocumentStatusDraftErrorReason::IntVerZero,
                )])
            } else {
                Ok(())
            }
        },
        VersionNumber::SemVer(semver) => {
            let mut errors: Option<Vec<ValidationError>> = None;
            if semver.get_major() == 0 {
                errors.get_or_insert_default().push(generate_status_version_error(
                    &doc_version,
                    &doc_status,
                    &DocumentStatusDraftErrorReason::SemVerMajorZero,
                ))
            }
            if semver.has_prerelease() {
                errors.get_or_insert_default().push(generate_status_version_error(
                    &doc_version,
                    &doc_status,
                    &DocumentStatusDraftErrorReason::SemVerHasPre,
                ))
            }
            errors.map_or(Ok(()), Err)
        },
    }
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
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_17() {
        // TODO: Add unit test for doc status check
        // TODO: Add unit test for unparseable version
        // TODO: Add unit tests here to check for intver 0, semver pre and semver major 0 + pre
        let case_01 = Err(vec![generate_status_version_error(
            &VersionNumber::from_str("0.9.5").unwrap(),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_17.expect(case_01.clone());
        TESTS_2_1.test_6_1_17.expect(case_01);
    }
}
