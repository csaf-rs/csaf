use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::{TestFinding, TestFindingData};
use strum::{AsRefStr, Display};

#[derive(Display, AsRefStr)]
pub enum DocumentStatusDraftErrorReason {
    #[strum(serialize = "Version 0 is")]
    IntVerZero,
    #[strum(serialize = "Versions 0.y.z are")]
    SemVerMajorZero,
    #[strum(serialize = "Versions with prerelease are")]
    SemVerHasPre,
}

fn generate_status_version_error(
    version: &CsafVersionNumber,
    status: &DocumentStatus,
    reason: &DocumentStatusDraftErrorReason,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The document version is '{version}' but the document status is '{status}'. {reason} reserved for document status 'Draft'"
        ),
        instance_path: "/document/tracking/version".to_string(),
    })
}

/// 6.1.17 Document Status Draft
///
/// For `/document/version` to be `0`, `0.y.z` or contain a pre-release part (e.g. `-alpha`),
/// the `/document/status` must be `draft`.
///
/// This implementation checks this in an inverted order:
/// We first check if `/document/status` is `draft`. If so, the secondary condition is always fulfilled,
/// and this test can only pass, so we can return early.
/// If not, we know the secondary criteria is not fulfilled, and we can check if `/document/version`
/// meets one of the failing criteria and generate the corresponding error(s).
pub fn test_6_1_17_document_status_draft(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let tracking = doc.get_document().get_tracking();

    // This is explicitly **NOT** a wasSkipped. The test prose does not mention skipping,
    // we just check the two relevant conditions in inverted order and return early here.
    let doc_status = tracking.get_status();
    if DocumentStatus::Draft == doc_status {
        return Ok(());
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
            let mut errors: Option<Vec<TestFinding>> = None;
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
        CsafVersionNumber::Invalid(_) => Ok(()), // #409 this may be skipped, as the version is invalid and will be caught by schema test
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_17, test_6_1_17_document_status_draft);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_17 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_17 as ExpectedResults_2_1;
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

        TESTS_2_0.test_6_1_17.expect(ExpectedResults_2_0 {
            case_01: case_final_with_semver_0.clone(),
            case_s01: case_final_with_semver_0_ignored_metadata.clone(),
            case_s02: case_final_with_semver_prerelease.clone(),
            case_s03: case_final_with_semver_0_prerelease.clone(),
            case_s04: case_interim_with_semver_0.clone(),
            case_s05: case_final_with_intver_0.clone(),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
            case_s15: Ok(()),
        });
        TESTS_2_1.test_6_1_17.expect(ExpectedResults_2_1 {
            case_01: case_final_with_semver_0,
            case_s01: case_final_with_semver_0_ignored_metadata,
            case_s02: case_final_with_semver_prerelease,
            case_s03: case_final_with_semver_0_prerelease,
            case_s04: case_interim_with_semver_0,
            case_s05: case_final_with_intver_0,
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
            case_s15: Ok(()),
        });
    }
}
