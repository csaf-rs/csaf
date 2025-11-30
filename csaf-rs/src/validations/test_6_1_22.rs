use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

/// Test 6.1.22: Multiple Definition in Revision History
///
/// Items of the revision history must not contain the same string in the
/// `/document/tracking/revision_history[]/number` field.
pub fn test_6_1_22_multiple_definition_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let revision_history = tracking.get_revision_history();

    // Map occurrence paths indexes to revision numbers
    let mut number_paths: HashMap<String, Vec<usize>> = HashMap::new();
    for (i_r, revision) in revision_history.iter().enumerate() {
        let number = revision.get_number_string();
        let path = number_paths.entry(number.clone()).or_insert_with(Vec::new);
        path.push(i_r);
    }

    // Generate errors for revision numbers with multiple occurrence paths indexes
    let mut errors = Vec::new();
    for (number, paths) in &number_paths {
        if paths.len() > 1 {
            for path in paths.iter() {
                errors.push(ValidationError {
                    message: format!("Duplicate definition of revision history number {}", number),
                    instance_path: format!("/document/tracking/revision_history/{}/number", path),
                });
            }
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
    use crate::validations::test_6_1_22::test_6_1_22_multiple_definition_in_revision_history;

    #[test]
    fn test_test_6_1_22() {
        let errors = std::collections::HashMap::from([(
            "01",
            vec![
                crate::validation::ValidationError {
                    message: "Duplicate definition of revision history number 1".to_string(),
                    instance_path: "/document/tracking/revision_history/0/number".to_string(),
                },
                crate::validation::ValidationError {
                    message: "Duplicate definition of revision history number 1".to_string(),
                    instance_path: "/document/tracking/revision_history/1/number".to_string(),
                },
            ],
        )]);
        run_csaf20_tests(
            "22",
            test_6_1_22_multiple_definition_in_revision_history,
            errors.clone(),
        );
        run_csaf21_tests("22", test_6_1_22_multiple_definition_in_revision_history, errors);
    }
}
