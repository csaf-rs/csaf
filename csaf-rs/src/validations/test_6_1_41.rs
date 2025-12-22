use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::helpers::{MAX_UUID, NIL_UUID, SG_NAME_PRIVATE, SG_NAME_PUBLIC};
use crate::validation::ValidationError;

static MAX_UUID_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: format!("Max UUID requires sharing group name to be \"{}\".", SG_NAME_PUBLIC),
    instance_path: "/document/distribution/sharing_group/name".to_string(),
});

static NIL_UUID_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: format!("Nil UUID requires sharing group name to be \"{}\".", SG_NAME_PRIVATE),
    instance_path: "/document/distribution/sharing_group/name".to_string(),
});

/// Validates that a CSAF document with specific sharing group IDs has the correct corresponding name.
///
/// This function ensures that:
/// - When using the maximum UUID ("ffffffff-ffff-ffff-ffff-ffffffffffff"), the sharing group name
///   must be "Public".
/// - When using the nil UUID ("00000000-0000-0000-0000-000000000000"), the sharing group name
///   must be "No sharing allowed".
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
pub fn test_6_1_41_missing_sharing_group_name(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if let Some(sharing_group) = distribution.get_sharing_group() {
        // Check if max UUID is used
        if sharing_group.get_id() == MAX_UUID {
            // If max UUID is used, the name must exist and be NAME_PUBLIC
            match sharing_group.get_name() {
                Some(name) if name == SG_NAME_PUBLIC => {},
                _ => {
                    return Err(vec![MAX_UUID_SHARING_GROUP_ERROR.clone()]);
                },
            }
        }
        // Check if nil UUID is used
        else if sharing_group.get_id() == NIL_UUID {
            // If nil UUID is used, the name must exist and be NAME_PRIVATE
            match sharing_group.get_name() {
                Some(name) if name == SG_NAME_PRIVATE => {},
                _ => {
                    return Err(vec![NIL_UUID_SHARING_GROUP_ERROR.clone()]);
                },
            }
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_41
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_41_missing_sharing_group_name(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_41() {
        let max_uuid_err = Err(vec![MAX_UUID_SHARING_GROUP_ERROR.clone()]);
        let nil_uuid_err = Err(vec![NIL_UUID_SHARING_GROUP_ERROR.clone()]);

        // Only CSAF 2.1 has this test with 6 test cases (4 error cases, 2 success cases)
        TESTS_2_1.test_6_1_41.expect(
            max_uuid_err.clone(),
            nil_uuid_err.clone(),
            max_uuid_err.clone(),
            nil_uuid_err.clone(),
            Ok(()),
            Ok(()),
        );
    }
}
