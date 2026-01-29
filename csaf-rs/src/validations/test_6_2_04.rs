use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use crate::csaf::types::version_number::{CsafVersionNumber, SemVerVersion, VersionNumber};

/// 6.2.4 Build Metadata in Revision History
///
/// The revision history must not contain build metadata in their `number` field
pub fn test_6_2_04_build_metadata_in_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (revision_index, revision) in doc
        .get_document()
        .get_tracking()
        .get_revision_history()
        .iter()
        .enumerate()
    {
        let version_number = match revision.get_number() {
            CsafVersionNumber::Valid(version_number) => version_number,
            CsafVersionNumber::Invalid(err) => {
                errors.get_or_insert_default().push(err.get_validation_error(
                    format!("/document/tracking/revision_history/{revision_index}/number").as_str(),
                ));
                continue;
            },
        };
        match version_number {
            VersionNumber::IntVer(_) => {},
            VersionNumber::SemVer(semver) => {
                if semver.has_build_metadata() {
                    errors
                        .get_or_insert_default()
                        .push(create_build_metadata_in_rev_history_error(&semver, &revision_index));
                }
            },
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_build_metadata_in_rev_history_error(number: &SemVerVersion, revision_index: &usize) -> ValidationError {
    ValidationError {
        message: format!("Revision history item with  number '{number}' contains build metadata"),
        instance_path: format!("/document/tracking/revision_history/{revision_index}/number"),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_4
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_04_build_metadata_in_rev_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_4
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_04_build_metadata_in_rev_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::csaf::types::version_number::SemVerVersion;
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_04() {
        let case_01 = Err(vec![create_build_metadata_in_rev_history_error(
            &SemVerVersion::from(Version::from_str("1.0.0+exp.sha.ac00785").unwrap()),
            &0,
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_4.expect(case_01.clone());
        TESTS_2_1.test_6_2_4.expect(case_01);
    }
}
