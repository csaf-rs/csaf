use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
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
            DocumentStatusDraftErrorReason::SemVerHasPre => write!(f, "Versions with prerelease are"),
        }
    }
}

fn generate_status_version_error(
    version: &CsafVersionNumber,
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

    // Test does not apply if document status is "draft" (#409)
    let doc_status = tracking.get_status();
    if DocumentStatus::Draft == doc_status {
        return Ok(()); // ToDo return skipped/not applicable (#409)
    }

    let doc_version = tracking.get_version();
    match &doc_version {
        CsafVersionNumber::IntVer(intver) => {
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
        CsafVersionNumber::SemVer(semver) => {
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

    #[test]
    fn test_test_6_1_17() {
        let case_final_with_semver_0 = Err(vec![generate_status_version_error(
            &CsafVersionNumber::from("0.9.5"),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_semver_0_ignored_metadata = Err(vec![generate_status_version_error(
            &CsafVersionNumber::from("0.0.0+exp.sha.ac00785"),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_semver_prerelease = Err(vec![generate_status_version_error(
            &CsafVersionNumber::from("1.0.0-alpha"),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerHasPre,
        )]);

        let case_final_with_semver_0_prerelease = Err(vec![
            generate_status_version_error(
                &CsafVersionNumber::from("0.9.5-alpha"),
                &DocumentStatus::Final,
                &DocumentStatusDraftErrorReason::SemVerMajorZero,
            ),
            generate_status_version_error(
                &CsafVersionNumber::from("0.9.5-alpha"),
                &DocumentStatus::Final,
                &DocumentStatusDraftErrorReason::SemVerHasPre,
            ),
        ]);

        let case_interim_with_semver_0 = Err(vec![generate_status_version_error(
            &CsafVersionNumber::from("0.9.5"),
            &DocumentStatus::Interim,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_intver_0 = Err(vec![generate_status_version_error(
            &CsafVersionNumber::from("0"),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::IntVerZero,
        )]);

        // Case S11: document status is "draft", version is 0 (should be skipped)
        // Case S12: document status is "draft", version is 0.y.z (should be skipped)
        // Case S13: document status is "draft", version has prerelease (should be skipped)
        // Case S14: document status is "final", version has metadata

        TESTS_2_0.test_6_1_17.expect(
            case_final_with_semver_0.clone(),
            case_final_with_semver_0_ignored_metadata.clone(),
            case_final_with_semver_prerelease.clone(),
            case_final_with_semver_0_prerelease.clone(),
            case_interim_with_semver_0.clone(),
            case_final_with_intver_0.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_17.expect(
            case_final_with_semver_0,
            case_final_with_semver_0_ignored_metadata,
            case_final_with_semver_prerelease,
            case_final_with_semver_0_prerelease,
            case_interim_with_semver_0,
            case_final_with_intver_0,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
