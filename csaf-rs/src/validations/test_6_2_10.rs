use crate::csaf_traits::{CsafTrait, CsafVersion, DistributionTrait, DocumentTrait};
use crate::validation::ValidationError;
use std::sync::LazyLock;

/// 6.2.10 Missing TLP label
///
/// `/document/distribution/tlp/label` must be set.
pub fn test_6_2_10_missing_tlp_label(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // In CSAF 2.1 this field is mandatory and validated via the schema, so we skip this test
    if doc.get_document().get_csaf_version() == &CsafVersion::X21 {
        return Ok(()); // TODO #409 wasSkipped
    }
    // We just need to consider get_distribution_20 / get_tlp_20 here. If either is missing, return an error
    if doc.get_document().get_distribution_20()
        .and_then(|d| d.get_tlp_20())
        .is_none()
    {
        Err(vec![MISSING_TLP_LABEL_ERROR.clone()])
    } else {
        Ok(())
    }
}

static MISSING_TLP_LABEL_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The CSAF document has no TLP label".to_string(),
    instance_path: "/document/distribution/tlp/label".to_string(),
});

crate::test_validation::impl_validator!(csaf2_0, ValidatorForTest6_2_10, test_6_2_10_missing_tlp_label);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;

    #[test]
    fn test_test_6_2_10() {
        let err = Err(vec![MISSING_TLP_LABEL_ERROR.clone()]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_10.expect(err);
    }
}
