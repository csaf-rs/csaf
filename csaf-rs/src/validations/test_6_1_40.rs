use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::helpers::{MAX_UUID, NIL_UUID, SG_NAME_PRIVATE, SG_NAME_PUBLIC};
use crate::validation::ValidationError;

fn create_public_sharing_group_error() -> ValidationError {
    ValidationError {
        message: format!(
            "Sharing group name \"{}\" is prohibited without max UUID.",
            SG_NAME_PUBLIC
        ),
        instance_path: "/document/distribution/sharing_group/name".to_string(),
    }
}

fn create_private_sharing_group_error() -> ValidationError {
    ValidationError {
        message: format!(
            "Sharing group name \"{}\" is prohibited without nil UUID.",
            SG_NAME_PRIVATE
        ),
        instance_path: "/document/distribution/sharing_group/name".to_string(),
    }
}

/// Validates the sharing group name and ID combinations in a CSAF document.
///
/// This function checks if the sharing group name and ID in the document's distribution
/// follow specific rules:
///
/// - If the sharing group name is "Public", the ID must be the maximum UUID
///   ("ffffffff-ffff-ffff-ffff-ffffffffffff").
/// - If the sharing group name is "No sharing allowed", the ID must be the nil UUID
///   ("00000000-0000-0000-0000-000000000000").
///
/// # Arguments
///
/// * `doc` - A reference to an object implementing the `CsafTrait` interface.
///
/// # Returns
///
/// * `Ok(())` if the validation passes.
/// * `Err(vec![ValidationError])` if the validation fails, with a message explaining the reason
///   and the JSON path to the invalid element.
pub fn test_6_1_40_invalid_sharing_group_name(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if let Some(sharing_group) = distribution.get_sharing_group() {
        if let Some(sharing_group_name) = sharing_group.get_name() {
            if sharing_group_name == SG_NAME_PUBLIC {
                if sharing_group.get_id() != MAX_UUID {
                    return Err(vec![create_public_sharing_group_error()]);
                }
            } else if sharing_group_name == SG_NAME_PRIVATE && sharing_group.get_id() != NIL_UUID {
                return Err(vec![create_private_sharing_group_error()]);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_40() {
        run_csaf21_tests(
            "40",
            test_6_1_40_invalid_sharing_group_name,
            HashMap::from([
                ("01", vec![create_public_sharing_group_error()]),
                ("02", vec![create_private_sharing_group_error()]),
            ]),
        );
    }
}
