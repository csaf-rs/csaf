use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::validation::ValidationError;

static USAGE_OF_NIL_UUID_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The sharing group id uses the Nil UUID.".to_string(),
    instance_path: "/document/distribution/sharing_group/id".to_string(),
});

/// 6.2.29 Usage of Nil UUID
///
/// It MUST be tested that the Nil UUID is not used as sharing group id.
pub fn test_6_2_29_usage_of_nil_uuid(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if let Some(sharing_group) = distribution.get_sharing_group()
        && sharing_group.get_id().is_nil()
    {
        return Err(vec![USAGE_OF_NIL_UUID_ERROR.clone()]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_29, test_6_2_29_usage_of_nil_uuid);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_29() {
        let err = Err(vec![USAGE_OF_NIL_UUID_ERROR.clone()]);

        // Case 01: sharing group id is the Nil UUID
        // Case 11: sharing group with a regular UUID
        // Case 12: no sharing group present

        TESTS_2_1.test_6_2_29.expect(err, Ok(()), Ok(()));
    }
}
