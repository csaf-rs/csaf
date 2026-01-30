use crate::csaf::types::csaf_version_number::{CsafVersionNumber, SemVerVersion, ValidVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

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
    let mut errors: Option<Vec<ValidationError>> = None;
    // get version numbers from revision history
    let revision_history_numbers = doc.get_document().get_tracking().get_revision_history();
    for item in revision_history_numbers {
        match item.number {
            CsafVersionNumber::Invalid(err) => {
                // if number is invalid, add an error
                errors.get_or_insert_default().push(err.get_validation_error(format!("/document/tracking/revision_history/{}/number", item.path_index).as_str()));
            }
            CsafVersionNumber::Valid(number) => {
                match &number {
                    ValidVersionNumber::IntVer(_) => {
                        // Integer versions cannot have prerelease parts, so nothing to do here
                    },
                    ValidVersionNumber::SemVer(semver) => {
                        // If the semver version has a prerelease part, add an error
                        if semver.has_prerelease() {
                            errors
                                .get_or_insert_default()
                                .push(create_prerelease_version_error(semver, &item.path_index));
                        }
                    },
                }
            }
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
