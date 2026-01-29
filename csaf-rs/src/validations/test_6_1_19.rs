use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use crate::csaf::types::version_number::{CsafVersionNumber, SemVerVersion, VersionNumber};

fn create_prerelease_version_error(number: &SemVerVersion, index: &usize) -> ValidationError {
    ValidationError {
        message: format!("revision history item number '{number}' contains a pre-release part"),
        instance_path: format!("/document/tracking/revision_history/{index}/number"),
    }
}

/// 6.1.19 Revision History Entries for Pre-release Versions
///
/// No item in `/document/tracking/revision_history[]` may have a version with a pre-release part (i.e. "1.0.0-rc1").
pub fn test_6_1_19_revision_history_entries_for_prerelease_versions(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    // Check that no revision history item has a pre-release part
    let mut errors: Option<Vec<ValidationError>> = None;
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    for (revision_index, revision) in revision_history.iter().enumerate() {
        let number = match revision.get_number() {
            CsafVersionNumber::Valid(number) => number,
            CsafVersionNumber::Invalid(err) => {
                errors.get_or_insert_default().push(err.get_validation_error(
                    format!("/document/tracking/revision_history/{revision_index}/number").as_str(),
                ));
                continue;
            },
        };
        match number {
            VersionNumber::IntVer(_) => {},
            VersionNumber::SemVer(semver) => {
                if semver.has_prerelease() {
                    errors
                        .get_or_insert_default()
                        .push(create_prerelease_version_error(&semver, &revision_index));
                }
            },
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_19
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_19_revision_history_entries_for_prerelease_versions(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_19
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_19_revision_history_entries_for_prerelease_versions(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_19() {
        let case_01 = Err(vec![create_prerelease_version_error(
            &SemVerVersion::from(Version::from_str("1.0.0-rc").unwrap()),
            &0,
        )]);
        let case_02 = Err(vec![create_prerelease_version_error(
            &SemVerVersion::from(Version::from_str("1.0.0-rc").unwrap()),
            &0,
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_19.expect(case_01.clone(), case_02.clone());
        TESTS_2_1.test_6_1_19.expect(case_01, case_02);
    }
}
