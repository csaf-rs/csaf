use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::ValidationError;

static USAGE_OF_SHARING_GROUP_ON_TLP_CLEAR_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "A sharing group must not be used when the document is TLP:CLEAR.".to_string(),
    instance_path: "/document/distribution/sharing_group".to_string(),
});

/// 6.2.30 Usage of Sharing Group on TLP:CLEAR
///
/// It MUST be tested that no sharing group is used if the document is TLP:CLEAR.
pub fn test_6_2_30_usage_of_sharing_group_on_tlp_clear(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() == LabelOfTlp::Clear
        && distribution.get_sharing_group().is_some()
    {
        return Err(vec![USAGE_OF_SHARING_GROUP_ON_TLP_CLEAR_ERROR.clone()]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_30,
    test_6_2_30_usage_of_sharing_group_on_tlp_clear
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_30() {
        let err = Err(vec![USAGE_OF_SHARING_GROUP_ON_TLP_CLEAR_ERROR.clone()]);

        // Case 01: sharing group present with TLP:CLEAR
        // Case 11: TLP:CLEAR without sharing group
        // Case 12: sharing group present with TLP:RED

        TESTS_2_1.test_6_2_30.expect(err, Ok(()), Ok(()));
    }
}
