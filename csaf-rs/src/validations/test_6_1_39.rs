use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait, TlpTrait, TrackingTrait};
use crate::helpers::{MAX_UUID, NIL_UUID};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::schema::csaf2_1::schema::LabelOfTlp::Clear;
use crate::validation::ValidationError;

/// Validates that when a document is marked with TLP CLEAR, any associated sharing group
/// must either have a `MAX_UUID` as its ID or a `NIL_UUID` accompanied by the document status being "Draft".
///
/// This function checks the following (if TLP CLEAR):
/// - If the sharing group ID is `MAX_UUID`, the validation passes.
/// - If the sharing group ID is `NIL_UUID` and the document status is "Draft", the validation passes.
/// - Otherwise, the function returns a `ValidationError` with a relevant error message.
///
/// # Arguments
///
/// - `doc`: A document implementing the `CsafTrait` interface.
///
/// # Returns
///
/// - `Ok(())` if the validation passes.
/// - `Err(vec![ValidationError])` if the requirements are not met.
pub fn test_6_1_39_public_sharing_group_with_no_max_uuid(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() == Clear {
        if let Some(sharing_group) = distribution.get_sharing_group() {
            let sharing_group_id = sharing_group.get_id();
            return if sharing_group_id == MAX_UUID
                || (sharing_group_id == NIL_UUID
                    && doc.get_document().get_tracking().get_status() == DocumentStatus::Draft)
            {
                Ok(())
            } else {
                Err(vec![ValidationError {
                    message:
                        "Document with TLP CLEAR and sharing group must use max UUID or nil UUID plus draft status."
                            .to_string(),
                    instance_path: "/document/distribution/sharing_group/id".to_string(),
                }])
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_39::test_6_1_39_public_sharing_group_with_no_max_uuid;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_39() {
        let expected_error = ValidationError {
            message: "Document with TLP CLEAR and sharing group must use max UUID or nil UUID plus draft status."
                .to_string(),
            instance_path: "/document/distribution/sharing_group/id".to_string(),
        };

        run_csaf21_tests(
            "39",
            test_6_1_39_public_sharing_group_with_no_max_uuid,
            HashMap::from([
                ("01", vec![expected_error.clone()]),
                ("02", vec![expected_error.clone()]),
            ]),
        );
    }
}
