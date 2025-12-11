use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait, VersionNumber, RevisionHistorySortable};
use crate::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

/// 6.1.16 Latest Document Version
///
/// `/document/tracking/version` must be equal to the last `/document/tracking/revision_history[]/number` when
/// sorting the revision history ascending by `date`. Build metadata is ignored. Pre-release parts are ignored
/// if `/document/status` is "draft".
pub fn test_6_1_16_latest_document_version(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    let mut revision_history = doc.get_document().get_tracking().get_revision_history_tuples();
    revision_history.sort_by_date_then_number();

    if let Some(latest_revision_history_item) = revision_history.last() {
        let latest_number = &latest_revision_history_item.number;
        let doc_version = document.get_tracking().get_version();
        let doc_status = document.get_tracking().get_status();
        match latest_number {
            VersionNumber::Integer(_) => {
                // We can use the default eq here, as intver has no pre-release or build metadata
                // and the eq will return false if comparing intver with semver
                if doc_version == *latest_number {
                    return Ok(());
                }
            },
            VersionNumber::Semver(latest) => {
                // Manually check if comparing with intver
                if let VersionNumber::Semver(ref version) = doc_version {
                    // Manually compare the semver objs according to test req
                    let mut equal = true;
                    equal &= equal && version.major == latest.major;
                    equal &= equal && version.minor == latest.minor;
                    equal &= equal && version.patch == latest.patch;
                    if doc_status != DocumentStatus::Draft {
                        equal &= equal && version.pre == latest.pre;
                    }
                    if equal {
                        return Ok(());
                    }
                }
            },
        };

        return Err(vec![test_6_1_16_err_generator(
            doc_version.to_string(),
            latest_number.to_string(),
            doc_status.to_string(),
        )]);
    }

    // This should not be able to happen as revision history is a required property with 1..* items
    panic!("Revision history is empty, document is malformed.");
}

fn test_6_1_16_err_generator(doc_version: String, latest_number: String, doc_status: String) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{}' is not equal to the latest revision history number '{}' in document with status '{}'",
            doc_version, latest_number, doc_status
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_16::{test_6_1_16_err_generator, test_6_1_16_latest_document_version};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_16() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_16_err_generator(
                    "1".to_string(),
                    "2".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "02",
                vec![test_6_1_16_err_generator(
                    "1".to_string(),
                    "2".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "03",
                vec![test_6_1_16_err_generator(
                    "1".to_string(),
                    "2".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "04",
                vec![test_6_1_16_err_generator(
                    "1.0.0".to_string(),
                    "2.0.0".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "05",
                vec![test_6_1_16_err_generator(
                    "1.0.0".to_string(),
                    "2.0.0".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "06",
                vec![test_6_1_16_err_generator(
                    "9".to_string(),
                    "10".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "07",
                vec![test_6_1_16_err_generator(
                    "1.9.0".to_string(),
                    "1.10.0".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "08",
                vec![test_6_1_16_err_generator(
                    "1".to_string(),
                    "2".to_string(),
                    "final".to_string(),
                )],
            ),
            (
                "09",
                vec![test_6_1_16_err_generator(
                    "2".to_string(),
                    "1".to_string(),
                    "final".to_string(),
                )],
            ),
        ]);
        run_csaf20_tests("16", test_6_1_16_latest_document_version, errors.clone());
        run_csaf21_tests("16", test_6_1_16_latest_document_version, errors);
    }
}
