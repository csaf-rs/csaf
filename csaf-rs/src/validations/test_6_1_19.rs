use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_prerelease_version_error(number: impl std::fmt::Display, index: usize) -> ValidationError {
    ValidationError {
        message: format!("revision history item number '{}' contains a pre-release part", number),
        instance_path: format!("/document/tracking/revision_history/{}/number", index),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_14() {
        let errors = HashMap::from([
            ("01", vec![create_prerelease_version_error("1.0.0-rc", 0)]),
            ("02", vec![create_prerelease_version_error("1.0.0-rc", 0)]),
        ]);
        run_csaf20_tests(
            "19",
            test_6_1_19_revision_history_entries_for_prerelease_versions,
            errors.clone(),
        );
        run_csaf21_tests(
            "19",
            test_6_1_19_revision_history_entries_for_prerelease_versions,
            errors,
        );
    }
}
