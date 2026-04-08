use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::validation::ValidationError;

static USAGE_OF_MAX_UUID_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The sharing group id uses the Max UUID.".to_string(),
    instance_path: "/document/distribution/sharing_group/id".to_string(),
});

/// 6.2.28 Usage of Max UUID
///
/// It MUST be tested that the Max UUID is not used as sharing group id.
pub fn test_6_2_28_usage_of_max_uuid(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if let Some(sharing_group) = distribution.get_sharing_group()
        && sharing_group.get_id().is_max()
    {
        return Err(vec![USAGE_OF_MAX_UUID_ERROR.clone()]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_28,
    test_6_2_28_usage_of_max_uuid
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_28() {
        let err = Err(vec![USAGE_OF_MAX_UUID_ERROR.clone()]);

        // Case 01: sharing group id is the Max UUID
        // Case 11: no sharing group present
        // Case 12: sharing group with a regular UUID

        TESTS_2_1.test_6_2_28.expect(
            err,
            Ok(()),
            Ok(()),
        );
    }
}

