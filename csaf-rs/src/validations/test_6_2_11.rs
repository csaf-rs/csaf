use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;
use std::sync::LazyLock;

/// 6.2.11 Missing Canonical URL
///
/// `/document/references` must contain at least one item with:
/// - category = "self"
/// - url starts with "https://"
/// - url ends with the valid filename according to section 5.1
pub fn test_6_2_11_missing_canonical_url(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    if !doc.get_document().has_canonical_url() {
        return Err(vec![MISSING_CANONICAL_URL.clone()]);
    }
    Ok(())
}

static MISSING_CANONICAL_URL: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Document is missing a canonical URL".to_string(),
    instance_path: "/document/references".to_string(),
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

        // CSAF 2.0 has 2 test cases
        TESTS_2_0.test_6_2_11.expect(ExpectedResults_2_0 {
            case_01: err.clone(),
            case_11: ok.clone(),
        });

        // CSAF 2.1 has 6 test cases
        TESTS_2_1.test_6_2_11.expect(ExpectedResults_2_1 {
            case_01: err.clone(),
            case_02: err.clone(),
            case_03: err,
            case_11: ok.clone(),
            case_12: ok.clone(),
            case_13: ok,
        });
    }
}
