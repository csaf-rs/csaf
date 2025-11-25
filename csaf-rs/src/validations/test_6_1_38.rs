use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait, TlpTrait};
use crate::csaf2_1::schema::LabelOfTlp::Clear;
use crate::helpers::MAX_UUID;
use crate::validation::ValidationError;

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
            return Err(vec![ValidationError {
                message: "Document must be public (TLD CLEAR) when using max UUID as sharing group ID.".to_string(),
                instance_path: "/document/distribution/sharing_group/tlp/label".to_string(),
            }]);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_38::test_6_1_38_non_public_sharing_group_max_uuid;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_38() {
        let expected_error = ValidationError {
            message: "Document must be public (TLD CLEAR) when using max UUID as sharing group ID.".to_string(),
            instance_path: "/document/distribution/sharing_group/tlp/label".to_string(),
        };

        run_csaf21_tests(
            "38",
            test_6_1_38_non_public_sharing_group_max_uuid,
            HashMap::from([
                ("01", vec![expected_error.clone()]),
                ("02", vec![expected_error.clone()]),
                ("03", vec![expected_error.clone()]),
                ("04", vec![expected_error.clone()]),
            ]),
        );
    }
}
