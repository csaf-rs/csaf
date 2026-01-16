use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_prerelease_version_error(number: impl std::fmt::Display, index: usize) -> ValidationError {
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
    let mut errors = Vec::new();
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let number = revision.get_number();
        if number.is_semver_has_prerelease() {
            errors.push(create_prerelease_version_error(number, i_r));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
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

    #[test]
    fn test_test_6_1_19() {
        let case_01 = Err(vec![create_prerelease_version_error("1.0.0-rc", 0)]);
        let case_02 = Err(vec![create_prerelease_version_error("1.0.0-rc", 0)]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_19.expect(case_01.clone(), case_02.clone());
        TESTS_2_1.test_6_1_19.expect(case_01, case_02);
    }
}
