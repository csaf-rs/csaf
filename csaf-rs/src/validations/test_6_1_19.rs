use crate::csaf::types::version_number::{CsafVersionNumber, SemVerVersion};
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_prerelease_version_error(number: &SemVerVersion, index: usize) -> ValidationError {
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
        match revision.get_number() {
            CsafVersionNumber::IntVer(_) => {},
            CsafVersionNumber::SemVer(semver) => {
                if semver.has_prerelease() {
                    errors
                        .get_or_insert_default()
                        .push(create_prerelease_version_error(&semver, revision_index));
                }
            },
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_19,
    test_6_1_19_revision_history_entries_for_prerelease_versions
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_19() {
        // Case 01: 2 revision history items, one with pre-release, dates are different
        // Case 02: 2 revision history items, one with pre-release, dates are the same
        let has_pre = Err(vec![create_prerelease_version_error(
            &SemVerVersion::from(Version::from_str("1.0.0-rc").unwrap()),
            0,
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_19.expect(has_pre.clone(), has_pre.clone());
        TESTS_2_1.test_6_1_19.expect(has_pre.clone(), has_pre);
    }
}
