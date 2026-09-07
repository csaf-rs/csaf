use crate::csaf::macros::skip_if_document_status_is_not::skip_if_document_status_is_not;
use crate::csaf::types::version_number::{CsafVersionNumber, SemVerVersion};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::{TestFinding, TestFindingData};

fn create_status_version_error(status: &DocumentStatus, version: &SemVerVersion) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The document status is {status} but the document version {version} contains a pre-release part"
        ),
        instance_path: "/document/tracking/version".to_string(),
    })
}

/// 6.1.20 Non-draft Document Version
///
/// For documents with status "final" or "interim", the `/document/tracking/version` field must not contain
/// a pre-release part (e.g. "1.0.0-alpha").
pub fn test_6_1_20_non_draft_document_version(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let tracking = doc.get_document().get_tracking();

    let status = tracking.get_status();
    skip_if_document_status_is_not!(status, Final, Interim);

    match tracking.get_version() {
        // If version is integer versioning, this test does not apply
        CsafVersionNumber::IntVer(_) => {}, // ToDo maybe generate skipped/not applicable
        CsafVersionNumber::SemVer(semver) => {
            if semver.has_prerelease() {
                return Err(vec![create_status_version_error(&status, &semver)]);
            }
        },
        CsafVersionNumber::Invalid(_) => {}, // ignore invalid version numbers
    }

    Ok(())
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_20, test_6_1_20_non_draft_document_version);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_20 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_20 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::schema::csaf2_1::schema::DocumentStatus;
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_20() {
        let case_interim = Err(vec![create_status_version_error(
            &DocumentStatus::Interim,
            &SemVerVersion::from(Version::from_str("1.0.0-alpha").unwrap()),
        )]);
        let case_final = Err(vec![create_status_version_error(
            &DocumentStatus::Final,
            &SemVerVersion::from(Version::from_str("1.0.0-alpha").unwrap()),
        )]);

        TESTS_2_0.test_6_1_20.expect(ExpectedResults_2_0 {
            case_01: case_interim.clone(),
            case_s01: case_final.clone(),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_1_20.expect(ExpectedResults_2_1 {
            case_01: case_interim,
            case_s01: case_final,
            case_s11: Ok(()),
        });
    }
}
