use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait, TlpTrait};
use crate::helpers::MAX_UUID;
use crate::schema::csaf2_1::schema::LabelOfTlp::Clear;
use crate::validation::ValidationError;

fn create_non_public_sharing_group_error() -> ValidationError {
    ValidationError {
        message: "Document must be public (TLD CLEAR) when using max UUID as sharing group ID.".to_string(),
        instance_path: "/document/distribution/sharing_group/tlp/label".to_string(),
    }
}

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

    if let Some(sharing_group) = distribution.get_sharing_group() {
        if sharing_group.get_id() == MAX_UUID && distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() != Clear {
            return Err(vec![create_non_public_sharing_group_error()]);
        }
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_38
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_38_non_public_sharing_group_max_uuid(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_38() {
        // Only CSAF 2.1 has this test with 9 test cases (4 error cases, 5 success cases)
        TESTS_2_1.test_6_1_38.expect(
            Err(vec![create_non_public_sharing_group_error()]),
            Err(vec![create_non_public_sharing_group_error()]),
            Err(vec![create_non_public_sharing_group_error()]),
            Err(vec![create_non_public_sharing_group_error()]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
