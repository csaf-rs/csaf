use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

fn create_revision_history_error(
    status: &DocumentStatus,
    number: &impl std::fmt::Display,
    index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with status '{}' contains a revision history item with number '{}'",
            status, number
        ),
        instance_path: format!("/document/tracking/revision_history/{}/number", index),
    }
}

/// 6.1.18 Released Revision History
///
/// For documents with `/document/status` "final" or "interim", no item in `/document/tracking/revision_history[]`
/// may have the version 0 or 0.y.z.
pub fn test_6_1_18_released_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let status = doc.get_document().get_tracking().get_status();

    // This test is only relevant for documents with status 'interim' and 'final'
    if !(DocumentStatus::Final == status || DocumentStatus::Interim == status) {
        return Ok(());
    }

    // Check that no revision history item has version 0 or 0.y.z
    let mut errors = Vec::new();
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let number = revision.get_number();
        if number.is_intver_is_zero() || number.is_semver_is_major_zero() {
            errors.push(create_revision_history_error(&status, &number, i_r));
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
        let errors = HashMap::from([(
            "01",
            vec![create_revision_history_error(&DocumentStatus::Final, &"0", 0)],
        )]);
        run_csaf20_tests("18", test_6_1_18_released_revision_history, errors.clone());
        run_csaf21_tests("18", test_6_1_18_released_revision_history, errors);
    }
}
