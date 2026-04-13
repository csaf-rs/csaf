use crate::csaf_traits::{CsafTrait, CsafVersion, DistributionTrait, DocumentTrait};
use crate::validation::ValidationError;
use std::sync::LazyLock;

/// 6.2.10 Missing TLP label
///
/// `/document/distribution/tlp/label` must be set.
///
/// This test is obsolete in CSAF 2.1, as `distribution/tlp` is now required by the schema.
/// The test harness for CSAF 2.1 does not include the test.
/// If the test function was to be called programmatically on a CSAF 2.1 doc, we are returning
/// Ok(()). (later wasSkipped TODO)
pub fn test_6_2_10_missing_tlp_label(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // In CSAF 2.1 this field is mandatory and validated via the schema, so we skip this test
    if doc.get_document().get_csaf_version() == &CsafVersion::X21 {
        return Ok(()); // TODO #409 wasSkipped
    }
    // We just need to consider get_distribution_20 / get_tlp_20 here. If either is missing, return an error
    if doc
        .get_document()
        .get_distribution_20()
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

        // Case S11: A CSAF 2.0 document with a valid TLP label

        TESTS_2_0.test_6_2_10.expect(err, Ok(()));
    }

    /// Check that the test is skipped (returns Ok) for CSAF 2.1 documents.
    #[test]
    fn test_test_6_2_10_skipped_for_csaf_2_1() {
        let minimal_csaf_21: crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework =
            serde_json::from_value(serde_json::json!({
                "$schema": "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
                "document": {
                    "category": "csaf_base",
                    "csaf_version": "2.1",
                    "distribution": {
                        "tlp": { "label": "CLEAR" }
                    },
                    "publisher": {
                        "category": "other",
                        "name": "CSAF-RS Test Files",
                        "namespace": "https://github.com/csaf-rs/csaf/tree/main/type-generator/assets/tests"
                    },
                    "title": "Optional test: Missing TLP label (valid supplementary example 1 - skipped on CSAF 2.1)",
                    "tracking": {
                        "current_release_date": "2024-01-24T10:00:00.000Z",
                        "id": "CSAF-RS_CSAF-CSAF_2_1-6-2-10-S11",
                        "initial_release_date": "2024-01-24T10:00:00.000Z",
                        "revision_history": [{
                            "date": "2024-01-24T10:00:00.000Z",
                            "number": "1",
                            "summary": "Initial version."
                        }],
                        "status": "final",
                        "version": "1"
                    }
                }
            }))
            .expect("Failed to parse CSAF 2.1 document");

        let result = test_6_2_10_missing_tlp_label(&minimal_csaf_21);
        assert!(
            result.is_ok(),
            "Test 6.2.10 should be skipped (Ok) for CSAF 2.1 documents"
        );
    }
}
