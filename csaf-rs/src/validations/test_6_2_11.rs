use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use std::sync::LazyLock;

/// 6.2.11 Missing Canonical URL
///
/// `/document/references` must contain at least one item with:
/// - category = "self"
/// - url starts with "https://"
/// - url ends with the valid filename according to section 5.1
pub fn test_6_2_11_missing_canonical_url(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if !doc.get_document().has_canonical_url() {
        return Err(vec![MISSING_CANONICAL_URL.clone()]);
    }
    Ok(())
}

static MISSING_CANONICAL_URL: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "Document is missing a canonical URL".to_string(),
        instance_path: "/document/references".to_string(),
    })
});

crate::test_validation::impl_validator!(ValidatorForTest6_2_11, test_6_2_11_missing_canonical_url);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_11 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_11 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_11() {
        let err = Err(vec![MISSING_CANONICAL_URL.clone()]);
        let ok = Ok(());

        TESTS_2_0.test_6_2_11.expect(ExpectedResults_2_0 {
            case_01: err.clone(),
            case_11: ok.clone(),
        });

        // Failing test cases:
        // 01  - URL does not end with the canonical filename
        // 02  - invalid character in filename is not replaced
        // 03  - filename is not converted to lowercase
        // s01 - no references
        // s02 - external reference category instead of self
        // s03 - HTTP URL instead of HTTPS
        // s04 - no single reference satisfies all conditions
        // s05 - missing reference category
        // s06 - references with URL parameter and fragment

        // Valid test cases:
        // 11  - canonical URL
        // 12  - invalid character in URL path but outside the filename
        // 13  - canonical URL in a different path
        // s11 - one matching reference among non-matching references
        // s12 - multiple invalid character sequences in the tracking ID
        // s13 - multiple matching references
        TESTS_2_1.test_6_2_11.expect(ExpectedResults_2_1 {
            case_01: err.clone(),
            case_02: err.clone(),
            case_03: err.clone(),
            case_s01: err.clone(),
            case_s02: err.clone(),
            case_s03: err.clone(),
            case_s04: err.clone(),
            case_s05: err.clone(),
            case_s06: err,
            case_11: ok.clone(),
            case_12: ok.clone(),
            case_13: ok.clone(),
            case_s11: ok.clone(),
            case_s12: ok.clone(),
            case_s13: ok,
        });
    }
}
