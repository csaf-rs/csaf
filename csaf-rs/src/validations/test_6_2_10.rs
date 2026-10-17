use crate::schema::csaf2_0::schema::LabelOfTlp as LabelOfTlp20;
use crate::validation::{TestFinding, TestFindingData};
use serde_json::Value;
use std::sync::LazyLock;

/// 6.2.10 Missing TLP label
///
/// It MUST be tested that `/document/distribution/tlp/label` is present and valid.
///
/// This test is obsolete in CSAF 2.1, as `distribution/tlp` is now required by the schema.
/// The test harness for CSAF 2.1 does not include the test.
/// If the test function was to be called programmatically on a CSAF 2.1 doc, we are returning
/// Ok(()). (later wasSkipped)
pub fn test_6_2_10_missing_tlp_label(json: &Value) -> Result<(), Vec<TestFinding>> {
    // In CSAF 2.1 this field is mandatory and validated via the schema, so we skip this test
    if json.pointer("/document/csaf_version").and_then(Value::as_str) == Some("2.1") {
        return Ok(()); // TODO #409 wasSkipped
    }

    // if the distribution property is missing, the tlp label is missing
    let Some(distribution) = json.pointer("/document/distribution") else {
        return Err(vec![MISSING_TLP_LABEL_NO_DISTRIBUTION_ERROR.clone()]);
    };
    // if the tlp property is missing, the tlp label is missing
    let Some(tlp) = distribution.get("tlp") else {
        return Err(vec![MISSING_TLP_LABEL_NO_TLP_ERROR.clone()]);
    };
    // tlp label is missing
    let Some(label) = tlp.get("label") else {
        return Err(vec![MISSING_TLP_LABEL_NO_LABEL_ERROR.clone()]);
    };
    // tlp label is not a valid TLP 1.0 label
    if serde_json::from_value::<LabelOfTlp20>(label.clone()).is_err() {
        let value = label.as_str().map(str::to_owned).unwrap_or_else(|| label.to_string());
        return Err(vec![invalid_tlp_label_error(&value)]);
    }

    Ok(())
}

fn invalid_tlp_label_error(value: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The CSAF document has an invalid TLP label value `{value}`, expected one of: `WHITE`, `GREEN`, `AMBER`, `RED`."
        ),
        instance_path: "/document/distribution/tlp/label".to_string(),
    })
}

static MISSING_TLP_LABEL_NO_LABEL_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The CSAF document has no TLP label (there is no `label` property)".to_string(),
        instance_path: "/document/distribution/tlp".to_string(),
    })
});

static MISSING_TLP_LABEL_NO_TLP_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The CSAF document has no TLP label (there is no `tlp` property)".to_string(),
        instance_path: "/document/distribution".to_string(),
    })
});

static MISSING_TLP_LABEL_NO_DISTRIBUTION_ERROR: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The CSAF document has no TLP label (there is no `distribution` property)".to_string(),
        instance_path: "/document".to_string(),
    })
});

crate::test_validation::impl_raw_json_validator!(csaf2_0, ValidatorForTest6_2_10, test_6_2_10_missing_tlp_label);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_10 as ExpectedResults;
    use crate::csaf2_0::testcases::TESTS_2_0;

    #[test]
    fn test_test_6_2_10() {
        let err_no_label = Err(vec![MISSING_TLP_LABEL_NO_LABEL_ERROR.clone()]);
        let err_no_tlp = Err(vec![MISSING_TLP_LABEL_NO_TLP_ERROR.clone()]);
        let err_no_distribution = Err(vec![MISSING_TLP_LABEL_NO_DISTRIBUTION_ERROR.clone()]);
        let err_invalid_tlp_label = Err(vec![invalid_tlp_label_error("FOOBAR")]);
        let err_invalid_tlp_clear_label = Err(vec![invalid_tlp_label_error("CLEAR")]);
        let err_invalid_amber_strict_label = Err(vec![invalid_tlp_label_error("AMBER+STRICT")]);


        // Case S11-14: A CSAF 2.0 document with a valid TLP label WHITE/GREEN/AMBER/RED
        // Note: Case S02-S05 are schema-invalid
        
        TESTS_2_0.test_6_2_10.expect(ExpectedResults {
            case_01: err_no_tlp,
            case_s01: err_no_distribution,
            case_s02: err_no_label,
            case_s03: err_invalid_tlp_label,
            case_s04: err_invalid_tlp_clear_label,
            case_s05: err_invalid_amber_strict_label,
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });
    }

    /// Check that the test is skipped (returns Ok) for CSAF 2.1 documents.
    #[test]
    fn test_test_6_2_10_skipped_for_csaf_2_1() {
        let minimal_csaf_21 = serde_json::json!({
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
        });

        let result = test_6_2_10_missing_tlp_label(&minimal_csaf_21);
        assert!(
            result.is_ok(),
            "Test 6.2.10 should be skipped (Ok) for CSAF 2.1 documents"
        );
    }
}
