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
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_11() {
        let err = Err(vec![MISSING_CANONICAL_URL.clone()]);
        let ok = Ok(());

        // CSAF 2.0 has 2 test cases
        TESTS_2_0.test_6_2_11.expect(err.clone(), ok.clone());

        // CSAF 2.1 has 6 OASIS and 8 supplementary test cases (14 in total).
        TESTS_2_1.test_6_2_11.expect(
            err.clone(), // 01
            err.clone(), // 02
            err.clone(), // 03
            err.clone(), // s01 - no references
            err.clone(), // s02 - external reference category instead of self
            err.clone(), // s03 - HTTP URL instead of HTTPS
            err.clone(), // s04 - no single reference satisfies all conditions
            err,         // s05 - missing reference category
            ok.clone(),  // 11
            ok.clone(),  // 12
            ok.clone(),  // 13
            ok.clone(),  // s11 - one matching reference among non-matching references
            ok.clone(),  // s12 - multiple invalid character sequences in tracking ID
            ok,          // s13 - multiple matching references
        );
    }
}
