use std::sync::LazyLock;

use crate::csaf_traits::{
    CsafTrait, DistributionTrait, DocumentTrait, SG_NAME_PRIVATE, SG_NAME_PUBLIC, SharingGroupTrait,
};
use crate::validation::ValidationError;

static PUBLIC_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: format!("Sharing group name \"{SG_NAME_PUBLIC}\" is prohibited without max UUID."),
    instance_path: "/document/distribution/sharing_group/name".to_string(),
});

static PRIVATE_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: format!("Sharing group name \"{SG_NAME_PRIVATE}\" is prohibited without nil UUID."),
    instance_path: "/document/distribution/sharing_group/name".to_string(),
});

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
        // If the sharing group name is "Public", the ID must be max UUID
        if sharing_group.is_name_public() && !sharing_group.get_id().is_max() {
            return Err(vec![PUBLIC_SHARING_GROUP_ERROR.clone()]);
        // If the sharing group name is "No sharing allowed", the ID must be nil UUID
        } else if sharing_group.is_name_private() && !sharing_group.get_id().is_nil() {
            return Err(vec![PRIVATE_SHARING_GROUP_ERROR.clone()]);
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_40
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_40_invalid_sharing_group_name(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_40() {
        // Only CSAF 2.1 has this test with 6 test cases (2 error cases, 4 success cases)
        TESTS_2_1.test_6_1_40.expect(
            // Case 01: Name "Public" with regular UUID
            Err(vec![PUBLIC_SHARING_GROUP_ERROR.clone()]),
            // Case 02: Name "No sharing allowed" with regular UUID
            Err(vec![PRIVATE_SHARING_GROUP_ERROR.clone()]),
            // Case 11: Name "Public" with Max UUID
            Ok(()),
            // Case 12: Name "No sharing allowed" with Nil UUID
            Ok(()),
            // Case 13: Regular UUID without name
            Ok(()),
            // Case 14: Regular UUID with arbitrary name
            Ok(()),
        );
    }
}
