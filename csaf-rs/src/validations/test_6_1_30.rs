use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_mixed_versioning_error(part: &str) -> ValidationError {
    ValidationError {
        message: "mixed integer and semantic versioning used".to_string(),
        instance_path: format!("/document/tracking/{}", part),
    }
}

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();
    // make sure revision history is consistent in itself
    let mut errors: Option<Vec<ValidationError>> = None;

    let rev_history_tuples = tracking.aggregate_revision_history();
    let mut semver_count = 0;
    let mut intver_count = 0;

    for current in rev_history_tuples.iter() {
        match current.number {
            CsafVersionNumber::SemVer(_) => semver_count += 1,
            CsafVersionNumber::IntVer(_) => intver_count += 1,
            CsafVersionNumber::Invalid(_) => {}, // ignore invalid version numbers
        }
    }

    if semver_count > 0 && intver_count > 0 {
        errors
            .get_or_insert_default()
            .push(create_mixed_versioning_error("revision_history"));
    }

    let doc_version = doc.get_document().get_tracking().get_version();
    match doc_version {
        CsafVersionNumber::SemVer(_) => {
            if intver_count > 0 {
                errors
                    .get_or_insert_default()
                    .push(create_mixed_versioning_error("version"));
            }
        },
        CsafVersionNumber::IntVer(_) => {
            if semver_count > 0 {
                errors
                    .get_or_insert_default()
                    .push(create_mixed_versioning_error("version"));
            }
        },
        CsafVersionNumber::Invalid(_) => {}, // ignore invalid version numbers
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_30,
    test_6_1_30_mixed_integer_and_semantic_versioning
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_30() {
        let case_consistent_history_mismatch_to_document = Err(vec![create_mixed_versioning_error("version")]);
        let case_inconsistent_history_and_mismatch_to_document = Err(vec![
            create_mixed_versioning_error("version"),
            create_mixed_versioning_error("revision_history"),
        ]);

        TESTS_2_0.test_6_1_30.expect(
            case_inconsistent_history_and_mismatch_to_document.clone(),
            case_consistent_history_mismatch_to_document.clone(),
            case_consistent_history_mismatch_to_document.clone(),
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
        TESTS_2_1.test_6_1_30.expect(
            case_inconsistent_history_and_mismatch_to_document,
            case_consistent_history_mismatch_to_document.clone(),
            case_consistent_history_mismatch_to_document,
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
    }
}
