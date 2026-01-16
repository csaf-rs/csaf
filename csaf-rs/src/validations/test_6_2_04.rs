use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait, VersionNumber};
use crate::validation::ValidationError;

/// 6.2.4 Build Metadata in Revision History
///
/// The revision history must not contain build metadata in their `number` field
pub fn test_6_2_04_build_metadata_in_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (r_i, rev_history) in doc
        .get_document()
        .get_tracking()
        .get_revision_history()
        .iter()
        .enumerate()
    {
        if rev_history.get_number().is_semver_has_build_metadata() {
            errors
                .get_or_insert_with(Vec::new)
                .push(create_build_metadata_in_rev_history_error(
                    r_i,
                    rev_history.get_number(),
                ));
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_build_metadata_in_rev_history_error(revision_index: usize, number: VersionNumber) -> ValidationError {
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

    #[test]
    fn test_test_6_2_04() {
        let case_01 = Err(vec![create_build_metadata_in_rev_history_error(
            0,
            VersionNumber::from_number("1.0.0+exp.sha.ac00785"),
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_4.expect(case_01.clone());
        TESTS_2_1.test_6_2_4.expect(case_01);
    }
}
