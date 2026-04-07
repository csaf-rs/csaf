use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::ValidationError;

static NON_PUBLIC_SHARING_GROUP_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Document must be public (TLD CLEAR) when using max UUID as sharing group ID.".to_string(),
    instance_path: "/document/distribution/sharing_group/tlp/label".to_string(),
});

/// Validates that a CSAF document using the maximum UUID as the sharing group ID
/// has the TLP (Traffic Light Protocol) label set to `CLEAR`.
///
/// According to CSAF 2.1 specifications, when a document uses such a
/// sharing group ID, it must be publicly accessible, represented by
/// having the TLP label as `CLEAR`.
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
pub fn test_6_1_38_non_public_sharing_group_max_uuid(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let distribution = doc.get_document().get_distribution_21().map_err(|e| vec![e])?;

    if let Some(sharing_group) = distribution.get_sharing_group()
        && sharing_group.get_id().is_max()
        && distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() != LabelOfTlp::Clear
    {
        return Err(vec![NON_PUBLIC_SHARING_GROUP_ERROR.clone()]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_38,
    test_6_1_38_non_public_sharing_group_max_uuid
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_38() {
        let err = Err(vec![NON_PUBLIC_SHARING_GROUP_ERROR.clone()]);
        // Only CSAF 2.1 has this test with 9 test cases (4 error cases, 5 success cases)
        TESTS_2_1.test_6_1_38.expect(
            // Case 01: Max UUID with TLP:RED
            err.clone(),
            // Case 02: Max UUID with TLP:AMBER+STRICT
            err.clone(),
            // Case 03: Max UUID with TLP:AMBER
            err.clone(),
            // Case 04: Max UUID with TLP:GREEN
            err.clone(),
            // Case 11: Regular UUID with TLP:RED
            Ok(()),
            // Case 12: Regular UUID with TLP:AMBER+STRICT, no name
            Ok(()),
            // Case 13: Regular UUID with TLP:AMBER
            Ok(()),
            // Case 14: No sharing group with TLP:GREEN
            Ok(()),
            // Case 15: Max UUID with TLP:CLEAR
            Ok(()),
        );
    }
}
