use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait};
use crate::validation::ValidationError;
use std::sync::LazyLock;

/// 6.2.10 Missing TLP label
///
/// `/document/distribution/tlp/label` must be set.
pub fn test_6_2_10_missing_tlp_label(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // We just need to consider get_distribution_20 / get_tlp_20 here, in CSAF 2.1 this field is mandatory and
    // validated via the schema. This test will not run for CSAF 2.1 documents.
    if let Some(distribution_20) = doc.get_document().get_distribution_20() {
        if distribution_20.get_tlp_20().is_some() {
            return Ok(());
        }
    }
    Err(vec![MISSING_TLP_LABEL_ERROR.clone()])
}

static MISSING_TLP_LABEL_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The CSAF document has no TLP label".to_string(),
    instance_path: "/document/distribution/tlp/label".to_string(),
});

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_10
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_10_missing_tlp_label(doc)
    }
}

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
