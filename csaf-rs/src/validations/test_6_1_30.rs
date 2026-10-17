use std::sync::LazyLock;

use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::{TestFinding, TestFindingData};

static MIXED_VERSIONING_IN_REVISION_HISTORY_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message: "Revision history contains mixed integer and semantic versioning".to_string(),
        instance_path: "/document/tracking/revision_history".to_string(),
    })
});

static SEMVER_DOCUMENT_VERSION_WITH_INTVER_REVISION_HISTORY_ITEMS_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message: "The document version uses semantic versioning while an item in the revision history uses integer versioning".to_string(),
        instance_path: "/document/tracking/version".to_string(),
    })
});

static INTVER_DOCUMENT_VERSION_WITH_SEMVER_REVISION_HISTORY_ITEMS_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Error(TestFindingData {
        message: "The document version uses integer versioning while an item in the revision history uses semantic versioning".to_string(),
        instance_path: "/document/tracking/version".to_string(),
    })
});

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// It SHALL be tested that all elements of type `$['$defs'].version_t` follow either integer versioning or
/// semantic versioning homogeneously within the same document.
///
/// In order to create consistent results in the case of a value that is not a valid integer nor semantic version,
/// implementations SHOULD ignore such a value in this test as the schema test will fail.
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
///
/// We first check if the revision history is consistent in itself, and emit an error if both versioning schemes
/// are used, and emit an error if so.
/// Then we check if the scheme of the document is inconsistent with any of the revision history entries,
/// and emit an error if so.
/// This means that if the revision history is inconsistent (i.e. contains integer and semantic versioning),
/// it is guaranteed that the document version is inconsistent too, so both errors will be emitted.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let tracking = doc.get_document().get_tracking();
    let mut errors: Option<Vec<TestFinding>> = None;

    // count the number of semantic and integer versioning entries in the revision history
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

    // emit an error if both versioning schemes are used in the revision history
    if semver_count > 0 && intver_count > 0 {
        errors
            .get_or_insert_default()
            .push(MIXED_VERSIONING_IN_REVISION_HISTORY_ERROR.clone());
    }

    // emit an error if the document version is inconsistent with the revision history
    // this is guaranteed to be the case if the revision history is inconsistent
    let doc_version = tracking.get_version();
    match doc_version {
        CsafVersionNumber::SemVer(_) => {
            if intver_count > 0 {
                errors
                    .get_or_insert_default()
                    .push(SEMVER_DOCUMENT_VERSION_WITH_INTVER_REVISION_HISTORY_ITEMS_ERROR.clone());
            }
        },
        CsafVersionNumber::IntVer(_) => {
            if semver_count > 0 {
                errors
                    .get_or_insert_default()
                    .push(INTVER_DOCUMENT_VERSION_WITH_SEMVER_REVISION_HISTORY_ITEMS_ERROR.clone());
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
    use crate::csaf2_0::testcases::ExpectedResults_6_1_30 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_30 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_30() {
        let case_inconsistent_history_and_mismatch_to_document = Err(vec![
            INTVER_DOCUMENT_VERSION_WITH_SEMVER_REVISION_HISTORY_ITEMS_ERROR.clone(),
            MIXED_VERSIONING_IN_REVISION_HISTORY_ERROR.clone(),
        ]);
        let case_intver_document_with_semver_history = Err(vec![
            INTVER_DOCUMENT_VERSION_WITH_SEMVER_REVISION_HISTORY_ITEMS_ERROR.clone(),
        ]);
        let case_semver_document_with_intver_history = Err(vec![
            SEMVER_DOCUMENT_VERSION_WITH_INTVER_REVISION_HISTORY_ITEMS_ERROR.clone(),
        ]);

        // Case S11: only semver versioning
        // Case S11: only intver versioning

        TESTS_2_0.test_6_1_30.expect(ExpectedResults_2_0 {
            case_01: case_inconsistent_history_and_mismatch_to_document.clone(),
            case_s01: case_intver_document_with_semver_history.clone(),
            case_s02: case_semver_document_with_intver_history.clone(),
            case_11: Ok(()),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_1_30.expect(ExpectedResults_2_1 {
            case_01: case_inconsistent_history_and_mismatch_to_document,
            case_s01: case_intver_document_with_semver_history,
            case_s02: case_semver_document_with_intver_history,
            case_11: Ok(()),
            case_s11: Ok(()),
        });
    }
}
