use crate::csaf::types::version_number::{CsafVersionNumber, SemVerVersion};
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

/// 6.2.4 Build Metadata in Revision History
///
/// The revision history must not contain build metadata in their `number` field
pub fn test_6_2_04_build_metadata_in_rev_history(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (revision_index, revision) in doc
        .get_document()
        .get_tracking()
        .get_revision_history()
        .iter()
        .enumerate()
    {
        match revision.get_number() {
            CsafVersionNumber::IntVer(_) => {},
            CsafVersionNumber::SemVer(semver) => {
                if semver.has_build_metadata() {
                    errors
                        .get_or_insert_default()
                        .push(create_build_metadata_in_rev_history_error(&semver, &revision_index));
                }
            },
            CsafVersionNumber::Invalid(_) => {}, // ignore invalid version numbers
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_build_metadata_in_rev_history_error(number: &SemVerVersion, revision_index: &usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Revision history item with number '{number}' contains build metadata"),
        instance_path: format!("/document/tracking/revision_history/{revision_index}/number"),
    })
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_4, test_6_2_04_build_metadata_in_rev_history);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_4 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_4 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_test_6_2_04() {
        let revision_history_with_build_metadata = Err(vec![create_build_metadata_in_rev_history_error(
            &SemVerVersion::from(Version::from_str("1.0.0+exp.sha.ac00785").unwrap()),
            &0,
        )]);

        let revision_history_with_alternating_build_metadata = Err(vec![
            create_build_metadata_in_rev_history_error(
                &SemVerVersion::from(Version::from_str("1.1.0+exp.sha.ac00785").unwrap()),
                &1,
            ),
            create_build_metadata_in_rev_history_error(
                &SemVerVersion::from(Version::from_str("1.3.0+exp.sha.ac00785").unwrap()),
                &3,
            ),
        ]);

        let revision_history_with_build_metadata_after_pre_release =
            Err(vec![create_build_metadata_in_rev_history_error(
                &SemVerVersion::from(Version::from_str("1.1.0-rc.1+exp.sha.ac00785").unwrap()),
                &1,
            )]);

        // Case s11: revision_history without build metadata
        // Case s12: revision_history with integer versions (no build metadata allowed by schema)

        TESTS_2_0.test_6_2_4.expect(ExpectedResults_2_0 {
            case_01: revision_history_with_build_metadata.clone(),
            case_s01: revision_history_with_alternating_build_metadata.clone(),
            case_s02: revision_history_with_build_metadata_after_pre_release.clone(),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });

        TESTS_2_1.test_6_2_4.expect(ExpectedResults_2_1 {
            case_01: revision_history_with_build_metadata,
            case_s01: revision_history_with_alternating_build_metadata,
            case_s02: revision_history_with_build_metadata_after_pre_release,
            case_s11: Ok(()),
            case_s12: Ok(()),
        });
    }
}
