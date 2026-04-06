use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait, TlpTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::{DocumentStatus, LabelOfTlp};
use crate::validation::ValidationError;

static PUBLIC_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Document with TLP CLEAR and sharing group must use max UUID or nil UUID plus draft status.".to_string(),
    instance_path: "/document/distribution/sharing_group/id".to_string(),
});

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

    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() == LabelOfTlp::Clear
        && let Some(sharing_group) = distribution.get_sharing_group()
    {
        let sharing_group_id = sharing_group.get_id();
        return if sharing_group_id.is_max()
            || (sharing_group_id.is_nil() && doc.get_document().get_tracking().get_status() == DocumentStatus::Draft)
        {
            Ok(())
        } else {
            Err(vec![PUBLIC_SHARING_GROUP_ERROR.clone()])
        };
    }

    Ok(())
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_39, test_6_1_39_public_sharing_group_with_no_max_uuid);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_39() {
        let err = Err(vec![PUBLIC_SHARING_GROUP_ERROR.clone()]);
        // Only CSAF 2.1 has this test with 4 test cases (2 error cases, 2 success cases)
        TESTS_2_1.test_6_1_39.expect(
            // Case 01: TLP:CLEAR with regular UUID, status final
            err.clone(),
            // Case 02: TLP:CLEAR with Nil UUID, status final
            err.clone(),
            // Case 11: TLP:CLEAR with Max UUID, status final
            Ok(()),
            // Case 12: TLP:CLEAR with Nil UUID, status draft
            Ok(()),
        );
    }
}
