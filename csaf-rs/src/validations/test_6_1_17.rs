use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber};
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
    version: &ValidVersionNumber,
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
        return Ok(()); // ToDo return skipped/not applicable
    }

    // Check if doc version is valid
    let doc_version = match tracking.get_version() {
        CsafVersionNumber::Valid(version_number) => version_number,
        CsafVersionNumber::Invalid(err) => return Err(vec![err.get_validation_error("/document/version")]), // ToDo generate warning
    };

    match &doc_version {
        ValidVersionNumber::IntVer(intver) => {
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
        ValidVersionNumber::SemVer(semver) => {
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
        let case_final_with_semver_0 = Err(vec![generate_status_version_error(
            &ValidVersionNumber::from_str("0.9.5").unwrap(),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_semver_0_ignored_metadata = Err(vec![generate_status_version_error(
            &ValidVersionNumber::from_str("0.0.0+exp.sha.ac00785").unwrap(),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_semver_prerelease = Err(vec![generate_status_version_error(
            &ValidVersionNumber::from_str("1.0.0-alpha").unwrap(),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::SemVerHasPre,
        )]);

        let case_final_with_semver_0_prerelease = Err(vec![
            generate_status_version_error(
                &ValidVersionNumber::from_str("0.9.5-alpha").unwrap(),
                &DocumentStatus::Final,
                &DocumentStatusDraftErrorReason::SemVerMajorZero,
            ),
            generate_status_version_error(
                &ValidVersionNumber::from_str("0.9.5-alpha").unwrap(),
                &DocumentStatus::Final,
                &DocumentStatusDraftErrorReason::SemVerHasPre,
            ),
        ]);

        let case_interim_with_semver_0 = Err(vec![generate_status_version_error(
            &ValidVersionNumber::from_str("0.9.5").unwrap(),
            &DocumentStatus::Interim,
            &DocumentStatusDraftErrorReason::SemVerMajorZero,
        )]);

        let case_final_with_intver_0 = Err(vec![generate_status_version_error(
            &ValidVersionNumber::from_str("0").unwrap(),
            &DocumentStatus::Final,
            &DocumentStatusDraftErrorReason::IntVerZero,
        )]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_17.expect(
            case_final_with_semver_0.clone(),
            case_final_with_semver_0_ignored_metadata.clone(),
            case_final_with_semver_prerelease.clone(),
            case_final_with_semver_0_prerelease.clone(),
            case_interim_with_semver_0.clone(),
            case_final_with_intver_0.clone(),
            Ok(()), // draft with intver 0
            Ok(()), // draft with semver 0.y.z
            Ok(()), // draft with semver prerelease
            Ok(()), // final with semver 1.0.0+buildmetadata
        );
        TESTS_2_1.test_6_1_17.expect(
            case_final_with_semver_0,
            case_final_with_semver_0_ignored_metadata,
            case_final_with_semver_prerelease,
            case_final_with_semver_0_prerelease,
            case_interim_with_semver_0,
            case_final_with_intver_0,
            Ok(()), // draft with intver 0
            Ok(()), // draft with semver 0.y.z
            Ok(()), // draft with semver prerelease
            Ok(()), // final with semver 1.0.0+buildmetadata
        );
    }
}
