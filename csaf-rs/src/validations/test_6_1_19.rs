use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;

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
            errors.push(ValidationError {
                message: format!("revision history item number '{}' contains a pre-release part", number),
                instance_path: format!("/document/tracking/revision_history/{}/number", i_r),
            });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_19::test_6_1_19_revision_history_entries_for_prerelease_versions;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_14() {
        let errors = HashMap::from([
            (
                "01",
                vec![ValidationError {
                    message: "revision history item number '1.0.0-rc' contains a pre-release part".to_string(),
                    instance_path: "/document/tracking/revision_history/0/number".to_string(),
                }],
            ),
            (
                "02",
                vec![ValidationError {
                    message: "revision history item number '1.0.0-rc' contains a pre-release part".to_string(),
                    instance_path: "/document/tracking/revision_history/0/number".to_string(),
                }],
            ),
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
